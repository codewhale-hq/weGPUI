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


def focused_nodes():
    return [node for node in nodes() if node.getState().contains(pyatspi.STATE_FOCUSED)]


def names():
    return [(node.getRoleName(), node.name) for node in focused_nodes()]


def snapshot(name):
    time.sleep(.15)
    subprocess.run(['import', '-window', WINDOW, str(OUT / f'{name}.png')], check=True)
    tree = [{'role': node.getRoleName(), 'name': node.name,
             'states': [pyatspi.stateToString(state) for state in node.getState().getStates()],
             'bounds': rect(node) if node.getRoleName() != 'application' else None}
            for node in nodes() if node.getRoleName() != 'desktop frame']
    (OUT / f'{name}-atspi.json').write_text(json.dumps(tree, indent=2))
    state = {'step': name, 'focused': names()}
    with (OUT / 'steps.jsonl').open('a') as output:
        output.write(json.dumps(state) + '\n')
    print(json.dumps(state), flush=True)


def tab_to(name, role=None):
    for _ in range(35):
        if any(node.name == name and (role is None or node.getRoleName() == role)
               for node in focused_nodes()):
            return
        key('Tab')
    raise AssertionError(('tab target', name, role, names()))


# A real click gives the window an initial focused child before shortcut driving.
click(find('Default', button=True))
for shortcut, label, shot in [('F1', 'Default', 'default'), ('F2', 'Primary', 'primary'),
                              ('F3', 'Warn', 'warn'), ('F6', 'Homepage search', 'homepage')]:
    key(shortcut)
    assert any(node.name == label for node in focused_nodes()), (label, names())
    snapshot(shot)
for label, shot in [('拍照', 'camera'), ('返回', 'search-back'), ('清除', 'search-clear'),
                    ('搜索', 'search-submit'), ('取消', 'search-cancel'),
                    ('Standalone checkbox', 'checkbox-off')]:
    tab_to(label)
    snapshot(shot)
key('space')
snapshot('checkbox-on')
tab_to('Standalone switch')
snapshot('switch-on')
tab_to('Checkbox row')
snapshot('checkbox-row')
tab_to('Radio first')
snapshot('radio')
key('F4')
time.sleep(.7)
assert any(node.getRoleName() == 'combo box' for node in focused_nodes()), names()
snapshot('picker')
key('Escape')
time.sleep(.5)
key('F5')
time.sleep(.5)
tab_to('Focus image', 'image')
snapshot('image')
key('Escape')
print('PASS all focus targets reached with keyboard and captured', flush=True)
