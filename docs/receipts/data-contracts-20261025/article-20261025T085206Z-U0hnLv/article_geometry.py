"""Check article image layout bounds without substituting for screenshot review."""
import json

import pyatspi


def walk(node):
    yield node
    for index in range(node.childCount):
        yield from walk(node.getChildAtIndex(index))


def bounds(node):
    rect = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return dict(x=rect.x, y=rect.y, width=rect.width, height=rect.height)


app = next(node for node in walk(pyatspi.Registry.getDesktop(0))
           if node.getRoleName() == 'application' and node.name == 'gallery')
nodes = list(walk(app))
images = sorted((bounds(node) for node in nodes
                 if node.getRoleName() == 'image' and node.name == '配图'),
                key=lambda rect: rect['y'])
heading = bounds(next(node for node in nodes if node.name == '列表示例'))
assert len(images) == 2, images
assert images[0]['y'] + images[0]['height'] <= images[1]['y'], images
assert images[1]['y'] + images[1]['height'] <= heading['y'], (images, heading)
print(json.dumps(dict(images=images, following_heading=heading), ensure_ascii=False, indent=2))
