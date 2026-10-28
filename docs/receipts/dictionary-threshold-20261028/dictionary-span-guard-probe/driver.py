import json,os,subprocess,time
from pathlib import Path
out=Path(os.environ['PROBE_OUTPUT']);win=os.environ['PROBE_WINDOW'];log=out/'app.log';results=[]
def key(k):
 subprocess.run(['xdotool','key','--clearmodifiers',k],check=True);time.sleep(1)
def snap(label):
 key('F12'); deadline=time.monotonic()+30
 while True:
  rows=[json.loads(x[len('SNAPSHOT '):]) for x in log.read_text().splitlines() if x.startswith('SNAPSHOT ')]
  if len(rows)>len(results):break
  if time.monotonic()>deadline:raise AssertionError((label,'snapshot timeout'))
  time.sleep(.2)
 assert len(rows)==len(results)+1
 item=rows[-1];item['label']=label;results.append(item);(out/'result.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n');print(label,json.dumps(item,ensure_ascii=False),flush=True)
 subprocess.run(['import','-window',win,str(out/(label+'.png'))],check=True)
 return item
time.sleep(4)
key('F1');initial=snap('initial');assert initial['bytes']==24576 and initial['utf16']==8192 and initial['selection']==[0,0] and all(initial['rectangles'])
key('ctrl+End');a=snap('caret-at-end');assert a['selection']==[8192,8192]
key('F7');key('ctrl+v');a=snap('paste-at-end');assert a['bytes']==24579 and a['utf16']==8193 and a['selection']==[8193,8193]
key('F1');above=snap('above-prefix');assert above['selection']==[0,0] and above['prefix']==initial['prefix']
key('F3');a=snap('above-selection');assert a['selection']==[0,4] and a['marked'] is None
key('ctrl+z');a=snap('undo-paste');assert a['bytes']==24576 and a['utf16']==8192 and a['marked'] is None
key('F1');below=snap('below-prefix');assert below['rectangles']==initial['rectangles']
key('ctrl+shift+z');a=snap('redo-paste');assert a['bytes']==24579 and a['utf16']==8193
key('ctrl+End');key('BackSpace');a=snap('delete-at-end');assert a['bytes']==24576 and a['utf16']==8192
key('ctrl+z');a=snap('undo-delete');assert a['bytes']==24579 and a['utf16']==8193
key('F1');a=snap('restored-above-prefix');assert a['rectangles']==above['rectangles']
print('prefix geometry changed above threshold:',above['rectangles']!=initial['rectangles'],flush=True)
print('PASS native paste, undo/redo, deletion and selection at span limit',flush=True)
