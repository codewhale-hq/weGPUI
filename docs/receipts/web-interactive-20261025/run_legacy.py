"""Run the historical-image website checker against both real hosting mounts."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('interactive_checks', root / 'website/check_interactive.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
parser = argparse.ArgumentParser()
parser.add_argument('--site', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
if args.out.exists() and any(args.out.iterdir()):
    raise SystemExit('Choose a new output directory; earlier evidence is preserved.')
args.out.mkdir(parents=True, exist_ok=True)
source = root / 'website/check_browser.py'
(args.out / 'checker.py.txt').write_bytes(source.read_bytes())
results = []
for project in (False, True):
    name = 'project-prefix' if project else 'root'
    with module.server(args.site.resolve(), project, args.out / f'{name}-http.log') as base:
        command = [sys.executable, str(source), '--url', base, '--browser', '/usr/bin/chromium',
                   '--out', str(args.out / name)]
        with (args.out / f'{name}-console.log').open('w') as log:
            result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
        results.append({'mount': name, 'command': command, 'exit_code': result.returncode})
        print(name, result.returncode, flush=True)
summary = {'result': 'PASS' if all(item['exit_code'] == 0 for item in results) else 'FAIL',
           'native_gpui_test': False, 'checker_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
           'site': str(args.site.resolve()), 'results': results}
(args.out / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
raise SystemExit(0 if summary['result'] == 'PASS' else 1)
