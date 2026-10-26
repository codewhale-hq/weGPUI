from pathlib import Path
import json,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode=sys.argv[1] if len(sys.argv)>1 else 'light'
out=Path('<evidence>/layers-motion')/mode
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
from PIL import Image,ImageChops
log=(out/'application.log').open('w')
app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/gallery','--page','layers','--state','3d','--lang','zh-Hans',*flags,'--size','375x812','--position','40,40'],stdout=log,stderr=log)
steps=[]
def mark(text):steps.append(text)
def screenshot(name):
 subprocess.run(['import','-window',window,str(out/(name+'.png'))],check=True)
try:
 frame=find('weGPUI','frame')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.05)
 window=found.stdout.split()[-1]
 subprocess.run(['xdotool','windowactivate','--sync',window],check=True)
 time.sleep(.4);pump()
 x,y,w,h=frame.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
 def pointer_only():
  subprocess.run(['xdotool','mousemove',str(x+15),str(y+15),'click','1'],check=True)
  time.sleep(.2)
 pointer_only();time.sleep(.5);screenshot('stack-before')
 print('Window',window,'OS focus',subprocess.check_output(['xdotool','getwindowfocus'],text=True).strip(),'frame',(x,y,w,h),flush=True)
 assert layer_controls()==['Content','Mask','Navigation','Popout'];mark('All four stack controls initially exposed')
 saved_mask=find('Mask','push button').queryAction()
 for _ in range(12):
  key('Tab');pump()
  print('Focused',[(n.name,n.getRoleName()) for n in nodes() if n.getState().contains(pyatspi.STATE_FOCUSED)],flush=True)
  if find('Popout','push button').getState().contains(pyatspi.STATE_FOCUSED):break
 else:raise AssertionError('Popout did not receive focus')
 key('Return');find('Popout','heading');only_layer('Popout');mark('Enter selects Popout during normal transform')
 rejected=False
 try:rejected=not saved_mask.doAction(0)
 except Exception:rejected=True
 time.sleep(.08);find('Popout','heading');only_layer('Popout');mark('Cached hidden Mask action leaves Popout selected')
 screenshot('interrupted-popout')
 key('space');find('WeUI页面层级','heading');mark('Space reverses the active selection transform')
 time.sleep(.35)
 assert layer_controls()==['Content','Mask','Navigation','Popout'];mark('Return restores all four controls')
 assert find('Mask','push button').queryAction().doAction(0)
 find('Mask','heading');only_layer('Mask');mark('Mask can be selected while return animation is settling')
 time.sleep(.1)
 assert find('Mask','push button').queryAction().doAction(0)
 find('WeUI页面层级','heading');mark('Second Mask action reverses its active transform')
 time.sleep(.35)
 assert find('Content','push button').queryAction().doAction(0)
 find('Content','heading');only_layer('Content');mark('Content selects after interrupted Mask return')
 time.sleep(2.);pointer_only();screenshot('content-settled')
 assert find('Content','push button').queryAction().doAction(0)
 find('WeUI页面层级','heading');time.sleep(2.1);pointer_only();screenshot('stack-after')
 assert layer_controls()==['Content','Mask','Navigation','Popout'];mark('Final return restores stack controls')
 before=Image.open(out/'stack-before.png').convert('RGB');after=Image.open(out/'stack-after.png').convert('RGB')
 difference=ImageChops.difference(before,after);bbox=difference.getbbox()
 assert bbox is None,bbox
 mark('Final settled stack is pixel-identical to initial stack')
 (out/'result.json').write_text(json.dumps({'mode':mode,'passed':True,'reduce_motion':False,'steps':steps,'cached_hidden_action_returned_rejection':rejected,'final_stack_difference_bbox':bbox},indent=2)+'\n')
 print('PASS',mode,len(steps),'normal-motion interruption checks',flush=True)
finally:
 app.terminate()
 try:app.wait(timeout=2)
 except subprocess.TimeoutExpired:app.kill();app.wait()
 log.close()
