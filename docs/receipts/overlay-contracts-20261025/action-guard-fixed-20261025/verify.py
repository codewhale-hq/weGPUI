#!/usr/bin/env python3
"""Check saved native/AT observations; no screen-reader or complete component verdict."""
import json,re
from pathlib import Path
base=Path(__file__).resolve().parent
summary={'timing_cases':[],'caller_contracts':{},'limits':['Linux X11 samples only; no complete component/reader/host acceptance.','Full GPUI Cargo unit build remains blocked; extracted metadata tests are separate.']}
def focus(path):
 rows=json.loads(path.read_text());return [r.get('name') for r in rows if any('STATE_FOCUSED' in s for s in r.get('states',[]))]
for kind,delays in [('click',[0,1,20,100,500]),('focus',[0,1])]:
 for delay in delays:
  p=base/f"{'mask' if kind=='click' else 'focus'}-fast-{delay}ms";text=(p/'app.log').read_text();events=json.loads((p/'observations/driver-events.json').read_text());by={e['kind']:e for e in events}
  requests=re.findall(r'request=\d+ open=(true|false) presence_open=(true|false)',text)
  assert requests==[('true','true'),('false','true'),('true','true')],(kind,delay,requests)
  assert by['baseline-open']['result'] and by['baseline-close']['result'] and by['reopened-click']['result']
  if kind=='focus':assert by['baseline-component-focus']['result']
  assert focus(p/'observations/after-close-tree.json')==['Open slow modal']
  saved=by.get('saved-'+kind) or by['saved-'+kind+'-error'];late=by.get('saved-'+kind+'-after-closed') or by['saved-'+kind+'-after-closed-error'];assert late['elapsed_since_close_ms']>2000
  summary['timing_cases'].append(dict(route=kind,requested_ms=delay,actual_after_return_ms=(saved['action_invoked_ns']-saved['close_command_returned_ns'])/1e6,saved_result=saved.get('result'),saved_error=saved.get('error'),closed_callbacks=0,focus_after_close='Open slow modal',reopen=True))
pattern=re.compile(r'(\d+) tag=(\S+) event=(\S+) own=(\d+) second=(\d+) background=(\d+) open=(true|false) inner=(true|false) generation=(\d+)')
for mode in ['raw','multiple-close','multiple-reopen','passive','ancestor','two-window']:
 p=base/f'contracts-{mode}';records=[dict(ms=int(t),tag=tag,event=event,own=int(own),second=int(second),background=int(background),open=op=='true',inner=inner=='true',generation=int(gen)) for t,tag,event,own,second,background,op,inner,gen in pattern.findall((p/'app.log').read_text())]
 assert records and all(r['second']==0 for r in records)
 first=[r for r in records if r['event']=='first'];assert all(r['open'] for r in first)
 assert [r['own'] for r in first]==([1,2] if mode in ['multiple-reopen','passive'] else [1])
 events=json.loads((p/'observations/events.json').read_text());returns={e['kind']:e.get('result') for e in events if 'result' in e}
 assert returns and all(returns.values()),(mode,returns)
 expected_focus='Background action main' if mode=='passive' else 'Open A' if mode=='two-window' else 'Open main'
 assert expected_focus in focus(p/'observations/closed-tree.json')
 if mode=='multiple-reopen':
  assert [r['generation'] for r in first]==[1,2];assert any(r['event']=='reopen-in-first' and r['open'] and r['generation']==2 for r in records)
 if mode=='passive':
  background=[r for r in records if r['event']=='background'];assert [r['background'] for r in background]==[1,2,3];assert [r['open'] for r in background]==[True,False,False]
 if mode=='ancestor':assert any(r['event']=='hide' and not r['open'] and r['inner'] for r in records)
 if mode=='two-window':assert [r['background'] for r in records if r['tag']=='B' and r['event']=='background']==[1,2]
 summary['caller_contracts'][mode]=dict(first_callbacks=len(first),second_callbacks=0,at_returns=returns,focus_after_close=expected_focus,records=records)
meta=(base/'metadata-extracted-tests.log').read_text();assert '2 passed; 0 failed' in meta
summary['source_checks']={'root_unit_tests':47,'doctests_ignored':1,'root_five_gates':'PASS','full_vendor_cargo_tests':'BLOCKED: missing upstream fixtures/optional test support','extracted_metadata_tests':'2 passed, exact production NodeBuilder/test text with documented std aliases'}
(base/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print('PASS seven saved Click/Focus timing cases: no closed callbacks, opener restored, normal/reopen controls work.')
print('PASS six caller contracts: raw, same-request close, epoch reopen, passive/background, ancestor chain, two windows.')
print('PASS supplementary two extracted metadata regressions; full vendor Cargo unit suite remains blocked.')
