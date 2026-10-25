#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
receipt=/workspace/weGPUI/docs/receipts/data-contracts-20261025
out=$(mktemp -d "$receipt/run-$(date -u +%Y%m%dT%H%M%SZ)-XXXXXX")
cp "$receipt/action.py" "$receipt/session.sh" "$receipt/events.py" "$receipt/validate.py" "$receipt/vendor_manifest.py" "$receipt/source_manifest.py" "$out/"
echo "Evidence: $out"
export DISPLAY=:106 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/data-contracts-runtime.XXXXXX)
Xvfb "$DISPLAY" -screen 0 1280x1200x24 >"$out/xvfb.log" 2>&1 &
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
dbus-run-session -- bash "$out/session.sh" 2>&1 | tee "$out/runtime.log"
