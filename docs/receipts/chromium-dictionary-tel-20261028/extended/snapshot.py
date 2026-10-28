import hashlib
from pathlib import Path

ROOT = Path('/src')
PROBE = Path('/work/full-parity/dictionary-extended')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snapshot():
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock']
    for directory in ('src', 'vendor', 'assets', 'examples'):
        paths.extend(p for p in (ROOT / directory).rglob('*') if p.is_file()
                     and not any(part in ('target', '__pycache__', '.git') for part in p.parts))
    result = {f'source/{p.relative_to(ROOT).as_posix()}': sha(p) for p in sorted(paths)}
    for name in ('Cargo.toml', 'Cargo.lock', 'src/main.rs', 'corpus.json'):
        result['probe/' + name] = sha(PROBE / name)
    return result
