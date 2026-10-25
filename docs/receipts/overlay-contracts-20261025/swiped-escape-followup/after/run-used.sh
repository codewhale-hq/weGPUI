#!/usr/bin/env bash
# Native keyboard propagation through a controlled SwipedCell and a two-second Modal.
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then
  exec dbus-run-session -- bash "$0" "$@"
fi
bin=${1:?immutable compiled probe}
out=${2:?new output directory}
[[ ! -e "$out" ]]
mkdir -p "$out"
out=$(cd "$out" && pwd)
root=/workspace/weGPUI
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
consumer=/workspace/.wegpui-cloud/closing-actions-consumer
cp "${BASH_SOURCE[0]}" "$out/run-used.sh"
cp "$here/atspi_control.py" "$out/atspi_control.py"
cp "$consumer/Cargo.toml" "$out/consumer-Cargo.toml.txt"
cp "$consumer/Cargo.lock" "$out/consumer-Cargo.lock.txt"
cp "$consumer/src/main.rs" "$out/probe.rs.txt"
cp "$consumer/src/support/platform.rs" "$out/platform.rs.txt"
{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  uname -srmo
  cat /etc/os-release
  rustc -Vv
  cargo -V
  dpkg-query -W xvfb openbox mesa-vulkan-drivers libgl1-mesa-dri python3-pyatspi
  git -C "$root" rev-parse HEAD
  sha256sum "$bin" "$consumer/Cargo.lock" "$consumer/Cargo.toml" "$consumer/src/main.rs" "$root/src/components/swiped.rs" "$root/src/components/modal.rs"
  printf 'X11 DISPLAY=:114 GPUI_X11_SCALE_FACTOR=1, private D-Bus/Openbox, no WAYLAND_DISPLAY\n'
} >"$out/provenance.txt" 2>&1
git -C "$root" status --short >"$out/git-status.txt"
git -C "$root" diff -- src/components/swiped.rs >"$out/swiped-source.diff"
export DISPLAY=:114 GPUI_X11_SCALE_FACTOR=1
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-swiped-escape.XXXXXX)
unset WAYLAND_DISPLAY
pids=()
cleanup() {
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.txt" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[0]}"
openbox >"$out/openbox.txt" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[1]}"
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.txt" 2>&1 & pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.txt"
"$bin" >"$out/app.txt" 2>&1 & app=$!; pids+=("$app")
sleep 5
kill -0 "$app"
window=$(xdotool search --onlyvisible --pid "$app" | head -1)
timeout 10 xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 0 0
xwininfo -id "$window" >"$out/window.txt"
capture() {
  kill -0 "$app"
  import -window "$window" "$out/$1.png"
  python3 -I "$out/atspi_control.py" >"$out/$1.json"
}
keys() {
  printf '%s keys %s\n' "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" "$*" >>"$out/actions.txt"
  for token in "$@"; do xdotool key "$token"; sleep .4; done
}
pointer() {
  local coordinates x y
  coordinates=$(python3 -I "$out/atspi_control.py" "$1")
  read -r x y <<< "$coordinates"
  printf '%s pointer %s @%s,%s\n' "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" "$1" "$x" "$y" >>"$out/actions.txt"
  xdotool mousemove --window "$window" "$x" "$y" click 1
}
capture initial
pointer 'Open slow modal'; sleep 2.3
keys Tab; capture initially-collapsed
keys Escape; sleep 2.3; capture dismissed-from-collapsed
keys Return; sleep 2.3
keys Tab Left; capture expanded
keys Tab; capture action-focused
keys Escape; sleep .3; capture row-closed-modal-open
keys Escape; sleep 2.3; capture dismissed-after-row
