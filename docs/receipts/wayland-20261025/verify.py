"""Check recorded Wayland runtime contracts; visual review remains separate."""
import gzip
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def walk(node):
    yield node
    for child in node['children']:
        yield from walk(child)


def nodes(run, step):
    return list(walk(json.loads((ROOT / run / f'{step}-details.json').read_text())))


def named(run, step, name):
    matches = [node for node in nodes(run, step) if node['name'] == name]
    assert len(matches) == 1, (run, step, name, len(matches))
    return matches[0]


def focused(run, step, name):
    lines = (ROOT / run / f'{step}-atspi.txt').read_text().splitlines()
    assert any(repr(name) in line and "'focused'" in line for line in lines), (run, step, name)


def protocol(run):
    path = ROOT / run / 'wayland-protocol.log'
    if path.exists():
        return path.read_text()
    return gzip.decompress(path.with_suffix('.log.gz').read_bytes()).decode()


minimal = 'minimal-followup'
assert named(minimal, 'initial', 'Confirmed')['description'] == '0'
assert named(minimal, 'confirm', 'Confirmed')['description'] == '1'
assert named(minimal, 'open', 'A WeUI dialog rendered by native GPUI components.')['role'] == 'label'
for step, name in [('cancel', 'Cancel'), ('ok', 'OK'), ('wrap', 'Cancel'), ('back', 'OK'),
                   ('confirm', 'Open dialog'), ('dismiss', 'Open dialog')]:
    focused(minimal, step, name)
trace = protocol(minimal)
assert 'configure(640, 600' in trace and 'configure(900, 800' in trace

nested = 'nested-modals-final'
for step in ['inner', 'cancel', 'ok', 'wrap', 'back']:
    dialogs = [node['name'] for node in nodes(nested, step) if node['role'] == 'dialog']
    assert dialogs == ['Inner dialog'], (step, dialogs)
    assert not any(node['name'] in ['Outer sheet', 'Open inner dialog', 'Close sheet']
                   for node in nodes(nested, step))
focused(nested, 'closeinner', 'Open inner dialog')
focused(nested, 'closeouter', 'Open sheet')

component = 'component-states-final'
for step, expected in [('text', 'wayland'), ('copy', 'wayland'), ('paste', 'waylandwayland'),
                       ('undo', 'wayland'), ('redo', 'waylandwayland')]:
    entry = named(component, step, 'Display name')
    assert entry['text'] == expected, (step, entry)
    assert entry['caret_offset'] == len(expected), (step, entry)
assert named(component, 'copy', 'Display name')['selections'] == [[0, 7]]
focused(component, 'next', 'Send code')
assert protocol(component).count('saves=1') == 1
assert 'saves=2' not in protocol(component)

bounds = 'component-boundaries-final'
notes = 'First e\u0301\nFamily 👩‍👩‍👧‍👦\n'
assert len(notes) == 24
assert named(bounds, 'initial', 'Unicode sample')['text'] == 'e\u0301 👩‍👩‍👧‍👦'
assert named(bounds, 'initial', 'Password sample')['text'] == '•••'
for step, caret, selections in [('home', 0, []), ('end', 24, []),
                                ('selectstart', 0, [[0, 24]]),
                                ('selectend', 24, [[0, 24]])]:
    entry = named(bounds, step, 'Multiline sample')
    assert entry['text'] == notes, (step, entry)
    assert entry['caret_offset'] == caret, (step, entry)
    assert entry['selections'] == selections, (step, entry)
    focused(bounds, step, 'Multiline sample')

for run in [minimal, nested, component, bounds]:
    trace = protocol(run)
    assert 'wl_keyboard#' in trace and '.key(' in trace
    if run != bounds:
        assert 'wl_pointer#' in trace and '.button(' in trace
    assert 'panicked at' not in trace
print('PASS: Wayland dialog callback/value, focus/trap, resize, nested modal isolation,')
print('      input Text/caret/selection, clipboard/undo/redo and loading guard snapshots.')
print('PASS: Unicode/masked password Text readback and Ctrl+Home/End/Shift document boundaries.')
print('Scope: recorded samples only; screenshots reviewed separately; no screen-reader verdict.')
