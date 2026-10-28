#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/slider-contract-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
bus=$!
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
/target/debug/data_contracts_probe >"$out/app.log" 2>&1 &
app=$!
for _ in $(seq 1 60); do
  if /usr/bin/python3 -I "$out/action.py" snapshot >"$out/initial.json" 2>/dev/null; then break; fi
  if ! kill -0 "$app" 2>/dev/null; then cat "$out/app.log"; exit 1; fi
  sleep .25
done
/usr/bin/python3 -I "$out/action.py" snapshot >"$out/initial.json"
/usr/bin/python3 -I "$out/action.py" set-value 'Fractional slider' 0.375
sleep .5
/usr/bin/python3 -I "$out/action.py" snapshot >"$out/set.json"
/usr/bin/python3 -I "$out/action.py" focus 'Fractional slider'
xdotool key Right
sleep .5
/usr/bin/python3 -I "$out/action.py" snapshot >"$out/key.json"
wid=$(xdotool search --onlyvisible --name 'weGPUI data contracts' | head -1)
import -window "$wid" "$out/slider.png"
/usr/bin/python3 - "$out" <<'CHECK'
import json, pathlib, sys
root=pathlib.Path(sys.argv[1])
for step, value, label in [('initial',0.125,'0.125'),('set',0.375,'0.375'),('key',0.5,'0.5')]:
    nodes=json.loads((root/(step+'.json')).read_text())
    slider=[n for n in nodes if n['name']=='Fractional slider' and n['role']=='slider']
    assert len(slider)==1 and slider[0]['value']==value, (step,slider)
    statuses=[n for n in nodes if n['role']=='status bar' and n['name']==label]
    assert len(statuses)==1,(step,label,statuses)
    print(step, 'numeric', value, 'status', label, 'PASS')
CHECK
