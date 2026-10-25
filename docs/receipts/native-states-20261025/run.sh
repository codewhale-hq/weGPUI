#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
out=/workspace/weGPUI/docs/receipts/native-states-20261025
mkdir -p "$out"
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-sweep.XXXXXX)
export PYTHON=/opt/parity-python/bin/python
unset WAYLAND_DISPLAY
pids=()
cleanup() {
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
{
  date -u +%FT%TZ
  git rev-parse HEAD
  git diff -- src examples scripts/capture.py scripts/linux/sweep.sh | sha256sum
  sha256sum "$CARGO_TARGET_DIR/debug/examples/gallery"
  rustc -Vv
  cargo -V
  uname -a
  cat /etc/os-release
  Xvfb -version 2>&1 || true
  openbox --version
  import -version
  /opt/parity-python/bin/pip show pillow playwright
  printf 'Device: Vulkan lavapipe software renderer; X11 capture scale=2\n'
} > "$out/provenance.txt"
git diff -- src examples scripts/capture.py scripts/linux/sweep.sh > "$out/source.diff"
for mode in dark care brand; do
  case "$mode" in
    dark) display=:95; opts=(--theme dark);;
    care) display=:96; opts=(--care);;
    brand) display=:97; opts=(--brand 7C3AED);;
  esac
  Xvfb "$display" -screen 0 1600x3600x24 >"$out/$mode-xvfb.log" 2>&1 &
  pids+=("$!")
  sleep 1
  DISPLAY="$display" openbox >"$out/$mode-openbox.log" 2>&1 &
  pids+=("$!")
  sleep 1
  DISPLAY="$display" scripts/linux/sweep.sh "$out/$mode" "${opts[@]}" --states --only gpui >"$out/$mode-console.log" 2>&1 &
  runs+=("$!")
done
status=0
for pid in "${runs[@]}"; do wait "$pid" || status=1; done
for mode in dark care brand; do
  /opt/parity-python/bin/python -I scripts/contact_sheets.py "$out/$mode/raw" "$out/$mode/sheets" --glob '*-gpui.png' --per 6 --scale 0.5 --prefix "$mode-" >"$out/$mode-contact-sheets.log"
done
exit "$status"
