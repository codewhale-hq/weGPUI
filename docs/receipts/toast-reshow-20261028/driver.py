import json, os, subprocess, sys, time, traceback, pyatspi
from pathlib import Path
out=Path('/work/full-parity/toast-reshow-probe')
def nodes(node):
 yield node
 for i in range(node.childCount):
  child=node.getChildAtIndex(i)
  if child: yield from nodes(child)
def elements(name):
 apps=[n for n in nodes(pyatspi.Registry.getDesktop(0)) if n.getRoleName()=='application' and n.name==name]
 assert len(apps)==1,(name,len(apps))
 return list(nodes(apps[0]))
def snap(name):
 es=elements(name)
 status=[n.name for n in es if n.getRoleName()=='status bar' and n.name.startswith('Open ')]
 assert len(status)==1,status
 return {'ns':time.monotonic_ns(),'status':status[0], 'nodes':[{'role':n.getRoleName(),'name':n.name} for n in es]}
def action(name, label):
 targets=[n for n in elements(name) if n.getRoleName()=='push button' and n.name==label]
 assert len(targets)==1,(label,len(targets))
 assert targets[0].queryComponent().grabFocus(),label
 t0=time.monotonic_ns(); subprocess.run(['xdotool','key','Return'],check=True); t1=time.monotonic_ns()
 return [t0,t1]
def expect(s,opened,gen,shows,closes):
 expected=f'Open {str(opened).lower()}; generation {gen}; shows {shows}; closes {closes}'
 assert s['status']==expected,(expected,s['status'])
def run(variant,reduced):
 name='toast_reshow_'+variant; key=variant+('-reduced' if reduced else '-normal'); path=out/(key+'.json'); data={'name':name,'reduced':reduced}; log=open(out/(key+'.log'),'w')
 env=os.environ.copy()
 if reduced: env['TOAST_REDUCED']='1'
 proc=subprocess.Popen(['/target/debug/'+name],env=env,stdout=log,stderr=subprocess.STDOUT)
 try:
  for _ in range(100):
   try: data['initial']=snap(name); break
   except Exception:
    if proc.poll() is not None: raise RuntimeError('process exited')
    time.sleep(.1)
  else: raise RuntimeError('AT-SPI startup timed out')
  expect(data['initial'],False,0,0,0)
  buttons=[n for n in elements(name) if n.getRoleName()=='push button' and n.name=='Show toast']
  assert len(buttons)==1 and buttons[0].queryComponent().grabFocus()
  time.sleep(.2)
  data['first']=action(name,'Show toast'); time.sleep(1.2)
  data['second']=action(name,'Show toast'); time.sleep(2.1)
  data['overlap']=snap(name)
  assert data['overlap']['ns']-data['first'][1]>3_000_000_000
  assert data['overlap']['ns']-data['second'][0]<3_000_000_000
  expect(data['overlap'],variant=='candidate',1,2,0)
  time.sleep(1.2); data['second_done']=snap(name); expect(data['second_done'],False,1,2,0)
  data['single']=action(name,'Show toast'); time.sleep(3.3)
  data['single_done']=snap(name); expect(data['single_done'],False,2,3,0)
  data['before_hide']=action(name,'Show toast'); time.sleep(.4)
  data['hide']=action(name,'Hide toast'); time.sleep(.4)
  data['reopen']=action(name,'Show toast'); time.sleep(2.4)
  data['reopen_overlap']=snap(name)
  assert data['reopen_overlap']['ns']-data['before_hide'][1]>3_000_000_000
  assert data['reopen_overlap']['ns']-data['reopen'][0]<3_000_000_000
  expect(data['reopen_overlap'],True,4,5,1)
  time.sleep(1); data['reopen_done']=snap(name); expect(data['reopen_done'],False,4,5,1)
  data['cancel_show']=action(name,'Show toast'); data['cancel_hide']=action(name,'Hide toast'); time.sleep(3.3)
  data['cancel_done']=snap(name); expect(data['cancel_done'],False,5,6,2)
  data['passed']=True
 except Exception:
  data['passed']=False; data['error']=traceback.format_exc(); raise
 finally:
  path.write_text(json.dumps(data,indent=2)+'\n'); proc.terminate()
  try: proc.wait(timeout=3)
  except subprocess.TimeoutExpired: proc.kill(); proc.wait()
  log.close()
for variant in ('baseline','candidate'):
 for reduced in (False,True):
  print('starting',variant,'reduced',reduced,flush=True);run(variant,reduced);print('passed',variant,'reduced',reduced,flush=True)
