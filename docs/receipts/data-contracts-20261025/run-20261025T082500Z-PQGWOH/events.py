"""Capture actual AT-SPI announcement events; this is not screen-reader speech."""
from datetime import datetime, timezone
import json
import pyatspi


def event_received(event):
    print(json.dumps(dict(time=datetime.now(timezone.utc).isoformat(), type=event.type,
                          detail1=event.detail1, detail2=event.detail2,
                          text=str(event.any_data)), ensure_ascii=False), flush=True)


pyatspi.Registry.registerEventListener(event_received, 'object:announcement')
print('announcement listener ready', flush=True)
pyatspi.Registry.start()
