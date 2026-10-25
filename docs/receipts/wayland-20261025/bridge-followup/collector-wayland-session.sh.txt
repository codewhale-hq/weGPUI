#!/usr/bin/env bash
# Run a native Wayland client inside Weston nested on Xvfb. XTEST input is sent to
# Weston's X11 output; Weston delivers wl_pointer/wl_keyboard events to the client.
# Usage: scripts/linux/wayland-session.sh OUT COMMAND [ARGS...]
# WESTON_X11_DISPLAY selects a dedicated Xvfb display (default :191).
# STEPS uses LABEL:TOKEN TOKEN|... with xdotool key names, @X,Y clicks relative to
# the compositor output, and size=WIDTHxHEIGHT to resize that output.
set -euo pipefail

[[ $# -ge 2 ]] || { echo "usage: $0 OUT COMMAND [ARGS...]" >&2; exit 2; }
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then
  exec dbus-run-session -- bash "$0" "$@"
fi
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
out="$(mkdir -p "$1" && cd "$1" && pwd)"
shift
export DISPLAY="${WESTON_X11_DISPLAY:-:191}"
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-wayland.XXXXXX)
export WAYLAND_DISPLAY=wayland-wegpui XDG_SESSION_TYPE=wayland
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
  weston --version
  dpkg-query -W xvfb openbox weston mesa-vulkan-drivers libgl1-mesa-dri python3-pyatspi
  printf 'command:'; printf ' %q' "$@"; printf '\n'
  printf 'app DISPLAY: absent; WAYLAND_DISPLAY=%s; XDG_SESSION_TYPE=%s\n' "$WAYLAND_DISPLAY" "$XDG_SESSION_TYPE"
  printf 'input: XTEST -> Weston X11 backend -> Wayland keyboard/pointer\n'
  printf 'capture: ImageMagick import of Weston X11 output\n'
  git -C "$root" rev-parse HEAD
  sha256sum "$1" "$root/Cargo.lock"
} >"$out/provenance.txt" 2>&1
(
  cd "$root"
  git ls-files -z src examples Cargo.toml Cargo.lock | xargs -0 sha256sum
) >"$out/source-sha256.txt"

Xvfb "$DISPLAY" -screen 0 1280x1100x24 >"$out/xvfb.log" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[0]}"
openbox >"$out/openbox.log" 2>&1 & pids+=("$!")
sleep 1
kill -0 "${pids[1]}"
weston --backend=x11 --renderer=pixman --shell=kiosk-shell.so \
  --width="${WIDTH:-900}" --height="${HEIGHT:-800}" --socket="$WAYLAND_DISPLAY" \
  --idle-time=0 --no-config >"$out/weston.log" 2>&1 & pids+=("$!")
for _ in {1..30}; do
  test -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" && break
  sleep 1
done
test -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY"
wayland-info >"$out/wayland-info.log" 2>&1
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.log"
env -u DISPLAY WAYLAND_DEBUG=client "$@" >"$out/app.log" 2>"$out/wayland-protocol.log" &
app=$!
pids+=("$app")
sleep "${WAIT:-8}"
kill -0 "$app"
xwininfo -root -tree >"$out/xwindows.txt"
window=$(awk '/x11 output/ && /window id/ {print $NF; exit}' "$out/weston.log")
[[ $window =~ ^[0-9]+$ ]] || { echo "Weston output window not found" >&2; exit 1; }
timeout 10 xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 0 0

capture() {
  kill -0 "$app"
  import -window "$window" "$out/$1.png"
  python3 -I "$root/scripts/linux/atspi_dump.py" >"$out/$1-atspi.txt"
  python3 -I - "$out/$1-details.json" <<'PY'
import json
import sys
import pyatspi


def walk(node):
    interfaces = list(node.get_interfaces())
    item = {
        'role': node.getRoleName(),
        'name': node.name,
        'description': node.description,
        'interfaces': interfaces,
        'states': [pyatspi.stateToString(state) for state in node.getState().getStates()],
        'attributes': list(node.getAttributes()),
    }
    if any(interface.rsplit('.', 1)[-1] == 'Component' for interface in interfaces):
        bounds = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        item['bounds'] = [bounds.x, bounds.y, bounds.width, bounds.height]
    if any(interface.rsplit('.', 1)[-1] == 'Text' for interface in interfaces):
        text = node.queryText()
        item['text'] = text.getText(0, text.characterCount)
        item['caret_offset'] = text.caretOffset
        item['selections'] = [list(text.getSelection(i)) for i in range(text.getNSelections())]
    item['children'] = [walk(node.getChildAtIndex(i)) for i in range(node.childCount)]
    return item


with open(sys.argv[1], 'w') as output:
    json.dump(walk(pyatspi.Registry.getDesktop(0)), output, ensure_ascii=False, indent=2)
    output.write('\n')
PY
  xdotool getwindowgeometry --shell "$window" >"$out/$1-geometry.txt"
}
capture initial
IFS='|' read -ra steps <<< "${STEPS:-}"
for step in "${steps[@]}"; do
  label="${step%%:*}"
  tokens="${step#*:}"
  [[ $label =~ ^[a-zA-Z0-9_-]+$ ]] || { echo "invalid step label" >&2; exit 2; }
  printf '%s %s: %s\n' "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" "$label" "$tokens" | tee -a "$out/actions.log"
  for token in $tokens; do
    if [[ $token == @* ]]; then
      xy="${token#@}"
      xdotool mousemove --window "$window" "${xy%,*}" "${xy#*,}" click 1
    elif [[ $token == size=* ]]; then
      wh="${token#size=}"
      timeout 10 xdotool windowsize --sync "$window" "${wh%x*}" "${wh#*x}"
    else
      xdotool key "$token"
    fi
    sleep 0.5
  done
  sleep 1
  capture "$label"
done
