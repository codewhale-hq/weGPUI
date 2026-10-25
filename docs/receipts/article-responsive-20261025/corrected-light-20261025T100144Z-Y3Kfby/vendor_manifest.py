"""Record exact vendored GPUI inputs; the integrating commit preserves the files."""
import hashlib
from pathlib import Path
import sys

root = Path(sys.argv[1])
for path in sorted(root.rglob('*')):
    if path.is_file():
        print(hashlib.sha256(path.read_bytes()).hexdigest(),
              f'vendor/gpui_pre/{path.relative_to(root)}', sep='  ')
