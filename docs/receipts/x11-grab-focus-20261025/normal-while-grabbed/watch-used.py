"""Record actual AT-SPI window/focus events in an isolated test session."""
from datetime import datetime, timezone
import json
from pathlib import Path
import signal
import sys
import time

import pyatspi

output = Path(sys.argv[1]).open('w')


def event_received(event):
    item = {'utc': datetime.now(timezone.utc).isoformat(timespec='microseconds'),
            'monotonic_ns': time.monotonic_ns(), 'type': event.type,
            'detail1': event.detail1, 'detail2': event.detail2}
    try:
        item.update(name=event.source.name, role=event.source.getRoleName())
    except Exception as error:
        item['source_error'] = repr(error)
    output.write(json.dumps(item) + '\n')
    output.flush()


def stop(*_):
    pyatspi.Registry.stop()


signal.signal(signal.SIGTERM, stop)
pyatspi.Registry.registerEventListener(event_received, 'window:', 'object:state-changed:focused')
try:
    pyatspi.Registry.start()
finally:
    output.close()
