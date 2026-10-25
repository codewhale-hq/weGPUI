#!/usr/bin/env bash
set -u
root=/workspace/weGPUI/docs/receipts/control-contracts-20261025
export CONTROL_BINARY=/workspace/.wegpui-cloud/controls-integrated-20261025-0839/control_contracts
status=0
run() {
  local name=$1
  shift
  if bash "$root/run-v2.sh" "$root/final/$name" "$@" >"$root/final-$name-console.log" 2>&1; then
    echo "PASS $name"
  else
    echo "FAIL $name"
    status=1
  fi
}
for mode in light dark care dark-care brand dark-brand care-brand dark-care-brand; do
  opts=()
  [[ $mode != *dark* ]] || opts+=(--dark)
  [[ $mode != *care* ]] || opts+=(--care)
  [[ $mode != *brand* ]] || opts+=(--brand 7C3AED)
  run "$mode-agree" agree "${opts[@]}"
done
run long-link agree-link
run dark-care-brand-long-link agree-link --dark --care --brand 7C3AED
run switch-motion switch-motion
run care-buttons-medium buttons --size medium --care
run dark-brand-choices choices --dark --brand 7C3AED
run light-vcode vcode
exit "$status"
