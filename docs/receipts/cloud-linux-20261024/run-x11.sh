#!/usr/bin/env bash
# Run inside the isolated verification container, from /work, after the build.
set -euo pipefail
export DISPLAY=:99
export XDG_RUNTIME_DIR=/tmp/wegpui-runtime
export GPUI_X11_SCALE_FACTOR=1
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
export PYTHON=/usr/bin/python3
out=docs/receipts/cloud-linux-20261024
case "${1:-}" in
  minimal)
    STEPS='open:@190,126|confirm:@267,396|reopen:@190,126|dismiss:Escape' \
    SHOTS="$out/minimal" CROP=375x640+0+0 \
      dbus-run-session -- bash scripts/linux/session.sh sh -c '
        (sleep 3; xdotool search --sync --onlyvisible --pid "$$" "." windowmove %@ 0 0) &
        exec target/debug/examples/minimal
      '
    ;;
  select)
    STEPS='open:@190,170|next:Down|choose:Return|reopen:@190,170|dismiss:Escape' \
    SHOTS="$out/select" CROP=375x812+0+0 \
      dbus-run-session -- bash scripts/linux/session.sh \
      target/debug/examples/gallery --page form_select --position 0,0 --reduce-motion
    ;;
  *) echo 'Usage: bash run-x11.sh minimal|select' >&2; exit 2 ;;
esac
