"""Validate recorded contract observations; no screen-reader or cross-platform verdict."""
from pathlib import Path
import json
import sys

root = Path(sys.argv[1])


def snapshot(step):
    return json.loads((root / f'{step}.json').read_text())


def named(step, name):
    matches = [node for node in snapshot(step) if node['name'] == name]
    assert len(matches) == 1, (step, name, matches)
    return matches[0]


def selected(step, name):
    assert 'selected' in named(step, name)['states'], (step, name)


assert named('initial', 'Controlled upload')['value'] == 25
assert named('progress-advance', 'Controlled upload')['value'] == 50
assert not any(node['name'] == 'Cancel upload' for node in snapshot('cancel-hidden'))
for step, name in [('navbar-pointer', 'Details'), ('navbar-arrow', 'History'),
                   ('navbar-home', 'Overview'), ('navbar-at-action', 'History'),
                   ('tabbar-key', 'Files'), ('tabbar-at-action', 'Settings')]:
    selected(step, name)
assert named('slider-end', 'Step three slider')['value'] == 9
assert named('slider-set-value', 'Step three slider')['value'] == 9
assert named('fractional-set-value', 'Fractional slider')['value'] == 0.375
assert named('fractional-key', 'Fractional slider')['value'] == 0.5
assert 'focused' in named('skip-disabled-and-readonly', 'Open record，Controlled reusable MediaBox callback')['states']
assert named('disabled-set-value', 'Disabled slider')['value'] == 60
assert named('readonly-set-value', 'Read-only slider')['value'] == 40
assert 'read only' in named('readonly-set-value', 'Read-only slider')['states']
assert 'focusable' not in named('disabled-set-value', 'Disabled slider')['states']
if 'attributes' in snapshot('initial')[0]:
    headings = [node for node in snapshot('initial') if node['name'] == 'Repeated heading']
    assert len(headings) == 2 and all('level:6' in node['attributes'] for node in headings)
    disabled = named('disabled-set-value', 'Disabled slider')['states']
    assert 'enabled' not in disabled and 'sensitive' not in disabled
assert not any(node['name'] == 'Fixed footer callback' for node in snapshot('modal-isolation'))
assert any(node['name'] == 'Fixed footer callback' for node in snapshot('modal-restore'))
assert not any(node['name'] == 'Floating data notice' for node in snapshot('notice-close'))

log = (root / 'application.log').read_text()
for event, count in [('progress-50', 1), ('progress-cancel', 1), ('cancel-hidden', 1),
                     ('navbar-1', 1), ('navbar-2', 2), ('navbar-0', 1),
                     ('tabbar-1', 1), ('tabbar-2', 1), ('slider-integer-9', 1),
                     ('slider-fractional-0.375', 1), ('slider-fractional-0.5', 1),
                     ('panel-open', 2), ('cell-open', 2), ('grid-open', 1),
                     ('footer-open', 1), ('fixed-footer-open', 1),
                     ('modal-open', 1), ('modal-dismiss', 1),
                     ('notice-link', 2), ('notice-close', 1)]:
    assert log.count(f'event={event} ') == count, (event, count, log)
assert 'UNEXPECTED' not in log
announcements = [json.loads(line)['text'] for line in (root / 'announcements.log').read_text().splitlines()
                 if line.startswith('{')]
for text in ('Floating data notice', 'Loading data records', 'Upload 25 percent',
             'Upload 50 percent', 'Upload canceled'):
    assert text in announcements, (text, announcements)
assert 'Notice details' not in announcements
assert 'Close notice' not in announcements
skip_gallery = 'Skip gallery: 1' in (root / 'provenance.txt').read_text()
if skip_gallery:
    assert not (root / 'gallery-article-assert.txt').exists()
else:
    assert 'PASS:' in (root / 'gallery-article-assert.txt').read_text()
print('PASS: progress values/cancel/live events, controlled nav, slider snapping/guards, callbacks, fixed-footer isolation'
      + ('; gallery explicitly skipped' if skip_gallery else ', article semantics'))
