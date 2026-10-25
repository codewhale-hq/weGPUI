"""Check saved native Wayland nested-modal regression on the approved common GPUI source."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent / 'wayland-nested'


def walk(node):
    yield node
    for child in node['children']:
        yield from walk(child)


def nodes(step):
    return list(walk(json.loads((ROOT / f'{step}-details.json').read_text())))


def focused(step, name):
    assert any(node['name'] == name and 'focused' in node['states'] for node in nodes(step))


for step in ['inner', 'cancel', 'ok', 'wrap', 'back']:
    assert [node['name'] for node in nodes(step) if node['role'] == 'dialog'] == ['Inner dialog']
focused('cancel', 'Cancel')
focused('ok', 'OK')
focused('wrap', 'Cancel')
focused('back', 'OK')
focused('closeinner', 'Open inner dialog')
focused('closeouter', 'Open sheet')
assert 'app DISPLAY: absent' in (ROOT / 'provenance.txt').read_text()
print('PASS: real Wayland nested modal tree isolation, Tab/Shift+Tab trap and sequential Escape focus restoration.')
print('Scope: new approved common-GPUI artifact; no Wayland screen-reader/global-origin or full platform acceptance.')
