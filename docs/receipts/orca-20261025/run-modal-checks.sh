#!/usr/bin/env bash
# Invoked inside the shared Debian/Rust 1.98.1 runtime; source/artifacts frozen by root.
set -euo pipefail
out=docs/receipts/orca-20261025
Xvfb :92 -screen 0 1280x1100x24 >"$out/modals-xvfb.log" 2>&1 &
xpid=$!
sleep 1
DISPLAY=:92 openbox >"$out/modals-openbox.log" 2>&1 &
wm=$!
trap 'kill "$wm" "$xpid" 2>/dev/null || true; wait 2>/dev/null || true' EXIT
sleep 1
export DISPLAY=:92
STEPS='open:@160,112|dialog-summary:KP_Enter|cancel:Tab|ok:Tab|trap:Tab|escape:Escape|reopen:Return|to-ok:Tab Tab|confirm:Return' \
  scripts/linux/orca-session.sh "$out/minimal-fixed" /workspace/.wegpui-cloud/target-198/debug/examples/minimal
STEPS='sheet-open:@160,84|sheet-first:Tab|inner-open:Return|inner-summary:KP_Enter|inner-cancel:Tab|inner-ok:Tab|inner-wrap:Tab|inner-back:shift+Tab|inner-escape:Escape|outer-return:KP_Enter|outer-escape:Escape|page-return:KP_Enter' \
  scripts/linux/orca-session.sh "$out/nested-fixed" /workspace/.wegpui-cloud/target-198/debug/examples/nested_modals
