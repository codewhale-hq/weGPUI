import json
import subprocess
import sys
import pyatspi

mode = sys.argv[1]
variant = sys.argv[2]

def walk(node):
    yield node
    for i in range(node.childCount):
        child = node.getChildAtIndex(i)
        if child:
            yield from walk(child)

def nodes():
    apps = [n for n in walk(pyatspi.Registry.getDesktop(0)) if n.getRoleName() == 'application' and n.name == 'radio_set_probe']
    assert len(apps) == 1, ('application', len(apps))
    return list(walk(apps[0]))

def find(name, kind):
    roles = {'radio': {'radio button'}, 'button': {'button', 'push button'}, 'status': {'status bar'}}[kind]
    found = [node for node in nodes() if node.name == name and node.getRoleName() in roles]
    assert len(found) == 1, (name, kind, len(found))
    return found[0]

def state(node):
    return [pyatspi.stateToString(s) for s in node.getState().getStates()]

def attrs(node):
    return node.getAttributes() or []

def ancestors(node):
    result = []
    parent = node.parent
    while parent is not None:
        result.append({'name': parent.name, 'role': parent.getRoleName(), 'attributes': attrs(parent)})
        parent = parent.parent
    return result

if mode == 'snapshot':
    result = []
    for node in nodes():
        bounds = None
        try:
            rect = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            bounds = [rect.x, rect.y, rect.width, rect.height]
        except Exception:
            pass
        result.append({'role': node.getRoleName(), 'name': node.name, 'attributes': attrs(node),
                       'states': state(node), 'ancestors': ancestors(node), 'bounds': bounds})
    for prefix in ('Form', 'Cells'):
        for index, suffix in enumerate(('one', 'disabled', 'three'), 1):
            node = find(prefix + ' ' + suffix, 'radio')
            assert node.getRoleName() == 'radio button', (prefix, suffix, node.getRoleName())
            assert f'posinset:{index}' in attrs(node), (prefix, suffix, attrs(node))
            assert ('sensitive' not in state(node)) == (suffix == 'disabled'), (prefix, suffix, state(node))
            if variant == 'grouped':
                assert 'setsize:3' in attrs(node), (prefix, suffix, attrs(node))
                assert any(p['name'] == prefix + ' delivery' for p in ancestors(node)), (prefix, suffix, ancestors(node))
        extra = find(prefix + ' add more', 'button')
        if variant == 'grouped':
            assert not any(a.startswith('setsize:') for a in attrs(extra)), (prefix, attrs(extra))
            assert not any(p['name'] == prefix + ' delivery' for p in ancestors(extra)), (prefix, ancestors(extra))
    print(json.dumps(result, ensure_ascii=False, indent=2))
elif mode == 'focus':
    node = find(sys.argv[3], 'radio')
    wid = subprocess.check_output(['xdotool', 'search', '--onlyvisible', '--name', 'weGPUI radio set proof'], text=True).splitlines()[0]
    subprocess.run(['xdotool', 'windowactivate', '--sync', wid], check=True)
    assert node.queryComponent().grabFocus(), node.name
elif mode == 'selected':
    node = find(sys.argv[3], 'radio')
    assert 'checked' in state(node), (node.name, state(node))
elif mode == 'activate':
    node = find(sys.argv[3], 'button'); action = node.queryAction()
    assert action.nActions and action.doAction(0), node.name
elif mode == 'status':
    find(sys.argv[3], 'status')
