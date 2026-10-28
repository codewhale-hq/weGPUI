import json
from pathlib import Path
root = Path('/work/full-parity/vcode-deadline-probe')
def read(variant, step):
    nodes = json.loads((root/f'{variant}-{step}.json').read_text())
    buttons = [n for n in nodes if n['role'] == 'push button' and n['name'] != 'Reset']
    counts = [n['name'] for n in nodes if n['role'] == 'status bar']
    assert len(buttons) == len(counts) == 1, (variant,step,buttons,counts)
    return {'label':buttons[0]['name'], 'enabled':'enabled' in buttons[0]['states'], 'count':counts[0]}
expected = {
  'initial':('Send code', True, 'Sends 0'),
  'start':('Resend (3s)', False, 'Sends 1'),
  'reset':('Send code', True, 'Sends 1'),
  'restart':('Resend (3s)', False, 'Sends 2'),
  'after-cancel':('Send code', True, 'Sends 2'),
  'natural-end':('Send code', True, 'Sends 3'),
}
out = {}
for variant in ('baseline', 'candidate'):
    observed = {step:read(variant,step) for step in (*expected, 'after-pause')}
    for step, fields in expected.items(): assert tuple(observed[step].values()) == fields, (variant,step,observed[step])
    start = int((root/f'{variant}-pause-start.txt').read_text()); end = int((root/f'{variant}-pause-end.txt').read_text())
    assert end-start >= 4_500_000_000, (variant, end-start)
    observed['pause_monotonic_seconds'] = (end-start)/1e9
    out[variant] = observed
assert tuple(out['baseline']['after-pause'].values()) == ('Resend (2s)', False, 'Sends 1')
assert tuple(out['candidate']['after-pause'].values()) == ('Send code', True, 'Sends 1')
(root/'result.json').write_text(json.dumps(out,indent=2)+'\n')
print('PASS: process-pause expiration, callback counts, reset/restart, cancellation and natural completion')
