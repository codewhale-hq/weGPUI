#!/usr/bin/env bash
set -euo pipefail
case_name=${1:?pass choices/agree/buttons/cells/vcode}
out=${2:?pass a new output directory}
export CONTROL_CASE="$case_name"
export WAIT=3
unset DISPLAY WAYLAND_DISPLAY STEPS
binary=${CONTROLS_ARTIFACT:-/workspace/.wegpui-cloud/orca-controls-49268de/wegpui-orca-controls-probe}
exec docs/receipts/orca-controls-20261025/harness/orca-session-controls.sh "$out" "$binary" --page "$case_name"
