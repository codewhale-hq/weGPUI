"""Drive native XTEST input; assertions inspect fresh tree/native API snapshot receipts."""
import json
import os
from pathlib import Path
import subprocess
import sys
import time

out = Path(sys.argv[1])
window = sys.argv[2]
here = Path(__file__).resolve().parent
steps_path = Path(os.environ.get('FORMS_STEPS', str(here / 'steps.json')))
steps = json.loads(steps_path.read_text())
(out / 'steps.json').write_text(json.dumps(steps, ensure_ascii=False, indent=2))
failures = []

def run(*args):
    return subprocess.run(args, check=True, text=True, capture_output=True).stdout

def fresh(*args):
    return json.loads(run('/usr/bin/python3', '-I', str(here / 'inspect.py'), *args))

def snapshot(label):
    run('xdotool', 'key', '--clearmodifiers', 'F12')
    time.sleep(.3)
    run('import', '-window', window, str(out / f'{label}.png'))
    nodes = fresh()
    (out / f'{label}.json').write_text(json.dumps(nodes, ensure_ascii=False, indent=2))
    return nodes

snapshot('initial')
for step in steps:
    label = step['label']
    with (out / 'actions.jsonl').open('a') as log:
        log.write(json.dumps(dict(at=time.time(), **step), ensure_ascii=False) + '\n')
    for action in step['actions']:
        if 'focus' in action or 'label' in action:
            item = fresh(action.get('role', 'entry'), action.get('focus', action.get('label')))
            x, y, w, h = item['bounds']
            frame_x, frame_y, _, _ = item['frame_bounds']
            click_x = x - frame_x - 70 if 'label' in action else x - frame_x + max(1, w // 2)
            run('xdotool', 'mousemove', '--window', window, str(click_x), str(y - frame_y + max(1, h // 2)), 'click', '1')
        elif 'key' in action:
            run('xdotool', 'key', '--clearmodifiers', action['key'])
        elif 'text' in action:
            run('xdotool', 'type', '--clearmodifiers', '--delay', '25', '--', action['text'])
        elif 'wait' in action:
            time.sleep(action['wait'])
        time.sleep(.35)
    time.sleep(.5)
    nodes = snapshot(label)
    app_log = (out / 'app.log').read_text()
    snapshot_line = next((s for s in reversed(app_log.splitlines()) if s.startswith('snapshot ')), '')
    compact = ''.join(snapshot_line.split())
    errors = []
    for expected in step.get('snapshot', []):
        if ''.join(expected.split()) not in compact:
            errors.append(f'snapshot lacks {expected!r}: {snapshot_line}')
    for assertion in step.get('controls', []):
        candidates = [n for n in nodes if n.get('role') == assertion.get('role','entry') and n.get('name') == assertion['name']]
        if not candidates:
            errors.append(f'no control {assertion["name"]!r}')
            continue
        for key, expected in assertion.items():
            if key not in ('name','role') and candidates[0].get(key) != expected:
                errors.append(f'{assertion["name"]} {key}: {candidates[0].get(key)!r} != {expected!r}')
    for forbidden in step.get('absent', []):
        if any(n.get('role') == forbidden['role'] and n.get('name') == forbidden['name'] for n in nodes):
            errors.append(f'forbidden control present: {forbidden}')
    for prefix, expected in step.get('counts', {}).items():
        actual = sum(line.startswith(prefix) for line in app_log.splitlines())
        if actual != expected:
            errors.append(f'{prefix} callbacks {actual} != {expected}')
    if step.get('no_dialog') and any(n.get('role') == 'dialog' for n in nodes):
        errors.append('dialog remained open')
    result = dict(label=label, passed=not errors, errors=errors, snapshot=snapshot_line)
    with (out / 'results.jsonl').open('a') as log:
        log.write(json.dumps(result, ensure_ascii=False) + '\n')
    failures.extend(errors)
    print(label, 'FAIL' if errors else 'PASS', flush=True)
    for error in errors:
        print(error, flush=True)
(out / 'summary.json').write_text(json.dumps(dict(steps=len(steps), failures=len(failures), errors=failures), indent=2))
raise SystemExit(bool(failures))
