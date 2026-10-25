#!/usr/bin/env bash
# Bundle a native example on an authorized Apple host; never boots or launches a simulator.
set -euo pipefail
if [[ $(uname -s) != Darwin ]]; then
  echo 'An authorized Apple host with Xcode is required; Linux cannot build this bundle.' >&2
  exit 1
fi
example=${1:-gallery}
case "$example" in gallery|minimal) ;; *) echo 'Usage: build-simulator.sh [gallery|minimal]' >&2; exit 2 ;; esac
if (( $# > 1 )); then echo 'Unexpected arguments' >&2; exit 2; fi
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$root"
target=${IOS_SIMULATOR_TARGET:-aarch64-apple-ios-sim}
case "$target" in aarch64-apple-ios-sim|x86_64-apple-ios) ;; *) echo 'Expected an iOS simulator target' >&2; exit 2 ;; esac
export SDKROOT
SDKROOT=$(xcrun --sdk iphonesimulator --show-sdk-path)
export IPHONEOS_DEPLOYMENT_TARGET=15.0
target_dir=${CARGO_TARGET_DIR:-target}
[[ $target_dir == /* ]] || target_dir="$root/$target_dir"
mkdir -p "$target_dir/ios-simulator"
receipt=$(mktemp -d "$target_dir/ios-simulator/$example.XXXXXX")
echo "Build evidence: $receipt"
cargo build --example "$example" --target "$target" --locked \
  --message-format=json-render-diagnostics > "$receipt/build.jsonl" 2> "$receipt/build.stderr"
executable=$(python3 - "$receipt/build.jsonl" "$example" <<'PY'
import json, sys
with open(sys.argv[1]) as stream:
    events = [json.loads(line) for line in stream if line.strip()]
paths = [event['executable'] for event in events
         if event.get('reason') == 'compiler-artifact'
         and event.get('target', {}).get('name') == sys.argv[2]
         and 'example' in event.get('target', {}).get('kind', [])
         and event.get('executable')]
if len(paths) != 1:
    sys.exit('Expected exactly one matching example executable; build evidence retained')
print(paths[0])
PY
)
app="$receipt/WeGPUI.app"
mkdir "$app"
cp "$executable" "$app/wegpui"
cp "$root/examples/ios/Info.plist" "$app/Info.plist"
codesign --force --sign - "$app" > "$receipt/codesign.txt" 2>&1
echo "Built simulator bundle (not launched): $app"
