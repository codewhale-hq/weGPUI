"""Drive the native gallery with real input events and capture after each step.

Usage:
  python3 -I scripts/drive.py ROUTE --out DIR [--theme light|dark] [--height 812] [--state S] [--lang TAG] [--dir ltr|rtl] STEP...

Steps (coordinates are window-content points, origin top-left):
  click:X,Y          left click
  drag:X1,Y1,X2,Y2   press, move in 12 steps, release
  key:NAME           key press, e.g. key:tab key:shift-tab key:enter key:space key:escape key:left key:a key:cmd-a
  type:TEXT          type literal text (ASCII or Unicode via key events)
  wait:SECONDS
  shot:NAME          save DIR/NAME.png (content only, @2x)

The window is activated before input. This moves the real pointer and takes keyboard focus.
"""

import argparse
import pathlib
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from capture import BINARY, WIDTH, to_srgb, window_id  # noqa: E402

import Quartz  # noqa: E402

KEYCODES = {
    'a': 0, 's': 1, 'd': 2, 'f': 3, 'h': 4, 'g': 5, 'z': 6, 'x': 7, 'c': 8, 'v': 9, 'b': 11, 'q': 12,
    'w': 13, 'e': 14, 'r': 15, 'y': 16, 't': 17, '1': 18, '2': 19, '3': 20, '4': 21, '6': 22, '5': 23,
    '9': 25, '7': 26, '8': 28, '0': 29, 'o': 31, 'u': 32, 'i': 34, 'p': 35, 'l': 37, 'j': 38, 'k': 40,
    'n': 45, 'm': 46, 'return': 36, 'enter': 36, 'tab': 48, 'space': 49, 'delete': 51, 'backspace': 51,
    'escape': 53, 'left': 123, 'right': 124, 'down': 125, 'up': 126, 'home': 115, 'end': 119,
    'pageup': 116, 'pagedown': 121, '[': 33,
}
MODS = {'cmd': Quartz.kCGEventFlagMaskCommand, 'shift': Quartz.kCGEventFlagMaskShift,
        'alt': Quartz.kCGEventFlagMaskAlternate, 'ctrl': Quartz.kCGEventFlagMaskControl}


def window_bounds(wid):
    for info in Quartz.CGWindowListCopyWindowInfo(Quartz.kCGWindowListOptionIncludingWindow, wid):
        b = info['kCGWindowBounds']
        return b['X'], b['Y'], b['Width'], b['Height']


def post_mouse(kind, x, y):
    ev = Quartz.CGEventCreateMouseEvent(None, kind, (x, y), Quartz.kCGMouseButtonLeft)
    Quartz.CGEventPost(Quartz.kCGHIDEventTap, ev)


def post_key(spec):
    parts = spec.split('-')
    key, flags = parts[-1], 0
    for m in parts[:-1]:
        flags |= MODS[m]
    code = KEYCODES[key.lower()]
    for down in (True, False):
        ev = Quartz.CGEventCreateKeyboardEvent(None, code, down)
        Quartz.CGEventSetFlags(ev, flags)
        Quartz.CGEventPost(Quartz.kCGHIDEventTap, ev)
        time.sleep(0.03)


def post_text(text):
    for ch in text:
        for down in (True, False):
            ev = Quartz.CGEventCreateKeyboardEvent(None, 0, down)
            Quartz.CGEventSetFlags(ev, 0)
            Quartz.CGEventKeyboardSetUnicodeString(ev, len(ch.encode('utf-16-le')) // 2, ch)
            Quartz.CGEventPost(Quartz.kCGHIDEventTap, ev)
            time.sleep(0.02)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('route')
    ap.add_argument('steps', nargs='*')
    ap.add_argument('--out', required=True)
    ap.add_argument('--theme', default='light')
    ap.add_argument('--height', type=int, default=812)
    ap.add_argument('--state')
    ap.add_argument('--lang', default='zh-Hans')
    ap.add_argument('--dir', choices=['ltr', 'rtl'])
    ap.add_argument('--motion', action='store_true', help='keep animations enabled')
    args = ap.parse_args()
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    cmd = [str(BINARY), '--page', args.route, '--theme', args.theme, '--size', f'{WIDTH}x{args.height}',
           '--position', '60,60', '--motion' if args.motion else '--reduce-motion']
    cmd += ['--lang', args.lang]
    if args.dir:
        cmd += ['--dir', args.dir]
    if args.state:
        cmd += ['--state', args.state]
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        wid = None
        for _ in range(100):
            time.sleep(0.1)
            wid = window_id(proc.pid, args.height)
            if wid:
                break
        if not wid:
            raise SystemExit('gallery window not found')
        time.sleep(0.8)
        x0, y0, w, h = window_bounds(wid)
        top = y0 + h - args.height
        subprocess.run(['osascript', '-e', f'tell application "System Events" to set frontmost of (first process whose unix id is {proc.pid}) to true'],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        post_mouse(Quartz.kCGEventMouseMoved, x0 + w - 2, top + 2)
        time.sleep(0.4)
        for step in args.steps:
            kind, _, val = step.partition(':')
            if kind == 'click':
                x, y = (float(v) for v in val.split(','))
                post_mouse(Quartz.kCGEventMouseMoved, x0 + x, top + y)
                time.sleep(0.05)
                post_mouse(Quartz.kCGEventLeftMouseDown, x0 + x, top + y)
                time.sleep(0.06)
                post_mouse(Quartz.kCGEventLeftMouseUp, x0 + x, top + y)
                time.sleep(0.35)
            elif kind == 'drag':
                x1, y1, x2, y2 = (float(v) for v in val.split(','))
                post_mouse(Quartz.kCGEventMouseMoved, x0 + x1, top + y1)
                post_mouse(Quartz.kCGEventLeftMouseDown, x0 + x1, top + y1)
                for i in range(1, 13):
                    time.sleep(0.02)
                    post_mouse(Quartz.kCGEventLeftMouseDragged, x0 + x1 + (x2 - x1) * i / 12, top + y1 + (y2 - y1) * i / 12)
                post_mouse(Quartz.kCGEventLeftMouseUp, x0 + x2, top + y2)
                time.sleep(0.4)
            elif kind == 'key':
                post_key(val)
                time.sleep(0.25)
            elif kind == 'type':
                post_text(val)
                time.sleep(0.25)
            elif kind == 'wait':
                time.sleep(float(val))
            elif kind == 'shot':
                with tempfile.TemporaryDirectory() as tmp:
                    raw = pathlib.Path(tmp) / 'raw.png'
                    subprocess.run(['screencapture', '-x', '-o', '-l', str(wid), str(raw)], check=True)
                    from PIL import Image
                    im = Image.open(raw)
                    scale = round(im.width / WIDTH)
                    to_srgb(im).crop((0, im.height - args.height * scale, im.width, im.height)).save(out / f'{val}.png')
                print(out / f'{val}.png')
            else:
                raise SystemExit(f'unknown step {step}')
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


if __name__ == '__main__':
    main()
