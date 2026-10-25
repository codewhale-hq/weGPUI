#!/usr/bin/env bash
# Actual Orca output capture; audio intelligibility requires separate listening.
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- "$0" "$@"; fi
cd /workspace/weGPUI
out=${OUT:-/workspace/weGPUI/docs/receipts/input-contracts-20261025/orca-final}
mkdir -p "$out"
export DISPLAY=:102
pids=()
cleanup() {
  for ((i=${#pids[@]}-1; i>=0; i--)); do kill "${pids[i]}" 2>/dev/null || true; done
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1100x24 >"$out/xvfb.log" 2>&1 &
pids+=("$!")
sleep 1
openbox >"$out/openbox.log" 2>&1 &
pids+=("$!")
sleep 1
export STEPS='summary:KP_Enter|start:Home|first-word:ctrl+Right|second-word:ctrl+Right|selected-word:End ctrl+shift+Left|selection-summary:KP_Enter|notes:Tab ctrl+Home KP_Enter|password:Tab Tab KP_Enter|readonly:shift+Tab KP_Enter'
export ACTION_DELAY=1 WAIT=3
binary="${INPUT_ARTIFACT:-$CARGO_TARGET_DIR/debug/examples/input_contracts_probe}"
session_script=scripts/linux/orca-session.sh
if [[ ${HELD_KEYS:-0} == 1 ]]; then
  mkdir -p "$out/harness"
  python3 - "$out/harness" <<'PY_HELD'
from pathlib import Path
import hashlib
import shutil
import sys

out = Path(sys.argv[1])
source = Path('scripts/linux/orca-session.sh')
text = source.read_text()
needle = '    elif [[ $token == wait=* ]]; then'
replacement = """    elif [[ $token == down=* ]]; then
      xdotool keydown "${token#down=}"
    elif [[ $token == up=* ]]; then
      xdotool keyup "${token#up=}"
    elif [[ $token == plain=* ]]; then
      xdotool key "${token#plain=}"
    elif [[ $token == wait=* ]]; then"""
assert needle in text
(out / 'orca-session-held.sh').write_text(text.replace(needle, replacement))
(out / 'source-script-sha256.txt').write_text(hashlib.sha256(source.read_bytes()).hexdigest() + '\n')
for name in ('atspi_dump.py', 'orca-text.py'):
    shutil.copyfile(source.parent / name, out / name)
PY_HELD
  session_script="$out/harness/orca-session-held.sh"
  export STEPS='summary:KP_Enter|start:Home|first-word:down=Control_L plain=Right up=Control_L|second-word:down=Control_L plain=Right up=Control_L|selected-word:End down=Control_L down=Shift_L plain=Left up=Shift_L up=Control_L|selection-summary:KP_Enter|notes:Tab down=Control_L plain=Home up=Control_L KP_Enter|password:Tab Tab KP_Enter|readonly:shift+Tab KP_Enter|invalid:Tab Tab KP_Enter'
fi
bash "$session_script" "$out/session" "$binary" >"$out/console.log" 2>&1
