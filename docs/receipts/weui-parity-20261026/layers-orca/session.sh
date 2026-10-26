set -eu
source <reader-tools>/environment.sh
out=<evidence>/layers-orca
export reader_root
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d)
export HOME="$XDG_RUNTIME_DIR/home" XDG_CONFIG_HOME="$XDG_RUNTIME_DIR/config" XDG_CACHE_HOME="$XDG_RUNTIME_DIR/cache" XDG_DATA_HOME="$XDG_RUNTIME_DIR/data"
mkdir -p "$HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME" "$XDG_DATA_HOME" "$out/speechd" "$out/config"
export GPUI_X11_SCALE_FACTOR=1 XDG_CURRENT_DESKTOP=GNOME
unset WAYLAND_DISPLAY
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do if [ -f "$icd" ]; then export VK_ICD_FILENAMES="$icd"; break; fi; done
export SPEECHD_ADDRESS="unix_socket:$XDG_RUNTIME_DIR/speechd.sock"
export ALSA_CONFIG_PATH="$out/config/alsa.conf"
cat > "$ALSA_CONFIG_PATH" <<EOF
pcm.!default {
 type file
 slave.pcm { type null }
 file "$out/speech.raw"
 format "raw"
}
EOF
cat > "$out/config/speechd.conf" <<EOF
LogLevel 4
DefaultLanguage "en"
DefaultModule "espeak-ng"
AddModule "espeak-ng" "sd_espeak-ng" "$reader_root/etc/speech-dispatcher/modules/espeak-ng.conf"
AudioOutputMethod "alsa"
AudioALSADevice "default"
EOF
xvfb-run -a -s '-screen 0 1280x1024x24' dbus-run-session -- bash -c '
set -eu
out=<evidence>/layers-orca
openbox --sm-disable > "$out/openbox.log" 2>&1 &
/usr/libexec/at-spi-bus-launcher --launch-immediately > "$out/atspi.log" 2>&1 &
python3 -I <evidence>/layers-interaction/wait-wm.py
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true > /dev/null
speech-dispatcher --run-single --communication-method=unix_socket --socket-path="$XDG_RUNTIME_DIR/speechd.sock" --pid-file="$XDG_RUNTIME_DIR/speechd.pid" --log-dir="$out/speechd" --config-dir="$out/config" --module-dir="$reader_root/usr/lib/speech-dispatcher-modules" --timeout=0 > "$out/speech-dispatcher.log" 2>&1 &
speech_pid=$!
for i in $(seq 1 60); do if [ -S "$XDG_RUNTIME_DIR/speechd.sock" ]; then break; fi; sleep .1; done
[ -S "$XDG_RUNTIME_DIR/speechd.sock" ]
orca --replace --disable=braille --debug --debug-file="$out/orca-debug.log" > "$out/orca.log" 2>&1 &
orca_pid=$!
sleep 3
python3 <evidence>/layers-orca/drive.py
kill "$orca_pid" "$speech_pid" 2>/dev/null || true
sleep 1
for pid in "$orca_pid" "$speech_pid"; do if kill -0 "$pid" 2>/dev/null; then kill -KILL "$pid"; fi; done
'
