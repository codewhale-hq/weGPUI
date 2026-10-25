#!/usr/bin/env bash
# Invoke inside run-198 with dbus-run-session; builds are deliberately separate.
set -euo pipefail
cd /workspace/weGPUI
out=${OUT:-/workspace/weGPUI/docs/receipts/focus-reveal-20261025}
mkdir -p "$out"
export DISPLAY=:93 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-focus.XXXXXX)
cleanup() {
  for pid in ${probe:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
for attempt in {1..50}; do xdpyinfo >/dev/null 2>&1 && break; sleep .1; done
openbox >"$out/openbox.log" 2>&1 &
wm=$!
sleep 1
"$CARGO_TARGET_DIR/debug/examples/focus_reveal_probe" >"$out/probe.log" 2>&1 &
probe=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$probe" --name 'weGPUI focus reveal probe' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$probe"
  sleep .1
done
[[ -n $wid ]]
xdotool windowactivate --sync "$wid"
sleep 1
snapshot() {
  local label=$1
  xdotool key F8
  sleep .4
  import -window "$wid" "$out/probe-$label.png"
  echo "$label" >> "$out/actions.log"
}
key() {
  xdotool key "$1"
  sleep .6
}
: > "$out/actions.log"
xdotool mousemove --window "$wid" 200 40 click 1
snapshot 01-start
key Tab
snapshot 02-navbar-forward
key Right
snapshot 03-navbar-right
sleep 1
snapshot 04-navbar-visible-stable
xdotool mousemove --window "$wid" 400 200 click --repeat 5 --delay 80 5
sleep .6
snapshot 05-navbar-manual-scroll
sleep 1
snapshot 06-navbar-manual-preserved
key Tab
snapshot 07-tabbar-forward
key Right
snapshot 08-tabbar-right
key Tab
snapshot 09-input-forward
xdotool mousemove --window "$wid" 400 200 click --repeat 5 --delay 80 5
sleep .6
snapshot 10-input-manual-scroll
sleep 1
snapshot 11-input-manual-preserved
key Tab
snapshot 12-clear-forward
key Tab
snapshot 13-end-forward
key shift+Tab
snapshot 14-input-backward
key shift+Tab
snapshot 15-tabbar-backward
key shift+Tab
snapshot 16-navbar-backward
key shift+Tab
snapshot 17-start-backward
sleep 1
snapshot 18-start-visible-stable
/usr/bin/python3 - "$out/probe.log" <<'PY'
import re
import sys
from pathlib import Path

samples = {}
for line in Path(sys.argv[1]).read_text().splitlines():
    match = re.fullmatch(r'sample=(\d+) offset=(-?\d+(?:\.\d+)?)px focus=(\S+) navbar=(\d+) tabbar=(\d+)', line)
    if match:
        step, offset, focus, navbar, tabbar = match.groups()
        samples[int(step)] = (float(offset), focus, int(navbar), int(tabbar))
assert len(samples) == 18, samples
assert [samples[i][1] for i in (1, 9, 13, 14, 17)] == ['start', 'input', 'end', 'input', 'start'], samples
for first, second in ((3, 4), (5, 6), (10, 11), (17, 18)):
    assert samples[first] == samples[second], (first, second, samples)
assert samples[1][0] > samples[2][0] > samples[7][0] > samples[9][0], samples
assert samples[13][0] < samples[14][0] < samples[15][0] < samples[16][0] < samples[17][0], samples
assert samples[3][2] == 1 and samples[8][3] == 1, samples
assert samples[10][0] < samples[12][0] < samples[9][0], samples
print('PASS: forward/backward reveal, tab selection, stable visible focus, manual scroll preservation and clear-button reveal.')
PY
