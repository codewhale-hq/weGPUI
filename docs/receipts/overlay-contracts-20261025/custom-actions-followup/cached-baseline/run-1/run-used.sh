#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- bash "$0" "$@"; fi
cd /workspace/weGPUI
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
out=${1:?new output path}
bin=/workspace/.wegpui-cloud/artifacts/cached-a11y/probe-unchanged-gpui
consumer=/workspace/.wegpui-cloud/cached-a11y-consumer
mkdir "$out"
out=$(realpath "$out")
cp "$here/cache-drive.py" "$out/drive-used.py"
cp "${BASH_SOURCE[0]}" "$out/run-used.sh"
cp "$consumer/Cargo.toml" "$out/consumer-Cargo.toml"
cp "$consumer/Cargo.lock" "$out/consumer-Cargo.lock"
cp "$consumer/src/main.rs" "$out/consumer-main.rs.txt"
cp "$consumer/src/support/platform.rs" "$out/consumer-platform.rs.txt"
{ date -u +%FT%TZ; git rev-parse HEAD; sha256sum "$bin" "$consumer/src/main.rs"; rustc --version --verbose; cat /etc/os-release; } >"$out/provenance.txt"
git status --short >"$out/git-status.txt"
git diff --binary -- Cargo.toml Cargo.lock src >"$out/source.diff"
python3 - "$out/vendor-source.sha256" <<'PY'
import hashlib,sys
from pathlib import Path
with open(sys.argv[1],'w') as target:
    for source in sorted(Path('vendor').rglob('*')):
        if source.is_file():
            target.write(f'{hashlib.sha256(source.read_bytes()).hexdigest()}  {source}\n')
PY
export DISPLAY=:103 GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d /tmp/wegpui-cache-baseline.XXXXXX)
pids=()
cleanup() {
  status=$?
  trap - EXIT
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  sleep .2
  for pid in "${pids[@]}"; do kill -KILL "$pid" 2>/dev/null || true; done
  wait 2>/dev/null || true
  printf 'exit=%s\n' "$status" >"$out/run-status.txt"
}
trap cleanup EXIT
Xvfb :103 -screen 0 1280x1000x24 >"$out/xvfb.log" 2>&1 & pids+=("$!")
sleep .5
openbox >"$out/openbox.log" 2>&1 & pids+=("$!")
dbus-update-activation-environment DISPLAY XDG_RUNTIME_DIR
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi.log" 2>&1 & pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enabled.txt"
"$bin" >"$out/app.log" 2>&1 & app=$!; pids+=("$app")
for attempt in {1..100}; do
  wid=$(xdotool search --onlyvisible --pid "$app" --name 'Cached accessibility action probe' 2>/dev/null | head -1) || true
  [[ -n $wid ]] && break
  kill -0 "$app"
  sleep .1
done
[[ -n $wid ]]
printf '%s\n' "$wid" >"$out/window-id.txt"
sleep 1
/usr/bin/python3 -I "$out/drive-used.py" --window "$wid" --out "$out/observations" >"$out/driver.log" 2>&1
