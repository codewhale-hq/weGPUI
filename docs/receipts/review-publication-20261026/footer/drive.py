from pathlib import Path
import json,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode=sys.argv[1] if len(sys.argv)>1 else 'light'
out=Path('<evidence>/footer')/mode
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
app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/__review_footer_probe',mode],stdout=log,stderr=log)
try:
 frame=find('Footer review','frame')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.05)
 window=found.stdout.split()[-1];subprocess.run(['xdotool','windowactivate','--sync',window],check=True);time.sleep(.4);pump()
 boxes={name:list(find(name,'link').queryComponent().getExtents(pyatspi.DESKTOP_COORDS)) for name in ['Help','Terms']}
 a,b=boxes.values();overlap=max(0,min(a[0]+a[2],b[0]+b[2])-max(a[0],b[0]))*max(0,min(a[1]+a[3],b[1]+b[3])-max(a[1],b[1]))
 subprocess.run(['import','-window',window,str(out/'footer.png')],check=True)
 result={'bounds':boxes,'overlap_area':overlap,'passed':overlap==0}
 (out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(result,flush=True)
 if mode!='baseline':
  assert overlap==0
  calls=[]
  for name in ['Help','Terms']:
   x,y,w,h=boxes[name]
   assert w>=44 and h>=44
   for dx,dy in [(2,2),(w-2,2),(2,h-2),(w-2,h-2)]:
    subprocess.run(['xdotool','mousemove',str(x+dx),str(y+dy),'click','1'],check=True);time.sleep(.1)
    calls.append(name)
    actual=[line.split('=',1)[1] for line in (out/'application.log').read_text().splitlines() if line.startswith('callback=')]
    assert actual==calls,(actual,calls)
  result['edge_callbacks']=calls.copy()
  for name in ['Help','Terms']:
   for _ in range(6):
    if find(name,'link').getState().contains(pyatspi.STATE_FOCUSED):break
    key('Tab')
   else:raise AssertionError(('keyboard focus',name))
   for value in ['Return','space']:
    key(value);calls.append(name)
    actual=[line.split('=',1)[1] for line in (out/'application.log').read_text().splitlines() if line.startswith('callback=')]
    assert actual==calls,(actual,calls)
  result['all_callbacks']=calls
  (out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print('PASS eight edge and four keyboard callbacks',flush=True)
finally:
 app.terminate();app.wait(timeout=3);log.close()
