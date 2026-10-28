import json,os,subprocess,time
from pathlib import Path
out=Path(os.environ['PROBE_OUTPUT']); win=os.environ['PROBE_WINDOW']; log=out/'app.log'
def key(k):
 subprocess.run(['xdotool','key','--clearmodifiers',k],check=True);time.sleep(1)
def snap(label):
 key('F12'); deadline=time.monotonic()+30
 while True:
  rows=[json.loads(x[len('SNAPSHOT '):]) for x in log.read_text().splitlines() if x.startswith('SNAPSHOT ')]
  if len(rows)>len(results):break
  if time.monotonic()>deadline:raise AssertionError((label,'snapshot timeout',len(rows),len(results)))
  time.sleep(.2)
 assert len(rows)==len(results)+1,(label,len(rows),len(results)); item=rows[-1]; item['label']=label;results.append(item);(out/'result.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n'); print(label,json.dumps(item,ensure_ascii=False),flush=True)
 subprocess.run(['import','-window',win,str(out/(label+'.png'))],check=True)
 return item
results=[]
time.sleep(4)
key('F1'); a=snap('initial');assert a['bytes']==65536 and a['selection']==[0,0] and a['marked'] is None and all(a['rectangles'])
key('F2');a=snap('invalid-surrogate-caret');assert a['selection']==[0,0]
key('x');a=snap('real-key-insert');assert a['bytes']==65537 and a['prefix'].startswith('x😀') and a['selection']==[1,1]
key('BackSpace');a=snap('real-key-delete');assert a['bytes']==65536 and a['prefix'].startswith('😀') and a['selection']==[0,0]
key('F3');a=snap('select-grapheme');assert a['selection']==[0,4] and a['marked'] is None
key('F4');a=snap('synthetic-preedit');assert a['bytes']>65536 and a['marked']==[0,11] and a['prefix'].startswith('ภาษาไทยภาษา')
key('F5');a=snap('synthetic-cancel');assert a['bytes']==65536 and a['selection']==[0,4] and a['marked'] is None and a['prefix'].startswith('😀')
key('F4');a=snap('synthetic-preedit-again');assert a['bytes']>65536 and a['marked']==[0,11]
key('F6');a=snap('synthetic-commit');assert a['bytes']>65536 and a['marked'] is None and a['selection']==[11,11]
key('ctrl+z');a=snap('undo-commit');assert a['bytes']==65536 and a['marked'] is None and a['prefix'].startswith('😀')
print('PASS sampled threshold insertion/deletion, selection, synthetic composition and undo',flush=True)
