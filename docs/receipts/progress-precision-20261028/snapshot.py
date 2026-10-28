import hashlib, json, subprocess
from pathlib import Path

root = Path('/src')
art = Path('/work/full-parity/progress-precision-probe')
product = art / 'product'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
def inventory(path):
    return {p.relative_to(path).as_posix(): p for p in path.rglob('*') if p.is_file()}
base = {name: root / name for name in ('Cargo.toml', 'Cargo.lock', 'README.md')}
for folder in ('src', 'assets', 'vendor', 'examples'):
    base.update({f'{folder}/{name}': p for name, p in inventory(root / folder).items()})
copy = {'Cargo.toml': product / 'Cargo.toml', 'README.md': product / 'README.md'}
for folder in ('src', 'assets'):
    copy.update({f'{folder}/{name}': p for name, p in inventory(product / folder).items()})
assert set(copy) == {name for name in base if name in ('Cargo.toml', 'README.md') or name.startswith(('src/', 'assets/'))}
for name, path in copy.items():
    if name != 'src/components/progress.rs': assert path.read_bytes() == base[name].read_bytes(), name
expected = base['src/components/progress.rs'].read_text().replace(
    '            let brand = t.brand;\n            let bar = div()',
    '            let brand = t.brand;\n            let value = if self.value == 0. { 0. } else { self.value };\n            let bar = div()').replace(
    '.aria_numeric_value(self.value.round() as f64)', '.aria_numeric_value(value as f64)').replace(
    '.aria_value(SharedString::from(format!("{}%", self.value.round())))',
    '.aria_value(SharedString::from(format!("{value}%")))')
assert copy['src/components/progress.rs'].read_text() == expected
assert (art/'baseline/src/main.rs').read_bytes() == (art/'candidate/src/main.rs').read_bytes()
snapshot = {'head': subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip(),
            'status': subprocess.check_output(['git', '-C', str(root), 'status', '--short'], text=True),
            'base': {name: sha(path) for name, path in sorted(base.items())},
            'candidate_progress': sha(copy['src/components/progress.rs']),
            'fixture': {f'{variant}/{name}': sha(art/variant/name) for variant in ('baseline','candidate') for name in ('Cargo.toml', 'Cargo.lock', 'src/main.rs')},
            'patch': sha(art/'progress.patch')}
print(json.dumps(snapshot, sort_keys=True, indent=2))
