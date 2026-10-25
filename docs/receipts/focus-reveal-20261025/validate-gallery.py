"""Validate that sampled keyboard focus bounds lie inside the 480x420 client area."""
import re
import sys
from pathlib import Path

folder = Path(sys.argv[1])
expected = {'article': 4, 'home': 14, 'form_checkbox': 10, 'form_vcode': 16, 'form_page': 24}
for route, count in expected.items():
    text = (folder / f'gallery-{route}-focus.log').read_text()
    bounds = re.findall(r'bounds=\((-?\d+), (-?\d+), (\d+), (\d+)\)', text)
    assert len(bounds) == count, (route, len(bounds), count)
    for values in bounds:
        x, y, width, height = map(int, values)
        assert 0 <= x and x + width <= 480 and 0 <= y and y + height <= 420, (route, values)
    print(f'PASS {route}: {count} sampled forward/backward focus bounds inside 480x420')
