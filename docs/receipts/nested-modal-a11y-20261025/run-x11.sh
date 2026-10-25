#!/usr/bin/env bash
# Run inside wegpui-198 under dbus-run-session after a separate all-target build.
set -euo pipefail
cd /workspace/weGPUI
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${OUT:-/workspace/weGPUI/docs/receipts/nested-modal-a11y-20261025}
mkdir -p "$out"
export DISPLAY=:93 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-nested-a11y.XXXXXX)
cleanup() {
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
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
"$CARGO_TARGET_DIR/debug/examples/nested_modals" >"$out/app.log" 2>&1 &
app=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI nested modals' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
xdotool getwindowgeometry --shell "$wid" >"$out/window-geometry.txt"
sleep 1
snapshot() {
  local label=$1
  sleep .6
  /usr/bin/python3 -I scripts/linux/atspi_dump.py >"$out/$label-atspi.txt"
  import -window "$wid" "$out/$label.png"
}
click_button() {
  local xy
  xy=$(/usr/bin/python3 -I "$here/click-button.py" "$1")
  read -r x y <<< "$xy"
  echo "$1 $xy" >> "$out/clicks.log"
  xdotool mousemove --window "$wid" "$x" "$y" click 1
}
snapshot initial
click_button 'Open sheet'
snapshot outer
xdotool key Tab
snapshot outer-tab
xdotool key Return
snapshot inner
xdotool key Tab
snapshot inner-cancel
xdotool key Tab
snapshot inner-ok
xdotool key Tab
snapshot inner-wrap
xdotool key shift+Tab
snapshot inner-reverse
xdotool key Escape
snapshot outer-restored
xdotool key Escape
snapshot page-restored
kill "$app"
wait "$app" || true
"$CARGO_TARGET_DIR/debug/examples/gallery" --page toast --state success --size 480x420 \
  --position 0,0 --reduce-motion >"$out/toast-app.log" 2>&1 &
app=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name weGPUI 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
sleep 1
snapshot toast-active
xdotool key Tab
snapshot toast-tab
/usr/bin/python3 -I "$here/validate.py" "$out"
