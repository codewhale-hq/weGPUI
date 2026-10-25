"""Native SearchBar callback/focus/state contracts via XTEST and separate AT-SPI queries."""
from collections import Counter
import json
from pathlib import Path
import re
import subprocess
import sys
import time

import pyatspi

OUT = Path(sys.argv[1])
WINDOW = sys.argv[2]
LOG = OUT / 'application.log'
BUTTONS = {'button', 'push button'}


def nodes():
    result = []

    def visit(node):
        if node is None:
            return
        result.append(node)
        for child in node:
            visit(child)

    visit(pyatspi.Registry.getDesktop(0))
    return result


def find(name, button=False):
    matches = [node for node in nodes() if node.name == name and
               ((node.getRoleName() in BUTTONS) == button)]
    assert len(matches) == 1, (name, [(node.getRoleName(), node.name) for node in matches])
    return matches[0]


def frame():
    return next(node for node in nodes() if node.getRoleName() == 'frame')


def rect(node):
    value = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    origin = frame().queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return value.x - origin.x, value.y - origin.y, value.width, value.height


def field(name):
    return find(name)


def text(name):
    return field(name).queryText().getText(0, -1)


def buttons_for(name, label):
    _, y, _, height = rect(field(name))
    return [node for node in nodes() if node.getRoleName() in BUTTONS and node.name == label
            and abs((lambda r: r[1] + r[3] / 2)(rect(node)) - (y + height / 2)) < 25]


def scoped_button(name, label):
    matches = buttons_for(name, label)
    assert len(matches) == 1, (name, label, len(matches))
    return matches[0]


def click_rect(value):
    x, y, width, height = value
    subprocess.run(['xdotool', 'mousemove', '--window', WINDOW, str(x + width // 2),
                    str(y + height // 2), 'click', '1'], check=True)
    time.sleep(.2)


def click(node):
    click_rect(rect(node))


def key(*keys):
    for value in keys:
        subprocess.run(['xdotool', 'key', '--clearmodifiers', value], check=True)
        time.sleep(.2)


def type_text(value):
    subprocess.run(['xdotool', 'type', '--clearmodifiers', '--delay', '40', value], check=True)
    time.sleep(.25)


def events():
    return Counter('/'.join(match.groups()) for match in re.finditer(r'^(\w+) event=(\w+)$', LOG.read_text(), re.M))


def expect_delta(before, name, count):
    actual = events()[name] - before[name]
    assert actual == count, (name, actual, count, events(), before)


def focused():
    return [node.name for node in nodes() if node.getState().contains(pyatspi.STATE_FOCUSED)]


def snapshot(name):
    key('F8')
    subprocess.run(['import', '-window', WINDOW, str(OUT / f'{name}.png')], check=True)
    tree = [{'role': node.getRoleName(), 'name': node.name,
             'states': [pyatspi.stateToString(state) for state in node.getState().getStates()],
             'bounds': rect(node) if node.getRoleName() != 'application' else None}
            for node in nodes() if node.getRoleName() != 'desktop frame']
    (OUT / f'{name}-atspi.json').write_text(json.dumps(tree, indent=2))
    state = {'step': name, 'events': dict(events()), 'focused': focused(),
             'outlined': text('Outlined search'), 'filled': text('Filled search')}
    with (OUT / 'steps.jsonl').open('a') as output:
        output.write(json.dumps(state) + '\n')


assert text('Outlined search') == 'preserved'
assert not buttons_for('Outlined search', '清除')
assert any(node.name == 'Keyword' and node.getRoleName() == 'label' for node in nodes())
click(find('拍照', button=True))
snapshot('initial-readonly-keyword')

key('F1')
snapshot('homepage-api-focus')
assert 'Home search' in focused(), focused()
assert 'home_focused=true' in LOG.read_text().split('snapshot')[-1]
key('Tab', 'shift+Tab')
snapshot('homepage-keyboard-ring')
before = events()
click(find('Home search', button=True))
key('Return', 'space')
expect_delta(before, 'home/Open', 3)
before = events()
click(find('拍照', button=True))
key('Return', 'space')
expect_delta(before, 'home/Camera', 3)
assert {name for name in events() if name.startswith('home/')} == {'home/Open', 'home/Camera'}
snapshot('homepage-events')

before = events()
key('F2')
expect_delta(before, 'outlined/Cancel', 1)
expect_delta(before, 'outlined/Changed', 0)
assert text('Outlined search') == 'preserved'
snapshot('outlined-initial-cancel-preserves')
before = events()
key('F2')
expect_delta(before, 'outlined/Cancel', 1)

before = events()
click(scoped_button('Outlined search', '搜索'))
key('Return', 'space')
expect_delta(before, 'outlined/Submit', 3)
assert text('Outlined search') == 'preserved'

before = events()
click(field('Outlined search'))
key('ctrl+a')
type_text('blocked')
key('BackSpace', 'Delete')
assert text('Outlined search') == 'preserved'
expect_delta(before, 'outlined/Changed', 0)
snapshot('readonly-text-unchanged')

key('F3')
assert text('Outlined search') == 'editable'
assert buttons_for('Outlined search', '清除')
key('ctrl+a')
type_text('alpha')
assert text('Outlined search') == 'alpha'
clear_rect = rect(scoped_button('Outlined search', '清除'))
snapshot('editable-clear-visible')

before = events()
key('F4')
snapshot('readonly-clear-hidden')
assert not buttons_for('Outlined search', '清除'), 'readonly clear button remains exposed'
key('ctrl+a')
type_text('blocked')
key('BackSpace', 'Delete')
click_rect(clear_rect)
assert text('Outlined search') == 'alpha'
expect_delta(before, 'outlined/Changed', 0)
expect_delta(before, 'outlined/Clear', 0)

key('F5')
assert buttons_for('Outlined search', '清除')
before = events()
click(scoped_button('Outlined search', '清除'))
assert text('Outlined search') == ''
expect_delta(before, 'outlined/Clear', 1)
expect_delta(before, 'outlined/Changed', 1)
assert 'Outlined search' in focused(), focused()
snapshot('clear-empty-restores-input-focus')

before = events()
key('Return')
expect_delta(before, 'outlined/Submit', 1)
before = events()
click(scoped_button('Outlined search', '返回'))
key('Return', 'space')
expect_delta(before, 'outlined/Back', 3)
before = events()
click(scoped_button('Outlined search', '取消'))
key('Return', 'space')
expect_delta(before, 'outlined/Cancel', 3)
snapshot('outlined-buttons-events')

click(find('Filled search', button=True))
type_text('beta')
assert text('Filled search') == 'beta'
before = events()
key('Return')
expect_delta(before, 'filled/Submit', 1)
snapshot('filled-typed-submit')
before = events()
click(scoped_button('Filled search', '清除'))
assert text('Filled search') == ''
expect_delta(before, 'filled/Clear', 1)
expect_delta(before, 'filled/Changed', 1)
assert 'Filled search' in focused(), focused()
type_text('again')
before = events()
click(scoped_button('Filled search', '取消'))
assert text('Filled search') == ''
expect_delta(before, 'filled/Cancel', 1)
expect_delta(before, 'filled/Changed', 1)
assert find('Filled search', button=True)
snapshot('filled-cancel-clears')

(OUT / 'events.json').write_text(json.dumps(dict(events()), indent=2))
print('PASS search contracts', dict(events()))
