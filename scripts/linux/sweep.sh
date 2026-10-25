#!/usr/bin/env bash
# Capture native/upstream pairs for many routes or states with scripts/capture.py on X11 Linux.
#
# Usage:
#   scripts/linux/sweep.sh OUTDIR [--theme dark] [--states] [ROUTE...]
#
# With ROUTE arguments, captures those routes in their default state. With --states, captures
# every "route:state" key registered in scripts/parity/*.json instead. With neither, captures
# every route listed in docs/UPSTREAM-COVERAGE.md. Writes OUTDIR/raw/*.png, OUTDIR/pairs/*.png
# and OUTDIR/sweep.log (one "ok"/"FAIL" line per capture; stderr in OUTDIR/err-*.txt).
#
# Requirements: an X display large enough for a 2x 375x812 window (for example
# `Xvfb :98 -screen 0 1600x3600x24 &` and DISPLAY=:98), a gallery built with gpui_platform's
# x11 feature (docs/proposals/linux-windowing.patch), `scripts/weui_upstream.py serve` running,
# Python with playwright and pillow (PYTHON, default python3), xdotool and ImageMagick.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
out="$1"
shift
theme=light
states=0
routes=()
while (($#)); do
  case "$1" in
    --theme) theme="$2"; shift 2 ;;
    --states) states=1; shift ;;
    *) routes+=("$1"); shift ;;
  esac
done
python="${PYTHON:-python3}"
mkdir -p "$out/raw" "$out/pairs"
export PAIR_DIR="$out/pairs"
log="$out/sweep.log"

capture() {
  local name="$1"
  shift
  if timeout 90 "$python" -I "$root/scripts/capture.py" "$@" --theme "$theme" --out "$out/raw" \
    >/dev/null 2>"$out/err-$name.txt"; then
    echo "ok $name" | tee -a "$log"
  else
    echo "FAIL $name" | tee -a "$log"
  fi
}

if ((states)); then
  "$python" -I -c '
import glob, json, sys
for path in sorted(glob.glob(sys.argv[1] + "/scripts/parity/*.json")):
    for key in json.load(open(path)):
        print(key)
' "$root" | while IFS= read -r key || [[ -n $key ]]; do
    capture "${key/:/-}" "${key%%:*}" --state "${key#*:}"
  done
else
  if ((${#routes[@]} == 0)); then
    mapfile -t routes < <(grep -oE '^\| `[^`]+` \| .* \| `[^`]+` \|' "$root/docs/UPSTREAM-COVERAGE.md" |
      sed -E 's/.*\| `([^`]+)` \|$/\1/' | awk '!seen[$0]++')
  fi
  for route in "${routes[@]}"; do
    capture "$route" "$route"
  done
fi
echo "done: $(grep -c '^ok' "$log") ok, $(grep -c '^FAIL' "$log" || true) failed"
