"""Passive xev observer; creates no window, grabs no input and changes no focus/services."""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import signal
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument('--out', type=Path, required=True)
parser.add_argument('--window', help='Existing native X11 client; otherwise poll its title')
parser.add_argument('--seconds', type=float, default=55)
args = parser.parse_args()
args.out.mkdir(parents=True, exist_ok=False)


def stamp():
    return {'received_utc': datetime.now(timezone.utc).isoformat(timespec='microseconds'),
            'received_monotonic_ns': time.monotonic_ns()}


window = args.window
deadline = time.monotonic() + 40
while not window and time.monotonic() < deadline:
    search = subprocess.run(['xdotool', 'search', '--onlyvisible', '--name',
                             'weGPUI control contracts'], capture_output=True, text=True)
    matches = search.stdout.strip().splitlines()
    if matches:
        window = matches[0]
        break
    time.sleep(.1)
if not window:
    raise SystemExit('No existing GPUI client found; no observer attached.')
command = ['stdbuf', '-oL', '-eL', 'xev', '-id', window, '-event', 'focus']
(args.out / 'observer.json').write_text(json.dumps({
    **stamp(), 'command': command, 'window': window,
    'scope': 'Select FocusChangeMask on existing window only; timestamps are observer receive times, not X server event times.'
}, indent=2) + '\n')
with (args.out / 'xev.txt').open('w') as raw, (args.out / 'events.jsonl').open('w') as events:
    process = subprocess.Popen(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    timer = time.monotonic() + args.seconds

    def stop(*_):
        process.terminate()

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGALRM, stop)
    signal.setitimer(signal.ITIMER_REAL, args.seconds)
    try:
        for line in process.stdout:
            raw.write(line)
            raw.flush()
            events.write(json.dumps({**stamp(), 'line': line.rstrip('\n')}) + '\n')
            events.flush()
            if time.monotonic() >= timer:
                process.terminate()
        process.wait()
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        if process.poll() is None:
            process.terminate()
        process.wait()
(args.out / 'complete.json').write_text(json.dumps({**stamp(), 'xev_returncode': process.returncode}) + '\n')
