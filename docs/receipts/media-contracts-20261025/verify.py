"""Check recorded native media contracts; screenshots and screen readers are separate."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def flatten(node):
    yield node
    for child in node['children']:
        yield from flatten(child)


def nodes(step, run='runtime'):
    return list(flatten(json.loads((ROOT / run / f'{step}.json').read_text())))


def named(step, name, run='runtime'):
    return [node for node in nodes(step, run) if node['name'] == name]


def focused(step, name, run='runtime'):
    assert any('focused' in node['states'] for node in named(step, name, run)), (run, step, name)


def row(step):
    return next(node for node in named(step, 'Controlled row') if 'focusable' in node['states'])


def summary(step):
    return next(node['name'] for node in nodes(step) if node['role'] == 'label' and node['name'].startswith('Files '))


assert named('initial', 'Image 1') and named('initial', 'Add image')
assert named('capacity-progress', '20%') and not named('capacity-progress', 'Add image')
focused('capacity-progress', 'Fail upload')
assert named('error', '错误') and not named('error', 'Image 2')
assert named('done', 'Image 2') and not named('done', '错误')
assert any(node['role'] == 'dialog' for node in nodes('thumbnail-preview'))
for step in ['keyboard-dismiss', 'pointer-dismiss', 'escape-dismiss']:
    assert not any(node['role'] == 'dialog' for node in nodes(step))
    focused(step, 'Image 1')
assert not named('pointer-gallery-delete', 'Image 1')
assert named('pointer-gallery-delete', 'Image 2')
focused('pointer-gallery-delete', 'Add image')
assert not named('keyboard-gallery-delete', 'Image 2')
focused('keyboard-gallery-delete', 'Add image')
assert not named('thumbnail-delete-empty', 'Image 3')
assert summary('thumbnail-delete-empty').startswith('Files 0;')
assert named('swipe-open', 'Archive') and named('swipe-open', 'Delete row')
assert 'swipe true' in summary('swipe-open')
assert summary('swipe-repeat-open') == summary('swipe-open')
focused('swipe-first-action', 'Archive')
for step in ['swipe-escape', 'swipe-action-callback', 'swipe-delete-callback', 'swipe-rejected', 'swipe-right-close']:
    assert 'swipe false' in summary(step), step
    assert not named(step, 'Archive') and not named(step, 'Delete row'), step
    focused(step, 'Controlled row')
assert 'accepted false' in summary('swipe-rejected')
assert named('swipe-drag-open', 'Archive') and 'swipe true' in summary('swipe-drag-open')
assert not any('focusable' in node['states'] for node in named('initial', 'No actions'))
focused('image-focus', 'Image preview', 'focus-runtime')
focused('delete-focus', 'Delete preview image', 'focus-runtime')
log = (ROOT / 'runtime/app.log').read_text()
for message, count in [('gallery dismiss', 3), ('gallery delete', 2), ('thumbnail delete', 1),
                       ('archive callback', 1), ('row delete callback', 1)]:
    assert log.splitlines().count(message) == count, (message, count)
assert 'panicked at' not in log
for run in ['gallery-dark-care', 'gallery-brand']:
    before = sum(node['role'] == 'image' for node in named('initial', '图片标题', run))
    after = sum(node['role'] == 'image' for node in named('gallery-delete', '图片标题', run))
    assert before == 5 and after == 4, (run, before, after)
    focused('image-focus', '大图', run)
    focused('delete-focus', '删除', run)
    focused('gallery-delete', '上传图片', run)
before = nodes('a11y-open', 'a11y-before')
assert not any(node['name'] == 'Archive' for node in before)
assert (ROOT / 'a11y-before/app.log').read_text() == ''
for step in ['progress-no-preview', 'error-no-preview']:
    assert not any(node['role'] == 'dialog' for node in nodes(step, 'a11y-after'))
a11y_log = (ROOT / 'a11y-after/app.log').read_text()
assert 'preview ' not in a11y_log
assert a11y_log.splitlines().count('swipe requested true; accepted true') == 1
assert a11y_log.splitlines().count('swipe requested false; accepted true') == 1
assert named('a11y-open', 'Archive', 'a11y-after')
assert not named('a11y-close', 'Archive', 'a11y-after')
for step, text in [('a11y-open', '已展开'), ('a11y-close', '已收起')]:
    target = next(node for node in named(step, 'Controlled row', 'a11y-after')
                  if 'focusable' in node['states'])
    assert text in target['description'], (step, target)
    focused(step, 'Controlled row', 'a11y-after')
print('PASS: AT-SPI default Click now toggles controlled state and focuses the row;')
print('      progress/error pointer activation does not preview.')
print('PASS: add/capacity/progress/error/done/empty, preview/dismiss/delete callbacks,')
print('      controlled swipe acceptance/rejection, logical Tab order, focus restoration and drag.')
print('PASS: actual uploader gallery dark+care and custom-brand thumbnail/keyboard/delete samples.')
print('Tree/input observations only; no screen-reader or full parity acceptance.')
