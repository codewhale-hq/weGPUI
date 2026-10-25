#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
receipt=/workspace/weGPUI/docs/receipts/article-responsive-20261025
out=$(mktemp -d "$receipt/${1:-check}-$(date -u +%Y%m%dT%H%M%SZ)-XXXXXX")
cp "$receipt/run.sh" "$receipt/verify.py" "$receipt/atspi_metrics.py" "$receipt/validate.py" "$out/"
cp docs/receipts/data-contracts-20261025/source_manifest.py docs/receipts/data-contracts-20261025/vendor_manifest.py "$out/"
artifact=$(mktemp -d /workspace/.wegpui-cloud/artifacts/article-responsive.XXXXXX)
cp "${ARTICLE_BINARY:-/workspace/.wegpui-cloud/target-198/debug/examples/gallery}" "$artifact/gallery"
cp examples/gallery/pages/article.rs "$out/article.rs"
git diff -- src examples Cargo.toml Cargo.lock >"$out/runtime-working-tree.diff"
if [[ -n ${ARTICLE_EXPECTED_SOURCE:-} ]]; then
  cp "$ARTICLE_EXPECTED_SOURCE/Cargo.toml" "$ARTICLE_EXPECTED_SOURCE/Cargo.lock" "$out/"
  cp "$ARTICLE_EXPECTED_SOURCE/src/input.rs" "$out/expected-build-input.rs"
  cp "$ARTICLE_EXPECTED_SOURCE/expected-build-source.diff" "$out/"
  /usr/bin/python3 -I "$out/source_manifest.py" "$ARTICLE_EXPECTED_SOURCE" >"$out/expected-build-source-files.sha256"
  /usr/bin/python3 -I "$out/vendor_manifest.py" "$ARTICLE_EXPECTED_SOURCE/vendor/gpui_pre" >"$out/expected-build-vendor.sha256"
else
  cp Cargo.toml Cargo.lock "$out/"
  /usr/bin/python3 -I "$out/source_manifest.py" /workspace/weGPUI >"$out/runtime-source-files.sha256"
  /usr/bin/python3 -I "$out/vendor_manifest.py" vendor/gpui_pre >"$out/runtime-vendor.sha256"
fi
{
  date -u +%FT%TZ
  git rev-parse HEAD
  printf 'Expected binary source: %s\n' "${ARTICLE_EXPECTED_SOURCE:-recorded canonical baseline}"
  printf 'Runtime working tree is not a build attestation; integrating build logs identify compiled source.\n'
  sha256sum "$artifact/gallery"
  rustc -Vv
  chromium --version
  /opt/parity-python/bin/python -m pip show playwright pillow
} >"$out/provenance.txt"
echo "Evidence: $out"
export DISPLAY=:106 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/article-responsive-runtime.XXXXXX)
Xvfb "$DISPLAY" -screen 0 1280x2000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
cleanup() {
  local driver_status=$?
  printf '%s\n' "$driver_status" >"$out/exit-status.txt"
  kill "${wm:-}" "$xvfb" 2>/dev/null || true
  rm -rf "$XDG_RUNTIME_DIR"
}
trap cleanup EXIT
sleep 1
openbox >"$out/openbox.log" 2>&1 &
wm=$!
sleep 1
dbus-run-session -- /opt/parity-python/bin/python -I "$out/verify.py" "$out" "$artifact/gallery" 2>&1 | tee "$out/runtime.log"
/opt/parity-python/bin/python -I "$out/validate.py" "$out" >"$out/validation.txt" 2>&1
