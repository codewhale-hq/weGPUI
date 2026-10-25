#!/usr/bin/env bash
# Capture native/upstream pairs for many routes or states with scripts/capture.py on X11 Linux.
#
# Usage:
#   scripts/linux/sweep.sh OUTDIR [--theme light|dark] [--care] [--brand RRGGBB]
#       [--only gpui|weui] [--states | ROUTE...]
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
usage() {
  echo "usage: $0 OUTDIR [--theme light|dark] [--care] [--brand RRGGBB] [--only gpui|weui] [--states | ROUTE...]"
}
[[ $# -gt 0 ]] || { usage >&2; exit 2; }
if [[ $1 == --help || $1 == -h ]]; then usage; exit 0; fi
out="$1"
[[ -n $out && $out != --* ]] || { echo "OUTDIR must be a directory path" >&2; exit 2; }
shift
theme=light
care=0
brand=
only=
states=0
routes=()
while (($#)); do
  case "$1" in
    --theme) [[ $# -gt 1 ]] || { echo "--theme needs a value" >&2; exit 2; }; theme="$2"; shift 2 ;;
    --brand)
      [[ $# -gt 1 ]] || { echo "--brand needs RRGGBB" >&2; exit 2; }
      [[ $2 =~ ^[[:xdigit:]]{6}$ ]] || { echo "brand must be six hexadecimal digits (RRGGBB)" >&2; exit 2; }
      brand="$2"; shift 2 ;;
    --care) care=1; shift ;;
    --only)
      [[ $# -gt 1 ]] || { echo "--only needs gpui or weui" >&2; exit 2; }
      [[ $2 == gpui || $2 == weui ]] || { echo "--only must be gpui or weui" >&2; exit 2; }
      only="$2"; shift 2 ;;
    --states) states=1; shift ;;
    --help|-h) usage; exit 0 ;;
    -*) echo "unknown option: $1" >&2; exit 2 ;;
    *) routes+=("$1"); shift ;;
  esac
done
[[ $theme == light || $theme == dark ]] || { echo "theme must be light or dark" >&2; exit 2; }
(( !states || ${#routes[@]} == 0 )) || { echo "--states cannot be combined with route arguments" >&2; exit 2; }
for route in "${routes[@]}"; do
  [[ $route =~ ^[a-zA-Z0-9_]+$ ]] || { echo "invalid route: $route" >&2; exit 2; }
done
capture_options=(--theme "$theme")
if ((care)); then capture_options+=(--care); fi
if [[ -n $brand ]]; then capture_options+=(--brand "$brand"); fi
if [[ -n $only ]]; then capture_options+=(--only "$only"); fi
python="${PYTHON:-python3}"
mkdir -p "$out/raw" "$out/pairs"
export PAIR_DIR="$out/pairs"
log="$out/sweep.log"
printf 'begin theme=%s care=%s brand=%s only=%s states=%s\n' "$theme" "$care" "${brand:-default}" "${only:-both}" "$states" | tee -a "$log"
ok=0
failed=0

capture() {
  local name="$1"
  shift
  if timeout 90 "$python" -I "$root/scripts/capture.py" "$@" "${capture_options[@]}" --out "$out/raw" \
    >/dev/null 2>"$out/err-$name.txt"; then
    echo "ok $name" | tee -a "$log"
    ok=$((ok + 1))
  else
    echo "FAIL $name" | tee -a "$log"
    failed=$((failed + 1))
  fi
}

if ((states)); then
  keys=$("$python" -I -c '
import glob, json, sys
for path in sorted(glob.glob(sys.argv[1] + "/scripts/parity/*.json")):
    for key in json.load(open(path)):
        print(key)
' "$root")
  [[ -n $keys ]] || { echo "no states found" >&2; exit 1; }
  while IFS= read -r key || [[ -n $key ]]; do
    capture "${key/:/-}" "${key%%:*}" --state "${key#*:}"
  done <<< "$keys"
else
  if ((${#routes[@]} == 0)); then
    mapfile -t routes < <(grep -oE '^\| `[^`]+` \| .* \| `[^`]+` \|' "$root/docs/UPSTREAM-COVERAGE.md" |
      sed -E 's/.*\| `([^`]+)` \|$/\1/' | awk '!seen[$0]++')
  fi
  ((${#routes[@]} > 0)) || { echo "no routes found" >&2; exit 1; }
  for route in "${routes[@]}"; do
    capture "$route" "$route"
  done
fi
echo "done: $ok ok, $failed failed"
((failed == 0))
