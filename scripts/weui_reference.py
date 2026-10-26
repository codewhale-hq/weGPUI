"""Prepare an immutable WeUI UI reference without its unused WeChat SDK dependency."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess


ROOT = Path(__file__).resolve().parents[1]
REVISION = "7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e"
VERSION = "2.6.26"
CSS_SHA256 = "8db42592f96d571568c49ae1b8484f55042e6a6dab25f8144460cd4a23530fd3"
JS_SHA256 = "87707d2d625c95f247e1957d5eee8c43bbe0c056983336990868f325d9b0c6fe"
SDK_TAG = '<script type="text/javascript" src="https://res.wx.qq.com/open/js/jweixin-1.0.0.js"></script>'
JS_URL = "https://res.wx.qq.com/t/wx_fed/weui.js/res/1.2.17/weui.min.js"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def prepare(source, output, javascript=None):
    """Copy pinned upstream dist assets, remove the unused SDK tag and localise pinned weui.js."""
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=source, text=True).strip()
    if revision != REVISION:
        raise ValueError(f"Expected upstream {REVISION}; got {revision}")
    if subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=no"], cwd=source):
        raise ValueError("Upstream checkout has modified tracked files")
    if json.loads((source / "package.json").read_text())["version"] != VERSION:
        raise ValueError("Upstream package version differs from the parity target")
    css = (source / "dist/style/weui.css").read_bytes()
    if sha256(css) != CSS_SHA256:
        raise ValueError("Upstream CSS differs from the pinned distribution")
    javascript = javascript or ROOT / "website/playground/vendor/weui.min.js"
    script = javascript.read_bytes()
    if sha256(script) != JS_SHA256:
        raise ValueError("weui.js differs from the pinned 1.2.17 runtime")
    original = (source / "dist/example/index.html").read_text()
    if original.count(SDK_TAG) != 1 or original.count(f'src="{JS_URL}"') != 1:
        raise ValueError("Expected exactly the two pinned external script tags")
    inline_scripts = "\n".join(re.findall(r"<script\b[^>]*>(.*?)</script>", original, re.S))
    if re.search(r"\bwx\s*(?:\.|\[)", inline_scripts):
        raise ValueError("Example uses the WeChat SDK; it cannot be omitted for this reference")
    license_text = (source / "LICENSE.txt").read_bytes()
    if output.exists() and any(output.iterdir()):
        raise ValueError("Choose a new empty reference directory")
    output.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source / "dist", output / "dist")
    (output / "vendor").mkdir()
    (output / "vendor/weui.min.js").write_bytes(script)
    (output / "vendor/LICENSE-WEUI").write_bytes(license_text)
    modified = original.replace(SDK_TAG, "").replace(JS_URL, "../../vendor/weui.min.js")
    (output / "dist/example/index.html").write_text(modified)
    metadata = {
        "upstream": "https://github.com/Tencent/weui",
        "revision": revision,
        "version": VERSION,
        "scope": "WeUI CSS, example markup and weui.js UI behaviour; WeChat SDK behaviour excluded",
        "css_sha256": CSS_SHA256,
        "weui_js_sha256": JS_SHA256,
        "original_index_sha256": sha256(original.encode()),
        "reference_index_sha256": sha256(modified.encode()),
        "changes": ["Remove unused WeChat SDK script tag", "Use byte-identical local weui.js 1.2.17"],
    }
    (output / "REFERENCE.json").write_text(json.dumps(metadata, indent=2) + "\n")
    return metadata


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    try:
        metadata = prepare(args.source.resolve(), args.out.resolve())
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Reference preparation failed: {error}\n")
    print(json.dumps(metadata, indent=2))


if __name__ == "__main__":
    main()
