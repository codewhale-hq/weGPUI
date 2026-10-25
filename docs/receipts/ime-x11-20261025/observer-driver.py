"""Real fcitx5 Pinyin via XIM; AT-SPI observes values, not screen-reader speech."""
from pathlib import Path
import json
import os
import re
import subprocess
import time
import pyatspi

out = Path(os.environ['OUT'])
binary = os.environ['IME_BINARY']
report = {'actions': [], 'observations': [], 'failures': []}
app = None
window = None


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


def pause():
    time.sleep(.65)


def action(kind, **data):
    item = {'at': time.time(), 'kind': kind, **data}
    report['actions'].append(item)
    with (out / 'actions.jsonl').open('a') as log:
        log.write(json.dumps(item, ensure_ascii=False) + '\n')


def walk(node):
    yield node
    for index in range(node.childCount):
        child = node.getChildAtIndex(index)
        if child is not None:
            yield from walk(child)


def fields():
    found = []
    for node in walk(pyatspi.Registry.getDesktop(0)):
        if node.getRoleName() in ('entry', 'text', 'password text'):
            try:
                text = node.queryText()
                found.append((node, {'name': node.name, 'role': node.getRoleName(),
                    'value': text.getText(0, -1), 'caret': text.caretOffset,
                    'selection': [list(text.getSelection(i)) for i in range(text.getNSelections())],
                    'bounds': list(node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)),
                    'states': [pyatspi.stateToString(s) for s in node.getState().getStates()]}))
            except Exception as error:
                found.append((node, {'name': node.name, 'error': repr(error)}))
    return found


def find(name):
    return next(node for node, data in fields() if data['name'] == name)


def value(name):
    return find(name).queryText().getText(0, -1)


def snapshot(label):
    key('F12')
    item = {'label': label, 'fields': [data for _, data in fields()],
            'input_method': run('fcitx5-remote', '-n').strip(),
            'activation': run('fcitx5-remote').strip(), 'changes': counts(),
            'native_snapshots': [line for line in log_text().splitlines() if line.startswith('snapshot ')][-6:]}
    report['observations'].append(item)
    (out / f'{label}.json').write_text(json.dumps(item, ensure_ascii=False, indent=2) + '\n')
    run('import', '-window', window, str(out / f'{label}-app.png'))
    run('import', '-window', 'root', str(out / f'{label}-desktop.png'))
    (out / f'{label}-windows.txt').write_text(run('xwininfo', '-root', '-tree'))
    save()
    print(json.dumps(item, ensure_ascii=False), flush=True)
    return item


def save():
    (out / 'runtime.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')


def key(token):
    action('key', key=token)
    run('xdotool', 'key', '--clearmodifiers', token)
    pause()


def type_ascii(text):
    action('type', text=text)
    run('xdotool', 'type', '--clearmodifiers', '--delay', '110', '--', text)
    pause()


def ime(active):
    action('ime', active=active)
    run('fcitx5-remote', '-s', 'pinyin')
    run('fcitx5-remote', '-o' if active else '-c')
    pause()


def click(name):
    node = find(name)
    x, y, w, h = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    action('click', field=name, desktop=[x + min(20, w // 2), y + h // 2])
    run('xdotool', 'mousemove', str(x + min(20, w // 2)), str(y + h // 2), 'click', '1')
    pause()


def start(route, label):
    global app, window
    if app is not None:
        app.terminate()
        app.wait(timeout=15)
    action('launch', route=route)
    log = (out / f'{label}-application.log').open('w')
    app = subprocess.Popen([binary], stdout=log, stderr=subprocess.STDOUT)
    log.close()
    for _ in range(100):
        found = subprocess.run(['xdotool', 'search', '--onlyvisible', '--pid', str(app.pid)], capture_output=True, text=True)
        if found.returncode == 0 and found.stdout.strip():
            window = found.stdout.splitlines()[0]
            break
        assert app.poll() is None, 'application exited'
        time.sleep(.1)
    else:
        raise AssertionError('no gallery window')
    run('xdotool', 'windowactivate', '--sync', window)
    time.sleep(2)
    snapshot(label)


def check(name, expected, label):
    actual = value(name)
    assert actual == expected, f'{label}: {actual!r} != {expected!r}'


PRIMARY = 'A你好B e\u0301 👩‍💻 C'
NOTES = 'first e\u0301\nA你好after'


def log_text():
    return (out / '01-observer-initial-application.log').read_text()


def counts():
    result = {name: 0 for name in ('Primary', 'Limited', 'Notes', 'Password', 'Read only', 'Disabled')}
    for name, count in re.findall(r'^event field="([^"]+)" change=(\d+)', log_text(), re.MULTILINE):
        result[name] = int(count)
    return result


def delta(before, name, expected, label):
    actual = counts()[name] - before[name]
    assert actual == expected, f'{label}: Changed delta {actual} != {expected}'


def selected_primary(reversed=False):
    click('Primary')
    key('Home')
    for _ in range(3 if reversed else 1):
        key('Right')
    for _ in range(2):
        key('shift+Left' if reversed else 'shift+Right')
    assert list(find('Primary').queryText().getSelection(0)) == [1, 3]


def native(item, name):
    return next(line for line in item['native_snapshots'] if f'field="{name}"' in line)


def reset():
    ime(False)
    key('F5')
    check('Primary', PRIMARY, 'reset primary')


try:
    start('observer', '01-observer-initial')
    report['case_categories'] = {
        '02..25': 'real fcitx5 Pinyin via XIM and native keyboard',
        '26..34': 'direct public EntityInputHandler calls from the temporary probe, not an actual IME',
    }
    selected_primary()
    before = counts()
    ime(True)
    type_ascii('shijie')
    preedit = snapshot('02-selected-preedit')
    assert 'marked=Some(' in native(preedit, 'Primary')
    delta(before, 'Primary', 0, 'preedit has no Changed')
    key('Escape')
    canceled = snapshot('03-selected-canceled')
    check('Primary', PRIMARY, 'cancel restores selected text')
    assert list(find('Primary').queryText().getSelection(0)) == [1, 3]
    assert 'reversed=false' in native(canceled, 'Primary')
    delta(before, 'Primary', 0, 'cancel has no Changed')

    ime(False)
    selected_primary(reversed=True)
    before = counts()
    ime(True)
    type_ascii('shijie')
    snapshot('04-reversed-preedit')
    key('Escape')
    canceled = snapshot('05-reversed-canceled')
    check('Primary', PRIMARY, 'cancel restores reversed selection text')
    assert list(find('Primary').queryText().getSelection(0)) == [1, 3]
    assert 'reversed=true' in native(canceled, 'Primary')
    delta(before, 'Primary', 0, 'reverse cancel has no Changed')

    type_ascii('shijie')
    snapshot('06-reversed-commit-preedit')
    key('space')
    committed = snapshot('07-reversed-committed')
    check('Primary', PRIMARY.replace('你好', '世界'), 'commit replaces reversed selection')
    assert 'marked=None' in native(committed, 'Primary')
    delta(before, 'Primary', 1, 'real IME commit emits exactly once')
    ime(False)
    key('ctrl+z')
    undone = snapshot('08-real-commit-undo')
    check('Primary', PRIMARY, 'undo real IME')
    assert 'reversed=true' in native(undone, 'Primary')
    delta(before, 'Primary', 2, 'undo emits once')
    key('ctrl+shift+z')
    snapshot('09-real-commit-redo')
    check('Primary', PRIMARY.replace('你好', '世界'), 'redo real IME')
    delta(before, 'Primary', 3, 'redo emits once')

    reset()
    click('Limited')
    key('Home')
    key('Right')
    before = counts()
    ime(True)
    type_ascii('nihao')
    preedit = snapshot('10-limited-preedit')
    assert 'marked=Some(' in native(preedit, 'Limited')
    key('F11')
    same = snapshot('11-same-limit-preserves-preedit')
    assert value('Limited') == next(field['value'] for field in preedit['fields'] if field['name'] == 'Limited')
    assert 'marked=Some(' in native(same, 'Limited')
    delta(before, 'Limited', 0, 'same limit and preedit emit no Changed')
    key('space')
    snapshot('12-limited-commit')
    check('Limited', 'A你😀BC', 'UTF16 limit preserves emoji and suffix')
    delta(before, 'Limited', 1, 'limited commit emits once')
    ime(False)
    key('ctrl+z')
    snapshot('13-limited-undo')
    check('Limited', 'A😀BC', 'limited undo')
    delta(before, 'Limited', 2, 'limited undo emits once')

    click('Notes')
    key('ctrl+End')
    key('Home')
    key('Right')
    key('shift+Right')
    key('shift+Right')
    before = counts()
    ime(True)
    type_ascii('shijie')
    snapshot('14-notes-selected-preedit')
    key('Escape')
    snapshot('15-notes-selected-canceled')
    check('Notes', NOTES, 'textarea selected-text cancellation')
    delta(before, 'Notes', 0, 'textarea cancel emits no Changed')
    type_ascii('shijie')
    key('space')
    snapshot('16-notes-commit')
    check('Notes', NOTES.replace('你好', '世界'), 'textarea commit preserves newline/suffix')
    delta(before, 'Notes', 1, 'textarea commit emits once')
    ime(False)
    key('ctrl+z')
    snapshot('17-notes-undo')
    check('Notes', NOTES, 'textarea undo')
    delta(before, 'Notes', 2, 'textarea undo emits once')
    key('ctrl+shift+z')
    snapshot('18-notes-redo')
    delta(before, 'Notes', 3, 'textarea redo emits once')

    reset()
    selected_primary()
    before = counts()
    ime(True)
    type_ascii('shijie')
    snapshot('19-before-readonly-change')
    key('F1')
    changed = snapshot('20-readonly-canceled-preedit')
    check('Primary', PRIMARY, 'readonly change restores committed text')
    assert 'readonly=true' in native(changed, 'Primary')
    assert 'marked=None' in native(changed, 'Primary')
    key('Escape')
    ime(False)
    key('F1')
    delta(before, 'Primary', 0, 'readonly state changes do not emit Changed')

    selected_primary(reversed=True)
    before = counts()
    ime(True)
    type_ascii('shijie')
    snapshot('21-before-disabled-change')
    key('F2')
    changed = snapshot('22-disabled-canceled-preedit')
    check('Primary', PRIMARY, 'disabled change restores committed text')
    assert 'disabled=true' in native(changed, 'Primary')
    key('Escape')
    ime(False)
    click('Limited')
    key('F2')
    reenabled = snapshot('22b-reenabled-from-enabled-focus')
    assert 'disabled=false' in native(reenabled, 'Primary')
    delta(before, 'Primary', 0, 'disabled state changes do not emit Changed')

    selected_primary()
    before = counts()
    ime(True)
    type_ascii('shijie')
    snapshot('23-before-limit-change')
    key('F3')
    snapshot('24-limit-change-canceled-preedit')
    check('Primary', 'A你好B e\u0301', 'new limit applies to committed base')
    key('Escape')
    snapshot('25-limit-change-no-stale-restore')
    check('Primary', 'A你好B e\u0301', 'empty callback cannot restore pre-limit text')
    delta(before, 'Primary', 1, 'only committed truncation emits Changed')
    ime(False)

    for key_name, label in [('F6', '26-empty-before-commit'), ('F7', '28-empty-after-commit')]:
        reset()
        selected_primary(reversed=True)
        before = counts()
        key(key_name)
        snapshot(label)
        check('Primary', PRIMARY.replace('你好', '世界'), label)
        delta(before, 'Primary', 1, label + ' emits one Changed')
        key('ctrl+z')
        undone = snapshot(label + '-undo')
        check('Primary', PRIMARY, label + ' undo')
        assert 'reversed=true' in native(undone, 'Primary')
        delta(before, 'Primary', 2, label + ' Undo emits one Changed')

    reset()
    selected_primary(reversed=True)
    before = counts()
    key('F8')
    canceled = snapshot('30-repeated-empty-preedit')
    check('Primary', PRIMARY, 'repeated empty callbacks preserve selected original')
    assert 'reversed=true' in native(canceled, 'Primary')
    delta(before, 'Primary', 0, 'repeated cancellation emits no Changed')
    key('F9')
    snapshot('31-explicit-empty-commit-deletes')
    check('Primary', PRIMARY.replace('你好', ''), 'explicit empty commit still deletes selection')
    delta(before, 'Primary', 1, 'explicit empty commit emits one Changed')
    key('ctrl+z')
    snapshot('32-empty-commit-undo')
    check('Primary', PRIMARY, 'empty commit undo')

    reset()
    selected_primary(reversed=True)
    before = counts()
    key('F10')
    snapshot('33-protocol-limit-cancel')
    check('Primary', 'A你好', 'protocol cancellation then new max3')
    key('F12')
    snapshot('34-protocol-limit-stays-truncated')
    check('Primary', 'A你好', 'no stale snapshot restores truncated text')
    delta(before, 'Primary', 1, 'one programmatic committed truncation event')
    report['result'] = 'PASS'
except Exception as error:
    report['result'] = 'FAIL'
    report['failures'].append(repr(error))
    raise
finally:
    save()
    if app is not None:
        app.terminate()
        app.wait(timeout=15)

