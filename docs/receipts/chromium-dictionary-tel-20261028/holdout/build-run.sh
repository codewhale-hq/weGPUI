#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
: "${CARGO_TARGET_DIR:=/Volumes/VIXinSSD/cw-build-cache/target/wegpui-khmer-stream-probe}"
export CARGO_TARGET_DIR
exec python3 /Volumes/VIXinSSD/CW/codewhale/scripts/build-lock.py /Volumes/VIXinSSD/cw-build-cache/build.lock -- cargo run --locked --offline --manifest-path "$root/Cargo.toml" -- "${1:-$root/expanded-oracle.json}"
