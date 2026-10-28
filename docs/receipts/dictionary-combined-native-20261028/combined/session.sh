#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/dictionary-combined-native-combined
"${CARGO_TARGET_DIR}/debug/wegpui-dictionary-combined-native-combined-probe" >"$out/app.log" 2>&1 & app=$!
trap 'kill "$app" 2>/dev/null || true' EXIT
for _ in $(seq 1 100); do window=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI threshold input probe' 2>/dev/null | head -1 || true); [[ -n $window ]] && break; kill -0 "$app"; sleep .2; done
[[ -n ${window:-} ]]
xdotool windowactivate --sync "$window"
export PROBE_WINDOW="$window" PROBE_OUTPUT="$out"
python3 "$out/driver.py"
