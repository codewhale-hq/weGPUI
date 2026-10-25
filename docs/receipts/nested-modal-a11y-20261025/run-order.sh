#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
out=${OUT:?set OUT to the capture directory}
mkdir -p "$out"
export DISPLAY=:93 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-modal-order.XXXXXX)
cleanup() {
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
  rmdir "$XDG_RUNTIME_DIR" 2>/dev/null || true
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
for attempt in {1..50}; do xdpyinfo >/dev/null 2>&1 && break; sleep .1; done
openbox >"$out/openbox.log" 2>&1 &
wm=$!
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi.log" 2>&1 &
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >/dev/null
"$CARGO_TARGET_DIR/debug/examples/modal_order_probe" >"$out/app.log" 2>&1 &
app=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI modal order probe' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
sleep 1
snapshot() {
  sleep .5
  /usr/bin/python3 -I scripts/linux/atspi_dump.py >"$out/$1-atspi.txt"
  import -window "$wid" "$out/$1.png"
}
snapshot all-open
xdotool key Tab
snapshot b-cancel
xdotool key Tab
snapshot b-ok
xdotool key Escape
snapshot c-visible
xdotool key Tab
snapshot c-focused
xdotool key Escape
snapshot a-visible
xdotool key Escape
snapshot page
