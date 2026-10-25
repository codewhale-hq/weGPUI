"""Fresh-client accessibility observations; these do not establish reader acceptance."""
import json
import sys
import pyatspi

nodes = []
def walk(node, depth=0):
    try:
        states = node.getState()
        item = dict(role=node.getRoleName(), name=node.name, depth=depth,
                    states=[pyatspi.stateToString(s) for s in states.getStates()],
                    focused=states.contains(pyatspi.STATE_FOCUSED),
                    selected=states.contains(pyatspi.STATE_SELECTED),
                    invalid=states.contains(pyatspi.STATE_INVALID_ENTRY),
                    readonly=states.contains(pyatspi.STATE_READ_ONLY),
                    sensitive=states.contains(pyatspi.STATE_SENSITIVE),
                    description=node.description)
        try:
            r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            item['bounds'] = [r.x, r.y, r.width, r.height]
        except (NotImplementedError, AttributeError):
            pass
        if item['role'] == 'entry':
            try:
                text = node.queryText()
                item.update(text=text.getText(0, -1), caret=text.caretOffset,
                            selections=[text.getSelection(i) for i in range(text.getNSelections())])
            except (NotImplementedError, AttributeError) as error:
                item['text_error'] = str(error)
        nodes.append(item)
        for i in range(node.childCount):
            child = node.getChildAtIndex(i)
            if child is not None:
                walk(child, depth + 1)
    except Exception as error:
        nodes.append(dict(error=str(error), depth=depth))

walk(pyatspi.Registry.getDesktop(0))
if len(sys.argv) > 1:
    role, prefix = sys.argv[1:3]
    found = next((n for n in nodes if n.get('role') == role and n.get('name', '').startswith(prefix)), None)
    if found is None:
        raise SystemExit(f'No {role} {prefix!r} found')
    found['frame_bounds'] = next((n.get('bounds', [0,0,0,0]) for n in nodes if n.get('role') == 'frame'), [0,0,0,0])
    print(json.dumps(found, ensure_ascii=False))
else:
    print(json.dumps(nodes, ensure_ascii=False, indent=2))
