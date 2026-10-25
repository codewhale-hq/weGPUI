"""Real Fcitx Pinyin composition across temporary native keyboard grabs."""
import ctypes as C
from pathlib import Path

# Reuse the already reviewed real-XIM observation/keyboard helpers, not its main matrix.
base = Path('/workspace/weGPUI/docs/receipts/x11-grab-focus-20261025/xim-base-driver.py.txt')
source = base.read_text()
helpers = source.split("\ntry:\n    start('observer', '01-observer-initial')", 1)[0]
assert len(helpers) < len(source)
exec(compile(helpers, str(base), 'exec'))

lib = C.CDLL('libX11.so.6')
display_t, window_t = C.c_void_p, C.c_ulong
lib.XOpenDisplay.argtypes = [C.c_char_p]
lib.XOpenDisplay.restype = display_t
lib.XDefaultRootWindow.argtypes = [display_t]
lib.XDefaultRootWindow.restype = window_t
lib.XCreateSimpleWindow.argtypes = [display_t, window_t, C.c_int, C.c_int,
                                   C.c_uint, C.c_uint, C.c_uint, C.c_ulong, C.c_ulong]
lib.XCreateSimpleWindow.restype = window_t
lib.XStoreName.argtypes = [display_t, window_t, C.c_char_p]
lib.XMapWindow.argtypes = [display_t, window_t]
lib.XGetInputFocus.argtypes = [display_t, C.POINTER(window_t), C.POINTER(C.c_int)]
lib.XGrabKeyboard.argtypes = [display_t, window_t, C.c_int, C.c_int, C.c_int, C.c_ulong]
lib.XGrabKeyboard.restype = C.c_int
lib.XUngrabKeyboard.argtypes = [display_t, C.c_ulong]
lib.XSync.argtypes = [display_t, C.c_int]
lib.XDestroyWindow.argtypes = [display_t, window_t]
lib.XCloseDisplay.argtypes = [display_t]
display = lib.XOpenDisplay(None)
assert display
control = None


def grab_cycle(label):
    expected = value('Primary')
    before = counts()
    status = lib.XGrabKeyboard(display, control, 0, 1, 1, 0)
    lib.XSync(display, 0)
    assert status == 0, status
    time.sleep(.5)
    focus, revert = window_t(), C.c_int()
    lib.XGetInputFocus(display, C.byref(focus), C.byref(revert))
    action('actual-keyboard-grab', label=label, native_input_focus=focus.value,
           app_window=int(window), control_window=control, primary=value('Primary'))
    assert focus.value == int(window)
    check('Primary', expected, 'temporary grab preserves preedit')
    delta(before, 'Primary', 0, 'temporary grab emits no Changed')
    lib.XUngrabKeyboard(display, 0)
    lib.XSync(display, 0)
    time.sleep(.5)
    after = snapshot(label)
    check('Primary', expected, 'ungrab preserves preedit')
    assert 'marked=Some(' in native(after, 'Primary')
    delta(before, 'Primary', 0, 'temporary ungrab emits no Changed')


try:
    start('observer', '01-observer-initial')
    report['case_categories'] = {'composition': 'actual fcitx5 Pinyin via XIM',
                                'grab': 'actual libX11 keyboard grab with unchanged native input focus',
                                'snapshots': 'public input trait reads only; no synthetic preedit/commit calls'}
    control = lib.XCreateSimpleWindow(display, lib.XDefaultRootWindow(display),
                                     1200, 100, 240, 100, 0, 0, 0xDDDDDD)
    lib.XStoreName(display, control, b'Isolated XIM grab control')
    lib.XMapWindow(display, control)
    lib.XSync(display, 0)
    time.sleep(1)
    run('xdotool', 'windowactivate', '--sync', window)
    selected_primary()
    before = counts()
    ime(True)
    type_ascii('shijie')
    preedit = snapshot('02-real-preedit-before-grab')
    assert 'marked=Some(' in native(preedit, 'Primary')
    delta(before, 'Primary', 0, 'preedit has no Changed')
    grab_cycle('03-preedit-after-grab-ungrab')
    key('space')
    committed = snapshot('04-real-commit')
    check('Primary', PRIMARY.replace('你好', '世界'), 'real commit after temporary grab')
    assert 'marked=None' in native(committed, 'Primary')
    delta(before, 'Primary', 1, 'one commit callback')
    ime(False)
    key('ctrl+z')
    snapshot('05-real-commit-undo')
    check('Primary', PRIMARY, 'undo real IME commit')
    delta(before, 'Primary', 2, 'undo emits once')

    reset()
    selected_primary(reversed=True)
    before = counts()
    ime(True)
    type_ascii('shijie')
    preedit = snapshot('06-reversed-real-preedit')
    assert 'marked=Some(' in native(preedit, 'Primary')
    grab_cycle('07-reversed-preedit-after-grab-ungrab')
    key('Escape')
    canceled = snapshot('08-real-cancel')
    check('Primary', PRIMARY, 'real Escape cancellation restores original selected text')
    assert list(find('Primary').queryText().getSelection(0)) == [1, 3]
    assert 'marked=None' in native(canceled, 'Primary')
    assert 'reversed=true' in native(canceled, 'Primary')
    delta(before, 'Primary', 0, 'cancellation emits no Changed')
    report['result'] = 'PASS'
except Exception as error:
    report['result'] = 'FAIL'
    report['failures'].append(repr(error))
    raise
finally:
    lib.XUngrabKeyboard(display, 0)
    if control:
        lib.XDestroyWindow(display, control)
    lib.XSync(display, 0)
    lib.XCloseDisplay(display)
    save()
    if app is not None:
        app.terminate()
        app.wait(timeout=15)
