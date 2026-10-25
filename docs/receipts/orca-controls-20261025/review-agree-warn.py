"""Check actual Orca summaries between each KP_Enter and its named capture."""
from datetime import datetime
import array
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import wave

ROOT=Path(__file__).parent
OUT=ROOT/'agree-warn-final2/session'
actions=[]
for line in(OUT/'actions.txt').read_text().splitlines():
    stamp,action=line.split(' ',1)
    actions.append((datetime.fromisoformat(stamp),action))
speech=[]
for line in(OUT/'speech-transcript.txt').read_text().splitlines():
    match=re.match(r"(\d\d:\d\d:\d\d\.\d+) - SPEECH OUTPUT: '(.*)' \{",line)
    if match:
        stamp=datetime.fromisoformat('2026-10-25T'+match[1]+'+00:00')
        speech.append((stamp,match[2]))
phases=[('warn-false-unchecked',False,False),('warn-true-unchecked',True,False),
        ('warn-cleared-unchecked',False,False),('warn-false-checked',False,True),
        ('warn-true-checked',True,True),('warn-cleared-checked',False,True)]
results=[]
for label,invalid,checked in phases:
    index=next(i for i,(_,action)in enumerate(actions)if action.startswith('capture '+label+' '))
    end=actions[index][0]
    start=next(stamp for stamp,action in reversed(actions[:index])if action=='XTEST key KP_Enter')
    phrases=[phrase for stamp,phrase in speech if start<=stamp<=end]
    assert('invalid entry.'in phrases)==invalid,(label,phrases)
    expected='Read and accept Terms check box '+('checked.'if checked else'not checked.')
    assert expected in phrases,(label,phrases)
    tree=json.loads((OUT/f'{label}-native.json').read_text())
    node=next(n for n in tree if n['name']=='Read and accept Terms')
    assert('invalid entry'in node['states'])==invalid,(label,node)
    assert('checked'in node['states'])==checked,(label,node)
    assert'enabled'in node['states']and'focusable'in node['states'],(label,node)
    results.append({'phase':label,'from':start.isoformat(),'to':end.isoformat(),
                    'invalid':invalid,'checked':checked,'actual_reader_phrases':phrases,'native_states':node['states']})
log=(OUT/'application.log').read_text()
assert re.findall(r'^WARN change (\d+) checked=(\w+)$',log,re.M)==[('1','true')],log
assert re.findall(r'^WARN terms (\d+)$',log,re.M)==['1'],log
assert'WARN SNAP warn=false checked=true changes=1 links=1'in log,log
with wave.open(str(OUT/'speech.wav'),'rb')as f:
    samples=array.array('h',f.readframes(f.getnframes()))
    audio={'duration_seconds':f.getnframes()/f.getframerate(),'sample_rate':f.getframerate(),
            'channels':f.getnchannels(),'sample_width':f.getsampwidth(),
            'peak':max(map(abs,samples),default=0),'rms':math.sqrt(sum(s*s for s in samples)/max(1,len(samples)))}
assert audio['sample_rate']==16000 and audio['channels']==1 and audio['sample_width']==2 and audio['rms']>0
(ROOT/'agree-warn-review.json').write_text(json.dumps({'phases':results,'audio':audio,'callbacks':{'change':1,'terms':1},
    'transcript_sha256':hashlib.sha256((OUT/'speech-transcript.txt').read_bytes()).hexdigest(),
    'pcm_sha256':hashlib.sha256((OUT/'speech.wav').read_bytes()).hexdigest()},indent=2)+'\n')
for name in('agree-warn-final1','agree-warn-final2'):
    p=ROOT/name/'session/orca-debug.log'
    if p.exists():
        with p.with_suffix('.log.gz').open('wb')as target:
            with gzip.GzipFile(filename='',mode='wb',fileobj=target,mtime=0)as zipped:zipped.write(p.read_bytes())
print('PASS6 actual reader/native validity summaries, true/false clearing in both checked states, one change and one link callback, nonzero PCM')
