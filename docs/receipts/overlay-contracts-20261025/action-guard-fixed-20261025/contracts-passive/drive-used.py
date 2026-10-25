#!/usr/bin/env python3
import argparse,json,subprocess,time,traceback
from pathlib import Path
import pyatspi

p=argparse.ArgumentParser();p.add_argument('--window',required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--case',required=True);a=p.parse_args();a.out.mkdir();events=[]
def note(kind,**kw):
 events.append(dict(kind=kind,monotonic_ns=time.monotonic_ns(),**kw));(a.out/'events.json').write_text(json.dumps(events,indent=2))
def walk(n,d=0):
 if d>30:return
 yield n
 try:children=list(n)
 except Exception:return
 for c in children:yield from walk(c,d+1)
def find(name):
 for n in walk(pyatspi.Registry.getDesktop(0)):
  try:
   if n.name==name:return n
  except Exception:pass
 raise RuntimeError('No target '+name)
def snapshot(label):
 rows=[]
 for n in walk(pyatspi.Registry.getDesktop(0)):
  try:rows.append(dict(name=n.name,role=n.getRoleName(),states=[str(s) for s in n.getState().getStates()]))
  except Exception as e:rows.append(dict(error=repr(e)))
 (a.out/f'{label}-tree.json').write_text(json.dumps(rows,indent=2));subprocess.run(['import','-window',a.window,str(a.out/f'{label}.png')],check=True);note('snapshot',label=label)
def pointer(name):
 subprocess.run(['xdotool','windowactivate','--sync',a.window],check=True)
 n=find(name);r=n.queryComponent().getExtents(pyatspi.WINDOW_COORDS)
 subprocess.run(['xdotool','mousemove','--window',a.window,str(r.x+r.width//2),str(r.y+r.height//2),'mousedown','1','mouseup','1'],check=True);note('pointer',name=name)
def click_for(n):
 action=n.queryAction();names=[action.getName(i) for i in range(action.nActions)];i=next(i for i,s in enumerate(names) if s.lower()=='click');note('actions',name=n.name,names=names);return lambda:action.doAction(i)
def invoke(label,fn):
 began=time.monotonic_ns()
 try:note(label,result=bool(fn()),invoked_ns=began)
 except Exception as e:note(label,error=repr(e),invoked_ns=began)
try:
 subprocess.run(['xdotool','windowactivate','--sync',a.window],check=True)
 tag='A' if a.case=='two-window' else 'main'
 pointer('Open '+tag);time.sleep(2.2)
 own=click_for(find('Caller action '+tag));own_focus=find('Caller action '+tag).queryComponent().grabFocus
 snapshot('opened')
 if a.case.startswith('multiple'):
  invoke('first-request',own);time.sleep(.3);snapshot('after-first')
  if a.case=='multiple-reopen':
   time.sleep(2.1);invoke('new-session-request',click_for(find('Caller action '+tag)));time.sleep(.3);snapshot('after-new-session')
  time.sleep(2.2);snapshot('closed')
 elif a.case=='passive':
  invoke('open-own-control',own);time.sleep(.2)
  bg=click_for(find('Background action main'));invoke('open-background-control',bg);time.sleep(.2)
  start=time.monotonic_ns();pointer('Hide main');returned=time.monotonic_ns();note('hide-command',started_ns=start,returned_ns=returned)
  invoke('saved-own-while-closed',own);invoke('background-while-closed',bg);time.sleep(.2);snapshot('during-close')
  time.sleep(2.2);snapshot('closed');invoke('background-after-close',bg)
  pointer('Open main');time.sleep(2.2);invoke('reopened-own-control',click_for(find('Caller action main')));time.sleep(.2);snapshot('reopened')
 elif a.case=='ancestor':
  invoke('open-own-control',own);time.sleep(.2)
  start=time.monotonic_ns();subprocess.run(['xdotool','keydown','F6','keyup','F6'],check=True);returned=time.monotonic_ns();note('hide-parent-command',started_ns=start,returned_ns=returned)
  invoke('saved-inner-click-after-parent-close',own);invoke('saved-inner-focus-after-parent-close',own_focus);time.sleep(.2);snapshot('during-parent-close')
  time.sleep(2.2);snapshot('closed')
 else:
  invoke('open-own-control',own);time.sleep(.2)
  if a.case=='two-window':
   bg=click_for(find('Background action B'));invoke('other-window-open-control',bg);time.sleep(.2)
  subprocess.run(['xdotool','windowactivate','--sync',a.window],check=True)
  start=time.monotonic_ns();subprocess.run(['xdotool','mousemove','--window',a.window,'10','10','mousedown','1','mouseup','1'],check=True);returned=time.monotonic_ns();note('hide-command',started_ns=start,returned_ns=returned)
  invoke('saved-caller-while-closed',own);time.sleep(.2);snapshot('during-close');time.sleep(2.2);snapshot('closed')
  if a.case=='two-window':invoke('other-window-closed-control',bg);time.sleep(.2);snapshot('other-window-after-close')
except Exception:
 (a.out/'driver-error.txt').write_text(traceback.format_exc());raise
finally:note('finished')
