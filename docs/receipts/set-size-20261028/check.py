import json
import sys
import pyatspi

mode = sys.argv[1]

def walk(node):
    yield node
    for i in range(node.childCount):
        child = node.getChildAtIndex(i)
        if child:
            yield from walk(child)

desktop = pyatspi.Registry.getDesktop(0)
apps = [node for node in walk(desktop) if node.getRoleName() == 'application' and node.name == 'set_size_probe']
assert len(apps) == 1, ('application count', len(apps))
nodes = list(walk(apps[0]))
output = []
for node in nodes:
    output.append({'role': node.getRoleName(), 'name': node.name,
                   'attributes': node.getAttributes() or [],
                   'states': [pyatspi.stateToString(s) for s in node.getState().getStates()]})
if mode == 'date':
    expected = [('2000', 101, 201, False)]
else:
    prefix = 'Select' if mode == 'select' else 'Picker'
    expected = [(prefix + ' one', 1, 3, False), (prefix + ' disabled', 2, 3, True), (prefix + ' three', 3, 3, False)]
for name, position, total, disabled in expected:
    matching = [node for node in output if node['name'] == name and node['role'] == 'list item']
    assert len(matching) == 1, (mode, name, matching, output)
    node = matching[0]
    assert f'posinset:{position}' in node['attributes'], (mode, node)
    assert f'setsize:{total}' in node['attributes'], (mode, node)
    assert ('sensitive' not in node['states']) == disabled, (mode, node)
print(json.dumps(output, ensure_ascii=False, indent=2))
