import json
from pathlib import Path

root = Path(__file__).parent
prior = root.parent / 'fallback-preservation-probe'
def read(base, name):
    return {(c['id'], c['width']): c for c in json.loads((base / f'{name}.json').read_text())}
def shape(c):
    return (c['lines'], c['unwrapped_width'], c['row_widths'],
            [[(g['glyph_id'], g['index'], g['x'], g['y'], g['emoji'])
              for g in run['glyphs']] for run in c['shaped_runs']])

current = {name: read(root, name) for name in ('missing-list', 'installed-list', 'missing-none', 'missing-unavailable')}
previous = {name: read(prior, name) for name in ('missing-list-after', 'installed-list-before', 'missing-none-before', 'missing-unavailable-before')}
assert all(len(cases) == 8 for cases in current.values())
for now, old in [('missing-list', 'missing-list-after'), ('installed-list', 'installed-list-before'),
                 ('missing-none', 'missing-none-before'), ('missing-unavailable', 'missing-unavailable-before')]:
    assert all(shape(case) == shape(previous[old][key]) for key, case in current[now].items()), now
assert all(shape(case) == shape(current['installed-list'][key]) for key, case in current['missing-list'].items())
assert all(shape(case) == shape(current['missing-none'][key]) for key, case in current['missing-unavailable'].items())
for key in [('arabic', 400.0), ('hebrew', 400.0)]:
    assert shape(current['missing-list'][key]) != shape(current['missing-none'][key])
for key in [('control', 400.0), ('emoji', 400.0)]:
    assert shape(current['missing-list'][key]) == shape(current['missing-none'][key])
print('PASS: committed product reproduces 32 prior control shapes; missing/installed primaries')
print('honor the explicit fallback identically; no-list and unavailable-fallback controls match.')
