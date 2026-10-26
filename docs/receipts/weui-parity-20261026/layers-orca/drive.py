from pathlib import Path
import json,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode=sys.argv[1] if len(sys.argv)>1 else 'light'
out=Path('<evidence>/layers-orca')
out.mkdir(exist_ok=True)
flags=['--theme',mode.split('-')[0]]
if 'care' in mode:flags.append('--care')
if 'brand' in mode:flags += ['--brand','#e64340']
def nodes(node=None):
 if node is None:node=pyatspi.Registry.getDesktop(0)
 yield node
 for index in range(node.childCount):
  try:yield from nodes(node.getChildAtIndex(index))
  except Exception:pass
def pump():
 context=GLib.MainContext.default()
 for _ in range(100):
  if not context.pending():break
  context.iteration(False)
def find(name,role):
 end=time.monotonic()+6
 while time.monotonic()<end:
  pump()
  for node in nodes():
   try:
    if node.name==name and (node.getRoleName() in {'button','push button'} if role=='push button' else node.getRoleName()==role):return node
   except Exception:pass
  time.sleep(.05)
 raise AssertionError((name,role))
def layer_controls():
 pump()
 return sorted(n.name for n in nodes() if n.getRoleName() in {'button','push button'} and n.name in {'Popout','Mask','Navigation','Content'})
def only_layer(name):
 end=time.monotonic()+3
 while time.monotonic()<end:
  if layer_controls()==[name]:break
  time.sleep(.05)
 else:raise AssertionError(('visible layer controls',layer_controls(),name))
 assert find(name,'push button').getState().contains(pyatspi.STATE_FOCUSED)
def key(value):subprocess.run(['xdotool','key','--clearmodifiers',value],check=True);time.sleep(.15)
from datetime import datetime,timezone
log=(out/'application.log').open('w')
app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/gallery','--page','layers','--state','3d','--lang','en','--theme','light','--size','375x812','--position','40,40','--reduce-motion'],stdout=log,stderr=log)
steps=[]
def stamp(name):
 steps.append({'step':name,'utc':datetime.now(timezone.utc).isoformat(),'focused':[n.name for n in nodes() if n.getState().contains(pyatspi.STATE_FOCUSED)],'layer_controls':layer_controls()})
 print(name,flush=True)
def read_current():
 subprocess.run(['xdotool','keydown','Insert','key','KP_Begin','keyup','Insert'],check=True)
 time.sleep(2.)
try:
 frame=find('weGPUI','frame')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.05)
 window=found.stdout.split()[-1]
 subprocess.run(['xdotool','windowactivate','--sync',window],check=True);time.sleep(.5);pump()
 for name in ['Popout','Mask','Navigation','Content']:
  for _ in range(12):
   key('Tab');pump()
   if find(name,'push button').getState().contains(pyatspi.STATE_FOCUSED):break
  else:raise AssertionError(('focus',name))
  stamp(name+' focused');read_current()
  key('Return');find(name,'heading');only_layer(name);stamp(name+' selected');read_current()
  subprocess.run(['import','-window',window,str(out/(name.lower()+'-selected.png'))],check=True)
  key('space');time.sleep(.6);assert layer_controls()==['Content','Mask','Navigation','Popout'];stamp(name+' restored')
 time.sleep(1.)
 (out/'result.json').write_text(json.dumps({'passed':True,'steps':steps,'qualification':'Real Orca keyboard/flat-review collection; speech and audio are verified separately.'},indent=2)+'\n')
finally:
 app.terminate()
 try:app.wait(timeout=2)
 except subprocess.TimeoutExpired:app.kill();app.wait()
 log.close()
