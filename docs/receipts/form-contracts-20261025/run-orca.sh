#!/usr/bin/env bash
set -euo pipefail
out=docs/receipts/form-contracts-20261025
bin=${1:?pass the compiled forms probe}
orca_out=${2:-"$out/orca-date"}
Xvfb :105 -screen 0 1300x1200x24 >"$out/orca-xvfb.log" 2>&1 &
xpid=$!
sleep 1
DISPLAY=:105 openbox >"$out/orca-openbox.log" 2>&1 &
wm=$!
trap 'kill "$wm" "$xpid" 2>/dev/null || true; wait 2>/dev/null || true' EXIT
sleep 1
DISPLAY=:105 STEPS='bad-date:ctrl+a text=2024-02-30 Return|repeat-one:Return|repeat-two:Return|error-readback:KP_Enter|valid-date:ctrl+a text=2024-03-01 Return|valid-readback:KP_Enter|clear:ctrl+a BackSpace Return|empty-readback:KP_Enter' \
  scripts/linux/orca-session.sh "$orca_out" "$bin"
