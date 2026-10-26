set -eu
export XDG_RUNTIME_DIR
XDG_RUNTIME_DIR=$(mktemp -d)
export GPUI_X11_SCALE_FACTOR=1 XDG_CURRENT_DESKTOP=GNOME
unset WAYLAND_DISPLAY
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do if [ -f "$icd" ]; then export VK_ICD_FILENAMES="$icd"; break; fi; done
xvfb-run -a -s '-screen 0 1280x1024x24' dbus-run-session -- bash -c '
openbox --sm-disable > /dev/null 2>&1 &
/usr/libexec/at-spi-bus-launcher --launch-immediately > /dev/null 2>&1 &
python3 -I <evidence>/layers-interaction/wait-wm.py
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true > /dev/null
for mode in light dark light-care dark-care light-brand dark-brand light-care-brand dark-care-brand; do
for variant in grey filled outlined; do
python3 -I <evidence>/search-contracts/drive.py "$mode" "$variant" || exit $?
done
done'
