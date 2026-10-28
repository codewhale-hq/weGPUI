#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/radio-empty-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
bus=$!
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
sha256sum /target/debug/radio_empty_probe "$out/src/main.rs" "$out/Cargo.lock" "$out/wegpui/src/components/radio.rs" >"$out/input-sha256.txt"
for items in populated empty; do
  for direction in ltr rtl; do
    for variant in legacy grouped; do
      key="$items-$variant-$direction"
      RADIO_ITEMS="$items" RADIO_VARIANT="$variant" RADIO_DIRECTION="$direction" /target/debug/radio_empty_probe >"$out/$key.log" 2>&1 &
      app=$!
      passed=0
      for _ in $(seq 1 80); do
        if /usr/bin/python3 -I "$out/check.py" snapshot "$variant" "$items" >"$out/$key.json" 2>"$out/$key-error.txt"; then passed=1; break; fi
        if ! kill -0 "$app" 2>/dev/null; then cat "$out/$key.log"; exit 1; fi
        sleep .25
      done
      if [[ "$passed" != 1 ]]; then cat "$out/$key-error.txt"; exit 1; fi
      wid=$(xdotool search --onlyvisible --name 'weGPUI radio set proof' | head -1)
      import -window "$wid" "$out/$key.png"
      if [[ "$items" == populated ]]; then
        /usr/bin/python3 -I "$out/check.py" focus "$variant" 'Form one'
        xdotool key Down; sleep .3
        /usr/bin/python3 -I "$out/check.py" selected "$variant" 'Form three'
        /usr/bin/python3 -I "$out/check.py" focus "$variant" 'Cells one'
        xdotool key Down; sleep .3
        /usr/bin/python3 -I "$out/check.py" selected "$variant" 'Cells three'
      fi
      /usr/bin/python3 -I "$out/check.py" activate "$variant" 'Form add more'
      /usr/bin/python3 -I "$out/check.py" activate "$variant" 'Cells add more'
      sleep .3
      /usr/bin/python3 -I "$out/check.py" status "$variant" 'Form actions 1'
      /usr/bin/python3 -I "$out/check.py" status "$variant" 'Cells actions 1'
      kill "$app"; wait "$app" || true; unset app
      echo "$items $variant $direction accessibility and interaction PASS"
    done
  done
done
/usr/bin/python3 - "$out" <<'CHECK'
import json, pathlib, sys
from PIL import Image, ImageChops
p = pathlib.Path(sys.argv[1])
for items in ('populated', 'empty'):
  for direction in ('ltr', 'rtl'):
    old = Image.open(p / f'{items}-legacy-{direction}.png').convert('RGB')
    new = Image.open(p / f'{items}-grouped-{direction}.png').convert('RGB')
    assert old.size == new.size and ImageChops.difference(old, new).getbbox() is None, (items, direction)
    baseline = json.loads((p / f'{items}-legacy-{direction}.json').read_text())
    grouped = json.loads((p / f'{items}-grouped-{direction}.json').read_text())
    origin_a = next(n for n in baseline if n['name'] == 'weGPUI radio set proof' and n['role'] == 'frame')['bounds']
    origin_b = next(n for n in grouped if n['name'] == 'weGPUI radio set proof' and n['role'] == 'frame')['bounds']
    for prefix in ('Form', 'Cells'):
      for suffix in (('one', 'disabled', 'three', 'add more') if items == 'populated' else ('add more',)):
        name = prefix + ' ' + suffix
        roles = {'button', 'push button'} if suffix == 'add more' else {'radio button'}
        a = next(n for n in baseline if n['name'] == name and n['role'] in roles)['bounds']
        b = next(n for n in grouped if n['name'] == name and n['role'] in roles)['bounds']
        a = [a[0] - origin_a[0], a[1] - origin_a[1], a[2], a[3]]
        b = [b[0] - origin_b[0], b[1] - origin_b[1], b[2], b[3]]
        assert a == b, (items, direction, name, a, b)
    print(items, direction, 'matching screenshot and accessibility bounds PASS')
CHECK
