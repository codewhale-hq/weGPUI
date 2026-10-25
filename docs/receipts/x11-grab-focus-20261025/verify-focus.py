"""Verify recorded native X focus modes, real input focus and AT-SPI activation."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent / 'normal-while-grabbed'
data = json.loads((ROOT / 'mode-state-correlation.json').read_text())
for label, mode, kind, active in [
    ('normal-away', 'NotifyNormal', 'FocusOut', False),
    ('normal-back', 'NotifyNormal', 'FocusIn', True),
    ('while-grabbed-away', 'NotifyWhileGrabbed', 'FocusOut', False),
    ('while-grabbed-back', 'NotifyWhileGrabbed', 'FocusIn', True),
]:
    case = data[label]
    assert any(event['mode'] == mode and event['kind'] == kind
               for event in case['native_modes']), label
    assert case['settled']['frame_active'] == active
    target = 'app_window' if active else 'control_window'
    assert case['settled']['native_input_focus'] == case['settled'][target]
    required = 'window:activate' if active else 'window:deactivate'
    assert any(event['type'] == required for event in case['atspi_events']), label
for label, mode in [('temporary-grab', 'NotifyGrab'), ('temporary-ungrab', 'NotifyUngrab')]:
    case = data[label]
    assert any(event['mode'] == mode for event in case['native_modes']), label
    assert case['settled']['frame_active']
    assert case['settled']['native_input_focus'] == case['settled']['app_window']
    assert not [event for event in case['atspi_events']
                if event['type'] in ['window:activate', 'window:deactivate',
                                     'object:state-changed:focused']], label
assert 'panicked at' not in (ROOT / 'app.txt').read_text()
print('PASS: actual Normal and WhileGrabbed focus transfers preserve native focus and AT activation updates.')
print('PASS: actual Grab/Ungrab keep native focus and produce no false AT activation/focus pulses.')
print('Scope: isolated X11 focus protocol controls; real IME and screen-reader checks are separate.')
