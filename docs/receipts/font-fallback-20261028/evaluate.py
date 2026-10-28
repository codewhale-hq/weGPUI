import json
from pathlib import Path

root = Path(__file__).parent
data = {name: json.loads((root / f'{name}.json').read_text()) for name in (
    'missing-list-before', 'missing-list-after', 'installed-list-before',
    'installed-list-after', 'missing-none-before', 'missing-none-after')}

def shape(case):
    return (case['lines'], case['unwrapped_width'], case['row_widths'],
            [[(glyph['glyph_id'], glyph['index'], glyph['x'], glyph['y'], glyph['emoji'])
              for glyph in run['glyphs']] for run in case['shaped_runs']])

def by_id(cases): return {(c['id'], c['width']): c for c in cases}

cases = {name: by_id(value) for name, value in data.items()}
assert all(len(value) == 8 for value in cases.values())
assert cases['installed-list-before'] == cases['installed-list-after']
assert cases['missing-none-before'] == cases['missing-none-after']
assert all(shape(cases['missing-list-after'][key]) == shape(value)
           for key, value in cases['installed-list-before'].items())
for key in [('arabic', 400.0), ('hebrew', 400.0)]:
    assert shape(cases['missing-list-before'][key]) != shape(cases['missing-list-after'][key])
for key in [('control', 400.0), ('emoji', 400.0)]:
    assert shape(cases['missing-list-before'][key]) == shape(cases['missing-list-after'][key])
print('PASS: 8 absent-primary rows match installed-primary explicit-fallback controls;')
print('installed-primary and absent-primary/no-list controls are unchanged;')
print('Arabic/Hebrew use their requested fallback while ordinary Latin and emoji controls remain unchanged.')
