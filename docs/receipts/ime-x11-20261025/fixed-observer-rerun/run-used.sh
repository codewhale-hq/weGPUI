#!/usr/bin/env bash
set -euo pipefail
if [[ -z ${DBUS_SESSION_BUS_ADDRESS:-} ]]; then exec dbus-run-session -- "$0" "$@"; fi
cd /workspace/weGPUI
out=${OUT:-/workspace/weGPUI/docs/receipts/ime-x11-20261025/attempt1}
mkdir -p "$out"
export OUT="$out" DISPLAY=:102 GPUI_X11_SCALE_FACTOR=1 LC_ALL=C.UTF-8
export IME_BINARY=${IME_BINARY:-/workspace/.wegpui-cloud/ime-gallery-49268de7}
driver=${IME_DRIVER:-docs/receipts/ime-x11-20261025/drive.py}
unset WAYLAND_DISPLAY
session=$(mktemp -d /tmp/wegpui-xim.XXXXXX)
export XDG_RUNTIME_DIR="$session/runtime" XDG_CONFIG_HOME="$session/config"
export XDG_DATA_HOME="$session/data" XDG_CACHE_HOME="$session/cache"
export XMODIFIERS=@im=fcitx
mkdir -p "$XDG_RUNTIME_DIR" "$XDG_CONFIG_HOME/fcitx5" "$XDG_DATA_HOME" "$XDG_CACHE_HOME"
chmod 700 "$XDG_RUNTIME_DIR"
cat > "$XDG_CONFIG_HOME/fcitx5/profile" <<'EOF'
[Groups/0]
Name=Default
Default Layout=us
DefaultIM=pinyin

[Groups/0/Items/0]
Name=keyboard-us
Layout=

[Groups/0/Items/1]
Name=pinyin
Layout=

[GroupOrder]
0=Default
EOF
cp "$0" "$out/run-used.sh"
cp "$driver" "$out/drive-used.py"
if [[ ${XIM_ON_THE_SPOT:-0} == 1 ]]; then
  mkdir -p "$XDG_CONFIG_HOME/fcitx5/conf"
  printf '%s\n' 'UseOnTheSpot=True' > "$XDG_CONFIG_HOME/fcitx5/conf/xim.conf"
fi
pids=()
cleanup() {
  for ((i=${#pids[@]}-1; i>=0; i--)); do kill "${pids[i]}" 2>/dev/null || true; done
  cp -a "$XDG_CONFIG_HOME/fcitx5" "$out/fcitx5-config-final" || true
  rm -rf "$session"
}
trap cleanup EXIT
Xvfb "$DISPLAY" -screen 0 1600x2100x24 >"$out/xvfb.log" 2>&1 &
pids+=("$!")
sleep 1
openbox >"$out/openbox.log" 2>&1 &
pids+=("$!")
sleep 1
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/a11y-bus.log" 2>&1 &
pids+=("$!")
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/a11y-enabled.txt"
fcitx5 -D --verbose 'default=4,key_trace=5' >"$out/fcitx5.log" 2>&1 &
pids+=("$!")
sleep 3
fcitx5-remote -n >"$out/initial-input-method.txt"
xprop -root XIM_SERVERS >"$out/xim-servers.txt"
dbus-send --session --print-reply --dest=org.fcitx.Fcitx5 /controller org.fcitx.Fcitx.Controller1.GetConfig string:fcitx://config/addon/xim >"$out/xim-addon-config.txt"
dbus-send --session --print-reply --dest=org.fcitx.Fcitx5 /controller org.fcitx.Fcitx.Controller1.GetConfig string:fcitx://config/inputmethod/pinyin >"$out/pinyin-config.txt"
{
  date -u +%FT%TZ
  git rev-parse HEAD
  sha256sum "$IME_BINARY" src/input.rs
  dpkg-query -W fcitx5 fcitx5-pinyin fcitx5-modules libimepinyin0 libime-data
  uname -a
  locale
  env | sort | grep -E 'DISPLAY|XMODIFIERS|GPUI_X11_SCALE_FACTOR|XDG_|XIM_ON_THE_SPOT|ONLY_TEXTAREA|IME_BINARY|IME_DRIVER'
} >"$out/provenance.txt"
/usr/bin/python3 "$driver" >"$out/driver.log" 2>&1
