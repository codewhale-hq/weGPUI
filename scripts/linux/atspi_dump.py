"""Print the AT-SPI accessibility tree of every application on the session bus.

Usage (inside a D-Bus session with AT-SPI enabled; see scripts/linux/session.sh):
  python3 -I scripts/linux/atspi_dump.py

Needs the distribution's python3 with `python3-pyatspi` (python3-gi). Each line is
`role 'name' [states] [DISABLED] [attributes]`, indented by depth. States are limited to
focused, checked, selected, focusable, modal and showing; attributes to level, posinset and
setsize. AccessKit reports `focused` only for an active window, which a bare Xvfb never has.
"""

import pyatspi

KEEP = (
    pyatspi.STATE_FOCUSED,
    pyatspi.STATE_CHECKED,
    pyatspi.STATE_SELECTED,
    pyatspi.STATE_FOCUSABLE,
    pyatspi.STATE_MODAL,
    pyatspi.STATE_SHOWING,
)
ATTRS = ('level', 'posinset', 'setsize')


def walk(node, depth=0):
    try:
        states = [pyatspi.stateToString(s) for s in node.getState().getStates() if s in KEEP]
        disabled = '' if node.getState().contains(pyatspi.STATE_SENSITIVE) else ' DISABLED'
        attrs = [a for a in (node.getAttributes() or []) if a.split(':')[0] in ATTRS]
        suffix = f' {attrs}' if attrs else ''
        print('  ' * depth + f'{node.getRoleName()} {node.name!r} {states}{disabled}{suffix}')
        for i in range(node.childCount):
            walk(node.getChildAtIndex(i), depth + 1)
    except Exception as error:
        print('  ' * depth + f'<error {error}>')


desktop = pyatspi.Registry.getDesktop(0)
print('apps', desktop.childCount)
for index in range(desktop.childCount):
    app = desktop.getChildAtIndex(index)
    if app is not None:
        walk(app)
