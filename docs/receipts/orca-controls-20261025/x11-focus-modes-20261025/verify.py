"""Check saved X11 mode/reader correlation data; no GUI, service or source is changed."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
run = ROOT / 'icons-attempt4'
data = json.loads((run / 'correlation.json').read_text())
events = data['events_within_command_plus_800ms']
assert [(event['kind'], event['mode']) for event in events] == [
    ('FocusOut', 'NotifyGrab'), ('FocusIn', 'NotifyGrab'),
    ('FocusOut', 'NotifyUngrab'), ('FocusIn', 'NotifyUngrab')]
assert all(event['window'] == '0xa00001' and event['detail'] == 'NotifyNonlinear' for event in events)
assert not data['normal_events_within_interval']
assert all(value == [{'role': 'button', 'name': 'icon-0'}]
           for value in data['app_atspi_focused_snapshots'].values())
assert any(event['detail'] == 'NotifyPointer' for event in data['cleanup_normal_events'])
assert all(event['received_utc'].startswith('2026-10-25T10:11:46.')
           for event in data['cleanup_normal_events'])
text = (run / 'orca-event-excerpt.txt').read_text()
for term in ['Queueing window:deactivate', 'Queueing window:activate',
             'Quitting flat review', 'Creating new context',
             "focused for [button: 'icon-0']"]:
    assert term in text
raw = (run / 'xev.txt').read_text()
assert 'synthetic NO' in raw and 'NotifyGrab' in raw and 'NotifyUngrab' in raw
observer = json.loads((run / 'observer.json').read_text())
assert observer['command'] == ['stdbuf', '-oL', '-eL', 'xev', '-id', '10485761', '-event', 'focus']
print('PASS: same-client Grab/Ungrab focus pairs coincide with held reader command; no Normal in interval.')
print('PASS: sampled app AT-SPI focus remains icon-0; Orca activation/focus pulses and flat-review reset are recorded.')
print('Scope: observed unpatched failure and source trace; proposal uncompiled/unapplied, no causal or reader-acceptance pass.')
