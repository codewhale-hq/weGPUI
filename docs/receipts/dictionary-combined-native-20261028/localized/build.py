import json,os,subprocess
from pathlib import Path
from snapshot import snapshot,sha
root=Path('/work/full-parity/dictionary-combined-native-localized'); before=snapshot()
command=['cargo','build','--manifest-path',str(root/'Cargo.toml'),'--locked','--offline']
subprocess.run(command,check=True,env=os.environ.copy())
if before!=snapshot():raise SystemExit('source changed during build')
binary=Path(os.environ['CARGO_TARGET_DIR'])/'debug'/'wegpui-dictionary-combined-native-localized-probe'
(root/'build-manifest.json').write_text(json.dumps({'source':before,'binary_sha256':sha(binary),'command':command},indent=2,sort_keys=True)+'\n')
print('build manifest recorded')
