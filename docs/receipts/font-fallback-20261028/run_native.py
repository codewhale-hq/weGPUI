import json, os, subprocess, time
from pathlib import Path

root = Path('/work/full-parity/fallback-preservation-probe')
output = root / 'native-diagnostic.json'
if output.exists():
    raise SystemExit('Existing diagnostic output will not be overwritten')
env = dict(os.environ, DISPLAY=':216', GPUI_X11_SCALE_FACTOR='2', LIBGL_ALWAYS_SOFTWARE='1',
           VK_ICD_FILENAMES='/usr/share/vulkan/icd.d/lvp_icd.json', XDG_RUNTIME_DIR='/work/runtime',
           WEGPUI_RENDER_CORPUS='/work/full-parity/fallback-preservation-probe/corpus.json')
def display_up():
    return subprocess.run(['xdpyinfo', '-display', ':216'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0
if display_up(): raise SystemExit('Display :216 already in use')
with (root / 'native.log').open('w') as log:
    xvfb = subprocess.Popen(['Xvfb', ':216', '-screen', '0', '2400x2100x24', '-nolisten', 'tcp'], env=env, stdout=log, stderr=log)
    wm = proc = None
    try:
        for _ in range(100):
            if display_up(): break
            if xvfb.poll() is not None: raise RuntimeError('Xvfb exited')
            time.sleep(.05)
        else: raise RuntimeError('Xvfb failed to start')
        wm = subprocess.Popen(['openbox', '--sm-disable'], env=env, stdout=log, stderr=log)
        proc = subprocess.Popen(['/target/debug/wegpui-fallback-preservation-probe'], env=env, stdout=log, stderr=log)
        for _ in range(1800):
            if output.exists():
                try:
                    data = json.loads(output.read_text())
                    if len(data) == 8: break
                except (OSError, json.JSONDecodeError): pass
            if proc.poll() is not None: raise RuntimeError('Diagnostic exited before completing')
            time.sleep(.05)
        else: raise RuntimeError('Timed out')
        print(f'Collected {len(data)} cases in {output}')
    finally:
        for process in (proc, wm, xvfb):
            if process is not None:
                process.terminate()
                process.wait(timeout=5)
