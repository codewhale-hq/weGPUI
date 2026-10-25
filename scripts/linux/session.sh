#!/usr/bin/env bash
# Run a GUI command under an AT-SPI-enabled D-Bus session on an X display, optionally drive it
# with real XTEST input, and dump the accessibility tree after each step.
#
# Usage:
#   dbus-run-session -- scripts/linux/session.sh COMMAND [ARGS...]
#
# Environment:
#   DISPLAY   X display to use (default :99; start one with `Xvfb :99 -screen 0 1280x1024x24 &`)
#   WAIT      seconds to wait after launch (default 7)
#   STEPS     steps separated by '|', each LABEL:TOKEN TOKEN ...; a token is an xdotool key name
#             (Tab, Down, ctrl+z) or @X,Y for a left click at screen coordinates
#   SHOTS     if set, save a screenshot after each step to "$SHOTS-LABEL.png" (ImageMagick)
#   CROP      screenshot geometry for SHOTS (default 380x815+0+0)
#   PYTHON    interpreter with pyatspi (default: the first of the distro pythons that imports it)
#
# Without a window manager, keyboard input reaches the app only after a real click inside its
# window, so begin STEPS with a click when keys matter. Requires at-spi2-core, python3-pyatspi,
# xdotool and dbus; ImageMagick only for SHOTS.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export DISPLAY="${DISPLAY:-:99}"
python="${PYTHON:-}"
if [[ -z $python ]]; then
  for candidate in /usr/bin/python3 /usr/bin/python3.12 /usr/bin/python3.11 python3; do
    if "$candidate" -I -c 'import pyatspi' >/dev/null 2>&1; then
      python="$candidate"
      break
    fi
  done
fi
[[ -n $python ]] || { echo "no python with pyatspi found; set PYTHON" >&2; exit 1; }
dump() {
  if ! kill -0 "$app" 2>/dev/null; then
    status=0
    wait "$app" || status=$?
    echo "application exited before capture (status $status)" >&2
    return 1
  fi
  "$python" -I "$here/atspi_dump.py"
}

/usr/libexec/at-spi-bus-launcher --launch-immediately >/dev/null 2>&1 &
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >/dev/null

"$@" &
app=$!
trap 'kill "$app" 2>/dev/null || true' EXIT
sleep "${WAIT:-7}"

echo "## initial"
dump
IFS='|' read -ra steps <<< "${STEPS:-}"
for step in "${steps[@]}"; do
  label="${step%%:*}"
  tokens="${step#*:}"
  for token in $tokens; do
    if [[ $token == @* ]]; then
      xy="${token#@}"
      xdotool mousemove "${xy%,*}" "${xy#*,}" click 1
    else
      xdotool key "$token"
    fi
    sleep 0.5
  done
  sleep 1
  echo "## $label ($tokens)"
  dump
  if [[ -n "${SHOTS:-}" ]]; then
    import -window root -crop "${CROP:-380x815+0+0}" "$SHOTS-$label.png"
  fi
done
