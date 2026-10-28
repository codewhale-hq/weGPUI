import difflib,hashlib,json,subprocess
from pathlib import Path
root=Path('/src'); art=Path('/work/full-parity/date-canada-candidate'); product=art/'product'
def files(p): return {x.relative_to(p).as_posix():x for x in p.rglob('*') if x.is_file()}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
base={n:root/n for n in ('Cargo.toml','Cargo.lock','README.md')}
for folder in ('src','assets','vendor','examples'):
    base.update({f'{folder}/{name}':p for name,p in files(root/folder).items()})
candidate={'Cargo.toml':product/'Cargo.toml','README.md':product/'README.md'}
for folder in ('src','assets'):
    candidate.update({f'{folder}/{name}':p for name,p in files(product/folder).items()})
assert set(candidate)=={name for name in base if name in ('Cargo.toml','README.md') or name.startswith(('src/','assets/'))}
for name,p in candidate.items():
    if name!='src/components/date_field.rs': assert p.read_bytes()==base[name].read_bytes(),name
before=base['src/components/date_field.rs'].read_text();after=candidate['src/components/date_field.rs'].read_text()
assert ''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/src/components/date_field.rs',tofile='b/src/components/date_field.rs'))==(art/'canada.patch').read_text()
assert (art/'src/main.rs').read_bytes()==Path('/work/full-parity/date-canada-probe/src/main.rs').read_bytes()
out={'head':subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),
     'status':subprocess.check_output(['git','-C',str(root),'status','--short'],text=True),
     'base':{n:sha(p) for n,p in sorted(base.items())},'candidate_date':sha(candidate['src/components/date_field.rs']),
     'fixture':{n:sha(art/n) for n in ('Cargo.toml','Cargo.lock','src/main.rs','canada.patch')}}
print(json.dumps(out,sort_keys=True,indent=2))
