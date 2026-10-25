"""Launch the native gallery on every route in each language, capture the window and scan its
accessibility tree for untranslated Chinese.

Usage:
  python3 -I scripts/i18n_sweep.py --langs en,ar [--routes all|button,form] [--out DIR]
      [--no-capture] [--no-scan] [--settle 1.0]

For each (language, route) the gallery runs with `--page ROUTE --lang LANG` at 375x812.
Captures use the gallery window only (macOS `screencapture -l`). The scan dumps the window's
accessibility tree with scripts/ax_dump.js and reports CJK ideographs that survive in a
non-Chinese language; the Languages page is exempt because its samples are intentionally native.
Writes DIR/summary.json. The exit status is 1 when any leak, launch failure or scan failure is found.
macOS only; needs Screen Recording (capture) and Accessibility (scan) permission.
"""

import argparse
import json
import os
import pathlib
import re
import subprocess
import sys
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parent.parent
TARGET = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
BINARY = TARGET / 'debug/examples/gallery'
WIDTH, HEIGHT = 375, 812
CJK = re.compile(r'[぀-ヿ㐀-䶿一-鿿＀-￯]')
EXEMPT_ROUTES = {'languages'}


def all_routes():
    text = (ROOT / 'examples/gallery/pages/mod.rs').read_text()
    return re.findall(r'^\s+\("([^"]+)", [a-z_]+::build\),$', text, re.M)


def window_id(pid):
    import Quartz
    for info in Quartz.CGWindowListCopyWindowInfo(Quartz.kCGWindowListOptionAll, Quartz.kCGNullWindowID):
        if info.get('kCGWindowOwnerPID') == pid and info.get('kCGWindowLayer') == 0:
            bounds = info.get('kCGWindowBounds')
            if bounds and abs(bounds['Width'] - WIDTH) <= 1 and HEIGHT <= bounds['Height'] <= HEIGHT + 60:
                return info['kCGWindowNumber']
    return None


def run_one(lang, route, out, args):
    cmd = [str(BINARY), '--page', route, '--lang', lang, '--size', f'{WIDTH}x{HEIGHT}',
           '--position', '60,60', '--reduce-motion']
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    result = {'lang': lang, 'route': route, 'status': 'ok', 'leaks': []}
    try:
        wid = None
        for _ in range(100):
            time.sleep(0.1)
            wid = window_id(proc.pid)
            if wid or proc.poll() is not None:
                break
        if not wid:
            result['status'] = 'no-window'
            return result
        time.sleep(args.settle)
        if not args.no_capture:
            png = out / f'{route}-{lang}.png'
            subprocess.run(['screencapture', '-x', '-o', '-l', str(wid), str(png)], check=True)
            result['png'] = png.name
        if not args.no_scan:
            dump = subprocess.run(['osascript', '-l', 'JavaScript', str(ROOT / 'scripts/ax_dump.js'), str(proc.pid)],
                                  capture_output=True, text=True, timeout=120)
            if dump.returncode != 0 or not dump.stdout.strip():
                result['status'] = 'scan-failed'
                result['error'] = dump.stderr.strip()[:300]
            elif not lang.startswith('zh') and route not in EXEMPT_ROUTES:
                result['leaks'] = sorted({line.strip() for line in dump.stdout.splitlines() if CJK.search(line)})
                if result['leaks']:
                    result['status'] = 'leak'
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
    return result


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--langs', default='en')
    ap.add_argument('--routes', default='all')
    ap.add_argument('--out', default=tempfile.mkdtemp(prefix='wegpui-i18n-'))
    ap.add_argument('--settle', type=float, default=1.0)
    ap.add_argument('--no-capture', action='store_true')
    ap.add_argument('--no-scan', action='store_true')
    args = ap.parse_args()
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    routes = all_routes() if args.routes == 'all' else args.routes.split(',')
    results = []
    for lang in args.langs.split(','):
        for route in routes:
            r = run_one(lang, route, out, args)
            results.append(r)
            flag = '' if r['status'] == 'ok' else f"  <-- {r['status']} {len(r['leaks'])}"
            print(f"{lang:8} {route:28}{flag}", flush=True)
    (out / 'summary.json').write_text(json.dumps(results, ensure_ascii=False, indent=1))
    bad = [r for r in results if r['status'] != 'ok']
    print(f"{len(results)} runs, {len(bad)} problems; summary: {out / 'summary.json'}")
    sys.exit(1 if bad else 0)


if __name__ == '__main__':
    main()
