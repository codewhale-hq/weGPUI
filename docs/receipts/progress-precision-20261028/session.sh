#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/progress-precision-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & bus=$!
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
for variant in baseline candidate; do
  name="progress_precision_$variant"
  /target/debug/"$name" >"$out/$variant.log" 2>&1 & app=$!
  complete=false
  for _ in $(seq 1 80); do
    if /usr/bin/python3 -I "$out/atspi.py" "$name" >"$out/$variant.json.tmp" 2>"$out/$variant-error.txt"; then complete=true; break; fi
    if ! kill -0 "$app" 2>/dev/null; then cat "$out/$variant.log"; exit 1; fi
    sleep .25
  done
  $complete || { cat "$out/$variant-error.txt"; exit 1; }
  mv "$out/$variant.json.tmp" "$out/$variant.json"
  import -window root "$out/$variant.png"
  kill "$app"; wait "$app" || true
  unset app
done
