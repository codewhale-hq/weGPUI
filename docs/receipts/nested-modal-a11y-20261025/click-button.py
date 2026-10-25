"""Print the window-relative centre of an exposed, named AT-SPI button."""
import sys
import pyatspi


def find(node):
    if node.getRoleName() == 'button' and node.name == sys.argv[1]:
        bounds = node.queryComponent().getExtents(pyatspi.WINDOW_COORDS)
        print(bounds.x + bounds.width // 2, bounds.y + bounds.height // 2)
        return True
    for child in node:
        if child is not None and find(child):
            return True
    return False


if not find(pyatspi.Registry.getDesktop(0)):
    sys.exit(f'button not found: {sys.argv[1]}')
