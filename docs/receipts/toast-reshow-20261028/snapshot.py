import hashlib,json,pathlib,subprocess
root=pathlib.Path('/src'); here=pathlib.Path('/work/full-parity/toast-reshow-probe'); prod=here/'product'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
paths=[pathlib.Path('Cargo.toml'),pathlib.Path('README.md')]
for directory in ['src','assets']:
 paths.extend(p.relative_to(root) for p in (root/directory).rglob('*') if p.is_file())
for p in paths:
 if str(p)!='src/components/modal.rs': assert sha(root/p)==sha(prod/p),str(p)
r=subprocess.run(['diff','-u','--label','a/src/components/modal.rs','--label','b/src/components/modal.rs',str(root/'src/components/modal.rs'),str(prod/'src/components/modal.rs')],capture_output=True,text=True)
assert r.returncode==1 and r.stdout==(here/'presence.patch').read_text()
assert (here/'baseline/src/main.rs').read_bytes()==(here/'candidate/src/main.rs').read_bytes()
tracked=subprocess.check_output(['git','-C',str(root),'ls-files','-z','--','Cargo.toml','Cargo.lock','src','assets','vendor','examples/support/platform.rs']).split(b'\0')
files={s.decode():sha(root/s.decode()) for s in tracked if s and (root/s.decode()).is_file()}
fixtures={str(p.relative_to(here)):sha(p) for p in [here/'presence.patch',here/'candidate/src/main.rs',here/'baseline/Cargo.toml',here/'candidate/Cargo.toml',here/'baseline/Cargo.lock',here/'candidate/Cargo.lock',prod/'src/components/modal.rs']}
print(json.dumps({'head':subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),'source':files,'fixture':fixtures},sort_keys=True,indent=2))
