"""Real XTEST control contracts. Run inside the receipt's X11/D-Bus session."""
import ast
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import time

import pyatspi

OUT = Path(sys.argv[1])
PAGE = sys.argv[2]
WINDOW = sys.argv[3]
LOG = OUT / 'application.log'


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


def find(name):
    matches = [node for node in nodes() if node.name == name]
    assert len(matches) == 1, (name, [(node.getRoleName(), node.name) for node in matches])
    return matches[0]


def bounds(name):
    rect = find(name).queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return (rect.x, rect.y, rect.width, rect.height)


def click_at(rect, left=False):
    x, y, width, height = rect
    subprocess.run(['xdotool', 'mousemove', '--window', WINDOW, str(x + (10 if left else width // 2)),
                    str(y + height // 2), 'click', '1'], check=True)
    time.sleep(.14)


def click(name, left=False):
    click_at(bounds(name), left)


def key(*keys):
    for token in keys:
        subprocess.run(['xdotool', 'key', '--clearmodifiers', token], check=True)
        time.sleep(.16)


def counts():
    result = {}
    for line in LOG.read_text().splitlines():
        if line.startswith('CALL '):
            _, name, count = line.split(' ')
            result[name] = int(count)
    return result


def snap(name):
    key('F8')
    subprocess.run(['import', '-window', WINDOW, str(OUT / f'{name}.png')], check=True)
    tree = [{'role': node.getRoleName(), 'name': node.name,
             'states': [pyatspi.stateToString(state) for state in node.getState().getStates()]}
            for node in nodes()]
    (OUT / f'{name}-atspi.json').write_text(json.dumps(tree, indent=2))
    with (OUT / 'actions.jsonl').open('a') as dest:
        dest.write(json.dumps({'step': name, 'counts': counts(), 'focused': focused(), 'time': time.time()}) + '\n')


def focused():
    return [node.name for node in nodes() if node.getState().contains(pyatspi.STATE_FOCUSED)]


def three(name):
    click(name)
    key('Return')
    key('space')


if PAGE in ('buttons', 'cells'):
    names = [node.name for node in nodes() if node.getRoleName() in ('button', 'push button')]
    names = sorted(name for name in names if name.startswith(('button-', 'cell-', 'icon-')))
    assert len(names) == (18 if PAGE == 'buttons' else 13), names
    active = [name for name in names if (name.startswith('icon-') and int(name[-1]) < 2)
              or (not name.startswith('icon-') and name.endswith('-0'))]
    rects = {name: bounds(name) for name in names}
    (OUT / 'control-bounds.json').write_text(json.dumps(rects, indent=2))
    for name in names:
        click_at(rects[name])
        if name in active:
            key('Return', 'space')
    snap('matrix-callbacks')
    if PAGE == 'cells':
        x, y, width, height = rects['icon-3']
        subprocess.run(['xdotool', 'mousemove', '--window', WINDOW, str(x + width // 2), str(y + height // 2), 'mousedown', '1'], check=True)
        snap('disabled-icon-held')
        subprocess.run(['xdotool', 'mouseup', '1'], check=True)
        snap('disabled-icon-released')
    actual = counts()
    for name in names:
        assert actual.get(name, 0) == (3 if name in active else 0), (name, actual)
    click('Start matrix')
    stops = []
    for _ in range(len(active) + 2):
        key('Tab')
        stops.append(focused())
    (OUT / 'tab-stops.json').write_text(json.dumps(stops, indent=2))
    assert all(name in active + ['End matrix', 'Start matrix'] for stop in stops for name in stop), stops
    assert {name for stop in stops for name in stop} == set(active + ['End matrix', 'Start matrix']), stops
    snap('matrix-focus')
elif PAGE == 'choices':
    for name in ['Standalone checkbox', 'Checkbox row', 'Switch']:
        three(name)
        if name == 'Standalone checkbox':
            key('Tab', 'shift+Tab')
        snap('choice-' + name.lower().replace(' ', '-'))
    for name in ['Disabled checkbox', 'Disabled checkbox row', 'Disabled switch', 'Radio disabled']:
        click(name)
    snap('choices-controlled')
    actual = counts()
    for name in ['checkbox', 'checkbox-row', 'switch']:
        assert actual.get(name) == 3, actual
    for name in ['checkbox-disabled', 'checkbox-row-disabled', 'switch-disabled', 'checkbox-parent']:
        assert actual.get(name, 0) == 0, actual
    click('Radio A')
    key('Right', 'Right', 'Left', 'space', 'Return')
    snap('radio-wrap-skips-disabled')
    actual = counts()
    assert actual.get('radio-0') == 2 and actual.get('radio-2') == 4, actual
    key('Tab', 'Right')
    snap('radio-independent')
    assert counts().get('second-radio-1') == 1, counts()
    assert 'radio=2 second=1' in LOG.read_text(), LOG.read_text()
elif PAGE == 'agree-regression':
    click('Terms 0')
    click('Agreement 0 Terms 0', left=True)
    first = counts().get('agree-0', 0)
    click('Agreement 0 Terms 0', left=True)
    second = counts().get('agree-0', 0)
    snap('terms-then-checkbox')
    (OUT / 'regression.json').write_text(json.dumps({'after_first_checkbox_click': first,
                                                   'after_second_checkbox_click': second}, indent=2))
    assert first == 1 and second == 2, (first, second)
elif PAGE == 'agree':
    for index in [0, 2, 3]:
        three(f'Terms {index}')
        click(f'Agreement {index} Terms {index}', left=True)
        key('Return', 'space')
    three('Terms 1')
    click('Agreement 1 Terms 1', left=True)
    snap('agree-enabled-disabled-warn')
    actual = counts()
    assert all(actual.get(f'agree-{index}') == 3 for index in [0, 2, 3]), actual
    assert actual.get('agree-1', 0) == 0 and actual.get('terms-1') == 3, actual
    assert all(actual.get(f'terms-{index}') == 3 for index in [0, 2, 3]), actual
    click('Start matrix')
    for _ in range(11):
        if 'Long terms' in focused():
            break
        key('Tab')
    assert 'Long terms' in focused(), focused()
    rect = bounds('Long terms')
    window_rect = subprocess.check_output(['xdotool', 'getwindowgeometry', '--shell', WINDOW], text=True)
    geometry = dict(line.split('=', 1) for line in window_rect.splitlines() if '=' in line)
    assert 0 <= rect[1] and rect[1] + rect[3] <= int(geometry['HEIGHT']), (rect, geometry)
    snap('agree-long-link-revealed')
elif PAGE == 'vcode':
    for index, name in enumerate(['Send rounded', 'Send plain']):
        rect = bounds(name)
        click_at(rect)
        click_at(rect)
        key('Return', 'space')
        assert counts().get(f'vcode-send-{index}') == 1, counts()
        snap(f'vcode-{index}-guard')
        key('Tab')
        assert f'Start twice {index}' in focused(), focused()
        snap(f'vcode-{index}-tab-skip')
        click(f'Reset {index}')
        click(f'Start twice {index}')
        snap(f'vcode-{index}-restart')
        assert counts().get(f'vcode-send-{index}') == 2, counts()
        time.sleep(.45)
        click(f'Reset {index}')
        click(f'Start twice {index}')
        time.sleep(.35)
        snap(f'vcode-{index}-cancel-old-timer')
        observed = ast.literal_eval(re.findall(r'remaining=(\[[^\]]+\])', LOG.read_text())[-1])
        assert observed[index] == 3, observed
        assert counts().get(f'vcode-send-{index}') == 3, counts()
        time.sleep(3.3)
        snap(f'vcode-{index}-idle')
        click(name)
        click(f'Reset {index}')
        assert counts().get(f'vcode-send-{index}') == 4, counts()
else:
    raise AssertionError(PAGE)

print('PASS', PAGE, counts())
