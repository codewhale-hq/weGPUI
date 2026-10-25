"""Record media-probe AT-SPI data or locate a named node for real pointer input."""
import json
import sys
import pyatspi


def walk(node):
    states = [pyatspi.stateToString(state) for state in node.getState().getStates()]
    item = {'role': node.getRoleName(), 'name': node.name, 'description': node.description,
            'states': states, 'attributes': list(node.getAttributes()), 'children': [walk(node.getChildAtIndex(i)) for i in range(node.childCount)]}
    interfaces = list(node.get_interfaces())
    item['interfaces'] = interfaces
    if 'Text' in interfaces:
        text = node.queryText()
        item['text'] = text.getText(0, text.characterCount)
        item['caret_offset'] = text.caretOffset
        item['selections'] = [list(text.getSelection(i)) for i in range(text.getNSelections())]
        if text.characterCount:
            item['first_character_desktop'] = list(text.getCharacterExtents(0, pyatspi.DESKTOP_COORDS))
            item['first_character_window'] = list(text.getCharacterExtents(0, pyatspi.WINDOW_COORDS))
    try:
        rectangle = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        item['bounds'] = [rectangle.x, rectangle.y, rectangle.width, rectangle.height]
    except NotImplementedError:
        pass
    return item


def flatten(node):
    yield node
    for child in node['children']:
        yield from flatten(child)


desktop = pyatspi.Registry.getDesktop(0)
if len(sys.argv) > 1 and sys.argv[1] == '--click':
    def accessible_nodes(node):
        yield node
        for i in range(node.childCount):
            yield from accessible_nodes(node.getChildAtIndex(i))
    target = next(node for node in accessible_nodes(desktop)
                  if node.name == sys.argv[2]
                  and node.getState().contains(pyatspi.STATE_FOCUSABLE))
    action = target.queryAction()
    names = [action.getName(i) for i in range(action.nActions)]
    print('AT-SPI actions', names, flush=True)
    assert 'click' in names, names
    assert action.doAction(names.index('click'))
    raise SystemExit(0)
tree = walk(desktop)
if len(sys.argv) == 1:
    print(json.dumps(tree, ensure_ascii=False, indent=2))
else:
    matches = [node for node in flatten(tree) if node['name'] == sys.argv[1]
               and (len(sys.argv) < 3 or node['role'] == sys.argv[2])]
    matches.sort(key=lambda node: 'focusable' not in node['states'])
    if not matches:
        raise SystemExit(f'no accessible target {sys.argv[1:]}')
    node = matches[int(sys.argv[3]) if len(sys.argv) > 3 else 0]
    x, y, width, height = node['bounds']
    assert width > 0 and height > 0, node
    frame = next(item for item in flatten(tree) if item['role'] == 'frame')
    fx, fy, _, _ = frame['bounds']
    print(x - fx + width // 2, y - fy + height // 2)
