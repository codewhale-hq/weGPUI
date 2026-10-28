import json
from pathlib import Path
from snapshot import snapshot, sha
root = Path('/work/full-parity/radio-set-probe')
manifest = json.loads((root / 'build-manifest.json').read_text())
if manifest['source'] != snapshot() or manifest['binary_sha256'] != sha(Path('/target/debug/radio_set_probe')):
    raise SystemExit('Source or binary differs from build manifest')
