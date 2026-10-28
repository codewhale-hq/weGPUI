#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/vcode-deadline-probe
export DISPLAY=:219 GPUI_X11_SCALE_FACTOR=1 LIBGL_ALWAYS_SOFTWARE=1 VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json XDG_RUNTIME_DIR=/work/runtime
unset WAYLAND_DISPLAY
if xdpyinfo -display "$DISPLAY" >/dev/null 2>&1; then echo 'display already in use' >&2; exit 1; fi
Xvfb "$DISPLAY" -screen 0 1024x768x24 -nolisten tcp >"$out/xvfb.log" 2>&1 & xvfb=$!
trap 'kill "${wm:-}" "$xvfb" 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep .1; done
xdpyinfo -display "$DISPLAY" >/dev/null
openbox >"$out/openbox.log" 2>&1 & wm=$!
dbus-run-session -- bash "$out/session.sh" 2>&1 | tee "$out/runtime.log"
