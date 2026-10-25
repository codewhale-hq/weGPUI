"""Real XTEST control driving while Orca and PCM recording run independently.

AT-SPI only identifies native targets, bounds and state; no speech labels are injected.
"""
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import sys
import time
import pyatspi

OUT=Path(sys.argv[1]);WINDOW=sys.argv[2];CASE=sys.argv[3]
HERE=Path(__file__).parent


def log(action):
    with (OUT/'actions.txt').open('a')as f:
        f.write(datetime.now(timezone.utc).isoformat()+' '+action+'\n')


def nodes():
    result=[]
    def visit(node):
        if node is None:return
        result.append(node)
        for child in node:visit(child)
    visit(pyatspi.Registry.getDesktop(0));return result


def find(name):
    matches=[n for n in nodes()if n.name==name]
    assert len(matches)==1,(name,[(n.getRoleName(),n.name)for n in matches])
    return matches[0]


def focused():
    return[n for n in nodes()if n.getState().contains(pyatspi.STATE_FOCUSED)]


def rectangle(node):
    frame=next(n for n in nodes()if n.getRoleName()=='frame')
    a=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    b=frame.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return[a.x-b.x,a.y-b.y,a.width,a.height]


def click(name):
    x,y,w,h=rectangle(find(name));log(f'XTEST click {name!r} window-local={x+w//2},{y+h//2}')
    subprocess.run(['xdotool','mousemove','--window',WINDOW,str(x+w//2),str(y+h//2),'click','1'],check=True)
    time.sleep(1)


def key(value,delay=1):
    log(f'XTEST key {value}')
    subprocess.run(['xdotool','key','--clearmodifiers',value],check=True);time.sleep(delay)


def held(modifier,value,delay=1):
    log(f'XTEST held {modifier} + {value}')
    subprocess.run(['xdotool','keydown',modifier],check=True);time.sleep(.08)
    subprocess.run(['xdotool','key',value],check=True);time.sleep(.08)
    subprocess.run(['xdotool','keyup',modifier],check=True);time.sleep(delay)


def snapshot(label):
    subprocess.run(['import','-window',WINDOW,str(OUT/f'{label}.png')],check=True)
    with(OUT/f'{label}-atspi.txt').open('w')as f:
        subprocess.run(['/usr/bin/python3','-I',str(HERE/'atspi_dump.py')],stdout=f,check=True)
    tree=[{'role':n.getRoleName(),'name':n.name,'bounds':rectangle(n),
           'states':[pyatspi.stateToString(s)for s in n.getState().getStates()],
           'attributes':n.getAttributes()}for n in nodes()if n.getRoleName()not in('desktop frame','application')]
    (OUT/f'{label}-native.json').write_text(json.dumps(tree,indent=2))
    log(f'capture {label} focused={[(n.getRoleName(),n.name)for n in focused()]}')


def summary(label):
    key('KP_Enter',3)
    snapshot(label)


def tab_to(name):
    for _ in range(25):
        if any(n.name==name for n in focused()):return
        key('Tab',.6)
    raise AssertionError(('focus target',name,[(n.getRoleName(),n.name)for n in focused()]))


def checked(name):
    return find(name).getState().contains(pyatspi.STATE_CHECKED)


click('Start matrix');summary('start')
if CASE=='choices':
    tab_to('Standalone checkbox');summary('checkbox-unchecked')
    key('space');summary('checkbox-checked');assert checked('Standalone checkbox')
    key('space');summary('checkbox-unchecked-again');assert not checked('Standalone checkbox')
    tab_to('Checkbox row');summary('checkbox-row-unchecked')
    key('Return');summary('checkbox-row-checked');assert checked('Checkbox row')
    tab_to('Switch');summary('switch-off')
    key('space');summary('switch-on')
    key('space');summary('switch-off-again')
    tab_to('Radio A');summary('first-radio-one')
    key('Down');summary('first-radio-three');assert checked('Radio C')and checked('Second A')
    tab_to('Second A');summary('second-radio-one')
    key('Down');summary('second-radio-two');assert checked('Radio C')and checked('Second B')
    held('Shift_L','Tab');summary('first-radio-still-three')
    key('Tab');summary('second-radio-still-two')
else:
    raise AssertionError(('unknown driver case',CASE))
key('F8',.5);snapshot('final')
log('driver completed; speech/audio interpretation remains separate')
print('PASS native target/state driver completed; review real Orca transcript and PCM separately')
