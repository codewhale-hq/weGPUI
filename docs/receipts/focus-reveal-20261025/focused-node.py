"""Report actual AT-SPI focus and screen bounds; this is not a screen-reader check."""
import pyatspi


def walk(node):
    if node.getState().contains(pyatspi.STATE_FOCUSED):
        try:
            bounds = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            bounds = (bounds.x, bounds.y, bounds.width, bounds.height)
        except Exception as error:
            bounds = str(error)
        print(f"{node.getRoleName()} {node.name!r} bounds={bounds}")
    for child in node:
        if child is not None:
            walk(child)


walk(pyatspi.Registry.getDesktop(0))
