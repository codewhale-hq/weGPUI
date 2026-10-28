import hashlib,json
from pathlib import Path
ROOT=Path('/src'); PROBE=Path('/work/full-parity/dictionary-combined-native-localized'); OVERLAY=PROBE/'vendor/gpui_pre'
PROPOSAL=PROBE
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def files(root):return {p.relative_to(root).as_posix():p for p in root.rglob('*') if p.is_file() and not any(x in ('.git','target','__pycache__') for x in p.parts)}
def snapshot():
 paths=[ROOT/'Cargo.toml',ROOT/'Cargo.lock']
 for directory in ('src','vendor','assets','examples'):paths.extend(files(ROOT/directory).values())
 out={f'source/{p.relative_to(ROOT).as_posix()}':sha(p) for p in sorted(paths)}
 overlay=files(OVERLAY); orig=files(ROOT/'vendor/gpui_pre')
 proposed_manifest=json.loads((PROPOSAL/'proposal-manifest.json').read_text()); expected={k.removeprefix('vendor/gpui_pre/'):v for k,v in proposed_manifest['proposed_sha256'].items() if k.startswith('vendor/gpui_pre/')}
 if set(overlay)!=set(orig)|set(expected):raise SystemExit('overlay inventory mismatch')
 for name,p in overlay.items():
  wanted=expected.get(name,sha(orig[name]) if name in orig else None)
  if sha(p)!=wanted:raise SystemExit('overlay hash mismatch: '+name)
  out['overlay/'+name]=sha(p)
 for name in ['Cargo.toml','Cargo.lock','proposal-manifest.json','src/main.rs','snapshot.py','build.py','run.sh','session.sh','driver.py']:out['probe/'+name]=sha(PROBE/name)
 return out
