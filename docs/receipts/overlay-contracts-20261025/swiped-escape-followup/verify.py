"""Verify saved native Escape propagation and controlled callbacks across Modal/SwipedCell."""
import json
from pathlib import Path
import re
import sys

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent / 'after'


def walk(node):
    yield node
    for child in node['children']:
        yield from walk(child)


def nodes(step):
    return list(walk(json.loads((ROOT / f'{step}.json').read_text())))


def named(step, name):
    matches = [node for node in nodes(step) if node['name'] == name]
    matches.sort(key=lambda node: 'focusable' not in node['states'])
    assert matches, (step, name)
    return matches[0]


def focused(step, name):
    assert 'focused' in named(step, name)['states'], (step, name)


row = 'Saved custom action row'
for step in ['initially-collapsed', 'expanded', 'action-focused', 'row-closed-modal-open']:
    assert named(step, 'Slow modal')['role'] == 'dialog'
    assert not any(node['name'] == 'Open slow modal' for node in nodes(step))
focused('initially-collapsed', row)
assert 'collapsed' in named('initially-collapsed', row)['states']
assert 'expanded' in named('expanded', row)['states']
focused('action-focused', 'Archive')
focused('row-closed-modal-open', row)
assert 'collapsed' in named('row-closed-modal-open', row)['states']
for step in ['dismissed-from-collapsed', 'dismissed-after-row']:
    assert not any(node['role'] == 'dialog' for node in nodes(step))
    focused(step, 'Open slow modal')
output = (ROOT / 'app.txt').read_text()
assert len(re.findall(r'^\d+ open$', output, re.MULTILINE)) == 2
assert len(re.findall(r'^\d+ hide$', output, re.MULTILINE)) == 2
assert re.findall(r'request=(\d+) open=(true|false) presence_open=(true|false)', output) == [
    ('1', 'true', 'true'), ('2', 'false', 'true')]
assert 'panicked at' not in output
print('PASS: Escape on initially collapsed focused row dismisses Modal and restores opener focus.')
print('PASS: Escape from open row action closes only row/restores row focus; second Escape dismisses Modal.')
print('PASS: exactly one controlled open and close callback, both while Modal open; two modal hides, no panic.')
print('Scope: native X11 keys/pointer plus queried AT-SPI; no screen-reader, all-theme or cross-host acceptance.')
