#!/usr/bin/env python3
"""Independent fresh trees plus retained AT action proxies; no modal guards present."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time
import traceback

import pyatspi

def walk(node, depth=0):
    if depth > 30:
        return
    yield node
    try:
        children = list(node)
    except Exception:
        return
    for child in children:
        yield from walk(child, depth + 1)

def rows():
    result = []
    for node in walk(pyatspi.Registry.getDesktop(0)):
        try:
            result.append(dict(name=node.name, role=node.getRoleName(),
                path=str(getattr(node, 'path', '')),
                states=[str(s) for s in node.getState().getStates()]))
        except Exception as error:
            result.append(dict(error=repr(error)))
    return result

if '--dump' in sys.argv:
    print(json.dumps(rows(), indent=2))
    raise SystemExit(0)

if '--fresh-action' in sys.argv:
    name, operation = sys.argv[-2:]
    target = next(n for n in walk(pyatspi.Registry.getDesktop(0)) if n.name == name)
    if operation == 'focus':
        result = target.queryComponent().grabFocus()
    else:
        action = target.queryAction()
        index = next(i for i in range(action.nActions) if action.getName(i).lower() == 'click')
        result = action.doAction(index)
    print(json.dumps(dict(name=name, operation=operation, result=bool(result))))
    raise SystemExit(0)

parser = argparse.ArgumentParser()
parser.add_argument('--window', required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
args.out.mkdir()
events = []

def note(kind, **fields):
    events.append(dict(kind=kind, monotonic_ns=time.monotonic_ns(), **fields))
    (args.out/'events.json').write_text(json.dumps(events, indent=2))
    print(kind, fields, flush=True)

def key(name):
    subprocess.run(['xdotool', 'key', '--clearmodifiers', name], check=True)
    note('key', key=name)
    time.sleep(.5)

def snapshot(label):
    tree = subprocess.run([sys.executable, '-I', __file__, '--dump'], check=True,
                          capture_output=True, text=True).stdout
    (args.out/f'{label}-tree.json').write_text(tree)
    subprocess.run(['import','-window',args.window,str(args.out/f'{label}.png')],check=True)
    key('F3')
    note('snapshot', label=label,
         names=[r.get('name') for r in json.loads(tree)])

def find(name):
    for node in walk(pyatspi.Registry.getDesktop(0)):
        try:
            if node.name == name:
                return node
        except Exception:
            pass
    raise RuntimeError(f'Missing target {name!r}')

def click_for(node):
    action = node.queryAction()
    names = [action.getName(i) for i in range(action.nActions)]
    note('action-interface', name=node.name, actions=names)
    index = next(i for i,name in enumerate(names) if name.lower()=='click')
    return lambda: action.doAction(index)

def invoke(label, fn):
    try:
        note(label, result=bool(fn()))
    except Exception as error:
        note(label, error=repr(error))
    time.sleep(.4)

def fresh_invoke(label, operation):
    process = subprocess.run([sys.executable, '-I', __file__, '--fresh-action',
                              'Cached child action', operation],capture_output=True,text=True)
    note(label, exit=process.returncode, stdout=process.stdout, stderr=process.stderr)
    time.sleep(.4)

try:
    subprocess.run(['xdotool','windowactivate','--sync',args.window],check=True)
    key('F3')
    time.sleep(.5)
    cached = find('Cached child action')
    live = find('Live child action')
    cached_click = click_for(cached)
    cached_focus = cached.queryComponent().grabFocus
    live_click = click_for(live)
    live_focus = live.queryComponent().grabFocus
    snapshot('initial')
    invoke('baseline-cached-click', cached_click)
    invoke('baseline-cached-focus', cached_focus)
    snapshot('baseline-cached-focused')
    invoke('baseline-live-focus', live_focus)
    snapshot('baseline-positive-controls')
    key('F1')
    snapshot('parent-only-first')
    invoke('saved-cached-click-after-parent', cached_click)
    snapshot('saved-cached-click-result')
    invoke('live-click-after-parent', live_click)
    snapshot('live-click-result')
    key('F2')
    snapshot('cached-notify-rescue')
    invoke('cached-click-after-rescue', cached_click)
    fresh_invoke('fresh-cached-click-after-rescue', 'click')
    snapshot('fresh-cached-click-rescue-result')
    invoke('live-focus-before-second-parent', live_focus)
    snapshot('before-second-parent')
    key('F1')
    snapshot('parent-only-second')
    invoke('saved-cached-focus-after-parent', cached_focus)
    snapshot('saved-cached-focus-result')
    key('F4')
    snapshot('full-refresh-rescue')
    invoke('cached-focus-after-full-refresh', cached_focus)
    fresh_invoke('fresh-cached-focus-after-full-refresh', 'focus')
    snapshot('final-focus-control')
except Exception:
    (args.out/'driver-error.txt').write_text(traceback.format_exc())
    raise
finally:
    note('finished')
