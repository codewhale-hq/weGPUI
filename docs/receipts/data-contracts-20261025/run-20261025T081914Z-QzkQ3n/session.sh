#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
out="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
python=/usr/bin/python3
binary=/workspace/.wegpui-cloud/target-198/debug/examples/data_contracts_probe
source="${DATA_SOURCE_TREE:-/workspace/weGPUI}"
{
  date -u +%FT%TZ
  printf 'Source tree: %s\n' "$source"
  if [[ $source == /workspace/weGPUI ]]; then
    git rev-parse HEAD
    git diff -- src examples | sha256sum
  else
    printf 'Source: immutable 9c4515c archive plus data-source.diff and copied probe.rs\n'
    cat "$source/data-source.sha256"
  fi
  sha256sum "$binary"
  rustc -Vv
  cat /etc/os-release
  openbox --version
} > "$out/provenance.txt"
if [[ $source == /workspace/weGPUI ]]; then
  git diff -- src examples > "$out/source.diff"
else
  cp "$source/data-source.diff" "$out/source.diff"
fi
cp "$source/examples/data_contracts_probe.rs" "$out/probe.rs"
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
bus=$!
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.txt"
"$python" -u -I "$out/events.py" >"$out/announcements.log" 2>&1 &
listener=$!
sleep 1
"$binary" >"$out/application.log" 2>&1 &
app=$!
trap 'kill "$app" "$bus" "$listener" 2>/dev/null || true' EXIT
sleep 6
wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI data contracts' | head -1)
step() {
  local label="$1"
  shift
  printf '%s %s\n' "$(date -u +%FT%TZ)" "$label" | tee -a "$out/actions.txt"
  "$@"
  sleep 0.6
  "$python" -I "$out/action.py" snapshot >"$out/$label.json"
  "$python" -I scripts/linux/atspi_dump.py >"$out/$label-atspi.txt"
  import -window "$wid" "$out/$label.png"
}
step initial "$python" -I "$out/action.py" assert-state 1
step notice-position "$python" -I "$out/action.py" notice-geometry
step footer-position "$python" -I "$out/action.py" footer-geometry
step progress-advance "$python" -I "$out/action.py" activate 'Advance progress'
step cancel-pointer "$python" -I "$out/action.py" pointer 'Cancel upload'
step cancel-hidden "$python" -I "$out/action.py" activate 'Hide cancel'
step cancel-absent "$python" -I "$out/action.py" assert-state 0
step navbar-pointer "$python" -I "$out/action.py" pointer 'Details'
step navbar-arrow xdotool key Right
step navbar-home xdotool key Home
step navbar-at-action "$python" -I "$out/action.py" activate 'History'
step tabbar-focus "$python" -I "$out/action.py" focus 'Home'
step tabbar-key xdotool key Right
step tabbar-at-action "$python" -I "$out/action.py" activate 'Settings'
step slider-focus "$python" -I "$out/action.py" focus 'Step three slider'
step slider-end xdotool key End
step slider-set-value "$python" -I "$out/action.py" set-value 'Step three slider' 9.7
step fractional-set-value "$python" -I "$out/action.py" set-value 'Fractional slider' 0.375
step fractional-focus "$python" -I "$out/action.py" focus 'Fractional slider'
step fractional-key xdotool key Right
step disabled-set-value "$python" -I "$out/action.py" set-value 'Disabled slider' 80
step readonly-set-value "$python" -I "$out/action.py" set-value 'Read-only slider' 80
step panel-at-action "$python" -I "$out/action.py" activate 'Open record，Controlled reusable MediaBox callback'
step cell-at-action "$python" -I "$out/action.py" activate 'Open cell record'
step grid-focus "$python" -I "$out/action.py" focus 'Open grid record'
step grid-key xdotool key space
step footer-focus "$python" -I "$out/action.py" focus 'Footer callback'
step footer-key xdotool key Return
step fixed-footer-at-action "$python" -I "$out/action.py" activate 'Fixed footer callback'
step modal-open "$python" -I "$out/action.py" activate 'Open data modal'
step modal-isolation "$python" -I "$out/action.py" modal-state 1
step modal-dismiss xdotool key Escape
step modal-restore "$python" -I "$out/action.py" modal-state 0
step notice-link "$python" -I "$out/action.py" activate 'Notice details'
step scrolled-notice-position "$python" -I "$out/action.py" notice-geometry
step notice-close "$python" -I "$out/action.py" activate 'Close notice'
step final "$python" -I "$out/action.py" assert-state 0
