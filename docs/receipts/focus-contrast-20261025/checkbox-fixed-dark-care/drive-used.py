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


def checked():
    return find('Standalone checkbox').getState().contains(pyatspi.STATE_CHECKED)


def callbacks():
    return re.findall(r'^checkbox-change (true|false)$', LOG.read_text(), re.M)


click(find('Default', button=True))
assert not checked()
snapshot('checkbox-unfocused')
click(find('Standalone checkbox'))
assert checked() and callbacks() == ['true'], (checked(), callbacks())
snapshot('checkbox-pointer-on')
key('space')
assert not checked() and callbacks() == ['true', 'false'], (checked(), callbacks())
snapshot('checkbox-keyboard-off')
key('Return')
assert checked() and callbacks() == ['true', 'false', 'true'], (checked(), callbacks())
snapshot('checkbox-enter-on')
action = find('Standalone checkbox').queryAction()
actions = [action.getName(i) for i in range(action.nActions)]
assert action.nActions, actions
assert action.doAction(0), actions
time.sleep(.3)
assert not checked() and callbacks() == ['true', 'false', 'true', 'false'], (checked(), callbacks(), actions)
snapshot('checkbox-at-off')
(OUT / 'checkbox-callbacks.json').write_text(json.dumps({'callbacks': callbacks(), 'at_actions': actions,
    'final_checked': checked()}, indent=2))
from PIL import Image
node = find('Standalone checkbox')
x,y,w,h = rect(node)
centre = (int(x+w/2), int(y+h/2))
initial = Image.open(OUT/'checkbox-unfocused.png').convert('RGB')
focused = Image.open(OUT/'checkbox-keyboard-off.png').convert('RGB')
a,b = initial.getpixel(centre), focused.getpixel(centre)
assert a == b, ('transparent unchecked centre changed on keyboard focus', centre, a,b)
points = [(int(x-1),int(y+h/2)), (int(x-3),int(y+h/2))]
colors = [focused.getpixel(point) for point in points]
assert colors == [(0,0,0),(255,255,255)], (points, colors)
(OUT / 'checkbox-pixels.json').write_text(json.dumps({'centre':centre,'unfocused':a,'focused':b,
    'band_points':points,'band_colors':colors}, indent=2))
print('PASS pointer/Space/Enter/AT toggle callbacks exactly once each; unchecked centre preserved; two 2px bands', flush=True)
