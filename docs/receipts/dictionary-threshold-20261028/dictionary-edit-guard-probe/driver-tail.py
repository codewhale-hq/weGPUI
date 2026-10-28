import json,os,subprocess,time
from pathlib import Path
out=Path(os.environ['PROBE_OUTPUT']);win=os.environ['PROBE_WINDOW'];log=out/'tail-app.log';results=[]
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
 item=rows[-1];item['label']=label;results.append(item);(out/'result-tail.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n');print(label,json.dumps(item,ensure_ascii=False),flush=True)
 subprocess.run(['import','-window',win,str(out/('tail-'+label+'.png'))],check=True)
 return item
time.sleep(4)
key('F1');initial=snap('initial');assert initial['bytes']==65536 and initial['selection']==[0,0]
key('ctrl+End');a=snap('caret-at-end');assert a['selection']==[a['utf16'],a['utf16']]
key('x');a=snap('insert-at-end');assert a['bytes']==65537 and a['selection']==[a['utf16'],a['utf16']]
key('F1');above=snap('above-prefix');assert above['bytes']==65537 and above['selection']==[0,0] and above['prefix']==initial['prefix']
key('ctrl+End');key('BackSpace');a=snap('delete-at-end');assert a['bytes']==65536 and a['selection']==[a['utf16'],a['utf16']]
key('F1');below=snap('below-prefix');assert below['bytes']==65536 and below['selection']==[0,0] and below['prefix']==initial['prefix']
assert below['rectangles']==initial['rectangles'],(initial,below)
print('prefix geometry changed while above threshold:', above['rectangles']!=initial['rectangles'],flush=True)
print('PASS real tail insertion/deletion and prefix geometry restoration',flush=True)
