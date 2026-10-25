"""Verify saved new-core AT-SPI state, bounds, Wayland focus and text observations."""
import gzip
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent
WORDS = 'alpha e\u0301 beta gamma'
NOTES = ('A wrapped paragraph with enough words to cross several visual rows. '
         'Combining e\u0301 and family 👩‍👩‍👧‍👦 stay intact.\nSecond hard line.\n')


def walk(node):
    yield node
    for child in node['children']:
        yield from walk(child)


def nodes(run, step='initial'):
    name = f'{step}-details.json' if run in ['media', 'nested', 'text'] else f'{step}.json'
    return list(walk(json.loads((ROOT / run / name).read_text())))


def named(run, name, step='initial'):
    matches = [node for node in nodes(run, step) if node['name'] == name]
    matches.sort(key=lambda node: 'focusable' not in node['states'])
    assert matches, (run, step, name)
    return matches[0]


def focused(run, name, step):
    assert 'focused' in named(run, name, step)['states'], (run, name, step)


for step in ['initial', 'row', 'close']:
    states = named('media', 'Controlled row', step)['states']
    assert 'collapsed' in states and 'expandable' in states and 'expanded' not in states
states = named('media', 'Controlled row', 'open')['states']
assert 'expanded' in states and 'expandable' in states and 'collapsed' not in states
focused('media', 'Archive', 'first')
focused('media', 'Controlled row', 'close')
for step in ['inner', 'cancel', 'ok', 'wrap', 'back']:
    assert [node['name'] for node in nodes('nested', step) if node['role'] == 'dialog'] == ['Inner dialog']
focused('nested', 'OK', 'ok')
focused('nested', 'Cancel', 'wrap')
focused('nested', 'OK', 'back')
focused('nested', 'Open inner dialog', 'closeinner')
focused('nested', 'Open sheet', 'closeouter')
assert named('text', 'Word editing')['text'] == WORDS
assert named('text', 'Word editing', 'word-end')['caret_offset'] == len(WORDS)
assert named('text', 'Word editing', 'word-selected')['selections'] == [[0, len(WORDS)]]
assert named('text', 'Wrapped notes', 'notes-end')['text'] == NOTES
assert named('text', 'Wrapped notes', 'notes-end')['caret_offset'] == len(NOTES)
assert named('text', 'Wrapped notes', 'notes-selected')['selections'] == [[0, len(NOTES)]]
for run in ['text', 'x11-input', 'x11-text-bounds']:
    readonly = named(run, 'Read only')['states']
    disabled = named(run, 'Disabled')['states']
    invalid = named(run, 'UTF16 limit')['states']
    assert 'read only' in readonly and 'editable' not in readonly
    assert not any(state in disabled for state in ['enabled', 'sensitive', 'focusable', 'editable'])
    assert 'invalid entry' in invalid
for kind in range(3):
    assert 'busy' in named('x11-cells', f'cell-{kind}-2')['states']
    disabled = named('x11-cells', f'cell-{kind}-1')['states']
    assert 'enabled' not in disabled and 'sensitive' not in disabled
headings = [node for node in nodes('x11-headings') if node['role'] == 'heading']
assert len(headings) == 2 and all('level:6' in node['attributes'] for node in headings)


def origin(step):
    text = (ROOT / 'x11-text-bounds' / f'{step}-xwininfo.txt').read_text()
    return [int(re.search(r'Absolute upper-left X:\s*(-?\d+)', text).group(1)),
            int(re.search(r'Absolute upper-left Y:\s*(-?\d+)', text).group(1))]


characters = []
for step in ['initial', 'moved']:
    frame = next(node for node in nodes('x11-text-bounds', step) if node['role'] == 'frame')
    assert frame['bounds'][:2] == origin(step)
    word = named('x11-text-bounds', 'Word editing', step)
    assert word['text'] == WORDS and word['caret_offset'] == len(WORDS)
    desktop, relative = word['first_character_desktop'], word['first_character_window']
    assert desktop[2] > 0 and desktop[3] > 0
    assert desktop[:2] == [relative[i] + origin(step)[i] for i in range(2)]
    characters.append(desktop)
assert [characters[1][i] - characters[0][i] for i in range(2)] == [200, 120]
for run in ['media', 'nested', 'text']:
    path = ROOT / run / 'wayland-protocol.log'
    trace = path.read_text() if path.exists() else gzip.decompress(path.with_suffix('.log.gz').read_bytes()).decode()
    assert 'wl_keyboard#' in trace and '.key(' in trace and 'xdg_toplevel#' in trace
    assert 'panicked at' not in trace
print('PASS: new AT-SPI expanded/collapsed, invalid, busy, readonly/disabled and heading level.')
print('PASS: Wayland media actions, nested modal isolation/focus and Text/caret/selection regression.')
print('PASS: X11 frame and first-character desktop bounds match native client origin and window move.')
print('Scope: recorded Linux samples only; tree/interface observations are not screen-reader acceptance.')
