#!/usr/bin/env bash
set -euo pipefail
root=/workspace/weGPUI/docs/receipts/control-contracts-20261025
css=/workspace/.wegpui-cloud/target-198/weui-upstream/weui/package/dist/style/weui.css
for size in large medium mini xmini; do
  /opt/parity-python/bin/python "$root/static_reference.py" "$root/static-css/$size-light" --css "$css" --size "$size"
done
for mode in dark care dark-care brand dark-care-brand; do
  opts=()
  [[ $mode != *dark* ]] || opts+=(--dark)
  [[ $mode != *care* ]] || opts+=(--care)
  [[ $mode != *brand* ]] || opts+=(--brand 7C3AED)
  /opt/parity-python/bin/python "$root/static_reference.py" "$root/static-css/large-$mode" --css "$css" "${opts[@]}"
done
