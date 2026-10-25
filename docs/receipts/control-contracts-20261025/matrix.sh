#!/usr/bin/env bash
set -u
root=/workspace/weGPUI/docs/receipts/control-contracts-20261025
export CONTROL_BINARY=/workspace/.wegpui-cloud/controls-candidate-9c/control_contracts
status=0
for mode in light dark care dark-care brand dark-brand care-brand dark-care-brand; do
  opts=()
  [[ $mode != *dark* ]] || opts+=(--dark)
  [[ $mode != *care* ]] || opts+=(--care)
  [[ $mode != *brand* ]] || opts+=(--brand 7C3AED)
  for size in large medium mini xmini; do
    out="$root/matrix/$mode-buttons-$size"
    if bash "$root/run.sh" "$out" buttons --size "$size" "${opts[@]}" >"$root/$mode-buttons-$size-console.log" 2>&1; then
      echo "PASS $mode buttons $size"
    else
      echo "FAIL $mode buttons $size"
      status=1
    fi
  done
  for page in cells choices agree vcode; do
    out="$root/matrix/$mode-$page"
    if bash "$root/run.sh" "$out" "$page" "${opts[@]}" >"$root/$mode-$page-console.log" 2>&1; then
      echo "PASS $mode $page"
    else
      echo "FAIL $mode $page"
      status=1
    fi
  done
done
exit "$status"
