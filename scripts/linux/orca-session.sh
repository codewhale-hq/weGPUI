#!/usr/bin/env bash
# Record real Orca speech, its debug transcript, screenshots and separate AT-SPI trees.
# Usage: scripts/linux/orca-session.sh OUT COMMAND [ARGS...]
# Needs Orca, Xvfb/openbox (unless DISPLAY is supplied), PulseAudio, speech-dispatcher,
# an installed synthesizer, xdotool, ImageMagick and the distro Python with pyatspi.
# STEPS uses LABEL:TOKEN TOKEN ... separated by '|'. Tokens: X keys, @X,Y (window-local
# click), text=WORD, wait=SECONDS. This is evidence collection, not an acceptance oracle.
set -euo pipefail

if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then
  exec dbus-run-session -- "$0" "$@"
fi
[[ $# -ge 2 ]] || { echo "usage: $0 OUT COMMAND [ARGS...]" >&2; exit 2; }
out=$1
shift
mkdir -p "$(dirname "$out")"
mkdir "$out" # Refuse to overwrite a previous run, including failed evidence.
out=$(realpath "$out")
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
runtime=$(mktemp -d /tmp/wegpui-orca.XXXXXX)
export XDG_RUNTIME_DIR="$runtime"
export XDG_CONFIG_HOME="$runtime/config"
export XDG_DATA_HOME="$runtime/data"
export XDG_CACHE_HOME="$runtime/cache"
mkdir -p "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_CACHE_HOME" "$runtime/pulse"
export PULSE_SERVER="unix:$runtime/pulse/native"
export PULSE_SINK=orca_capture
export GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
pids=()
recorder=
summarize() {
/usr/bin/python3 - "$out" >"$out/audio-summary.txt" <<'PY'
import array
import math
from pathlib import Path
import re
import sys
import wave

out = Path(sys.argv[1])
with (out / 'orca-debug.log').open() as source:
    with (out / 'speech-transcript.txt').open('w') as transcript:
        collecting = False
        for line in source:
            if re.match(r'^\d\d:\d\d:\d\d\.\d+ - ', line):
                collecting = 'SPEECH OUTPUT' in line
            if collecting:
                transcript.write(line)
with wave.open(str(out / 'speech.wav'), 'rb') as audio:
    samples = array.array('h', audio.readframes(audio.getnframes()))
    if sys.byteorder != 'little':
        samples.byteswap()
    print('channels:', audio.getnchannels())
    print('sample_width:', audio.getsampwidth())
    print('sample_rate:', audio.getframerate())
    print('duration_seconds:', audio.getnframes() / audio.getframerate())
    print('peak:', max(map(abs, samples), default=0))
    print('rms:', math.sqrt(sum(s * s for s in samples) / max(1, len(samples))))
PY
}
cleanup() {
  local status=$?
  trap - EXIT
  if [[ -n $recorder ]]; then
    kill -INT "$recorder" 2>/dev/null || true
    wait "$recorder" 2>/dev/null || true
  fi
  for ((i=${#pids[@]}-1; i>=0; i--)); do
    kill "${pids[i]}" 2>/dev/null || true
  done
  # Orca can hang during shutdown after a speech server is terminated. Bound cleanup
  # to the exact children started here; never match process command lines.
  sleep 1
  for pid in "${pids[@]}"; do
    if kill -0 "$pid" 2>/dev/null; then
      printf 'forced shutdown pid=%s\n' "$pid" >>"$out/cleanup.txt"
      kill -KILL "$pid" 2>/dev/null || true
    fi
  done
  wait 2>/dev/null || true
  printf "exit=%s\n" "$status" >"$out/run-status.txt"
  if [[ -f $out/speech.wav && -f $out/orca-debug.log ]]; then
    summarize || true
  fi
  for log in "$out"/speech-dispatcher/*.log; do
    [[ ! -f $log ]] || gzip -n "$log"
  done
  rm -rf "$runtime"
}
trap cleanup EXIT

if [[ -z ${DISPLAY:-} ]]; then
  export DISPLAY=:91
  Xvfb "$DISPLAY" -screen 0 1280x1100x24 >"$out/xvfb.log" 2>&1 &
  pids+=("$!")
  sleep 1
  openbox >"$out/openbox.log" 2>&1 &
  pids+=("$!")
  sleep 1
fi
dbus-update-activation-environment DISPLAY XDG_RUNTIME_DIR XDG_CONFIG_HOME XDG_DATA_HOME XDG_CACHE_HOME
# Start the a11y bus before even asking Orca for its version: that command initializes
# GTK and can otherwise auto-launch a first bus before DISPLAY is available.
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus \
  org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled \
  variant:boolean:true >"$out/atspi-enable.txt" 2>&1

{
  date -u +%FT%TZ
  git rev-parse HEAD
  git status --short
  git diff -- src examples Cargo.toml Cargo.lock rust-toolchain.toml | sha256sum
  sha256sum "$(command -v "$1")"
  rustc --version
  uname -a
  cat /etc/os-release
  orca --version
  speech-dispatcher --version
  pulseaudio --version
  espeak-ng --version
  printf 'command:'
  printf ' %q' "$@"
  printf '\n'
} >"$out/provenance.txt" 2>&1
orca --help >"$out/orca-help.txt" 2>&1

pulseaudio -n --daemonize=no --exit-idle-time=-1 --disable-shm \
  --load="module-native-protocol-unix socket=$runtime/pulse/native auth-anonymous=1" \
  --load="module-null-sink sink_name=orca_capture rate=16000 channels=1" \
  >"$out/pulseaudio.log" 2>&1 &
pids+=("$!")
ready=false
for ((i=0; i<40; i++)); do
  if pactl info >"$out/pulse-info.txt" 2>&1; then
    ready=true
    break
  fi
  sleep 0.25
done
$ready || { echo "PulseAudio did not become ready" >&2; exit 1; }
parec --device=orca_capture.monitor --rate=16000 --channels=1 --file-format=wav \
  "$out/speech.wav" >"$out/recorder.log" 2>&1 &
recorder=$!
mkdir -p "$out/speech-dispatcher"
export SPEECHD_ADDRESS="unix_socket:$runtime/speechd.sock"
speech-dispatcher --run-single --communication-method=unix_socket \
  --socket-path="$runtime/speechd.sock" --pid-file="$runtime/speechd.pid" \
  --log-dir="$out/speech-dispatcher" --timeout=0 >"$out/speech-dispatcher.log" 2>&1 &
pids+=("$!")
for ((i=0; i<40; i++)); do
  [[ ! -S $runtime/speechd.sock ]] || break
  sleep 0.25
done
[[ -S $runtime/speechd.sock ]] || { echo "speech-dispatcher socket not ready" >&2; exit 1; }

orca --replace --debug --debug-file="$out/orca-debug.log" >"$out/orca.log" 2>&1 &
pids+=("$!")
sleep 3
"$@" >"$out/application.log" 2>&1 &
app=$!
pids+=("$app")
window=
for ((i=0; i<60; i++)); do
  window=$(xdotool search --onlyvisible --pid "$app" 2>/dev/null | head -n 1 || true)
  [[ -z $window ]] || break
  kill -0 "$app" || { echo "application exited before a window appeared" >&2; exit 1; }
  sleep 0.5
done
[[ -n $window ]] || { echo "application window not found" >&2; exit 1; }
xdotool windowactivate --sync "$window"
xdotool windowmove "$window" 20 20
sleep "${WAIT:-5}"

capture() {
  local label=$1
  kill -0 "$app" || { echo "application exited before $label" >&2; return 1; }
  import -window "$window" "$out/$label.png"
  local tree_status=0
  /usr/bin/python3 -I "$here/atspi_dump.py" >"$out/$label-atspi.txt" 2>&1 || tree_status=$?
  if ((tree_status != 0)); then
    printf '%s tree capture exit=%s\n' "$label" "$tree_status" >>"$out/capture-errors.txt"
  fi
  local text_status=0
  /usr/bin/python3 -I "$here/orca-text.py" >"$out/$label-text-interface.txt" 2>&1 || text_status=$?
  if ((text_status != 0)); then
    printf '%s text probe exit=%s\n' "$label" "$text_status" >>"$out/capture-errors.txt"
  fi
}
capture initial
IFS='|' read -ra steps <<< "${STEPS:-}"
for step in "${steps[@]}"; do
  [[ -n $step ]] || continue
  label=${step%%:*}
  [[ $label =~ ^[A-Za-z0-9_-]+$ ]] || { echo "invalid step label: $label" >&2; exit 2; }
  tokens=${step#*:}
  printf '%s %s\n' "$(date -u +%FT%TZ)" "$step" >>"$out/actions.txt"
  for token in $tokens; do
    if [[ $token == @* ]]; then
      xy=${token#@}
      xdotool mousemove --window "$window" "${xy%,*}" "${xy#*,}" click 1
    elif [[ $token == text=* ]]; then
      xdotool type --clearmodifiers --delay 120 -- "${token#text=}"
    elif [[ $token == wait=* ]]; then
      sleep "${token#wait=}"
    else
      xdotool key --clearmodifiers "$token"
    fi
    sleep "${ACTION_DELAY:-2}"
  done
  capture "$label"
done
sleep 3
kill -INT "$recorder" 2>/dev/null || true
wait "$recorder" 2>/dev/null || true
recorder=
