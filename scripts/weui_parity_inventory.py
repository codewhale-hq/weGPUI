"""Inventory pinned WeUI styles/examples and enumerate the native paired-capture matrix."""

import argparse
import hashlib
import itertools
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[1]
FAMILIES = {
    "weui-agree": ["agree"], "weui-animate": ["modal"], "weui-button": ["button"],
    "weui-flex": ["flex"], "weui-footer": ["footer"], "weui-grid": ["grid"],
    "weui-loading": ["loading"], "weui-media-box": ["panel"], "weui-panel": ["panel"],
    "weui-picker": ["picker"], "weui-progress": ["progress"], "weui-searchbar": ["search_bar"],
    "weui-slider": ["slider"], "weui-steps": ["steps"], "weui-tab": ["navbar", "tabbar"],
    "weui-text": ["msg", "form", "footer", "half_screen_dialog"],
}
CELL_FILES = {
    "weui-access": ["cells", "select"], "weui-cell": ["cells"],
    "weui-cell_global": ["cells"], "weui-cell_swiped": ["swiped"],
    "weui-cells__group": ["cells", "form"], "weui-check": ["checkbox", "radio", "agree"],
    "weui-check_common": ["checkbox", "radio"], "weui-checkbox": ["checkbox", "agree"],
    "weui-radio": ["radio"], "weui-gallery": ["gallery"], "weui-switch": ["switch"],
    "weui-uploader": ["uploader"], "weui-cell-form": ["form", "input", "textarea"],
    "weui-form": ["form", "input", "textarea", "select", "vcode"],
    "weui-form_common": ["form", "input", "textarea"],
    "weui-form-preview": ["form_preview"], "weui-select": ["select"], "weui-vcode": ["vcode"],
}
PAGE_FILES = {"weui-article": ["article"], "weui-form": ["form"], "weui-msg": ["msg"]}
TIP_FILES = {
    "weui-actionsheet": ["action_sheet"], "weui-badge": ["badge"], "weui-dialog": ["dialog"],
    "weui-half-screen-dialog": ["half_screen_dialog"], "weui-information-bar": ["information_bar"],
    "weui-list-tips": ["information_bar"], "weui-loadmore": ["loadmore"], "weui-mask": ["modal"],
    "weui-toast": ["toast"], "weui-toptips": ["toptips"],
}


def modules_for_style(relative):
    """Map one widget LESS file to its native source modules; unknown families fail explicitly."""
    family = relative.parts[0]
    specific = {"weui-cell": CELL_FILES, "weui-page": PAGE_FILES, "weui-tips": TIP_FILES}
    if family in specific:
        modules = specific[family].get(relative.stem)
    else:
        modules = FAMILIES.get(family)
    if not modules:
        raise ValueError(f"Unclassified upstream widget style: {relative}")
    return modules


def capture_matrix(routes, states):
    """Enumerate every default/state scene in all eight theme/care/brand combinations."""
    scenes = [(route, None) for route in sorted(routes)]
    for key in sorted(states):
        route, separator, state = key.partition(":")
        if not separator or not state or route not in routes:
            raise ValueError(f"Replay does not name an upstream route/state: {key}")
        scenes.append((route, state))
    records = []
    for theme, care, brand in itertools.product(("light", "dark"), (False, True), (None, "7C3AED")):
        mode = "-".join([theme] + (["care"] if care else []) + (["brand"] if brand else []))
        for route, state in scenes:
            name = route + ("-" + state if state else "")
            records.append({"id": f"{mode}/{name}", "route": route, "state": state,
                            "theme": theme, "care": care, "brand": brand})
    if len({record["id"] for record in records}) != len(records):
        raise ValueError("Capture scene identifiers collide")
    return records


def inventory(upstream, reference):
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=upstream, text=True).strip()
    if revision != reference["revision"]:
        raise ValueError("Upstream checkout differs from the prepared reference")
    css_hash = hashlib.sha256((upstream / "dist/style/weui.css").read_bytes()).hexdigest()
    if css_hash != reference["css_sha256"]:
        raise ValueError("Upstream CSS differs from the reference")
    registry = dict(re.findall(r'\("([\w-]+)",\s*(\w+)::build\)',
                               (ROOT / "examples/gallery/pages/mod.rs").read_text()))
    browser = json.loads((ROOT / "website/playground/vendor/PROVENANCE.json").read_text())
    routes = {record["id"] for record in browser["routes"]}
    if set(registry) != routes | {"languages"}:
        raise ValueError("Native registry differs from upstream routes plus Languages")
    styles = []
    style_root = upstream / "src/style/widget"
    for path in sorted(style_root.rglob("*.less")):
        relative = path.relative_to(style_root)
        modules = modules_for_style(relative)
        for module in modules:
            source = ROOT / "src" / ("input.rs" if module == "input" else f"components/{module}.rs")
            if not source.is_file():
                raise ValueError(f"Missing native module for {relative}: {module}")
        styles.append({"source": f"src/style/widget/{relative.as_posix()}", "modules": modules,
                       "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                       "qualification": "mapped; runtime parity remains to be reviewed"})
    states = {}
    for path in sorted((ROOT / "scripts/parity").glob("*.json")):
        for key, replay in json.loads(path.read_text()).items():
            if key in states:
                raise ValueError(f"Duplicate state replay: {key}")
            states[key] = replay
    matrix = capture_matrix(routes, states)
    data = {"reference": reference, "upstream_examples": len(list((upstream / "src/example").rglob("*.html"))),
            "upstream_routes": sorted(routes), "native_only_routes": ["languages"], "styles": styles,
            "replays": states, "scene_count": len(routes) + len(states), "pair_count": len(matrix),
            "qualification": "source coverage and capture worklist; not component acceptance"}
    return data, matrix


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--reference", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    try:
        data, matrix = inventory(args.source, json.loads(args.reference.read_text()))
        if args.out.exists() and any(args.out.iterdir()):
            raise ValueError("Choose a new empty inventory directory")
        args.out.mkdir(parents=True, exist_ok=True)
        (args.out / "inventory.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
        (args.out / "matrix.jsonl").write_text("".join(json.dumps(row) + "\n" for row in matrix))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Parity inventory failed: {error}\n")
    print(f"{data['upstream_examples']} upstream HTML files; {len(data['styles'])} widget styles mapped; "
          f"{data['scene_count']} scenes; {len(matrix)} requested pairs")


if __name__ == "__main__":
    main()
