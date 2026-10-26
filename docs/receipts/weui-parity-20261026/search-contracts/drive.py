from pathlib import Path
from collections import Counter
import json,re,subprocess,time,sys
import pyatspi
from gi.repository import GLib
mode,variant=sys.argv[1:3]
out=Path('<evidence>/search-contracts')/mode/variant
out.mkdir(parents=True,exist_ok=True)
def nodes(node=None):
 if node is None:node=pyatspi.Registry.getDesktop(0)
 yield node
 for i in range(node.childCount):
  try:yield from nodes(node.getChildAtIndex(i))
  except Exception:pass
def pump():
 context=GLib.MainContext.default()
 for _ in range(100):
  if not context.pending():break
  context.iteration(False)
def matches(name,button=False):
 pump()
 return [n for n in nodes() if n.name==name and ((n.getRoleName() in {'button','push button'})==button)]
def find(name,button=False):
 end=time.monotonic()+5
 while time.monotonic()<end:
  found=matches(name,button)
  if len(found)==1:return found[0]
  time.sleep(.04)
 raise AssertionError((name,button,[(n.name,n.getRoleName()) for n in nodes()]))
def text():return find('Contract search').queryText().getText(0,-1)
def key(value):
 subprocess.run(['xdotool','key','--clearmodifiers',value],check=True);time.sleep(.08)
def type_text(value):
 subprocess.run(['xdotool','type','--clearmodifiers','--delay','10',value],check=True);time.sleep(.12)
def point(node):
 x,y,w,h=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
 return x+w//2,y+h//2
def click_at(p):
 subprocess.run(['xdotool','mousemove',str(p[0]),str(p[1]),'click','1'],check=True);time.sleep(.12)
def click(name,button=False):click_at(point(find(name,button)))
def events():return Counter(re.findall(r'^event=(\w+)$',(out/'application.log').read_text(),re.M))
def delta(before,name,count):assert events()[name]-before[name]==count,(name,events(),before,count)
steps=[]
def snapshot(name):
 key('F10')
 steps.append({'step':name,'text':text(),'events':dict(events()),'input_states':[pyatspi.stateToString(s) for s in find('Contract search').getState().getStates()]})
log=(out/'application.log').open('w')
app=subprocess.Popen(['<toolchain-cache>/target/debug/examples/__parity_search_probe',mode,variant],stdout=log,stderr=log)
try:
 find('Search contracts')
 for _ in range(100):
  found=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(app.pid)],capture_output=True,text=True)
  if found.stdout.strip():break
  time.sleep(.04)
 window=found.stdout.split()[-1]
 subprocess.run(['xdotool','windowactivate','--sync',window],check=True)
 time.sleep(.3);click_at((460,350));key('F5');time.sleep(.2);click('Contract search');find('Contract search')
 assert text()==''
 before=events();type_text('alpha');key('Return')
 assert text()=='alpha',(text(),events(),[(n.name,n.getRoleName()) for n in nodes() if n.getState().contains(pyatspi.STATE_FOCUSED)]);delta(before,'Changed',5);delta(before,'Submit',1);snapshot('type-and-submit')
 clear_position=point(find('Clear',True))
 key('F6');time.sleep(.1)
 assert not find('Contract search').getState().contains(pyatspi.STATE_ENABLED)
 assert not matches('Clear',True)
 before=events();type_text('z');key('BackSpace');click_at(clear_position)
 assert text()=='alpha',(text(),events(),[(n.name,n.getRoleName()) for n in nodes() if n.getState().contains(pyatspi.STATE_FOCUSED)]);delta(before,'Changed',0);delta(before,'Clear',0);snapshot('disabled-preserves-text')
 key('F7');key('F5');time.sleep(.1)
 assert find('Contract search').getState().contains(pyatspi.STATE_ENABLED)
 before=events();click('Clear',True)
 assert text()=='';delta(before,'Changed',1);delta(before,'Clear',1)
 assert find('Contract search').getState().contains(pyatspi.STATE_FOCUSED);snapshot('reenabled-clear-refocus')
 type_text('beta');key('F8');time.sleep(.1)
 assert not matches('Clear',True)
 assert find('Contract search').getState().contains(pyatspi.STATE_READ_ONLY)
 before=events();type_text('z');key('BackSpace');assert text()=='beta';delta(before,'Changed',0);delta(before,'Clear',0);snapshot('readonly-preserves-text')
 key('F9');time.sleep(.1)
 before=events();click('Cancel',True)
 assert text()=='';delta(before,'Changed',1);delta(before,'Cancel',1);snapshot('editable-cancel-clears')
 subprocess.run(['import','-window',window,str(out/'after-cancel.png')],check=True)
 (out/'result.json').write_text(json.dumps({'mode':mode,'variant':variant,'passed':True,'steps':steps},indent=2)+'\n')
 print('PASS',mode,variant,flush=True)
finally:
 app.terminate()
 try:app.wait(timeout=2)
 except subprocess.TimeoutExpired:app.kill();app.wait()
 log.close()
