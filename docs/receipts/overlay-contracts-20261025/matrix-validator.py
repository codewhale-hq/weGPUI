"""Validate actual overlay event counts, focus states and tree observations."""
from pathlib import Path
import re
import sys

base = Path(sys.argv[1])

def tree(case, stage):
    return (base/case/f'{stage}-atspi.txt').read_text()

def events(case):
    return re.findall(r'^event \d+ (.+)$', (base/case/'app.log').read_text(), re.M)

def focus(case, stage, name):
    assert any(repr(name) in line and "'focused'" in line for line in tree(case, stage).splitlines()), (case, stage, name)

for case in ('action', 'android'):
    actual = events(case)
    assert actual.count('menu') == actual.count('tips') == actual.count('warn') == 1, (case, actual)
    assert actual.count('dismiss') == 2, (case, actual)
    assert actual.count('cancel') == (case == 'action'), (case, actual)
    assert "dialog 'Contract actions'" in tree(case, 'warn'), (case, 'menu unexpectedly closed')
    assert "dialog '" not in tree(case, 'escape-dismiss'), case
    focus(case, 'escape-dismiss', 'Open overlay')
    if case == 'android':
        assert "'Cancel action'" not in tree(case, 'opened')

actual = events('dialog')
assert actual.count('confirm') == 2 and actual.count('disabled') == 0 and actual.count('dismiss') == 1, actual
for stage in ('mask-stays', 'disabled-stays'):
    assert "dialog 'Contract dialog'" in tree('dialog', stage), stage
focus('dialog', 'cancel-focused', 'Cancel dialog')
focus('dialog', 'confirm-focused', 'Confirm dialog')
focus('dialog', 'rapid-confirm-closed', 'Open overlay')
actual = events('raw-modal')
assert actual.count('confirm') == 1 and actual.count('dismiss') == 1, actual
focus('raw-modal', 'rapid-confirm-closed', 'Open overlay')
focus('raw-modal', 'rapid-mask-closed', 'Open overlay')

for stage, name in (('opened', 'Option 20'), ('end', 'Option 30'), ('end-disabled-skipped', 'Option 30'),
                    ('home', 'Option 1'), ('option4', 'Option 4'), ('option6', 'Option 6'),
                    ('reopened-selected', 'Option 6')):
    focus('select', stage, name)
assert events('select').count('select=6') == 1, events('select')
assert events('select').count('dismiss') == 3, events('select')
focus('select', 'selected', 'Open overlay')
assert 'disabled-select' not in events('select-disabled')
assert any("'Disabled value'" in line and 'DISABLED' in line for line in tree('select-disabled', 'opened').splitlines())
focus('select-disabled', 'disabled-tab-skipped', 'Open overlay')

actual = events('picker')
assert sum(value.startswith('confirm=') for value in actual) == 1, actual
assert all(value not in ('change=1', 'change=3') for value in actual), actual
assert 'confirm=0' in actual, actual
focus('picker', 'confirmed', 'Open overlay')
log = (base/'picker'/'app.log').read_text()
assert 'values=["0"]' in log, log
actual = events('reentrant')
assert actual.count('reopened-from-confirm') == 1 and actual.count('confirm=0') == 1 and actual.count('close') == 2, actual
assert "dialog 'Contract picker'" in tree('reentrant', 'confirmed')
focus('reentrant', 'reopened-cancel', 'Open overlay')

assert not any(value.startswith('confirm=') for value in events('all-disabled'))
assert "dialog 'Contract picker'" in tree('all-disabled', 'cannot-confirm')
assert any("'Unavailable'" in line and 'DISABLED' in line for line in tree('all-disabled', 'opened').splitlines())

log = (base/'date'/'app.log').read_text()
for value in ('change=2000,3,31', 'change=2000,2,29', 'change=2147483647,2,28',
              'change=2147483646,2,28', 'change=-2147483648,2,28', 'confirm=-2147483648,2,28'):
    assert value in log, (value, log)
assert len(tree('date', 'opened').splitlines()) < 100, 'date tree allocated the entire year range'
focus('date', 'confirmed', 'Open overlay')

assert events('sheet').count('dismiss') == 3, events('sheet')
focus('sheet', 'drag-dismissed', 'Open overlay')
assert "dialog 'Contract sheet'" in tree('sheet', 'rapid-reopen-end')
from PIL import Image
def panel_top(case, stage):
    image = Image.open(base/case/f'{stage}.png').convert('RGB')
    return next(y for y in range(image.height) if min(image.getpixel((24, y))) > 240)
assert panel_top('sheet', 'opened') == panel_top('sheet', 'short-return-end') == panel_top('sheet', 'rapid-reopen-end') == 385
assert panel_top('sheet', 'short-held') == 415 and panel_top('sheet', 'long-held') == 455
assert 385 <= panel_top('sheet', 'short-return') < 415
assert 16 <= panel_top('sheet-short', 'short-viewport') <= 18
print('PASS: overlay callbacks, dismissal/focus matrix, disabled selections, reentrant picker and bounded full-range date runtime.')
