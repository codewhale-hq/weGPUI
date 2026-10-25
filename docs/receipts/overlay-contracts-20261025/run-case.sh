#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
case_name=${1:?case}
out=${2:?output directory}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bin=${BIN:-$CARGO_TARGET_DIR/debug/examples/overlays_contracts_probe}
mkdir -p "$out"
export DISPLAY=:103 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-overlay-contract.XXXXXX)
cleanup() {
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
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
"$bin" "$case_name" >"$out/app.log" 2>&1 &
app=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI overlay contracts' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
sleep .5
capture() {
  local label=$1 wait=${2:-.3}
  sleep "$wait"
  echo "$label" >> "$out/actions.log"
  xdotool key F8
  /usr/bin/python3 -I scripts/linux/atspi_dump.py >"$out/$label-atspi.txt"
  import -window "$wid" "$out/$label.png"
}
click_name() {
  local xy x y
  xy=$(/usr/bin/python3 -I "$here/geometry.py" "$1")
  read -r x y <<< "$xy"
  echo "click $1 at $x $y" >> "$out/actions.log"
  xdotool mousemove --window "$wid" "$x" "$y" click 1
}
open_overlay() {
  click_name 'Open overlay'
  sleep 1.1
}
key() { xdotool key "$1"; sleep .15; }
capture initial
open_overlay
capture opened
case "$case_name" in
  action|android)
    click_name 'Menu action'; capture menu
    click_name 'Menu with tips Secondary line'; capture tips
    click_name 'Warn action'; capture warn
    if [[ $case_name == action ]]; then
      click_name 'Cancel action'; capture cancel 1.1
      open_overlay
    fi
    xdotool mousemove --window "$wid" 10 10 click 1
    capture mask-dismiss 1.1
    open_overlay
    key Escape
    capture escape-dismiss 1.1
    ;;
  dialog)
    xdotool mousemove --window "$wid" 10 10 click 1
    capture mask-stays
    click_name 'Disabled dialog'; capture disabled-stays
    key Tab; capture cancel-focused
    key Tab; capture confirm-focused
    key Return; capture confirm-closed 1.1
    open_overlay
    xy=$(/usr/bin/python3 -I "$here/geometry.py" 'Confirm dialog')
    read -r x y <<< "$xy"
    xdotool mousemove --window "$wid" "$x" "$y" click --repeat 5 --delay 15 1
    capture rapid-confirm-closed 1.1
    open_overlay
    key Escape; capture escape-dismiss 1.1
    ;;
  raw-modal)
    xy=$(/usr/bin/python3 -I "$here/geometry.py" 'Raw confirm')
    read -r x y <<< "$xy"
    xdotool mousemove --window "$wid" "$x" "$y" click --repeat 5 --delay 15 1
    capture rapid-confirm-closed 1.1
    open_overlay
    xdotool mousemove --window "$wid" 10 10 click --repeat 5 --delay 15 1
    capture rapid-mask-closed 1.1
    ;;
  select)
    key End; capture end
    key Down; capture end-disabled-skipped
    key Home; capture home
    key Down; key Down; key Down; capture option4
    key Down; capture option6
    key Return; capture selected 1.1
    open_overlay
    capture reopened-selected
    key Escape; capture escape-cancel 1.1
    open_overlay
    xdotool mousemove --window "$wid" 10 10 click 1
    capture mask-cancel 1.1
    open_overlay
    click_name '取消'; capture cancel-button 1.1
    ;;
  select-disabled)
    click_name 'Disabled value'; capture disabled-click
    key Tab; capture disabled-tab-skipped
    ;;
  inline)
    key Tab; capture inline-action
    key Tab; capture inline-dialog
    key Tab; capture inline-icon
    key shift+Tab; capture inline-dialog-back
    key shift+Tab; capture inline-action-back
    ;;
  picker|reentrant)
    key Down; capture skip-disabled
    key End; capture last
    key Up; capture skip-back
    key Home; capture first
    key Return; capture confirmed 1.1
    if [[ $case_name == reentrant ]]; then
      key Escape; capture reopened-cancel 1.1
    else
      open_overlay
      key Escape; capture escape-cancel 1.1
      open_overlay
      xdotool mousemove --window "$wid" 240 400 click 5 key --delay 0 F9 F10
      capture wheel-close-reopen .5
      key Escape; capture final-cancel 1.1
    fi
    ;;
  all-disabled)
    key Return; capture cannot-confirm
    key Escape; capture cancel 1.1
    ;;
  date)
    key Tab; capture month
    key Down; capture march29
    key Tab; key End; capture march31
    key shift+Tab; key Up; capture feb29
    key shift+Tab; key End; capture max-year-feb28
    key Up; capture previous-year
    key Home; capture min-year
    key Return; capture confirmed 1.1
    ;;
  sheet)
    xdotool mousemove --window "$wid" 300 405 mousedown 1 mousemove --window "$wid" 300 435
    capture short-held .2
    xdotool mouseup 1
    capture short-return .02
    capture short-return-end .4
    xdotool mousemove --window "$wid" 300 405 mousedown 1 mousemove --window "$wid" 300 475
    capture long-held .2
    xdotool mouseup 1
    capture drag-dismissed 1.1
    open_overlay
    xdotool mousemove --window "$wid" 300 405 mousedown 1 mousemove --window "$wid" 300 475
    capture reopen-held .2
    xdotool mouseup 1 key --delay 0 F10
    capture rapid-reopen .02
    capture rapid-reopen-end .5
    key Escape; capture escape-dismiss 1.1
    ;;
  sheet-short)
    capture short-viewport
    key Escape; capture escape-dismiss 1.1
    ;;
esac
