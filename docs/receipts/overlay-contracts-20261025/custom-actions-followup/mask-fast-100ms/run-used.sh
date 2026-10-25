#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
bin=${1:?immutable executable}
delay=${2:?delay milliseconds}
out=${3:?new output directory}
consumer=${CONSUMER_SOURCE:-/workspace/.wegpui-cloud/closing-actions-consumer}
[[ ! -e "$out" ]]
mkdir -p "$out"
cp "$here/drive.py" "$out/drive-used.py"
cp "${BASH_SOURCE[0]}" "$out/run-used.sh"
if [[ -d "$consumer" ]]; then
  cp "$consumer/Cargo.toml" "$out/consumer-Cargo.toml"
  cp "$consumer/Cargo.lock" "$out/consumer-Cargo.lock"
  cp "$consumer/src/main.rs" "$out/consumer-main.rs.txt"
  cp "$consumer/src/support/platform.rs" "$out/consumer-platform.rs.txt"
fi
{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  git rev-parse HEAD
  sha256sum "$bin" Cargo.lock Cargo.toml src/components/modal.rs src/components/swiped.rs src/accessibility.rs
  rustc --version --verbose
  cargo --version
  cat /etc/os-release
  openbox --version
  Xvfb -help 2>&1 | head -3 || true
} >"$out/provenance.txt" 2>&1
git status --short >"$out/git-status.txt"
git diff --binary -- Cargo.toml Cargo.lock src >"$out/source.diff"
find vendor -type f -print0 | sort -z | xargs -0 sha256sum >"$out/vendor-source.sha256"
export DISPLAY=:103 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-saved-action.XXXXXX)
cleanup() {
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do kill "$pid" 2>/dev/null || true; done
  for pid in ${app:-} ${wm:-} ${xvfb:-}; do wait "$pid" 2>/dev/null || true; done
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 &
xvfb=$!
for attempt in {1..50}; do xdpyinfo >/dev/null 2>&1 && break; sleep .1; done
openbox >"$out/openbox.log" 2>&1 &
wm=$!
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi.log" 2>&1 &
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enabled.txt"
"$bin" >"$out/app.log" 2>&1 &
app=$!
wid=
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'Saved overlay action probe' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
printf '%s\n' "$wid" >"$out/window-id.txt"
sleep .5
python3 -I "$out/drive-used.py" --window "$wid" --delay-ms "$delay" --out "$out/observations" >"$out/driver.log" 2>&1
