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
    apps = [n for n in walk(pyatspi.Registry.getDesktop(0)) if n.getRoleName() == 'application' and n.name == 'radio_empty_probe']
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
    def recorded(name, roles):
        found = [item for item in result if item['name'] == name and item['role'] in roles]
        assert len(found) == 1, (name, roles, found)
        assert found[0]['bounds'] is not None, (name, found[0])
        return found[0]
    recorded('weGPUI radio set proof', {'frame'})
    items = sys.argv[3]
    for prefix in ('Form', 'Cells'):
        recorded(prefix + ' actions 0', {'status bar'})
        if items == 'populated':
            for index, suffix in enumerate(('one', 'disabled', 'three'), 1):
                node = recorded(prefix + ' ' + suffix, {'radio button'})
                assert f'posinset:{index}' in node['attributes'], (prefix, suffix, node)
                assert ('sensitive' not in node['states']) == (suffix == 'disabled'), (prefix, suffix, node)
                if variant == 'grouped':
                    assert 'setsize:3' in node['attributes'], (prefix, suffix, node)
                    assert any(p['name'] == prefix + ' delivery' for p in node['ancestors']), (prefix, suffix, node)
        else:
            assert not any(n['role'] == 'radio button' and n['name'].startswith(prefix + ' ') for n in result), result
            if variant == 'grouped':
                assert not any(n['name'] == prefix + ' delivery' for n in result), result
        extra = recorded(prefix + ' add more', {'button', 'push button'})
        if variant == 'grouped':
            assert not any(a.startswith('setsize:') for a in extra['attributes']), (prefix, extra)
            assert not any(p['name'] == prefix + ' delivery' for p in extra['ancestors']), (prefix, extra)
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
