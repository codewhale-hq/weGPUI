#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- bash "$0" "$@"; fi
root=/workspace/weGPUI
here="$root/docs/receipts/media-contracts-20261025"
out="${1:-$here/runtime}"
mkdir -p "$out"
export DISPLAY=:104 GPUI_X11_SCALE_FACTOR=1
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-media.XXXXXX)
unset WAYLAND_DISPLAY
pids=()
cleanup() { for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done; }
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[0]}"
openbox >"$out/openbox.log" 2>&1 & pids+=("$!")
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.log"
bin="${MEDIA_BIN:-$CARGO_TARGET_DIR/debug/examples/media_contract_probe}"
{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  uname -srmo
  cat /etc/os-release
  rustc -Vv
  cargo -V
  printf 'MEDIA_CASE=%s MEDIA_ARGS=%s\n' "${MEDIA_CASE:-full}" "${MEDIA_ARGS:-}"
  git -C "$root" rev-parse HEAD
  sha256sum "$bin" "$root/Cargo.lock" "${MEDIA_SOURCE_FILE:-$root/examples/media_contract_probe.rs}"
} >"$out/provenance.txt"
(cd "$root"; git ls-files -z src examples Cargo.toml Cargo.lock | xargs -0 sha256sum) >"$out/source-sha256.txt"
read -ra app_args <<< "${MEDIA_ARGS:-}"
"$bin" "${app_args[@]}" >"$out/app.log" 2>&1 & app=$!; pids+=("$app")
sleep 7
kill -0 "$app"
window=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI' | head -1)
timeout 10 xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 0 0
capture() {
  local label="$1"
  kill -0 "$app"
  sleep 0.7
  import -window "$window" "$out/$label.png"
  python3 -I "$root/scripts/linux/atspi_dump.py" >"$out/$label-atspi.txt"
  python3 -I "$here/atspi_control.py" >"$out/$label.json"
  xdotool getwindowgeometry --shell "$window" >"$out/$label-geometry.txt"
  xwininfo -id "$window" >"$out/$label-xwininfo.txt"
}
click() {
  local coordinates
  coordinates=$(python3 -I "$here/atspi_control.py" "$@")
  read -r x y <<< "$coordinates"
  printf 'click %s (%s,%s)\n' "$*" "$x" "$y" | tee -a "$out/actions.log"
  xdotool mousemove --window "$window" "$x" "$y" click 1
  sleep 0.4
}
keys() {
  printf 'keys %s\n' "$*" | tee -a "$out/actions.log"
  for token in "$@"; do xdotool key "$token"; sleep 0.4; done
}
capture initial
if [[ ${MEDIA_CASE:-full} == snapshot ]]; then
  exit 0
fi
if [[ ${MEDIA_CASE:-full} == moved ]]; then
  printf 'move window to 200,120\n' | tee -a "$out/actions.log"
  xdotool windowmove "$window" 200 120
  capture moved
  exit 0
fi
if [[ ${MEDIA_CASE:-full} == a11y ]]; then
  click 'Add image'
  click '20%'
  capture progress-no-preview
  click 'Fail upload'
  click '错误'
  capture error-no-preview
  python3 -I "$here/atspi_control.py" --click 'Controlled row' | tee -a "$out/actions.log"
  capture a11y-open
  python3 -I "$here/atspi_control.py" --click 'Controlled row' | tee -a "$out/actions.log"
  capture a11y-close
  exit 0
fi
if [[ ${MEDIA_CASE:-full} == gallery ]]; then
  click '图片标题' image
  keys Tab
  capture image-focus
  keys Return
  capture image-dismiss
  click '图片标题' image
  keys Tab Tab
  capture delete-focus
  keys space
  capture gallery-delete
  exit 0
fi
if [[ ${MEDIA_CASE:-full} == focus ]]; then
  click 'Image 1' image
  keys Tab
  capture image-focus
  keys Tab
  capture delete-focus
  keys space
  capture delete-focus-result
  exit 0
fi
click 'Add image'
capture capacity-progress
click 'Fail upload'
capture error
click 'Complete upload'
capture done
click 'Image 1' image
capture thumbnail-preview
keys Tab Return
capture keyboard-dismiss
click 'Image 1' image
click 'Image preview' image
capture pointer-dismiss
click 'Image 1' image
keys Escape
capture escape-dismiss
click 'Image 1' image
# The upstream delete button fills the complete 60px operation bar.
read -r wx wy ww wh < <(xdotool getwindowgeometry --shell "$window" | python3 -c 'import sys; d=dict(line.strip().split("=",1) for line in sys.stdin); print(d["X"],d["Y"],d["WIDTH"],d["HEIGHT"])')
printf 'click delete-bar left edge (%s,%s)\n' "$((wx+10))" "$((wy+wh-30))" | tee -a "$out/actions.log"
xdotool mousemove "$((wx+10))" "$((wy+wh-30))" click 1
capture pointer-gallery-delete
click 'Image 2' image
keys Tab Tab space
capture keyboard-gallery-delete
click 'Add image'
click 'Complete upload'
click '删除' button
capture thumbnail-delete-empty
click 'Controlled row'
keys Left
capture swipe-open
keys Left
capture swipe-repeat-open
keys Tab
capture swipe-first-action
keys Escape
capture swipe-escape
keys Left Tab Return
capture swipe-action-callback
keys Left
click 'Delete row'
capture swipe-delete-callback
click 'Toggle request acceptance'
click 'Controlled row'
keys Left
capture swipe-rejected
click 'Toggle request acceptance'
read -r sx sy < <(python3 -I "$here/atspi_control.py" 'Controlled row')
printf 'drag left from (%s,%s)\n' "$((sx+100))" "$sy" | tee -a "$out/actions.log"
xdotool mousemove --window "$window" "$((sx+100))" "$sy" mousedown 1
for delta in 30 60 90 120 150 180 210 240 270 300; do
  xdotool mousemove --window "$window" "$((sx+100-delta))" "$sy"
  sleep 0.08
done
xdotool mouseup 1
capture swipe-drag-open
keys Right
capture swipe-right-close
