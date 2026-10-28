#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/vcode-deadline-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & bus=$!
trap 'kill -CONT "${app:-}" 2>/dev/null || true; kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
for variant in baseline candidate; do
  name="vcode_deadline_$variant"
  /target/debug/"$name" >"$out/$variant.log" 2>&1 & app=$!
  ready=false
  for _ in $(seq 1 80); do
    if /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-initial.json.tmp" 2>"$out/$variant-error.txt"; then ready=true; break; fi
    if ! kill -0 "$app" 2>/dev/null; then cat "$out/$variant.log"; exit 1; fi
    sleep .25
  done
  $ready || { cat "$out/$variant-error.txt"; exit 1; }
  mv "$out/$variant-initial.json.tmp" "$out/$variant-initial.json"
  /usr/bin/python3 -I "$out/atspi.py" "$name" 'Send code'
  sleep .2
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-start.json"
  kill -STOP "$app"
  /usr/bin/python3 -c 'import time; print(time.monotonic_ns())' >"$out/$variant-pause-start.txt"
  sleep 4.5
  /usr/bin/python3 -c 'import time; print(time.monotonic_ns())' >"$out/$variant-pause-end.txt"
  kill -CONT "$app"
  sleep .7
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-after-pause.json"
  /usr/bin/python3 -I "$out/atspi.py" "$name" Reset
  sleep .2
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-reset.json"
  /usr/bin/python3 -I "$out/atspi.py" "$name" 'Send code'
  sleep .2
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-restart.json"
  /usr/bin/python3 -I "$out/atspi.py" "$name" Reset
  sleep 3.5
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-after-cancel.json"
  /usr/bin/python3 -I "$out/atspi.py" "$name" 'Send code'
  sleep 3.5
  /usr/bin/python3 -I "$out/atspi.py" "$name" snapshot >"$out/$variant-natural-end.json"
  kill "$app"; wait "$app" || true; unset app
done
