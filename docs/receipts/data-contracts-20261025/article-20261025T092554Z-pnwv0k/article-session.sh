#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
out="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
gallery="${DATA_ARTICLE_BINARY:-/workspace/.wegpui-cloud/target-198/debug/examples/gallery}"
{
  date -u +%FT%TZ
  git rev-parse HEAD
  git diff -- src examples | sha256sum
  sha256sum "$gallery"
  rustc -Vv
  cat /etc/os-release
  openbox --version
} >"$out/provenance.txt"
git diff -- src examples >"$out/source.diff"
cp examples/gallery/pages/article.rs "$out/article.rs"
cp Cargo.toml Cargo.lock "$out/"
/usr/bin/python3 -I "$out/source_manifest.py" /workspace/weGPUI >"$out/source-files.sha256"
if [[ -d vendor/gpui_pre ]]; then
  /usr/bin/python3 -I "$out/vendor_manifest.py" vendor/gpui_pre >"$out/vendor-gpui-pre.sha256"
fi
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
bus=$!
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.txt"
trap 'kill "${app:-}" "$bus" 2>/dev/null || true' EXIT
status=0
for theme in light dark; do
  "$gallery" --page article --theme "$theme" --size 480x900 --position 0,0 --reduce-motion >"$out/$theme-application.log" 2>&1 &
  app=$!
  sleep 6
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'weGPUI' | head -1)
  /usr/bin/python3 -I scripts/linux/atspi_dump.py >"$out/$theme-atspi.txt"
  import -window "$wid" "$out/$theme.png"
  /usr/bin/python3 -I "$out/action.py" assert-article 2>&1 | tee "$out/$theme-assert.txt" || status=1
  /usr/bin/python3 -I "$out/article_geometry.py" >"$out/$theme-geometry.json" 2>"$out/$theme-geometry-error.txt" || status=1
  xdotool windowactivate --sync "$wid"
  xdotool mousemove --window "$wid" 350 650 click 5 click 5 click 5
  sleep 1
  import -window "$wid" "$out/$theme-scrolled.png"
  kill "$app"
  wait "$app" || true
done
exit "$status"
