"""Check the saved real Orca output and passive focus-mode observations."""
import gzip
import json
from pathlib import Path
import re
import wave

ROOT = Path(__file__).resolve().parent
session = ROOT / 'orca-icons-cells/session'
data = json.loads((ROOT / 'orca-correlation.json').read_text())
events = data['observed_focus_modes']
assert [(event['kind'], event['mode']) for event in events] == [
    ('FocusOut', 'NotifyGrab'), ('FocusIn', 'NotifyGrab'),
    ('FocusOut', 'NotifyUngrab'), ('FocusIn', 'NotifyUngrab')]
assert not data['at_window_focus_pulses'] and not data['flat_review_reset_lines']
assert any('icon-2 button grayed.' in line for line in data['speech_lines'])
assert (session / 'run-status.txt').read_text().strip() == 'exit=0'
log = session / 'application.log'
output = log.read_text() if log.exists() else gzip.decompress(log.with_suffix('.log.gz').read_bytes()).decode()
assert 'unknown contract page' not in output
assert 'panicked at' not in output
summary = (session / 'audio-summary.txt').read_text()
assert float(re.search(r'rms:\s*([\d.]+)', summary)[1]) > 0
with wave.open(str(session / 'speech.wav'), 'rb') as audio:
    assert audio.getnframes() > 0 and audio.getframerate() == 16000
print('PASS: actual Orca retained flat-review target and emitted "icon-2 button grayed." during held reader command.')
print('PASS: actual Grab/Ungrab remained observable without AT window/focus pulses or flat-review reset; PCM nonzero.')
print('Scope: named real-reader transcript/audio sample; full listening and screen-reader acceptance remain separate.')
