import json, sys, pyatspi
from gi.repository import Atspi

def descendants(node):
    yield node
    for i in range(node.childCount):
        child = node.getChildAtIndex(i)
        if child: yield from descendants(child)

apps = [n for n in descendants(pyatspi.Registry.getDesktop(0))
        if n.getRoleName() == 'application' and n.name == sys.argv[1]]
assert len(apps) == 1, [(a.name, a.getRoleName()) for a in apps]
nodes = []
for n in descendants(apps[0]):
    if n.getRoleName() != 'progress bar': continue
    value = n.queryValue()
    bounds = n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    nodes.append({'name':n.name, 'value':value.currentValue, 'value_text':Atspi.Value.get_text(value.obj),
                  'min':value.minimumValue, 'max':value.maximumValue,
                  'bounds':{'x':bounds.x, 'y':bounds.y, 'width':bounds.width, 'height':bounds.height}})
assert len(nodes) == 10, nodes
print(json.dumps(nodes, indent=2))
