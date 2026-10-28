import json, os, subprocess, time
from pathlib import Path

root = Path("/work/full-parity")
out = root / "overflow-geometry-probe"
result = root / "overflow-geometry-result.json"
result.unlink(missing_ok=True)
env = dict(os.environ, DISPLAY=":215", GPUI_X11_SCALE_FACTOR="2", LIBGL_ALWAYS_SOFTWARE="1", VK_ICD_FILENAMES="/usr/share/vulkan/icd.d/lvp_icd.json", XDG_RUNTIME_DIR="/work/runtime")
log = (out / "native.log").open("w")
xvfb = subprocess.Popen(["Xvfb", ":215", "-screen", "0", "2000x3200x24", "-nolisten", "tcp"], env=env, stdout=log, stderr=log)
wm = process = None
try:
    for _ in range(100):
        if xvfb.poll() is not None:
            raise RuntimeError("Xvfb exited; display may already be in use")
        if subprocess.run(["xdpyinfo", "-display", ":215"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
            break
        time.sleep(.05)
    else:
        raise RuntimeError("Xvfb not ready")
    wm = subprocess.Popen(["openbox", "--sm-disable"], env=env, stdout=log, stderr=log)
    process = subprocess.Popen(["/target/debug/wegpui-overflow-geometry-probe"], env=env, stdout=log, stderr=log)
    window = None
    for _ in range(400):
        if process.poll() is not None:
            raise RuntimeError("probe exited before completing")
        found = subprocess.run(["xdotool", "search", "--onlyvisible", "--pid", str(process.pid)], env=env, capture_output=True, text=True)
        if result.exists() and found.stdout.strip():
            window = found.stdout.splitlines()[0]
            break
        time.sleep(.05)
    if not window:
        raise RuntimeError("probe result or window not ready")
    subprocess.run(["import", "-window", window, str(out / "native.png")], env=env, check=True)
    data = json.loads(result.read_text())
    (out / "result.json").write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps({"passed": data["passed"], "observed_normal_overflow": data["observed_normal_overflow"], "cases": len(data["cases"])}, indent=2))
    if not data["passed"] or len(data["cases"]) != 12:
        raise SystemExit("Geometry probe did not pass; inspect result.json and native.log")
finally:
    for proc in (process, wm, xvfb):
        if proc is not None and proc.poll() is None:
            proc.terminate()
            proc.wait(timeout=5)
    log.close()
