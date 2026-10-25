#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- "$0" "$@"; fi
cd /workspace/weGPUI
out=${OUT:-/workspace/weGPUI/docs/receipts/input-contracts-20261025/scale-1}
mkdir -p "$out"
cp docs/receipts/input-contracts-20261025/runtime.py "$out/runtime-script.py"
export OUT="$out" DISPLAY=:102 GPUI_X11_SCALE_FACTOR="${GPUI_X11_SCALE_FACTOR:-1}"
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-input-contracts.XXXXXX)
pids=()
cleanup() {
  for ((i=${#pids[@]}-1; i>=0; i--)); do kill "${pids[i]}" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1600x1800x24 >"$out/xvfb.log" 2>&1 &
pids+=("$!")
sleep 1
openbox >"$out/openbox.log" 2>&1 &
pids+=("$!")
sleep 1
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/a11y-bus.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/a11y-enable.txt"
binary="$CARGO_TARGET_DIR/debug/examples/input_contracts_probe"
{
 date -u +%FT%TZ
 git rev-parse HEAD
 git diff -- src/input.rs src/components/textarea.rs | sha256sum
 sha256sum src/input.rs src/components/textarea.rs Cargo.toml Cargo.lock
 git status --short
 sha256sum "$binary"
 rustc -Vv
 uname -a
 cat /etc/os-release
 echo "DISPLAY=$DISPLAY GPUI_X11_SCALE_FACTOR=$GPUI_X11_SCALE_FACTOR"
} >"$out/provenance.txt"
"$binary" >"$out/application.log" 2>&1 &
app=$!
pids+=("$app")
export PROBE_WINDOW=
for attempt in {1..100}; do
 PROBE_WINDOW=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI input contracts probe' 2>/dev/null | head -1 || true)
 [[ -z $PROBE_WINDOW ]] || break
 kill -0 "$app"
 sleep .1
done
[[ -n $PROBE_WINDOW ]]
xdotool windowactivate --sync "$PROBE_WINDOW"
sleep 2
/usr/bin/python3 docs/receipts/input-contracts-20261025/runtime.py >"$out/runtime.log" 2>&1
/usr/bin/python3 scripts/linux/atspi_dump.py >"$out/final-atspi.txt" 2>&1
