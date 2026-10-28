#!/bin/sh
set -eu
exec python3 /Volumes/VIXinSSD/CW/codewhale/scripts/build-lock.py /Volumes/VIXinSSD/cw-build-cache/build.lock -- docker exec -e CARGO_HOME=/work/cargo -e CARGO_TARGET_DIR=/target -e TMPDIR=/work/tmp wegpui-rtl-qualification python3 /work/full-parity/dictionary-extended/build.py
