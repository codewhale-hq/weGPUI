#!/usr/bin/env bash
set -euo pipefail
cd /workspace/weGPUI
rustc --edition 2024 --test -C debug-assertions=no -C debuginfo=0 -A dead_code -L dependency=/workspace/.wegpui-cloud/target-198/debug/deps --extern accesskit=/workspace/.wegpui-cloud/target-198/debug/deps/libaccesskit-dda556dc5df9564f.rlib --extern smallvec=/workspace/.wegpui-cloud/target-198/debug/deps/libsmallvec-c17ecce8f563f135.rlib --extern log=/workspace/.wegpui-cloud/target-198/debug/deps/liblog-774efecd15e728c6.rlib --crate-name guard_metadata_extracted docs/receipts/overlay-contracts-20261025/action-guard-fixed-20261025/metadata-extracted.rs.txt -o /workspace/.wegpui-cloud/artifacts/native-guard-approved/guard-metadata-extracted
/workspace/.wegpui-cloud/artifacts/native-guard-approved/guard-metadata-extracted --nocapture
