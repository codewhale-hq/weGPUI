"""Compare one native interior pixel per button to composited static CSS colours."""
import json
from pathlib import Path
import re
from PIL import Image

root = Path(__file__).resolve().parent
checks = {}
for path in sorted((root / 'static-css').glob('*/computed-styles.json')):
    if path.parent.name.startswith('initial-'):
        continue
    source = json.loads((path.parent / 'provenance.json').read_text())
    size = source['size']
    mode = path.parent.name[len(size) + 1:]
    native = root / 'matrix' / f'{mode}-buttons-{size}'
    if not (native / 'control-bounds.json').exists():
        continue
    values = json.loads(path.read_text())
    bounds = json.loads((native / 'control-bounds.json').read_text())
    image = Image.open(native / 'initial.png').convert('RGB')

    def rgba(value):
        components = [float(number) for number in re.findall(r'[0-9.]+', value)]
        return tuple(components[:3]), components[3] if len(components) > 3 else 1.

    brand, _ = rgba(values['button-1-false-0']['backgroundColor'])
    for name, style in values.items():
        color, alpha = rgba(style['backgroundColor'])
        # The fixture's ordinary cards use the pinned BG-2: white or #191919.
        card = brand if '-true-' in name else ((25, 25, 25) if source['dark'] else (255, 255, 255))
        expected = tuple(round(channel * alpha + behind * (1 - alpha)) for channel, behind in zip(color, card))
        x, y, _, _ = bounds[name]
        actual = image.getpixel((x + 10, y + 10))
        error = max(abs(first - second) for first, second in zip(expected, actual))
        checks[path.parent.name + '/' + name] = {'expected': expected, 'native': actual, 'max_error': error}
(root / 'static-color-checks.json').write_text(json.dumps(checks, indent=2) + '\n')
assert len(checks) == 162, len(checks)
assert max(value['max_error'] for value in checks.values()) <= 1
print('PASS: 162 selected native interior colours match composited CSS within 1/255 per channel.')
