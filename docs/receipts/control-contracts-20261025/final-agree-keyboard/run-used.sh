#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- bash "$0" "$@"; fi
out=$1
page=$2
shift 2
mkdir -p "$out"
cp docs/receipts/control-contracts-20261025/drive.py "$out/drive-used.py"
cp docs/receipts/control-contracts-20261025/run-v2.sh "$out/run-used.sh"
export DISPLAY=${CONTROL_DISPLAY:-:107} GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-controls.XXXXXX)
pids=()
cleanup() {
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  for pid in "${pids[@]}"; do wait "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1100x24 >"$out/xvfb.log" 2>&1 &
pids+=("$!")
for _ in {1..50}; do xdpyinfo >/dev/null 2>&1 && break; sleep .1; done
xdpyinfo >/dev/null
openbox >"$out/openbox.log" 2>&1 &
pids+=("$!")
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/a11y-bus.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/a11y-enable.txt"
binary=${CONTROL_BINARY:-"$CARGO_TARGET_DIR/debug/examples/control_contracts"}
{
  date -u +%FT%TZ
  git rev-parse HEAD
  git diff -- src/components/{button,agree,checkbox,radio,switch,vcode,icons}.rs | sha256sum
  printf "binary=%s\n" "$binary"
  printf "source_root=%s\n" "${CONTROL_SOURCE_ROOT:-$(dirname "$binary")}"
  sha256sum "$binary" examples/control_contracts.rs "$out/drive-used.py" "$out/run-used.sh"
  rustc -Vv
  cat /etc/os-release
  printf 'page=%s args=' "$page"; printf ' %q' "$@"; printf '\n'
} >"$out/provenance.txt"
actual_page=$page
[[ $page != switch-motion ]] || actual_page=choices
[[ $page != agree-regression ]] || actual_page=agree
"$binary" --page "$actual_page" "$@" >"$out/application.log" 2>&1 &
app=$!
pids+=("$app")
wid=
for _ in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI control contracts' 2>/dev/null | head -1) || true
  [[ -z $wid ]] || break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
sleep 1
import -window "$wid" "$out/initial.png"
/usr/bin/python3 -I scripts/linux/atspi_dump.py >"$out/initial-atspi.txt"
/usr/bin/python3 -I "$out/drive-used.py" "$out" "$page" "$wid" >"$out/result.log" 2>&1
