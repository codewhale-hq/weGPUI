"""Check retained Orca-generated phrases, native callbacks and real PCM separately."""
import array
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import wave

ROOT=Path(__file__).parent
sessions={
 'choices':ROOT/'choices-attempt1',
 'buttons':ROOT/'buttons-attempt1/session',
 'agree':ROOT/'agree-attempt1/session',
 'vcode':ROOT/'vcode-attempt1/session',
 'cells':ROOT/'cells-attempt1/session',
 'icons2':ROOT/'icons-attempt2/session',
 'icons3-observed':ROOT/'icons-attempt3-observed/session',
 'icons4-observed':ROOT/'icons-attempt4-observed/session',
}
expected={
 'choices':['Standalone checkbox check box not checked.','Standalone checkbox check box checked.',
            'Checkbox row check box checked.','Switch toggle button not pressed.','Switch toggle button pressed.',
            'Radio C selected radio button.','3 of 3','Second B selected radio button.','2 of 2'],
 'buttons':['Entering flat review.','button-0-false-1 button grayed.','不可用.',
            'button-0-false-2 button.','正在加载.'],
 'agree':['Agreement 0 Terms 0 check box not checked.','Agreement 0 Terms 0 check box checked.',
          'Terms 0 link.','Terms 1 link.','Agreement 2 Terms 2 check box not checked.',
          'Agreement 3 Terms 3 check box not checked.'],
 'vcode':['Send rounded button.','重新发送(3s) button grayed.','重新发送(2s) button grayed.',
          '重新发送(1s) button grayed.','Send plain button.'],
 'cells':['cell-0-1 button grayed.','不可用.','cell-0-2 button.','正在加载.','icon-0 button.','icon-1 button.'],
}
summary={}
for name,path in sessions.items():
    transcript=(path/'speech-transcript.txt').read_text()
    phrases=[m.group(1)for m in re.finditer(r"SPEECH OUTPUT: '(.*)' \{",transcript)]
    for phrase in expected.get(name,[]):assert phrase in phrases,(name,phrase)
    calls={m.group(1):int(m.group(2))for m in re.finditer(r'^CALL (\S+) (\d+)$',(path/'application.log').read_text(),re.M)}
    if name=='choices':assert calls=={'start':1,'checkbox':2,'checkbox-row':1,'switch':2,'radio-2':1,'second-radio-1':1},calls
    if name=='agree':assert calls=={'start':1,'agree-0':2,'terms-0':1,'terms-1':1},calls
    if name=='vcode':assert calls=={'start':1,'vcode-send-0':2,'vcode-reset-0':1,'vcode-send-1':1},calls
    if name in('buttons','cells'):assert calls==({'start':1}if name=='buttons'else{'start':1,'icon-0':1}),calls
    with wave.open(str(path/'speech.wav'),'rb')as f:
        samples=array.array('h',f.readframes(f.getnframes()))
        audio={'channels':f.getnchannels(),'sample_width':f.getsampwidth(),'sample_rate':f.getframerate(),
               'duration_seconds':f.getnframes()/f.getframerate(),'peak':max(map(abs,samples),default=0),
               'rms':math.sqrt(sum(s*s for s in samples)/max(1,len(samples)))}
    assert audio['channels']==1 and audio['sample_width']==2 and audio['sample_rate']==16000 and audio['rms']>0,(name,audio)
    assert(path/'run-status.txt').read_text().strip()=='exit=0',name
    summary[name]={'path':str(path.relative_to(ROOT)),'phrases':phrases,'callbacks':calls,'audio':audio,
                   'transcript_sha256':hashlib.sha256((path/'speech-transcript.txt').read_bytes()).hexdigest(),
                   'pcm_sha256':hashlib.sha256((path/'speech.wav').read_bytes()).hexdigest()}
    debug=path/'orca-debug.log'
    if debug.exists():
        with (path/'orca-debug.log.gz').open('wb')as target:
            with gzip.GzipFile(filename='',mode='wb',fileobj=target,mtime=0)as zipped:zipped.write(debug.read_bytes())
(ROOT/'review-summary.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
print('PASS8 collection sessions, retained requested speech phrases in5 control runs, native callback checks, nonzero16kHzmonoPCM; no human listening claim')
