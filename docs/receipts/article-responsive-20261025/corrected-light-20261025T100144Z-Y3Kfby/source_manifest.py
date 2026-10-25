"""Record tracked and untracked source inputs, including newly added helpers."""
import hashlib
from pathlib import Path
import sys

root = Path(sys.argv[1])
paths = [path for folder in ('src', 'examples') for path in (root / folder).rglob('*')
         if path.is_file()]
paths += [root / 'Cargo.toml', root / 'Cargo.lock']
for path in sorted(paths):
    print(hashlib.sha256(path.read_bytes()).hexdigest(), path.relative_to(root), sep='  ')
