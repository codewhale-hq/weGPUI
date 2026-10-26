from pathlib import Path
import json,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode=sys.argv[1] if len(sys.argv)>1 else 'light'
out=Path('<evidence>/consumer')
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
log=(out/'application.log').open('w')
app=subprocess.Popen(['<toolchain-cache>/target/debug/wegpui-readme-consumer'],stdout=log,stderr=log)
steps=[]
def named(name):
 end=time.monotonic()+5
 while time.monotonic()<end:
  pump()
  matches=[n for n in nodes() if n.name==name]
  if len(matches)==1:return matches[0]
  time.sleep(.05)
 raise AssertionError((name,[(n.name,n.getRoleName()) for n in nodes()]))
def check_count(value):
 assert named('Confirmed').description==str(value),(value,named('Confirmed').description)
 assert not [n for n in nodes() if n.getRoleName()=='dialog']
 assert named('Open dialog').getState().contains(pyatspi.STATE_FOCUSED)
 steps.append('Confirmed '+str(value)+'; page restored and opener focused')
def click(name):
 n=named(name);x,y,w,h=n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
 subprocess.run(['xdotool','mousemove',str(x+w//2),str(y+h//2),'click','1'],check=True);time.sleep(.2)
try:
 named('Open dialog')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.05)
 window=found.stdout.split()[-1];subprocess.run(['xdotool','windowactivate','--sync',window],check=True);time.sleep(.4);pump()
 assert named('Confirmed').description=='0'
 click('Open dialog');find('weGPUI','dialog')
 assert not [n for n in nodes() if n.name=='Open dialog'];steps.append('Pointer opens named modal and hides page')
 key('Tab');assert named('Cancel').getState().contains(pyatspi.STATE_FOCUSED)
 key('Tab');assert named('OK').getState().contains(pyatspi.STATE_FOCUSED)
 key('Tab');assert named('Cancel').getState().contains(pyatspi.STATE_FOCUSED);steps.append('Tab stays within modal')
 key('shift+Tab');assert named('OK').getState().contains(pyatspi.STATE_FOCUSED);steps.append('Shift+Tab stays within modal')
 key('Return');time.sleep(.3);check_count(1)
 key('space');find('weGPUI','dialog');steps.append('Space reopens modal')
 key('Escape');time.sleep(.3);check_count(1)
 click('Open dialog');find('weGPUI','dialog');click('OK');time.sleep(.3);check_count(2)
 subprocess.run(['import','-window',window,str(out/'confirmed.png')],check=True)
 (out/'result.json').write_text(json.dumps({'passed':True,'steps':steps,'native_consumer':True,'no_reader_claim':True},indent=2)+'\n');print('PASS',steps,flush=True)
finally:
 app.terminate();app.wait(timeout=3);log.close()
