#!/usr/bin/env bash
# Isolated native focus controls; never reuses or changes the live reader session.
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- bash "$0" "$@"; fi
bin=${1:?immutable rebuilt minimal binary}
out=${2:?new evidence directory}
[[ ! -e "$out" ]]
mkdir -p "$out"
out=$(cd "$out" && pwd)
root=/workspace/weGPUI
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cp "${BASH_SOURCE[0]}" "$out/run-used.sh"
cp "$here/focus-control.py" "$out/control-used.py"
cp "$here/atspi-watch.py" "$out/watch-used.py"
cp "$root/docs/receipts/orca-controls-20261025/x11-focus-modes-20261025/observe.py" "$out/observer-used.py"
{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  uname -srmo
  cat /etc/os-release
  rustc -Vv
  cargo -V
  git -C "$root" rev-parse HEAD
  sha256sum "$bin" "$root/Cargo.lock" "$root/vendor/gpui_pre_linux/src/linux/x11/client.rs" "$root/vendor/gpui_pre/src/window.rs" "$root/vendor/gpui_pre/src/window/a11y.rs" "$root/vendor/gpui_pre/src/view.rs"
  printf 'Private X11 :115; no reader or IME service in this focus-control sample.\n'
} >"$out/provenance.txt" 2>&1
export DISPLAY=:115 GPUI_X11_SCALE_FACTOR=1
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-focus-controls.XXXXXX)
unset WAYLAND_DISPLAY
pids=()
cleanup() {
  for ((i=${#pids[@]}-1; i>=0; i--)); do kill "${pids[i]}" 2>/dev/null || true; done
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
/usr/bin/python3 -I "$out/watch-used.py" "$out/atspi-events.jsonl" >"$out/watch.txt" 2>&1 & pids+=("$!")
"$bin" >"$out/app.txt" 2>&1 & app=$!; pids+=("$app")
sleep 6
kill -0 "$app"
window=$(xdotool search --onlyvisible --pid "$app" | head -1)
printf '%s\n' "$window" >"$out/window.txt"
/usr/bin/python3 -I "$out/observer-used.py" --window "$window" --seconds 45 --out "$out/xevents" >"$out/observer.txt" 2>&1 & pids+=("$!")
sleep .5
/usr/bin/python3 -I "$out/control-used.py" --window "$window" --out "$out/controls" >"$out/driver.txt" 2>&1
sleep .5
