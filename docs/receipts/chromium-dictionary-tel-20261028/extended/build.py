import json
import os
import subprocess
from pathlib import Path
from snapshot import snapshot, sha

root = Path('/work/full-parity/dictionary-extended')
before = snapshot()
command = ['cargo', 'build', '--manifest-path', str(root / 'Cargo.toml'), '--locked', '--offline']
print('building:', command, flush=True)
subprocess.run(command, check=True, env=os.environ.copy())
after = snapshot()
if before != after:
    raise SystemExit('Source or corpus changed during build; build is not qualified')
binary = Path('/target/debug/wegpui-dictionary-extended-probe')
(root / 'build-manifest.json').write_text(json.dumps({
    'format': 1, 'source': before, 'binary_sha256': sha(binary), 'command': command,
}, indent=2, sort_keys=True) + '\n')
print('build manifest:', root / 'build-manifest.json')
