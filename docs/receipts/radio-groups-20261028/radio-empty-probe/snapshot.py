import hashlib
import difflib
from pathlib import Path

root = Path('/work/full-parity/radio-empty-probe')
product = Path('/src')
overlay = root / 'wegpui'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def files(directory):
    return {p.relative_to(directory).as_posix(): p for p in directory.rglob('*') if p.is_file()
            and not any(part in ('.git', 'target', '__pycache__') for part in p.parts)}

def snapshot():
    source = files(product / 'src'); candidate = files(overlay / 'src')
    if source.keys() != candidate.keys():
        raise SystemExit('Product and overlay source inventories differ')
    radio = 'components/radio.rs'
    baseline_radio = '72da11caf66938e51e01d443e4bb96a831ccd855dde4d8deefa3ac087b4f09f9'
    candidate_radio = '6eb2942a4421671a2855fc59bc2a2ad44c929143e4d6a799c091e464aaa7f759'
    if sha(source[radio]) != baseline_radio or sha(candidate[radio]) != candidate_radio:
        raise SystemExit('Radio baseline or candidate differs from frozen proof')
    expected_patch = ''.join(difflib.unified_diff(source[radio].read_text().splitlines(True),
        candidate[radio].read_text().splitlines(True), fromfile='product/radio.rs', tofile='proof/radio.rs'))
    if (root / 'radio-group.patch').read_text() != expected_patch:
        raise SystemExit('Radio patch is not the exact overlay diff')
    for name in source:
        if name != radio and sha(source[name]) != sha(candidate[name]):
            raise SystemExit(f'Overlay source differs from product: {name}')
    src_assets = files(product / 'assets'); overlay_assets = files(overlay / 'assets')
    if src_assets.keys() != overlay_assets.keys() or any(sha(src_assets[k]) != sha(overlay_assets[k]) for k in src_assets):
        raise SystemExit('Overlay assets differ from product')
    if sha(product / 'Cargo.toml') != sha(overlay / 'Cargo.toml'):
        raise SystemExit('Overlay package manifest differs from product')
    result = {f'product/src/{k}': sha(v) for k,v in sorted(source.items())}
    result.update({f'overlay/src/{k}': sha(v) for k,v in sorted(candidate.items())})
    result.update({f'product/assets/{k}': sha(v) for k,v in sorted(src_assets.items())})
    for name in ('gpui_pre', 'gpui_pre_linux', 'gpui_pre_ios', 'accesskit_unix', 'accesskit_atspi_common'):
        result.update({f'product/vendor/{name}/{k}': sha(v) for k,v in sorted(files(product / 'vendor' / name).items())})
    for name in ('Cargo.toml', 'Cargo.lock', 'src/main.rs', 'check.py', 'session.sh', 'run.sh', 'snapshot.py', 'build.py', 'build.sh', 'verify.py', 'README.md', 'radio-group.patch'):
        result['fixture/' + name] = sha(root / name)
    result['product/Cargo.toml'] = sha(product / 'Cargo.toml')
    return result
