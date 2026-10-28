#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/slider-contract-probe
export DISPLAY=:218 GPUI_X11_SCALE_FACTOR=1 LIBGL_ALWAYS_SOFTWARE=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR=/work/runtime
Xvfb "$DISPLAY" -screen 0 1280x1200x24 -nolisten tcp >"$out/xvfb.log" 2>&1 &
xvfb=$!
trap 'kill "${wm:-}" "$xvfb" 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep .1; done
openbox >"$out/openbox.log" 2>&1 &
wm=$!
dbus-run-session -- bash "$out/session.sh" 2>&1 | tee "$out/runtime.log"
