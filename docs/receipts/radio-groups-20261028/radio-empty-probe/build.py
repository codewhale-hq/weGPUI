import json
import os
import subprocess
from pathlib import Path
from snapshot import snapshot, sha
root = Path('/work/full-parity/radio-empty-probe')
before = snapshot()
command = ['cargo', 'build', '--manifest-path', str(root / 'Cargo.toml'), '--locked', '--offline']
subprocess.run(command, check=True, env=os.environ.copy())
if snapshot() != before:
    raise SystemExit('Source changed during build; the binary is unqualified')
binary = Path('/target/debug/radio_empty_probe')
(root / 'build-manifest.json').write_text(json.dumps({'source': before, 'binary_sha256': sha(binary), 'command': command}, indent=2, sort_keys=True) + '\n')
