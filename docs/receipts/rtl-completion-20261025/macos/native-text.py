import sys,pathlib,subprocess,time,json,os
root=pathlib.Path('REPO_ROOT');out=pathlib.Path(__file__).parent
sys.path.insert(0,str(root/'scripts'))
from drive import window_id,window_bounds,post_key,post_mouse,post_text,to_srgb
import Quartz
from PIL import Image
def post_text(text):
 for ch in text:
  for down in (True,False):
   event=Quartz.CGEventCreateKeyboardEvent(None,0,down)
   Quartz.CGEventSetFlags(event,0)
   Quartz.CGEventKeyboardSetUnicodeString(event,len(ch.encode('utf-16-le'))//2,ch)
   Quartz.CGEventPost(Quartz.kCGHIDEventTap,event)
   time.sleep(.03)
folder=out/'native-text';folder.mkdir(exist_ok=True)
proc=subprocess.Popen(['BUILD_TARGET/debug/examples/gallery','--page','languages','--lang','ar','--state','type:7:all','--size','375x812','--position','60,60','--reduce-motion'],stderr=(folder/'runtime.log').open('w'))
try:
 wid=None
 for _ in range(100):
  time.sleep(.1);wid=window_id(proc.pid,812)
  if wid:break
 assert wid,'no gallery window'
 subprocess.run(['osascript','-e',f'tell application "System Events" to set frontmost of (first process whose unix id is {proc.pid}) to true'],check=True)
 time.sleep(1)
 def shot(name):
  subprocess.run(['screencapture','-x','-o','-l',str(wid),str(folder/(name+'-raw.png'))],check=True)
  im=Image.open(folder/(name+'-raw.png'));to_srgb(im).crop((0,im.height-1624,im.width,im.height)).save(folder/(name+'.png'))
  result=subprocess.run(['osascript','-l','JavaScript',str(out/'text-ax.js'),str(proc.pid)],capture_output=True,text=True,check=True,timeout=30)
  (folder/(name+'.json')).write_text(result.stdout)
  fields=json.loads(result.stdout);field=next(x for x in fields if x['AXDescription']=='Arabic paragraph' or x['AXTitle']=='Arabic paragraph')
  print(name,json.dumps(field,ensure_ascii=False),flush=True)
  return field
 first=shot('all')
 post_key('right')
 for _ in range(3):post_key('shift-left')
 time.sleep(.3);partial=shot('partial')
 post_key('down');post_key('shift-up');time.sleep(.3);cross=shot('cross-row')
 post_key('cmd-a');post_text('مرحبا 2026 hello');time.sleep(.3);typed=shot('typed')
 post_key('cmd-z');time.sleep(.3);undo=shot('undo')
 for _ in range(3):post_key('cmd-z')
 time.sleep(.3);restored=shot('restored')
 x,y,w,h=window_bounds(wid);top=y+h-812
 post_mouse(Quartz.kCGEventLeftMouseDown,x+350,top+600)
 post_mouse(Quartz.kCGEventLeftMouseDragged,x+180,top+646)
 post_mouse(Quartz.kCGEventLeftMouseUp,x+180,top+646)
 time.sleep(.3);drag=shot('drag')
 assert first['AXSelectedText']==first['AXValue'],'select all'
 assert partial['AXSelectedText'] and partial['AXSelectedText'] in first['AXValue'],'partial'
 assert cross['AXSelectedText'] and cross['AXSelectedText'] in first['AXValue'],'cross row'
 assert typed['AXValue']=='مرحبا 2026 hello','typed'
 assert undo['AXValue']=='مرحبا 2026','word undo'
 assert restored['AXValue']==first['AXValue'],'full undo'
 assert drag['AXSelectedText'] and drag['AXSelectedText'] in first['AXValue'],'drag'
 (folder/'result.json').write_text(json.dumps({'status':'pass','checks':['select-all','partial-keyboard','cross-row-keyboard','unicode-insert','undo','pointer-drag']},indent=2)+'\n')
finally:
 proc.terminate();proc.wait(timeout=5)
