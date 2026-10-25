"""Verify saved real Fcitx composition and native temporary-grab observations."""
import gzip
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent / 'xim-real-grabs'
PRIMARY = 'A你好B e\u0301 👩‍💻 C'
data = json.loads((ROOT / 'runtime.json').read_text())
assert data['result'] == 'PASS' and not data['failures']
cases = {case['label']: case for case in data['observations']}


def primary(case):
    return next(field for field in case['fields'] if field['name'] == 'Primary')


for before, after in [('02-real-preedit-before-grab', '03-preedit-after-grab-ungrab'),
                      ('06-reversed-real-preedit', '07-reversed-preedit-after-grab-ungrab')]:
    assert primary(cases[before])['value'] == primary(cases[after])['value']
    assert cases[before]['changes']['Primary'] == cases[after]['changes']['Primary']
    assert any('field="Primary"' in line and 'marked=Some(1..8)' in line
               for line in cases[after]['native_snapshots'])
commit = cases['04-real-commit']
assert primary(commit)['value'] == PRIMARY.replace('你好', '世界')
assert commit['changes']['Primary'] == 1
assert primary(cases['05-real-commit-undo'])['value'] == PRIMARY
cancel = cases['08-real-cancel']
assert primary(cancel)['value'] == PRIMARY
assert primary(cancel)['selection'] == [[1, 3]]
assert any('field="Primary"' in line and 'marked=None' in line and 'reversed=true' in line
           for line in cancel['native_snapshots'])
assert cancel['changes']['Primary'] == cases['06-reversed-real-preedit']['changes']['Primary']
grabs = [action for action in data['actions'] if action['kind'] == 'actual-keyboard-grab']
assert len(grabs) == 2 and all(action['native_input_focus'] == action['app_window'] for action in grabs)
log = ROOT / '01-observer-initial-application.log'
output = log.read_text() if log.exists() else gzip.decompress(log.with_suffix('.log.gz').read_bytes()).decode()
assert 'panicked at' not in output
print('PASS: real Fcitx preedit/marked range survives actual native grab/ungrab with unchanged X input focus.')
print('PASS: real IME commit emits once; undo restores Unicode text; reversed-selection Escape cancellation restores text/selection without Changed.')
print('Scope: actual XIM input plus native/AT-SPI observations; no screen-reader or complete IME/appearance acceptance.')
