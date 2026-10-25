#!/usr/bin/env bash
# Run inside wegpui-198 under dbus-run-session; no Cargo work is performed.
set -euo pipefail
cd /workspace/weGPUI
out=${OUT:-/workspace/weGPUI/docs/receipts/focus-reveal-20261025}
mkdir -p "$out"
export DISPLAY=:93 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-focus-gallery.XXXXXX)
cleanup() {
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/gallery-xvfb.log" 2>&1 &
xvfb=$!
for attempt in {1..50}; do xdpyinfo >/dev/null 2>&1 && break; sleep .1; done
openbox >"$out/gallery-openbox.log" 2>&1 &
wm=$!
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/gallery-atspi.log" 2>&1 &
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >/dev/null
capture() {
  local route=$1 steps=$2
  "$CARGO_TARGET_DIR/debug/examples/gallery" --page "$route" --size 480x420 \
    --position 0,0 --reduce-motion >"$out/gallery-$route-app.log" 2>&1 &
  app=$!
  local wid=
  for attempt in {1..100}; do
    wid=$(xdotool search --onlyvisible --pid "$app" --name weGPUI 2>/dev/null | head -1) || true
    [[ -n $wid ]] && break
    kill -0 "$app"
    sleep .1
  done
  [[ -n $wid ]]
  xdotool windowactivate --sync "$wid"
  sleep 1
  import -window "$wid" "$out/gallery-$route-initial.png"
  : > "$out/gallery-$route-focus.log"
  for ((i=1; i<=steps; i++)); do
    xdotool key Tab
    sleep .5
    echo "Tab $i" >> "$out/gallery-$route-focus.log"
    /usr/bin/python3 -I "$out/focused-node.py" >> "$out/gallery-$route-focus.log"
    import -window "$wid" "$out/gallery-$route-tab-$i.png"
  done
  for ((i=1; i<=steps; i++)); do
    xdotool key shift+Tab
    sleep .5
    echo "Shift+Tab $i" >> "$out/gallery-$route-focus.log"
    /usr/bin/python3 -I "$out/focused-node.py" >> "$out/gallery-$route-focus.log"
    import -window "$wid" "$out/gallery-$route-back-$i.png"
  done
  kill "$app"
  wait "$app" || true
  unset app
}
capture article 2
capture home 7
capture form_checkbox 5
capture form_vcode 8
capture form_page 12
/usr/bin/python3 -I "$out/validate-gallery.py" "$out"
