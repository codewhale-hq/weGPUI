"""Real fcitx5 Pinyin via XIM; AT-SPI observes values, not screen-reader speech."""
from pathlib import Path
import json
import os
import subprocess
import time
import pyatspi

out = Path(os.environ['OUT'])
binary = '/workspace/.wegpui-cloud/ime-gallery-49268de7'
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
    item = {'label': label, 'fields': [data for _, data in fields()],
            'input_method': run('fcitx5-remote', '-n').strip(),
            'activation': run('fcitx5-remote').strip()}
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
    app = subprocess.Popen([binary, '--page', route, '--size', '780x1800', '--reduce-motion'], stdout=log, stderr=subprocess.STDOUT)
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


try:
    if os.environ.get('ONLY_TEXTAREA') != '1':
        start('form_input_status', '01-status-initial')
        click('卡号')
        ime(False)
        type_ascii('ABCD')
        key('Home')
        key('Right')
        ime(True)
        type_ascii('nihao')
        snapshot('02-card-preedit')
        key('space')
        snapshot('03-card-commit')
        check('卡号', 'A你好BCD', 'card commit preserves suffix')
        type_ascii('shijie')
        snapshot('04-card-cancel-preedit')
        key('Escape')
        snapshot('05-card-canceled')
        check('卡号', 'A你好BCD', 'cancel preserves committed value')
        ime(False)
        key('ctrl+z')
        snapshot('06-card-undo')
        check('卡号', 'ABCD', 'undo composition is one transaction')
        key('ctrl+shift+z')
        snapshot('07-card-redo')
        check('卡号', 'A你好BCD', 'redo composition')

        key('Home')
        key('Right')
        key('shift+Right')
        key('shift+Right')
        ime(True)
        type_ascii('shijie')
        snapshot('07b-selected-preedit')
        key('Escape')
        snapshot('07c-selected-canceled')
        check('卡号', 'A你好BCD', 'selection composition cancellation preserves original')
        assert list(find('卡号').queryText().getSelection(0)) == [1, 3]
        ime(False)

        key('ctrl+a')
        type_ascii('ABCDEFGHIJKLMNO')
        key('Home')
        key('Right')
        ime(True)
        type_ascii('nihao')
        snapshot('08-maxlength-preedit')
        key('space')
        snapshot('09-maxlength-commit')
        check('卡号', 'A你BCDEFGHIJKLMNO', 'maxlength16 limits insertion and preserves suffix')
        ime(False)
        key('ctrl+z')
        snapshot('10-maxlength-undo')
        check('卡号', 'ABCDEFGHIJKLMNO', 'maxlength composition undo')

        click('EMail')
        ime(True)
        type_ascii('nihao')
        snapshot('11-readonly-preedit')
        key('space')
        snapshot('12-readonly-after-commit')
        check('EMail', '1234567', 'readonly unchanged')
        key('Escape')
        ime(False)

        click('微信号')
        ime(True)
        type_ascii('nihao')
        key('space')
        snapshot('12b-disabled-after-commit')
        check('微信号', 'WeUI', 'disabled unchanged')
        check('EMail', '1234567', 'readonly remains unchanged after disabled click')
        key('Escape')
        ime(False)

        report['status_result'] = 'PASS'

    start('form_textarea', '13-textarea-initial')
    click('问题描述')
    ime(False)
    type_ascii('before')
    key('Return')
    type_ascii('after')
    key('Home')
    ime(True)
    type_ascii('nihao')
    snapshot('14-textarea-preedit')
    key('space')
    snapshot('15-textarea-commit')
    check('问题描述', 'before\n你好after', 'textarea composition preserves suffix')
    type_ascii('shijie')
    snapshot('16-textarea-cancel-preedit')
    key('Escape')
    snapshot('17-textarea-canceled')
    check('问题描述', 'before\n你好after', 'textarea cancellation')
    ime(False)
    key('ctrl+z')
    snapshot('18-textarea-undo')
    check('问题描述', 'before\nafter', 'textarea undo one composition')
    key('ctrl+shift+z')
    snapshot('19-textarea-redo')
    check('问题描述', 'before\n你好after', 'textarea redo')
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
