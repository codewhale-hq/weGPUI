"""Real X11 focus/grab controls via installed libX11; no extra Python dependency."""
import argparse
import ctypes as C
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import time

import pyatspi

parser = argparse.ArgumentParser()
parser.add_argument('--window', type=int, required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=False)

lib = C.CDLL('libX11.so.6')
display_t, window_t = C.c_void_p, C.c_ulong
lib.XOpenDisplay.argtypes = [C.c_char_p]
lib.XOpenDisplay.restype = display_t
lib.XDefaultRootWindow.argtypes = [display_t]
lib.XDefaultRootWindow.restype = window_t
lib.XCreateSimpleWindow.argtypes = [display_t, window_t, C.c_int, C.c_int,
                                   C.c_uint, C.c_uint, C.c_uint, C.c_ulong, C.c_ulong]
lib.XCreateSimpleWindow.restype = window_t
lib.XStoreName.argtypes = [display_t, window_t, C.c_char_p]
lib.XMapWindow.argtypes = [display_t, window_t]
lib.XSetInputFocus.argtypes = [display_t, window_t, C.c_int, C.c_ulong]
lib.XGetInputFocus.argtypes = [display_t, C.POINTER(window_t), C.POINTER(C.c_int)]
lib.XGrabKeyboard.argtypes = [display_t, window_t, C.c_int, C.c_int, C.c_int, C.c_ulong]
lib.XGrabKeyboard.restype = C.c_int
lib.XUngrabKeyboard.argtypes = [display_t, C.c_ulong]
lib.XSync.argtypes = [display_t, C.c_int]
lib.XDestroyWindow.argtypes = [display_t, window_t]
lib.XCloseDisplay.argtypes = [display_t]
display = lib.XOpenDisplay(None)
assert display, 'XOpenDisplay failed'
root = lib.XDefaultRootWindow(display)
control = lib.XCreateSimpleWindow(display, root, 750, 50, 240, 100, 0, 0, 0xDDDDDD)
lib.XStoreName(display, control, b'Isolated X11 focus control')
lib.XMapWindow(display, control)
lib.XSync(display, 0)
time.sleep(.7)
records = []


def stamp():
    return {'utc': datetime.now(timezone.utc).isoformat(timespec='microseconds'),
            'monotonic_ns': time.monotonic_ns()}


def walk(node):
    yield node
    for index in range(node.childCount):
        yield from walk(node.getChildAtIndex(index))


def record(label, phase):
    focus, revert = window_t(), C.c_int()
    lib.XGetInputFocus(display, C.byref(focus), C.byref(revert))
    nodes = list(walk(pyatspi.Registry.getDesktop(0)))
    frame = next(node for node in nodes if node.getRoleName() == 'frame')
    states = [pyatspi.stateToString(value) for value in frame.getState().getStates()]
    item = {**stamp(), 'label': label, 'phase': phase, 'native_input_focus': focus.value,
            'revert_to': revert.value, 'app_window': args.window, 'control_window': control,
            'frame_states': states, 'frame_active': 'active' in states,
            'app_atspi_focused': [{'role': node.getRoleName(), 'name': node.name}
                                 for node in nodes if node.getState().contains(pyatspi.STATE_FOCUSED)]}
    records.append(item)
    (args.out / 'states.json').write_text(json.dumps(records, indent=2) + '\n')
    return item


def settled(label, expected_focus, expected_active):
    time.sleep(.6)
    result = record(label, 'settled')
    subprocess.run(['import', '-window', str(args.window), str(args.out / f'{label}.png')], check=True)
    assert result['native_input_focus'] == expected_focus, result
    assert result['frame_active'] == expected_active, result


def focus(window):
    lib.XSetInputFocus(display, window, 2, 0)
    lib.XSync(display, 0)


def grab():
    result = lib.XGrabKeyboard(display, control, 0, 1, 1, 0)
    lib.XSync(display, 0)
    assert result == 0, f'XGrabKeyboard status {result}'


try:
    subprocess.run(['xdotool', 'windowactivate', '--sync', str(args.window)], check=True)
    subprocess.run(['xdotool', 'key', 'Tab'], check=True)
    settled('baseline', args.window, True)
    record('normal-away', 'before')
    focus(control)
    settled('normal-away', control, False)
    record('normal-back', 'before')
    focus(args.window)
    settled('normal-back', args.window, True)
    record('temporary-grab', 'before')
    grab()
    settled('temporary-grab', args.window, True)
    record('temporary-ungrab', 'before')
    lib.XUngrabKeyboard(display, 0)
    lib.XSync(display, 0)
    settled('temporary-ungrab', args.window, True)
    grab()
    time.sleep(.3)
    record('while-grabbed-away', 'before')
    focus(control)
    settled('while-grabbed-away', control, False)
    record('while-grabbed-back', 'before')
    focus(args.window)
    settled('while-grabbed-back', args.window, True)
    lib.XUngrabKeyboard(display, 0)
    lib.XSync(display, 0)
    settled('final-ungrab', args.window, True)
    (args.out / 'result.txt').write_text('PASS: native focus and AT window activation survive temporary grabs; real normal/while-grabbed transfers remain functional.\n')
finally:
    lib.XUngrabKeyboard(display, 0)
    lib.XDestroyWindow(display, control)
    lib.XSync(display, 0)
    lib.XCloseDisplay(display)
