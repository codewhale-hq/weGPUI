import json,os,subprocess,time
from pathlib import Path
out=Path(os.environ['PROBE_OUTPUT']);win=os.environ['PROBE_WINDOW'];log=out/'app.log';results=[]
def key(k):
 subprocess.run(['xdotool','key','--clearmodifiers',k],check=True);time.sleep(1)
def snap(label):
 key('F12');deadline=time.monotonic()+30
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
key('F1');initial=snap('initial');assert initial['bytes']==2475 and initial['utf16']==825 and initial['selection']==[0,0] and all(initial['rectangles'])
for name,clipboard in [('tai-le','F7'),('han','F8')]:
 key('ctrl+End');a=snap(name+'-caret-at-end');assert a['selection']==[825,825]
 key(clipboard);key('ctrl+v');a=snap(name+'-pasted');assert a['bytes']==2478 and a['utf16']==826 and a['selection']==[826,826]
 key('F1');after=snap(name+'-prefix');assert after['selection']==[0,0] and after['prefix']==initial['prefix']
 key('F3');a=snap(name+'-selection');assert a['selection']==[0,4] and a['marked'] is None
 key('ctrl+z');a=snap(name+'-undo');assert a['bytes']==2475 and a['utf16']==825 and a['marked'] is None
 key('F1');restored=snap(name+'-undo-prefix');assert restored['rectangles']==initial['rectangles']
 print(name,'prefix geometry changed:',after['rectangles']!=initial['rectangles'],flush=True)
print('PASS native Tai Le/Han paste, selection and undo controls',flush=True)
