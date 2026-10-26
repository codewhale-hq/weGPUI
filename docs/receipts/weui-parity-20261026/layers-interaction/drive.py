from pathlib import Path
import json,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode=sys.argv[1] if len(sys.argv)>1 else 'light'
out=Path('<evidence>/layers-interaction')/mode
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
app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/gallery','--page','layers','--state','3d','--lang','zh-Hans',*flags,'--size','375x812','--position','40,40','--reduce-motion'],stdout=log,stderr=log)
steps=[]
try:
 frame=find('weGPUI','frame')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.05)
 assert found.stdout.strip()
 window=found.stdout.split()[-1]
 subprocess.run(['xdotool','windowactivate','--sync',window],check=True)
 time.sleep(.3);pump()
 print('Window',window,'OS focus',subprocess.check_output(['xdotool','getwindowfocus'],text=True).strip(),flush=True)
 x,y,w,h=frame.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
 subprocess.run(['xdotool','mousemove',str(x+15),str(y+15),'click','1'],check=True)
 find('WeUI页面层级','heading')
 for _ in range(12):
  key('Tab');pump()
  print('Focused',[(n.name,n.getRoleName()) for n in nodes() if n.getState().contains(pyatspi.STATE_FOCUSED)],flush=True)
  if find('Popout','push button').getState().contains(pyatspi.STATE_FOCUSED):break
 else:raise AssertionError('Popout did not receive keyboard focus')
 window=subprocess.check_output(['xdotool','search','--onlyvisible','--pid',str(app.pid)],text=True).split()[-1]
 subprocess.run(['import','-window',window,str(out/'popout-focused.png')],check=True)
 key('Return');find('Popout','heading');only_layer('Popout');steps.append('Enter selects Popout')
 key('space');find('WeUI页面层级','heading');time.sleep(.6);steps.append('Space restores stack')
 subprocess.run(['xdotool','mousemove',str(x+187),str(y+525),'click','1'],check=True)
 find('Content','heading');only_layer('Content');steps.append('Projected Content pointer region selects Content')
 subprocess.run(['import','-window',window,str(out/'content-selected.png')],check=True)
 subprocess.run(['xdotool','mousemove',str(x+187),str(y+343),'click','1'],check=True)
 find('WeUI页面层级','heading');time.sleep(.6);steps.append('Flat Content pointer region restores stack')
 button=find('Mask','push button');action=button.queryAction();assert action.nActions>0;assert action.doAction(0)
 find('Mask','heading');only_layer('Mask');steps.append('AT default action selects Mask')
 assert find('Mask','push button').queryAction().doAction(0)
 find('WeUI页面层级','heading');steps.append('AT default action restores stack')
 (out/'result.json').write_text(json.dumps({'mode':mode,'steps':steps,'frame':[x,y,w,h],'passed':True,'selected_focus_and_hidden_controls':True},indent=2)+'\n')
 print('PASS',steps)
finally:
 app.terminate()
 try:app.wait(timeout=2)
 except subprocess.TimeoutExpired:app.kill();app.wait()
 log.close()
