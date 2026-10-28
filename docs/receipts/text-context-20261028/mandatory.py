import contextlib,hashlib,io,json,pathlib,runpy,subprocess,sys
root=pathlib.Path(__file__).resolve().parent;art=root.parent.parent
base=art/'chromium-icu-78-oracle/oracle.py';run=base.parent/'run-20261028T083419.883341Z';sys.argv=[str(base),str(run)];out=io.StringIO()
with contextlib.redirect_stdout(out):mod=runpy.run_path(str(base),run_name='__main__')
header=json.loads(out.getvalue());controls={'lf':'\n','crlf':'\r\n','nel':'\u0085','line-separator':'\u2028','paragraph-separator':'\u2029'}
words={'km':'ភាសាខ្មែរ','th':'ภาษาไทย','lo':'ພາສາລາວ','my':'မြန်မာစာ','cjk-fallback':'中文ภาษาไทย','non-complex':'Plain text'};cases=[]
for language,left in words.items():
 for key,control in controls.items():
  text=left+control+left;target,status=mod['boundaries'](text);mandatory=len((left+control).encode());between=len((left+'\r').encode()) if key=='crlf' else None
  cases.append({'id':language+'-'+key,'language':'km','text':text,'actual_language':language,'control':key,'mandatory_boundary':mandatory,'forbidden_crlf_mid':between,'exact_icu4c_boundaries':target,'status':status})
p=root/'mandatory-oracle.json';p.write_text(json.dumps({**header,'cases':cases},ensure_ascii=False,indent=2)+'\n')
binary=pathlib.Path('/Volumes/VIXinSSD/cw-build-cache/target/wegpui-context-probe/debug/wegpui-khmer-stream-probe');digest=hashlib.sha256(binary.read_bytes()).hexdigest();assert digest=='ce4ef833de3a76a79ecac556e6e0d731c70c4fbc5912c7eb166843a558c23527'
p=subprocess.run([str(binary),str(p)],capture_output=True,text=True,timeout=30);assert p.returncode==0,p.stderr
result=json.loads(p.stdout);index={x['id']:x for x in cases};bad=[]
for x in result['cases']:
 c=index[x['id']];must=c['mandatory_boundary'];mid=c['forbidden_crlf_mid'];
 if must not in x['candidate'] or (mid is not None and mid in x['candidate']):bad.append({'case':c,'result':x})
result['binary_sha256']=digest;result['mandatory_failures']=bad
(root/'mandatory-result.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
print('cases',len(cases),'oracle matches',sum(x['candidate_matches'] for x in result['cases']),'mandatory failures',len(bad))
if bad:raise SystemExit(1)
