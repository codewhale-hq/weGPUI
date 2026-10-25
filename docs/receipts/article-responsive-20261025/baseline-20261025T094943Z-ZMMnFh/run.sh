#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
receipt=/workspace/weGPUI/docs/receipts/article-responsive-20261025
out=$(mktemp -d "$receipt/${1:-check}-$(date -u +%Y%m%dT%H%M%SZ)-XXXXXX")
cp "$receipt/run.sh" "$receipt/verify.py" "$receipt/atspi_metrics.py" "$out/"
artifact=$(mktemp -d /workspace/.wegpui-cloud/artifacts/article-responsive.XXXXXX)
cp /workspace/.wegpui-cloud/target-198/debug/examples/gallery "$artifact/gallery"
cp examples/gallery/pages/article.rs "$out/article.rs"
cp Cargo.toml Cargo.lock "$out/"
git diff -- src examples Cargo.toml Cargo.lock >"$out/source.diff"
{
  date -u +%FT%TZ
  git rev-parse HEAD
  sha256sum "$artifact/gallery"
  rustc -Vv
  chromium --version
  /opt/parity-python/bin/python -m pip show playwright pillow
} >"$out/provenance.txt"
echo "Evidence: $out"
export DISPLAY=:106 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/article-responsive-runtime.XXXXXX)
Xvfb "$DISPLAY" -screen 0 1280x2000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
cleanup() {
  kill "${wm:-}" "$xvfb" 2>/dev/null || true
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
sleep 1
openbox >"$out/openbox.log" 2>&1 &
wm=$!
sleep 1
dbus-run-session -- /opt/parity-python/bin/python -I "$out/verify.py" "$out" "$artifact/gallery" 2>&1 | tee "$out/runtime.log"
