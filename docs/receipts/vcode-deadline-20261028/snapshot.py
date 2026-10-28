import hashlib, json, subprocess
from pathlib import Path

root = Path('/src')
art = Path('/work/full-parity/vcode-deadline-probe')
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
    if name != 'src/components/vcode.rs': assert path.read_bytes() == base[name].read_bytes(), name
expected = base['src/components/vcode.rs'].read_text().replace(
    '        self.remaining = self.seconds;\n        cx.emit(VcodeEvent::Send);',
    '        let seconds = self.seconds;\n        let started = cx.background_executor().now();\n        self.remaining = seconds;\n        cx.emit(VcodeEvent::Send);').replace(
    '                let done = this\n',
    '                let elapsed = cx\n                    .background_executor()\n                    .now()\n                    .saturating_duration_since(started);\n                let done = this\n').replace(
    '                        this.remaining = this.remaining.saturating_sub(1);',
    '                        this.remaining = seconds\n                            .saturating_sub(elapsed.as_secs().min(u64::from(seconds)) as u32);')
assert copy['src/components/vcode.rs'].read_text() == expected
assert (art/'baseline/src/main.rs').read_bytes() == (art/'candidate/src/main.rs').read_bytes()
snapshot = {'head': subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip(),
            'status': subprocess.check_output(['git', '-C', str(root), 'status', '--short'], text=True),
            'base': {name: sha(path) for name, path in sorted(base.items())},
            'candidate_vcode': sha(copy['src/components/vcode.rs']),
            'fixture': {f'{variant}/{name}': sha(art/variant/name) for variant in ('baseline','candidate') for name in ('Cargo.toml', 'Cargo.lock', 'src/main.rs')},
            'patch': sha(art/'vcode.patch')}
print(json.dumps(snapshot, sort_keys=True, indent=2))
