#!/usr/bin/env bash
# Actual X11 consumer process, isolated from gallery sessions. No Cargo invocation.
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then
  exec dbus-run-session -- bash "$0"
fi
root=/workspace/weGPUI
out="$root/docs/receipts/component-contracts-20261025/consumer/runtime"
bin=/workspace/.wegpui-cloud/artifacts/consumer-contracts/wegpui-preview-consumer
export DISPLAY=:111 GPUI_X11_SCALE_FACTOR=1
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-consumer.XXXXXX)
unset WAYLAND_DISPLAY
pids=()
cleanup() {
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  uname -srmo
  cat /etc/os-release
  rustc -Vv
  cargo -V
  dpkg-query -W xvfb openbox mesa-vulkan-drivers libgl1-mesa-dri python3-pyatspi
  printf 'Runtime: X11 DISPLAY=:111 GPUI_X11_SCALE_FACTOR=1; no WAYLAND_DISPLAY\n'
  printf 'Command: %s\n' "$bin"
  git -C "$root" rev-parse HEAD
  git -C "$root" status --short
  sha256sum "$bin" "$out/../Cargo.toml.txt" "$out/../Cargo.lock.txt" "$out/../src/main.rs.txt" "$out/../src/support/platform.rs.txt"
  printf 'The binary predates the pending focus/contrast source edits; current working-tree state is diagnostic only.\n'
} >"$out/provenance.txt" 2>&1
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[0]}"
openbox >"$out/openbox.log" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[1]}"
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.log"
"$bin" >"$out/app.log" 2>&1 & app=$!; pids+=("$app")
sleep 7
kill -0 "$app"
window=$(xdotool search --onlyvisible --pid "$app" | head -1)
timeout 10 xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 0 0
xwininfo -id "$window" >"$out/window.txt"
capture() {
  local label="$1"
  kill -0 "$app"
  sleep 1
  import -window "$window" "$out/$label.png"
  python3 -I "$root/scripts/linux/atspi_dump.py" >"$out/$label-atspi.txt"
  python3 -I "$out/atspi_control.py" >"$out/$label.json"
}
click() {
  local coordinates x y
  coordinates=$(python3 -I "$out/atspi_control.py" "$1")
  read -r x y <<< "$coordinates"
  printf 'pointer %s @%s,%s relative to client\n' "$1" "$x" "$y" >>"$out/actions.log"
  xdotool mousemove --window "$window" "$x" "$y" click 1
}
keys() {
  printf 'keyboard %s\n' "$*" >>"$out/actions.log"
  for token in "$@"; do xdotool key "$token"; sleep 0.4; done
}
capture initial
click 'Open dialog'; capture opened
keys Tab; capture cancel
keys Tab; capture ok
keys Tab; capture wrap
keys shift+Tab; capture back
keys Return; capture confirmed
keys Return; capture reopen-keyboard
keys Escape; capture close-keyboard
click 'Open dialog'; capture reopen-pointer
keys Tab Tab Return; capture confirmed-again
