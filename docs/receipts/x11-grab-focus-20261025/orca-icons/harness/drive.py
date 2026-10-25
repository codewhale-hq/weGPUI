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


click('Start warning sample'if CASE=='agree-warn'else'Start matrix');summary('start')
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
elif CASE=='agree-warn':
    label='Read and accept Terms'
    tab_to(label);summary('warn-false-unchecked')
    assert not checked(label)
    assert 'invalid entry' not in [pyatspi.stateToString(x)for x in find(label).getState().getStates()]
    key('F1',.3);summary('warn-true-unchecked')
    assert 'invalid entry' in [pyatspi.stateToString(x)for x in find(label).getState().getStates()]
    key('F2',.3);summary('warn-cleared-unchecked')
    assert 'invalid entry' not in [pyatspi.stateToString(x)for x in find(label).getState().getStates()]
    key('space');summary('warn-false-checked');assert checked(label)
    tab_to('Terms');summary('warn-terms-link');key('Return');snapshot('warn-terms-activated')
    held('Shift_L','Tab',.3);key('F1',.3);summary('warn-true-checked')
    assert checked(label) and 'invalid entry' in [pyatspi.stateToString(x)for x in find(label).getState().getStates()]
    key('F2',.3);summary('warn-cleared-checked')
    assert checked(label) and 'invalid entry' not in [pyatspi.stateToString(x)for x in find(label).getState().getStates()]
    assert 'WARN change 1 checked=true'in(OUT/'application.log').read_text()
    assert 'WARN terms 1'in(OUT/'application.log').read_text()
    assert 'WARN change 2'not in(OUT/'application.log').read_text()
elif CASE=='agree':
    tab_to('Agreement 0 Terms 0');summary('agreement-unchecked')
    key('space');summary('agreement-checked');assert checked('Agreement 0 Terms 0')
    tab_to('Terms 0');summary('terms-link')
    key('Return');snapshot('terms-activated')
    held('Shift_L','Tab');key('space');summary('agreement-unchecked-again')
    assert not checked('Agreement 0 Terms 0')
    tab_to('Terms 1');summary('disabled-agreement-terms-link')
    key('Return');snapshot('disabled-agreement-terms-activated')
    tab_to('Agreement 2 Terms 2');summary('warn-agreement')
    tab_to('Terms 2');summary('warn-terms-link')
    tab_to('Agreement 3 Terms 3');summary('shake-agreement')
elif CASE in('buttons','cells'):
    active='button-0-false-0'if CASE=='buttons'else'cell-0-0'
    second='button-1-false-0'if CASE=='buttons'else'cell-1-0'
    tab_to(active);summary('active')
    key('Tab');summary('next-active-skips-disabled-loading')
    assert any(n.name==second for n in focused()),[(n.getRoleName(),n.name)for n in focused()]
    held('Shift_L','Tab');summary('previous-active-skips-disabled-loading')
    key('KP_Subtract',2);snapshot('flat-review-on')
    key('KP_Up',2);snapshot('flat-active-line')
    key('KP_Page_Up',2);snapshot('flat-disabled-line')
    held('Insert','KP_Begin',3);snapshot('flat-disabled-object')
    key('KP_Page_Up',2);snapshot('flat-loading-line')
    held('Insert','KP_Begin',3);snapshot('flat-loading-object')
    key('KP_Subtract',2);summary('flat-review-off')
    if CASE=='cells':
        click('icon-0');summary('icon-enabled')
        key('KP_Subtract',2);key('KP_Up',2);snapshot('flat-icon-line')
        key('KP_Right',1);held('Insert','KP_Begin',3);snapshot('flat-icon-one')
        key('KP_Right',1);held('Insert','KP_Begin',3);snapshot('flat-disabled-icon')
elif CASE=='icons':
    click('icon-0');summary('icon-enabled')
    key('KP_Subtract',2);key('KP_Up',2);snapshot('flat-icon-line')
    key('KP_Right',.15);key('KP_Right',.15)
    held('Insert','KP_Begin',3);snapshot('flat-disabled-icon-two')
elif CASE=='vcode':
    tab_to('Send rounded');summary('rounded-ready')
    key('Return',.12);snapshot('rounded-countdown-start')
    key('KP_Enter',.15);snapshot('rounded-countdown-readback')
    key('Tab',.3);snapshot('rounded-tab-skips-countdown')
    assert any(n.name=='Start twice 0'for n in focused()),[(n.getRoleName(),n.name)for n in focused()]
    time.sleep(3.2)
    held('Shift_L','Tab',.5);summary('rounded-restored')
    key('Return',.1);key('Return',.1);snapshot('rounded-restart-guard')
    click('Reset 0');summary('rounded-reset-button')
    held('Shift_L','Tab',.3);held('Shift_L','Tab',.3);summary('rounded-after-reset')
    tab_to('Send plain');summary('plain-ready')
    key('space',.12);key('KP_Enter',.15);snapshot('plain-countdown-readback')
    time.sleep(3.2);summary('plain-restored')
else:
    raise AssertionError(('unknown driver case',CASE))
key('F8',.5);snapshot('final')
log('driver completed; speech/audio interpretation remains separate')
print('PASS native target/state driver completed; review real Orca transcript and PCM separately')
