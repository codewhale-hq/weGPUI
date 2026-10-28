import json, math, struct
from pathlib import Path
from PIL import Image

root = Path('/work/full-parity/progress-precision-probe')
records = {name: {c['name']:c for c in json.loads((root/f'{name}.json').read_text())} for name in ('baseline','candidate')}
expected = {'zero':0., 'fraction':0.4, 'middle':49.6, 'fine':0.125, 'integer':25.,
            'negative-zero':0., 'nan':0., 'positive-infinity':100., 'below':0., 'above':100.}
f32 = lambda x: struct.unpack('f', struct.pack('f', x))[0]
result = {}
for name, value in expected.items():
    baseline, fixed = records['baseline'][name], records['candidate'][name]
    assert fixed['value'] == f32(value), (name, fixed['value'], f32(value))
    assert baseline['min'] == fixed['min'] == 0 and baseline['max'] == fixed['max'] == 100
    assert baseline['bounds'] == fixed['bounds']
    if name in ('zero', 'negative-zero', 'nan', 'positive-infinity', 'below', 'above', 'integer'):
        assert baseline['value'] == fixed['value']
    if name == 'negative-zero': assert math.copysign(1., fixed['value']) == 1.
    rect = baseline['bounds']; x,y,w,h = (rect[k] for k in ('x','y','width','height'))
    before = Image.open(root/'baseline.png').convert('RGB').crop((x,y,x+w,y+h))
    after = Image.open(root/'candidate.png').convert('RGB').crop((x,y,x+w,y+h))
    assert list(before.getdata()) == list(after.getdata()), name
    green = sum(1 for px in list(before.getdata())[:w] if px[1] > px[0]*1.5 and px[1] > px[2]*1.15)
    result[name] = {'before':baseline['value'], 'after':fixed['value'], 'painted_green_pixels':green,
                    'bar_width_pixels':w, 'atspi_value_text_before':baseline['value_text'],
                    'atspi_value_text_after':fixed['value_text']}
assert result['fraction']['before'] == 0 and result['fraction']['painted_green_pixels'] == 2
assert result['middle']['before'] == 50 and result['middle']['painted_green_pixels'] == 198
assert result['fine']['before'] == 0 and result['fine']['after'] == 0.125
(root/'result.json').write_text(json.dumps(result,indent=2)+'\n')
print('PASS: 10 live AT-SPI values, clamp/negative-zero controls, and paired bar pixels;')
print('AT-SPI value text is unavailable in this adapter, so text formatting is source-reviewed only.')
