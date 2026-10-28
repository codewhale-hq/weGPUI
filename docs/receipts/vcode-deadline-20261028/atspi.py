import json, sys, pyatspi

def nodes(node):
    yield node
    for i in range(node.childCount):
        child = node.getChildAtIndex(i)
        if child: yield from nodes(child)

apps = [n for n in nodes(pyatspi.Registry.getDesktop(0))
        if n.getRoleName() == 'application' and n.name == sys.argv[1]]
assert len(apps) == 1, (sys.argv[1], len(apps))
elements = list(nodes(apps[0]))
if sys.argv[2] == 'snapshot':
    assert sum(n.getRoleName() == 'push button' for n in elements) == 2
    assert any(n.getRoleName() == 'status bar' and n.name.startswith('Sends ') for n in elements)
    print(json.dumps([{'name':n.name, 'role':n.getRoleName(),
                       'states':[pyatspi.stateToString(s) for s in n.getState().getStates()]}
                      for n in elements], ensure_ascii=False, indent=2))
else:
    target = [n for n in elements if n.name == sys.argv[2] and n.getRoleName() == 'push button']
    assert len(target) == 1, (sys.argv[2],len(target))
    actions = target[0].queryAction()
    assert actions.nActions > 0 and actions.doAction(0), sys.argv[2]
    print('activated', sys.argv[2])
