#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/set-size-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
bus=$!
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
sha256sum /target/debug/set_size_probe "$out/src/main.rs" "$out/Cargo.lock" /src/src/components/picker.rs /src/src/components/select.rs >"$out/input-sha256.txt"
for mode in select picker date; do
  WEGPUI_SET_SIZE_MODE="$mode" /target/debug/set_size_probe >"$out/$mode-app.log" 2>&1 &
  app=$!
  passed=0
  for _ in $(seq 1 80); do
    if /usr/bin/python3 -I "$out/check.py" "$mode" >"$out/$mode.json" 2>"$out/$mode-error.txt"; then passed=1; break; fi
    if ! kill -0 "$app" 2>/dev/null; then cat "$out/$mode-app.log"; exit 1; fi
    sleep .25
  done
  if [[ "$passed" != 1 ]]; then cat "$out/$mode-error.txt"; exit 1; fi
  wid=$(xdotool search --onlyvisible --name 'weGPUI set size probe' | head -1)
  import -window "$wid" "$out/$mode.png"
  kill "$app"; wait "$app" || true; unset app
  echo "$mode set-size attributes and disabled state PASS"
done
