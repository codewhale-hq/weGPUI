import datetime
import json
import os
import subprocess
import threading
import time
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from PIL import Image
from playwright.sync_api import sync_playwright
from snapshot import sha, snapshot

root = Path('/work/full-parity/dictionary-extended')
binary = Path('/target/debug/wegpui-dictionary-extended-probe')
manifest = json.loads((root / 'build-manifest.json').read_text())
if manifest['source'] != snapshot() or manifest['binary_sha256'] != sha(binary):
    raise SystemExit('Build inputs or binary changed; rebuild before comparison')
if subprocess.run(['xdpyinfo', '-display', ':215'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
    raise SystemExit('Display :215 is in use; choose a free fixture display')
name = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%fZ')
out = root / ('comparison-' + name)
out.mkdir()
(out / 'mismatches').mkdir()
env = dict(os.environ, DISPLAY=':215', GPUI_X11_SCALE_FACTOR='2', LIBGL_ALWAYS_SOFTWARE='1',
           VK_ICD_FILENAMES='/usr/share/vulkan/icd.d/lvp_icd.json', XDG_RUNTIME_DIR='/work/runtime')
log = (out / 'native.txt').open('w')
xvfb = subprocess.Popen(['Xvfb', ':215', '-screen', '0', '2400x2100x24', '-nolisten', 'tcp'], env=env, stdout=log, stderr=log)
wm = None
process = None
native_path = root / 'native.json'
native_path.unlink(missing_ok=True)
try:
    for _ in range(100):
        if subprocess.run(['xdpyinfo', '-display', ':215'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0:
            break
        if xvfb.poll() is not None:
            raise RuntimeError('Fixture Xvfb exited')
        time.sleep(.05)
    else:
        raise RuntimeError('Fixture Xvfb did not start')
    wm = subprocess.Popen(['openbox', '--sm-disable'], env=env, stdout=log, stderr=log)
    process = subprocess.Popen([str(binary)], env=env, stdout=log, stderr=log)
    for _ in range(1800):
        found = subprocess.run(['xdotool', 'search', '--onlyvisible', '--pid', str(process.pid)], env=env, capture_output=True, text=True)
        if found.stdout.strip() and native_path.exists():
            try:
                native = json.loads(native_path.read_text())
                if len(native) == 192:
                    break
            except (OSError, json.JSONDecodeError):
                pass
        if process.poll() is not None:
            raise RuntimeError('Native fixture exited before completing')
        time.sleep(.05)
    else:
        raise RuntimeError('Native fixture did not complete 192 shape cases')
    window = found.stdout.splitlines()[0]
    for _ in range(20):
        subprocess.run(['import', '-window', window, str(out / 'native.png')], env=env, check=True, capture_output=True)
        image = Image.open(out / 'native.png').convert('RGB')
        if any(low != high for low, high in image.getextrema()):
            break
        time.sleep(.25)
    else:
        raise RuntimeError('Native sample window remained blank')
    (out / 'native.json').write_text(json.dumps(native, indent=2, ensure_ascii=False) + '\n')
finally:
    if process is not None:
        process.terminate()
        process.wait(timeout=5)
    if wm is not None:
        wm.terminate()
        wm.wait(timeout=5)
    xvfb.terminate()
    xvfb.wait(timeout=5)
    log.close()

class Quiet(SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass

server = ThreadingHTTPServer(('127.0.0.1', 0), partial(Quiet, directory='/work/code-parity/reference'))
threading.Thread(target=server.serve_forever, daemon=True).start()
results = []
try:
    with sync_playwright() as playwright:
        browser = playwright.chromium.launch(executable_path='/usr/bin/chromium')
        page = browser.new_page(viewport={'width': 1024, 'height': 900}, locale='zh-CN')
        page.goto(f'http://127.0.0.1:{server.server_port}/dist/example/index.html#form_page')
        page.wait_for_function("Array.from(document.querySelectorAll('#container > .page')).at(-1)?.classList.contains('js_show')")
        desc = page.locator('.weui-form__desc')
        for case in native:
            desc.evaluate("(e,c)=>{e.textContent=c.text;e.style.width=c.width+'px';e.style.fontFamily=c.language==='lo'?\"'Noto Sans Lao',system-ui\":''}", case)
            measured = desc.evaluate('''e=>{let t=e.firstChild,rows={};for(let i=0;i<t.length;i++){let r=new Range();r.setStart(t,i);r.setEnd(t,i+1);let b=r.getBoundingClientRect();let row=rows[b.y]??={text:'',left:Infinity,right:-Infinity};row.text+=t.textContent[i];row.left=Math.min(row.left,b.left);row.right=Math.max(row.right,b.right)}return {lines:Object.values(rows).map(r=>r.text),widths:Object.values(rows).map(r=>r.right-r.left)}}''')
            session = page.context.new_cdp_session(page)
            session.send('DOM.enable')
            session.send('CSS.enable')
            document = session.send('DOM.getDocument')['root']['nodeId']
            node = session.send('DOM.querySelector', {'nodeId': document, 'selector': '.weui-form__desc'})['nodeId']
            fonts = session.send('CSS.getPlatformFontsForNode', {'nodeId': node})
            session.detach()
            normalize = lambda rows: [row.rstrip(' ') for row in rows]
            match = normalize(case['lines']) == normalize(measured['lines'])
            if not match:
                desc.screenshot(path=str(out / 'mismatches' / f"{case['id']}-{int(case['width'])}.png"))
            results.append(dict(case, reference_lines=measured['lines'], reference_row_widths=measured['widths'],
                                reference_fonts=fonts, match=match))
        version = browser.version
        browser.close()
finally:
    server.shutdown()

if manifest['source'] != snapshot() or manifest['binary_sha256'] != sha(binary):
    raise SystemExit(f'Source or binary changed during capture; partial evidence: {out}')
(out / 'comparison.json').write_text(json.dumps(results, indent=2, ensure_ascii=False) + '\n')
summary = {
    'comparison_count': len(results),
    'matching_row_sequences': sum(case['match'] for case in results),
    'per_language': {language: {'total': sum(case['language'] == language for case in results),
                                'matches': sum(case['language'] == language and case['match'] for case in results)}
                     for language in ('km', 'lo', 'my', 'th')},
    'browser_version': version, 'browser_locale': 'zh-CN',
    'font_note': 'Lao reference explicitly requests Noto Sans Lao; inspect CDP font evidence for all cases',
    'normalization': 'strip trailing U+0020 only; preserve NBSP and ZWSP',
    'binary_sha256': manifest['binary_sha256'], 'build_inputs': manifest['source'],
}
(out / 'summary.json').write_text(json.dumps(summary, indent=2, ensure_ascii=False, sort_keys=True) + '\n')
print(json.dumps(summary | {'build_inputs': f"{len(manifest['source'])} hashes", 'output': str(out)}, indent=2))
