#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/radio-set-probe
export DISPLAY=:220 GPUI_X11_SCALE_FACTOR=1 LIBGL_ALWAYS_SOFTWARE=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR=/work/runtime
python3 "$out/verify.py"
if xdpyinfo -display "$DISPLAY" >/dev/null 2>&1; then echo "Display already in use: $DISPLAY" >&2; exit 1; fi
Xvfb "$DISPLAY" -screen 0 1280x1200x24 -nolisten tcp >"$out/xvfb.log" 2>&1 &
xvfb=$!
trap 'kill "${wm:-}" "$xvfb" 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep .1; done
xdpyinfo -display "$DISPLAY" >/dev/null
openbox >"$out/openbox.log" 2>&1 &
wm=$!
dbus-run-session -- bash "$out/session.sh" 2>&1 | tee "$out/runtime.log"
python3 "$out/verify.py"
