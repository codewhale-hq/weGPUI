#!/bin/sh
set -eu
exec python3 /Volumes/VIXinSSD/CW/codewhale/scripts/build-lock.py /Volumes/VIXinSSD/cw-build-cache/build.lock -- docker exec -e CARGO_TARGET_DIR=/target wegpui-rtl-qualification python3 /work/full-parity/radio-set-probe/build.py
