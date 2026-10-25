"""One fresh AT-SPI client per operation; callback evidence, not screen-reader acceptance."""
import json
import subprocess
import sys
import time
import pyatspi
from gi.repository import Atspi


def nodes(node):
    yield node
    for index in range(node.childCount):
        child = node.getChildAtIndex(index)
        if child:
            yield from nodes(child)


def app_nodes():
    desktop = pyatspi.Registry.getDesktop(0)
    for app in nodes(desktop):
        if app.getRoleName() == 'application' and app.name == 'data_contracts_probe':
            return list(nodes(app))
    raise AssertionError('data_contracts_probe is absent')


def find(name):
    matches = [node for node in app_nodes() if node.name == name]
    assert len(matches) == 1, (name, len(matches))
    return matches[0]


def extents(node):
    r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return dict(x=r.x, y=r.y, width=r.width, height=r.height)


command = sys.argv[1]
if command == 'snapshot':
    result = []
    for node in app_nodes():
        item = dict(role=node.getRoleName(), name=node.name,
                    states=[pyatspi.stateToString(s) for s in node.getState().getStates()])
        if node.getRoleName() in ('slider', 'progress bar'):
            value = node.queryValue()
            item.update(value=value.currentValue, minimum=value.minimumValue,
                        maximum=value.maximumValue, increment=value.minimumIncrement)
        result.append(item)
    print(json.dumps(result, ensure_ascii=False, indent=2))
elif command == 'pointer':
    node = find(sys.argv[2])
    r = extents(node)
    subprocess.run(['xdotool', 'mousemove', str(r['x'] + r['width'] // 2),
                    str(r['y'] + r['height'] // 2), 'click', '1'], check=True)
    print('pointer', node.name, r)
elif command == 'focus':
    node = find(sys.argv[2])
    assert node.queryComponent().grabFocus(), node.name
    print('focus', node.name)
elif command == 'activate':
    node = find(sys.argv[2])
    action = node.queryAction()
    names = [action.getName(i) for i in range(action.nActions)]
    assert names, node.name
    assert action.doAction(0), (node.name, names)
    print('AT action', node.name, names[0])
elif command == 'set-value':
    node = find(sys.argv[2])
    value = node.queryValue()
    requested = float(sys.argv[3])
    result = Atspi.Value.set_current_value(value.obj, requested)
    print('AT set-value', node.name, requested, 'accepted', result)
elif command == 'assert-state':
    all_nodes = app_nodes()
    assert sum(node.name == 'Repeated heading' for node in all_nodes) == 2
    assert sum(node.name == 'Repeated list entry' for node in all_nodes) == 2
    assert any(node.getRoleName() == 'paragraph' and node.name == 'Accessible article paragraph.'
               for node in all_nodes)
    assert any(node.name == 'Accessible record group' for node in all_nodes)
    assert any(node.name == 'Application-owned record actions' for node in all_nodes)
    assert find('Dataset navigation').getRoleName() == 'page tab list'
    cancel = [node for node in all_nodes if node.name == 'Cancel upload']
    assert len(cancel) == int(sys.argv[2]), len(cancel)
    print('PASS: distinct duplicate headings/lists, accessible paragraph, named navbar, cancel visibility')
elif command == 'modal-state':
    all_nodes = app_nodes()
    opened = bool(int(sys.argv[2]))
    assert sum(node.name == 'Data modal' for node in all_nodes) == int(opened)
    assert sum(node.name == 'Fixed footer callback' for node in all_nodes) == int(not opened)
    print('PASS: fixed footer hidden from AT while modal open' if opened else 'PASS: fixed footer re-exposed after close')
elif command == 'notice-geometry':
    notice = extents(find('Floating data notice'))
    parent = extents(find('Inset container'))
    assert notice['x'] < parent['x'], (notice, parent)
    assert notice['width'] > parent['width'], (notice, parent)
    print('PASS: viewport notice escapes inset parent', json.dumps(dict(notice=notice, parent=parent)))
elif command == 'footer-geometry':
    footer = extents(find('Viewport footer'))
    parent = extents(find('Inset container'))
    assert footer['y'] > parent['y'] + parent['height'] + 300, (footer, parent)
    print('PASS: fixed footer is below inset parent at viewport bottom', json.dumps(dict(footer=footer, parent=parent)))
elif command == 'assert-article':
    all_nodes = []
    for app in nodes(pyatspi.Registry.getDesktop(0)):
        if app.getRoleName() == 'application' and app.name == 'gallery':
            all_nodes = list(nodes(app))
            break
    assert any(node.getRoleName() == 'paragraph' and node.name.startswith('Lorem ipsum dolor sit amet')
               for node in all_nodes)
    assert sum(node.getRoleName() == 'image' and node.name == '配图' for node in all_nodes) == 2
    print('PASS: gallery article exposes body paragraph and both upstream-alt images')
else:
    raise AssertionError(command)
time.sleep(0.3)
