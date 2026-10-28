#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/date-canada-candidate
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & bus=$!
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
capture() {
  local locale="$1" mode="$2" entry="$3" label="$4"
  DATE_PROBE_LOCALE="$locale" DATE_PROBE_MODE="$mode" DATE_PROBE_ENTRY="$entry" /target/debug/date_canada_candidate >"$out/$label.log" 2>&1 & app=$!
  local ready=false
  for _ in $(seq 1 80); do
    if { [[ "$mode" == picker || "$entry" == __NONE__ ]] || grep -q 'commit=' "$out/$label.log"; } && /usr/bin/python3 -I "$out/atspi.py" "$mode" >"$out/$label.json.tmp" 2>"$out/$label-error.txt"; then ready=true; break; fi
    if ! kill -0 "$app" 2>/dev/null; then cat "$out/$label.log"; exit 1; fi
    sleep .25
  done
  $ready || { cat "$out/$label-error.txt"; exit 1; }
  mv "$out/$label.json.tmp" "$out/$label.json"
  import -window root "$out/$label.png"
  kill "$app"; wait "$app" || true; unset app
}
for locale in en-CA fr-CA en en-US en-GB zh-CN de; do
  capture "$locale" display __NONE__ "$locale-display"
  capture "$locale" display '04/03/2027' "$locale-entry"
  capture "$locale" picker '' "$locale-picker"
done
capture en-CA display '2027-03-04' en-CA-iso
capture en-CA display '04/03/1899' en-CA-range
capture en-CA display '03/04/2027' en-CA-ambiguous
capture en-CA display '31/04/2027' en-CA-invalid
capture en-CA display '2027-02-29' en-CA-invalid-iso
capture en-CA display '2027/03/04' en-CA-year-first-slash
capture fr-CA display '03/04/2027' fr-CA-ambiguous
capture fr-CA display '31/04/2027' fr-CA-invalid
DATE_PROBE_DATETIME=1 capture en-CA display '03/04/2027 05:06' en-CA-datetime-legacy
DATE_PROBE_DATETIME=1 capture en-CA display '2027-03-04 05:06' en-CA-datetime-iso
