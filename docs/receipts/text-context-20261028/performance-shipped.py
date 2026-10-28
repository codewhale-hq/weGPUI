import hashlib, json, pathlib, statistics, subprocess, tempfile, time
root = pathlib.Path(__file__).resolve().parent
binaries = {
    'shipped': pathlib.Path('/Volumes/VIXinSSD/cw-build-cache/target/wegpui-shipped-baseline-probe/debug/wegpui-khmer-stream-probe'),
    'context': pathlib.Path('/Volumes/VIXinSSD/cw-build-cache/target/wegpui-context-probe/debug/wegpui-khmer-stream-probe'),
}
expected = {'shipped': '5c9b42830f769a939021abb221704844e8d434eee25558d3489b1273bc6682a8', 'context': 'ce4ef833de3a76a79ecac556e6e0d731c70c4fbc5912c7eb166843a558c23527'}
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
for k, p in binaries.items(): assert sha(p) == expected[k]
items = [('ascii', 'ordinary text and spaces ' * n) for n in [128, 512, 1024]] + [('thai', 'ภาษาไทย' * n) for n in [128, 512, 1024]] + [('alternating', 'กກ' * n) for n in [512, 1024, 2048]]
results=[]
with tempfile.TemporaryDirectory() as tmp:
    p=pathlib.Path(tmp)/'input.json'
    for label,text in items:
        payload={'version':[78,2,0,0], 'cases':[{'id':label,'language':'km','text':text,'exact_icu4c_boundaries':[0,len(text.encode())]}]}
        p.write_text(json.dumps(payload,ensure_ascii=False))
        entry={'input':label,'codepoints':len(text),'bytes':len(text.encode()),'samples':{k:[] for k in binaries}}
        outputs={}
        for repeat in range(5):
            for k in (['shipped','context'] if repeat % 2 == 0 else ['context','shipped']):
                start=time.perf_counter()
                result=subprocess.run([str(binaries[k]),str(p)],capture_output=True,text=True,timeout=30)
                elapsed=time.perf_counter()-start
                if result.returncode: raise RuntimeError((label,k,result.stderr))
                item=json.loads(result.stdout)['cases'][0]
                entry['samples'][k].append(elapsed)
                outputs[k]=item
            assert outputs['shipped']['baseline'] == outputs['context']['baseline']
        entry['boundary_counts']={k:len(v['candidate']) for k,v in outputs.items()}
        entry['median_seconds']={k:statistics.median(v) for k,v in entry['samples'].items()}
        results.append(entry)
for k,p in binaries.items(): assert sha(p) == expected[k]
(root/'performance-shipped-result.json').write_text(json.dumps({'mode':'debug executable end-to-end, process startup and JSON included, two provider calls per sample; five alternating-order samples per size', 'binary_sha256':expected, 'results':results},indent=2)+'\n')
for x in results: print(x['input'],x['codepoints'],x['median_seconds'])
