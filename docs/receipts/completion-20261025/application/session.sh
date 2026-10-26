set -eu
: "${CARGO_TARGET_DIR:?Set this to the rebuilt preview workspace target directory}"
export CARGO_TARGET_DIR XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d)
trap 'rm -rf "$XDG_RUNTIME_DIR"' EXIT
export XDG_CURRENT_DESKTOP=GNOME GPUI_X11_SCALE_FACTOR=1
unset WAYLAND_DISPLAY
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do
    if [ -f "$icd" ]; then export VK_ICD_FILENAMES="$icd"; break; fi
done
RECEIPT_DRIVER="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/drive.py"
export RECEIPT_DRIVER
xvfb-run -a -s '-screen 0 1280x1024x24' dbus-run-session -- bash -c '
set -eu
openbox --sm-disable >/dev/null 2>&1 &
/usr/libexec/at-spi-bus-launcher --launch-immediately >/dev/null 2>&1 &
sleep 1
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >/dev/null
python3 -I "$RECEIPT_DRIVER"
'
