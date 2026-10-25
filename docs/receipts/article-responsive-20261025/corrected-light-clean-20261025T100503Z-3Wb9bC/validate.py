"""Check responsive image geometry/paint, not whole-route screenshot parity."""
import json
from pathlib import Path
import sys

out = Path(sys.argv[1])
reference = json.loads((out / 'weui-static-geometry.json').read_text())
paint = json.loads((out / 'paint-boxes.json').read_text())['widths']
errors = []
for width, ref in reference.items():
    native = json.loads((out / f'{width}-native-geometry.json').read_text())
    expected_width = min(int(width) - 48, 690)
    expected_height = expected_width * 389 / 690
    for index, image in enumerate(native['images']):
        for field, actual, expected in [
            ('x', image['x'] - native['frame']['x'], 24),
            ('width', image['width'], expected_width),
            ('height', image['height'], expected_height),
        ]:
            if abs(actual - expected) > 1:
                errors.append(f'{width}: native image {index} {field} {actual} expected {expected}')
    for image in ref['images']:
        assert abs(image['bounds']['width'] - expected_width) <= 1, (width, image)
        assert abs(image['bounds']['height'] - expected_height) <= 1, (width, image)
    for side in ('native', 'weui-static'):
        boxes = paint[width][side]
        # Contain preserves the image ratio after integer layout-height rounding; the exact
        # background-color mask may lose one antialiased edge pixel on each side.
        width_tolerance = 2 if side == 'native' else 1
        if not boxes or any(abs(box['x'] - 24) > 1 or abs(box['width'] - expected_width) > width_tolerance for box in boxes):
            errors.append(f'{width}: {side} painted boxes {boxes}, expected left 24 width {expected_width}')
        if abs(sum(box['height'] for box in boxes) - expected_height * 2) > 2:
            errors.append(f'{width}: {side} total painted height {sum(box["height"] for box in boxes)} expected {expected_height * 2}')
if errors:
    print('\n'.join(errors))
    raise SystemExit(1)
print('PASS: static pinned-CSS and native image widths, ratios, intrinsic cap, left alignment and painted extents at ' + ', '.join(reference))
