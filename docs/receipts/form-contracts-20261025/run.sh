#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- "$0" "$@"; fi
out=$1
bin=$2
mkdir "$out"
out=$(realpath "$out")
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
runtime=$(mktemp -d /tmp/wegpui-forms.XXXXXX)
export XDG_RUNTIME_DIR="$runtime" DISPLAY=:105 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
pids=()
cleanup() {
  local status=$?
  trap - EXIT
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  sleep .5
  for pid in "${pids[@]}"; do kill -KILL "$pid" 2>/dev/null || true; done
  wait 2>/dev/null || true
  printf 'exit=%s\n' "$status" > "$out/run-status.txt"
  rmdir "$runtime" 2>/dev/null || true
}
trap cleanup EXIT
Xvfb :105 -screen 0 1300x1200x24 >"$out/xvfb.log" 2>&1 &
pids+=("$!")
sleep 1
openbox >"$out/openbox.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-update-activation-environment DISPLAY XDG_RUNTIME_DIR
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/a11y-bus.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/a11y-enable.txt"
{ date -u +%FT%TZ; git rev-parse HEAD; git status --short; git diff HEAD -- src examples Cargo.toml Cargo.lock | sha256sum; sha256sum "$bin"; rustc --version; uname -a; cat /etc/os-release; } > "$out/provenance.txt"
"$bin" >"$out/app.log" 2>&1 &
app=$!
pids+=("$app")
sleep 4
window=$(xdotool search --onlyvisible --pid "$app" | head -n1)
xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 20 20
xdotool getwindowgeometry --shell "$window" >"$out/window-geometry.txt"
xwininfo -id "$window" >"$out/window-info-raw.txt"
/usr/bin/python3 "$here/drive.py" "$out" "$window"
