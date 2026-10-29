import json,os,pathlib,subprocess,traceback
here=pathlib.Path(__file__).resolve().parent
base=pathlib.Path('/Volumes/VIXinSSD/CW/weGPUI/target/debug')
for variant in ('baseline','candidate'):
 for reduced in (False,True):
  tag=variant+('-reduced' if reduced else '-normal'); env=os.environ.copy(); env.pop('TOPTIPS_REDUCED',None)
  if reduced:env['TOPTIPS_REDUCED']='1'
  result={'variant':variant,'reduced':reduced}
  print('starting',tag,flush=True)
  try:
   process=subprocess.run([str(base/('toptips_macos_'+variant))],env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,timeout=45)
   (here/(tag+'.stdout')).write_text(process.stdout);(here/(tag+'.stderr')).write_text(process.stderr)
   result['exit']=process.returncode; assert process.returncode==0,process.returncode
   events=[json.loads(line) for line in process.stdout.splitlines() if line.startswith('{"stage":')]
   d={e['stage']:e for e in events}; result['events']=events
   assert len(events)==14 and len(d)==14,(len(events),list(d))
   def check(key,opened,shows,closes,opens):
    e=d[key];assert (e['open'],e['shows'],e['closes'],e['opens'])==(opened,shows,closes,opens),(key,e)
   check('first',True,1,0,1);check('second',True,2,0,1)
   check('overlap',variant=='candidate',2,0,1)
   assert d['overlap']['elapsed_ms']-d['first']['elapsed_ms']>2000
   assert d['overlap']['elapsed_ms']-d['second']['elapsed_ms']<2000
   check('second_done',False,2,0,1);check('single_done',False,3,0,2)
   check('reopen_overlap',True,5,1,4)
   assert d['reopen_overlap']['elapsed_ms']-d['before_hide']['elapsed_ms']>2000
   assert d['reopen_overlap']['elapsed_ms']-d['reopen']['elapsed_ms']<2000
   check('reopen_done',False,5,1,4);check('cancel_done',False,6,2,5)
   result['passed']=True
  except Exception:
   result['passed']=False; result['error']=traceback.format_exc()
   raise
  finally:
   (here/(tag+'.json')).write_text(json.dumps(result,indent=2)+'\n')
  print('passed',tag,flush=True)
