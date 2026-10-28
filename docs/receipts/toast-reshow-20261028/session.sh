#!/usr/bin/env bash
set -euo pipefail
out=/work/full-parity/toast-reshow-probe
/usr/libexec/at-spi-bus-launcher --launch-immediately >"$out/atspi-bus.log" 2>&1 & bus=$!
trap 'kill "$bus" 2>/dev/null || true' EXIT
dbus-send --session --print-reply --dest=org.a11y.Bus /org/a11y/bus org.freedesktop.DBus.Properties.Set string:org.a11y.Status string:IsEnabled variant:boolean:true >"$out/atspi-enable.txt"
/usr/bin/python3 -I "$out/driver.py"
