"""Capture a native weGPUI gallery page and the matching official WeUI example.

Usage:
  python3 -I scripts/capture.py ROUTE [--state NAME] [--theme light|dark] [--height 812]
      [--scroll Y] [--brand RRGGBB] [--care] [--name NAME] [--only gpui|weui] [--out DIR]

Writes DIR/NAME-{theme}-gpui.png and DIR/NAME-{theme}-weui.png (default DIR docs/parity,
NAME = ROUTE[-STATE]). The WeUI side replays the JavaScript registered for STATE in
scripts/parity/*.json ({"route:state": "js"}) against the page served at
WEUI_URL (default http://127.0.0.1:7193/dist/example/index.html, served by
scripts/weui_upstream.py). Native capture uses Quartz/screencapture on macOS and xdotool plus
ImageMagick `import` on X11 Linux (GPUI_X11_SCALE_FACTOR defaults to 2 there); see
docs/PARITY-TOOLING.md.
"""

import argparse
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parent.parent
TARGET = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
BINARY = TARGET / 'debug/examples/gallery'
WEUI_URL = os.environ.get('WEUI_URL', 'http://127.0.0.1:7193/dist/example/index.html')
WIDTH = 375


def weui_states():
    states = {}
    for f in sorted((ROOT / 'scripts/parity').glob('*.json')):
        states.update(json.loads(f.read_text()))
    return states


def to_srgb(im):
    import io
    from PIL import ImageCms
    icc = im.info.get('icc_profile')
    im = im.convert('RGB')
    if icc:
        src = ImageCms.ImageCmsProfile(io.BytesIO(icc))
        im = ImageCms.profileToProfile(im, src, ImageCms.createProfile('sRGB'), outputMode='RGB')
    return im


def window_id(pid, height):
    import Quartz
    infos = Quartz.CGWindowListCopyWindowInfo(Quartz.kCGWindowListOptionAll, Quartz.kCGNullWindowID)
    for info in infos:
        if info.get('kCGWindowOwnerPID') == pid and info.get('kCGWindowLayer') == 0:
            bounds = info.get('kCGWindowBounds')
            if bounds and abs(bounds['Width'] - WIDTH) <= 1 and height <= bounds['Height'] <= height + 60:
                return info['kCGWindowNumber']
    return None


def capture_gpui(args, out):
    cmd = [str(BINARY), '--page', args.route, '--theme', args.theme, '--size', f'{WIDTH}x{args.height}',
           '--position', '60,60', '--reduce-motion']
    if args.state:
        cmd += ['--state', args.state]
    if args.scroll:
        cmd += ['--scroll', str(args.scroll)]
    if args.brand:
        cmd += ['--brand', args.brand]
    if args.care:
        cmd += ['--care']
    if sys.platform.startswith('linux'):
        return capture_gpui_x11(args, cmd, out)
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    try:
        wid = None
        for _ in range(100):
            time.sleep(0.1)
            wid = window_id(proc.pid, args.height)
            if wid:
                break
        if not wid:
            raise SystemExit(f'gallery window not found; stderr: {proc.stderr.read1().decode() if proc.poll() is not None else ""}')
        time.sleep(args.settle)
        with tempfile.TemporaryDirectory() as tmp:
            raw = pathlib.Path(tmp) / 'raw.png'
            subprocess.run(['screencapture', '-x', '-o', '-l', str(wid), str(raw)], check=True)
            from PIL import Image
            im = Image.open(raw)
            scale = round(im.width / WIDTH)
            content_h = args.height * scale
            top = im.height - content_h
            to_srgb(im).crop((0, top, im.width, im.height)).save(out)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


def capture_weui(args, out):
    from playwright.sync_api import sync_playwright
    states = weui_states()
    key = f'{args.route}:{args.state}' if args.state else None
    if key and key not in states:
        raise SystemExit(f'no WeUI replay registered for {key}')
    with sync_playwright() as p:
        browser = p.chromium.launch()
        ctx = browser.new_context(viewport={'width': WIDTH, 'height': args.height}, device_scale_factor=2,
                                  color_scheme=args.theme)
        page = ctx.new_page()
        page.goto(f'{WEUI_URL}#{args.route}')
        page.wait_for_timeout(1200)
        if args.brand:
            page.evaluate(f"document.body.style.setProperty('--weui-BRAND', '#{args.brand}')")
        if args.care:
            page.evaluate("document.body.setAttribute('data-weui-mode', 'care')")
        if key:
            page.evaluate(f'(async () => {{ {states[key]} }})()')
            page.wait_for_timeout(1200)
        if args.scroll:
            page.evaluate(f"document.querySelector('.page:last-child').scrollTop = {args.scroll}")
            page.wait_for_timeout(300)
        page.screenshot(path=str(out))
        browser.close()


def capture_gpui_x11(args, cmd, out):
    env = dict(os.environ, GPUI_X11_SCALE_FACTOR=os.environ.get('GPUI_X11_SCALE_FACTOR', '2'))
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, env=env)
    try:
        wid = None
        for _ in range(100):
            time.sleep(0.1)
            found = subprocess.run(['xdotool', 'search', '--pid', str(proc.pid), '--name', 'weGPUI'],
                                   capture_output=True, text=True)
            if found.stdout.strip():
                wid = found.stdout.split()[0]
                break
        if not wid:
            raise SystemExit(f'gallery window not found; stderr: {proc.stderr.read1().decode() if proc.poll() is not None else ""}')
        time.sleep(args.settle)
        with tempfile.TemporaryDirectory() as tmp:
            raw = pathlib.Path(tmp) / 'raw.png'
            subprocess.run(['import', '-window', wid, str(raw)], check=True)
            from PIL import Image
            Image.open(raw).convert('RGB').save(out)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('route')
    ap.add_argument('--state')
    ap.add_argument('--theme', default='light', choices=['light', 'dark'])
    ap.add_argument('--height', type=int, default=812)
    ap.add_argument('--scroll', type=int, default=0)
    ap.add_argument('--brand')
    ap.add_argument('--care', action='store_true')
    ap.add_argument('--name')
    ap.add_argument('--only', choices=['gpui', 'weui'])
    ap.add_argument('--out', default=str(ROOT / 'docs/parity'))
    ap.add_argument('--settle', type=float, default=1.0)
    args = ap.parse_args()
    name = args.name or (args.route + (f'-{args.state}' if args.state else ''))
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    if args.only != 'weui':
        capture_gpui(args, out / f'{name}-{args.theme}-gpui.png')
    if args.only != 'gpui':
        capture_weui(args, out / f'{name}-{args.theme}-weui.png')
    gp, wp = out / f'{name}-{args.theme}-gpui.png', out / f'{name}-{args.theme}-weui.png'
    if gp.exists() and wp.exists() and os.environ.get('PAIR_DIR'):
        from PIL import Image
        a, b = Image.open(gp), Image.open(wp)
        pair = Image.new('RGB', (a.width + b.width + 20, max(a.height, b.height)), (255, 0, 255))
        pair.paste(a, (0, 0))
        pair.paste(b, (a.width + 20, 0))
        pair = pair.resize((pair.width // 2, pair.height // 2))
        dest = pathlib.Path(os.environ['PAIR_DIR'])
        dest.mkdir(parents=True, exist_ok=True)
        pair.save(dest / f'{name}-{args.theme}-pair.png')
        print(dest / f'{name}-{args.theme}-pair.png')
    print(gp, wp)


if __name__ == '__main__':
    sys.exit(main())
