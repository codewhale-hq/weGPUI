"""Capture a frozen native gallery against a prepared WeUI reference on Linux X11."""

import argparse
import atexit
from concurrent.futures import ProcessPoolExecutor
from functools import partial
import hashlib
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from importlib.metadata import version
import json
import multiprocessing
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import time
from threading import Thread


ROOT = Path(__file__).resolve().parents[1]
WORKER = {}


class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_):
        pass


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_records(records, inventory):
    if not records:
        raise ValueError("Choose a nonempty matrix")
    identifiers = set()
    for record in records:
        identifier = record.get("id", "")
        if not isinstance(identifier, str) or not re.fullmatch(r"[\w-]+/[\w-]+", identifier, re.ASCII):
            raise ValueError("Invalid scene identifier")
        if identifier in identifiers:
            raise ValueError("Duplicate scene identifiers would overwrite capture evidence")
        identifiers.add(identifier)
        route, state = record["route"], record["state"]
        theme, care, brand = record["theme"], record["care"], record["brand"]
        if route not in inventory["upstream_routes"] or (state is not None and
                (not isinstance(state, str) or not state or route + ":" + state not in inventory["replays"])):
            raise ValueError("Unknown route or state in capture matrix")
        if theme not in {"light", "dark"} or not isinstance(care, bool):
            raise ValueError("Invalid theme or care flag in capture matrix")
        if brand is not None and (not isinstance(brand, str) or not re.fullmatch(r"[0-9A-Fa-f]{6}", brand)):
            raise ValueError("Brand must contain six hexadecimal digits")
        mode = "-".join([theme] + (["care"] if care else []) + (["brand"] if brand else []))
        scene = route + ("-" + state if state else "")
        if identifier != mode + "/" + scene:
            raise ValueError("Scene identifier differs from its route, state or appearance")


def worker_init(binary, output, url, replays, display_base, font_family, width, height, scroll):
    from playwright.sync_api import sync_playwright

    index = multiprocessing.current_process()._identity[-1] - 1
    display = f":{display_base + index}"
    environment = dict(os.environ, DISPLAY=display, GPUI_X11_SCALE_FACTOR="2", LP_NUM_THREADS="2")
    environment.pop("WAYLAND_DISPLAY", None)
    logs = (Path(output) / f"worker-{index}.log").open("w")
    xvfb = subprocess.Popen(["Xvfb", display, "-screen", "0", "2400x2100x24", "-nolisten", "tcp"],
                            env=environment, stdout=logs, stderr=logs)
    for _ in range(100):
        if subprocess.run(["xdpyinfo", "-display", display], env=environment,
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
            break
        time.sleep(.05)
    else:
        raise RuntimeError(f"X display did not start: {display}")
    manager = subprocess.Popen(["openbox", "--sm-disable"], env=environment, stdout=logs, stderr=logs)
    playwright = sync_playwright().start()
    browser = playwright.chromium.launch(executable_path="/usr/bin/chromium")
    WORKER.update(binary=binary, output=Path(output), url=url, replays=replays, environment=environment,
                  browser=browser, font_family=font_family, width=width, height=height, scroll=scroll)

    def cleanup():
        browser.close()
        playwright.stop()
        manager.terminate()
        xvfb.terminate()
        logs.close()

    atexit.register(cleanup)


def capture(record):
    from PIL import Image, ImageChops, ImageStat

    started = time.monotonic()
    output = WORKER["output"] / record["id"]
    output.parent.mkdir(parents=True, exist_ok=True)
    native_path = output.with_name(output.name + "-gpui.png")
    reference_path = output.with_name(output.name + "-weui.png")
    command = [WORKER["binary"], "--page", record["route"], "--lang", "zh-Hans",
               "--theme", record["theme"], "--size", f"{WORKER['width']}x{WORKER['height']}",
               "--position", "60,60", "--reduce-motion"]
    if record["state"]:
        command += ["--state", record["state"]]
    if record["care"]:
        command += ["--care"]
    if record["brand"]:
        command += ["--brand", record["brand"]]
    if WORKER["font_family"]:
        command += ["--font-family", WORKER["font_family"]]
    if WORKER["scroll"]:
        command += ["--scroll", str(WORKER["scroll"])]
    process = None
    context = None
    result = dict(record, viewport=[WORKER["width"], WORKER["height"]], scroll=WORKER["scroll"])
    try:
        with output.with_name(output.name + "-native.log").open("w") as native_log:
            process = subprocess.Popen(command, env=WORKER["environment"], stdout=native_log, stderr=native_log)
            context = WORKER["browser"].new_context(viewport={"width": WORKER["width"], "height": WORKER["height"]},
                                                   device_scale_factor=2, color_scheme=record["theme"])
            page = context.new_page()
            page.set_default_timeout(15000)
            errors = []
            page.on("pageerror", lambda error: errors.append(str(error)))
            page.goto(WORKER["url"] + "#" + record["route"], wait_until="load")
            if record["care"]:
                page.evaluate("document.body.setAttribute('data-weui-mode', 'care')")
            if record["brand"]:
                page.evaluate("color => document.body.style.setProperty('--weui-BRAND', '#' + color)", record["brand"])
            if record["state"]:
                replay = WORKER["replays"][record["route"] + ":" + record["state"]]
                page.evaluate("(async () => {" + replay + "})()")
            window = None
            for _ in range(200):
                found = subprocess.run(
                    ["xdotool", "search", "--all", "--onlyvisible", "--pid", str(process.pid), "--name", "weGPUI"],
                    env=WORKER["environment"], capture_output=True, text=True,
                )
                if found.stdout.strip():
                    window = found.stdout.split()[0]
                    break
                if process.poll() is not None:
                    raise RuntimeError(f"Native process exited: {process.returncode}")
                time.sleep(.05)
            if window is None:
                raise RuntimeError("Native window was not found")
            page.wait_for_timeout(5000 if record["route"] == "layers" else 1200)
            if WORKER["scroll"]:
                scroll = page.evaluate("""y => {
                    const page = Array.from(document.querySelectorAll('#container > .page')).at(-1);
                    if (!page) throw new Error('Active reference page was not found');
                    page.scrollTop = y;
                    return {top: page.scrollTop, height: page.scrollHeight, viewport: page.clientHeight};
                }""", WORKER["scroll"])
                result["reference_scroll"] = scroll
                page.wait_for_timeout(300)
            page.screenshot(path=str(reference_path))
            subprocess.run(["import", "-window", window, str(native_path)],
                           env=WORKER["environment"], check=True, capture_output=True)
        native = Image.open(native_path).convert("RGB")
        reference = Image.open(reference_path).convert("RGB")
        if native.size != (WORKER["width"] * 2, WORKER["height"] * 2) or native.size != reference.size:
            raise ValueError(f"Capture sizes differ: {native.size}, {reference.size}")
        difference = ImageChops.difference(native, reference)
        channels = difference.split()
        maximum = ImageChops.lighter(ImageChops.lighter(channels[0], channels[1]), channels[2])
        mask = maximum.point(lambda value: 255 if value > 8 else 0)
        histogram = mask.histogram()
        result.update(status="captured", pixels_over_8_percent=round(100 * histogram[255] / (native.width * native.height), 4),
                      mean_rgb_difference=round(sum(ImageStat.Stat(difference).mean) / 3, 4),
                      difference_bbox=mask.getbbox(), native_sha256=sha256(native_path),
                      reference_sha256=sha256(reference_path), reference_errors=errors,
                      qualification="unreviewed paired capture; scores are not parity verdicts")
        if errors:
            raise RuntimeError("Reference page errors: " + "; ".join(errors))
    except Exception as error:
        result.update(status="failed", error=str(error))
    finally:
        if context is not None:
            context.close()
        if process is not None:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
    result["seconds"] = round(time.monotonic() - started, 3)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix", required=True, type=Path)
    parser.add_argument("--inventory", required=True, type=Path)
    parser.add_argument("--reference", required=True, type=Path)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--reuse-frozen-binary", action="store_true", help="Hard-link an already read-only frozen executable")
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--workers", type=int, default=4, choices=range(1, 5))
    parser.add_argument("--routes", help="Comma-separated development subset")
    parser.add_argument("--font-family", help="Explicit native font for controlled-font comparisons; reference CSS stays unchanged")
    parser.add_argument("--width", type=int, default=375)
    parser.add_argument("--height", type=int, default=812)
    parser.add_argument("--scroll", type=int, default=0)
    parser.add_argument("--display-base", type=int, default=80, choices=range(20, 500))
    args = parser.parse_args()
    if not (160 <= args.width <= 1100 and 320 <= args.height <= 1000 and 0 <= args.scroll <= 100000):
        parser.error("Width must be 160..1100, height 320..1000 and scroll 0..100000")
    if args.reuse_frozen_binary and args.binary.stat().st_mode & 0o222:
        parser.error("--reuse-frozen-binary requires a read-only executable")
    records = [json.loads(line) for line in args.matrix.read_text().splitlines() if line]
    inventory = json.loads(args.inventory.read_text())
    reference = json.loads((args.reference / "REFERENCE.json").read_text())
    if reference != inventory["reference"]:
        parser.error("Reference metadata differs from the inventory")
    for path, key in [("dist/style/weui.css", "css_sha256"), ("vendor/weui.min.js", "weui_js_sha256"),
                      ("dist/example/index.html", "reference_index_sha256")]:
        if sha256(args.reference / path) != reference[key]:
            parser.error(f"Reference asset changed: {path}")
    try:
        validate_records(records, inventory)
    except (ValueError, KeyError, TypeError, AttributeError) as error:
        parser.error(f"Invalid capture matrix: {error}")
    if args.routes:
        requested = set(args.routes.split(","))
        if not requested <= set(inventory["upstream_routes"]):
            parser.error("Unknown route in --routes")
        records = [row for row in records if row["route"] in requested]
    if not records or (args.out.exists() and any(args.out.iterdir())):
        parser.error("Choose a nonempty matrix and a new empty output directory")
    args.out.mkdir(parents=True, exist_ok=True)
    binary = args.out / "gallery-frozen"
    if args.reuse_frozen_binary:
        os.link(args.binary, binary)
    else:
        shutil.copyfile(args.binary, binary)
        binary.chmod(0o555)
    expected_count = (len(inventory["upstream_routes"]) + len(inventory["replays"])) * 8
    complete = (not args.routes and len(records) == expected_count == inventory["pair_count"]
                and all(record["brand"] in {None, "7C3AED"} for record in records))
    manifest = {"base_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                "binary_sha256": sha256(binary), "reference": inventory["reference"],
                "expected_pairs": len(records), "full_matrix": complete,
                "language": "zh-Hans", "viewport": [args.width, args.height], "scale": 2, "scroll": args.scroll,
                "native_font_family": args.font_family or ".SystemUIFont",
                "capture_script_sha256": sha256(Path(__file__)),
                "matrix_sha256": sha256(args.matrix), "inventory_sha256": sha256(args.inventory),
                "host": {"system": platform.system(), "architecture": platform.machine(),
                         "python": platform.python_version(), "playwright": version("playwright"),
                         "pillow": version("Pillow"),
                         "chromium": subprocess.check_output(["/usr/bin/chromium", "--version"], text=True).strip(),
                         "system_ui_font": subprocess.check_output(["fc-match", "system-ui"], text=True).strip()},
                "qualification": "capture completeness only; manual difference review required"}
    sources = [p for folder in ["src", "examples", "vendor"] for p in (ROOT / folder).rglob("*.rs")]
    sources += [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
    manifest["source_sha256"] = {p.relative_to(ROOT).as_posix(): sha256(p) for p in sorted(sources)}
    (args.out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    shutil.copyfile(args.matrix, args.out / "requested-matrix.jsonl")
    server = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(args.reference)))
    Thread(target=server.serve_forever, daemon=True).start()
    url = f"http://127.0.0.1:{server.server_port}/dist/example/index.html"
    results = []
    try:
        with ProcessPoolExecutor(max_workers=args.workers, mp_context=multiprocessing.get_context("spawn"),
                                 initializer=worker_init,
                                 initargs=(str(binary.resolve()), str(args.out.resolve()), url, inventory["replays"], args.display_base,
                                           args.font_family, args.width, args.height, args.scroll)) as executor:
            with (args.out / "results.jsonl").open("w") as output:
                for result in executor.map(capture, records, chunksize=1):
                    results.append(result)
                    output.write(json.dumps(result) + "\n")
                    output.flush()
                    print(f"{len(results)}/{len(records)} {result['status']} {result['id']}", flush=True)
    finally:
        server.shutdown()
    summary = {"expected": len(records), "captured": sum(r["status"] == "captured" for r in results),
               "failed": [r for r in results if r["status"] != "captured"],
               "full_matrix": manifest["full_matrix"], "qualification": manifest["qualification"]}
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    if summary["failed"] or len(results) != len(records):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
