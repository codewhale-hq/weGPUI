from pathlib import Path
from datetime import datetime
import array,hashlib,json,math,re,sys,wave
base=Path('<evidence>/layers-orca')
result=json.loads((base/'result.json').read_text());debug=(base/'orca-debug.log').read_text();speechd=(base/'speechd/speech-dispatcher.log').read_text()
lines=[line for line in debug.splitlines() if ' - SPEECH OUTPUT: ' in line]
(base/'speech-transcript.txt').write_text('\n'.join(lines)+'\n')
checks=[]
for i,step in enumerate(result['steps']):
 if not step['step'].endswith(('focused','selected')):continue
 name=step['step'].split()[0]
 start=datetime.fromisoformat(step['utc']).strftime('%H:%M:%S.%f')
 end=datetime.fromisoformat(result['steps'][i+1]['utc']).strftime('%H:%M:%S.%f')
 matching=[line for line in lines if start<=line.split(' - ')[0]<=end and f"SPEECH OUTPUT: '{name} button.'" in line]
 assert matching,(name,step['step'])
 assert step['focused']==[name]
 if step['step'].endswith('selected'):assert step['layer_controls']==[name]
 checks.append({'step':step['step'],'start':start,'end':end,'matching_speech':matching,'focus':step['focused'],'controls':step['layer_controls']})
assert len(checks)==8
assert 'Orca version 48.1' in debug
assert 'Speak the current flat review object' in debug
assert 'Module espeak-ng started successfully' in speechd
formats=set(re.findall(r'ALSA: Setting sample format to (\w+)',speechd));rates=set(re.findall(r'ALSA: Setting sample rate to (\d+)',speechd));channels=set(re.findall(r'ALSA: Setting channel count to (\d+)',speechd))
assert formats=={'S16_LE'} and rates=={'22050'} and channels=={'1'},(formats,rates,channels)
raw=(base/'speech.raw').read_bytes();samples=array.array('h');samples.frombytes(raw)
if sys.byteorder!='little':samples.byteswap()
assert len(samples)>22050 and max(map(abs,samples))>0
with wave.open(str(base/'speech.wav'),'wb') as audio:
 audio.setnchannels(1);audio.setsampwidth(2);audio.setframerate(22050);audio.writeframes(raw)
summary={'passed':True,'reader':'Orca 48.1','at_spi':'2.56.2','speech_dispatcher':'0.12.0','synthesizer':'eSpeak NG 1.52.0','gallery_binary_sha256':hashlib.sha256(Path('<toolchain-cache>/target/debug/examples/gallery').read_bytes()).hexdigest(),'native_steps':len(result['steps']),'speech_windows_verified':len(checks),'speech_checks':checks,'audio':{'method':'Real synthesizer server audio through ALSA file PCM, slave null; no utterances injected','format':'S16_LE','sample_rate':22050,'channels':1,'sample_count':len(samples),'duration_seconds':len(samples)/22050,'peak':max(map(abs,samples)),'rms':math.sqrt(sum(s*s for s in samples)/len(samples)),'raw_sha256':hashlib.sha256(raw).hexdigest(),'wav_sha256':hashlib.sha256((base/'speech.wav').read_bytes()).hexdigest()},'qualification':'Bounded English light-mode reader navigation/current-object generation and nonzero real synthesized PCM. No human listening, complete reader matrix, audio timing or other-host acceptance.'}
(base/'verification.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k!='speech_checks'},indent=2))
