"""Check the saved separate-consumer runtime contracts; no application is launched."""
import gzip
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent / 'runtime'


def walk(node):
    yield node
    for child in node['children']:
        yield from walk(child)


def nodes(step):
    return list(walk(json.loads((ROOT / f'{step}.json').read_text())))


def named(step, name):
    return next(node for node in nodes(step) if node['name'] == name)


def focused(step, name):
    assert 'focused' in named(step, name)['states'], (step, name)


assert named('initial', 'Confirmed')['description'] == '0'
for step in ['opened', 'cancel', 'ok', 'wrap', 'back', 'reopen-keyboard', 'reopen-pointer']:
    tree = nodes(step)
    assert [node['name'] for node in tree if node['role'] == 'dialog'] == ['weGPUI']
    assert not any(node['name'] == 'Open dialog' for node in tree)
    assert named(step, 'A WeUI dialog rendered by native GPUI components.')['role'] == 'label'
focused('opened', 'weGPUI')
focused('cancel', 'Cancel')
focused('ok', 'OK')
focused('wrap', 'Cancel')
focused('back', 'OK')
for step, count in [('confirmed', '1'), ('close-keyboard', '1'), ('confirmed-again', '2')]:
    assert named(step, 'Confirmed')['description'] == count
    assert not any(node['role'] == 'dialog' for node in nodes(step))
    focused(step, 'Open dialog')
app_log = ROOT / 'app.log'
output = app_log.read_text() if app_log.exists() else gzip.decompress(app_log.with_suffix('.log.gz').read_bytes()).decode()
assert 'panicked at' not in output
print('PASS: separate consumer launches, opens by pointer, traps Tab/Shift+Tab and confirms 0→1.')
print('PASS: focus restores; keyboard reopen/Escape preserves 1; pointer reopen/confirm updates 1→2.')
print('PASS: named dialog/body exposed; page hidden while modal; no application panic.')
print('Scope: exact pre-contrast binary, X11 and queried AT-SPI only; no screen-reader or final-source attestation.')
