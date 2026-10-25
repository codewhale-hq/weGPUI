"""Drive real XTEST keys and AT-SPI Text requests; no screen-reader acceptance claim."""
import json
import ctypes
import os
import re
from pathlib import Path
import subprocess
import time
import pyatspi

out = Path(os.environ['OUT'])
window = os.environ['PROBE_WINDOW']
report = {'scale': os.environ.get('GPUI_X11_SCALE_FACTOR', '1'), 'actions': []}

def walk(node):
    yield node
    for index in range(node.childCount):
        child = node.getChildAtIndex(index)
        if child is not None:
            yield from walk(child)

def field(name):
    desktop = pyatspi.Registry.getDesktop(0)
    for node in walk(desktop):
        if node.name == name and node.getRoleName() in ('text', 'entry', 'password text'):
            return node
    raise AssertionError(f'field missing: {name}')

def state(name):
    node = field(name)
    text = node.queryText()
    selection = [list(text.getSelection(index)) for index in range(text.getNSelections())]
    return {'value': text.getText(0, -1), 'caret': text.caretOffset, 'selection': selection,
            'description': node.description,
            'states': [pyatspi.stateToString(s) for s in node.getState().getStates()],
            'interfaces': list(node.get_interfaces())}

def record(label, name):
    item = {'label': label, 'field': name, **state(name)}
    report['actions'].append(item)
    (out / 'runtime.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
    subprocess.run(['import', '-window', window, str(out / f'{label}.png')], check=True)
    print(json.dumps(item, ensure_ascii=False), flush=True)
    return item

def key(*keys):
    for token in keys:
        subprocess.run(['xdotool', 'key', '--clearmodifiers', token], check=True)
        time.sleep(.35)
    time.sleep(.35)

def origin_check(name, offset, label):
    text = field(name).queryText()
    desktop = list(text.getCharacterExtents(offset, pyatspi.DESKTOP_COORDS))
    relative = list(text.getCharacterExtents(offset, pyatspi.WINDOW_COORDS))
    info = subprocess.run(['xwininfo', '-id', window], capture_output=True, text=True, check=True).stdout
    (out / f'{label}-xwininfo.txt').write_text(info)
    x = int(re.search(r'Absolute upper-left X:\s*(-?\d+)', info).group(1))
    y = int(re.search(r'Absolute upper-left Y:\s*(-?\d+)', info).group(1))
    xlib = ctypes.CDLL('libX11.so.6')
    xlib.XOpenDisplay.argtypes = [ctypes.c_char_p]
    xlib.XOpenDisplay.restype = ctypes.c_void_p
    xlib.XDefaultRootWindow.argtypes = [ctypes.c_void_p]
    xlib.XDefaultRootWindow.restype = ctypes.c_ulong
    xlib.XTranslateCoordinates.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_ulong,
        ctypes.c_int, ctypes.c_int, ctypes.POINTER(ctypes.c_int), ctypes.POINTER(ctypes.c_int),
        ctypes.POINTER(ctypes.c_ulong)]
    xlib.XTranslateCoordinates.restype = ctypes.c_int
    xlib.XCloseDisplay.argtypes = [ctypes.c_void_p]
    display = xlib.XOpenDisplay(None)
    assert display
    translated_x, translated_y, child = ctypes.c_int(), ctypes.c_int(), ctypes.c_ulong()
    try:
        translated = xlib.XTranslateCoordinates(display, int(window), xlib.XDefaultRootWindow(display),
            0, 0, ctypes.byref(translated_x), ctypes.byref(translated_y), ctypes.byref(child))
        assert translated
    finally:
        xlib.XCloseDisplay(display)
    assert [translated_x.value, translated_y.value] == [x, y]
    result = {'desktop': desktop, 'window': relative, 'x_client_origin': [x, y],
              'x_translate_origin': [translated_x.value, translated_y.value]}
    report[label] = result
    if os.environ.get('EXPECT_SCREEN_ORIGIN') == '1':
        assert desktop[0] == relative[0] + x
        assert desktop[1] == relative[1] + y
    return result

def caret(name, offset):
    assert field(name).queryText().setCaretOffset(offset)
    time.sleep(.7)

words = 'alpha e\u0301 beta gamma'
try:
    time.sleep(1)
    initial = record('01-initial', 'Word editing')
    assert initial['value'] == words
    report['word_boundary_at_zero'] = list(field('Word editing').queryText().getStringAtOffset(0, pyatspi.TEXT_GRANULARITY_WORD))
    assert report['word_boundary_at_zero'][0].strip() == 'alpha'
    key('Home', 'ctrl+Right')
    assert record('02-ctrl-right', 'Word editing')['caret'] == 5
    key('ctrl+Right')
    assert record('03-ctrl-right-unicode', 'Word editing')['caret'] == 8
    key('End', 'ctrl+shift+Left')
    selected = record('04-select-word', 'Word editing')
    start, end = selected['selection'][0]
    assert words[start:end] == 'gamma'
    key('Home', 'ctrl+Delete')
    assert record('05-delete-word-forward', 'Word editing')['value'] == ' e\u0301 beta gamma'
    key('ctrl+z')
    assert record('06-undo-forward-delete', 'Word editing')['value'] == words
    key('End', 'ctrl+BackSpace')
    assert record('07-delete-word-backward', 'Word editing')['value'] == 'alpha e\u0301 beta '
    key('ctrl+z')
    assert record('08-undo-backward-delete', 'Word editing')['value'] == words
    caret('Word editing', 6)
    assert record('09-at-caret', 'Word editing')['caret'] == 6
    key('Delete')
    assert record('10-delete-combining-grapheme', 'Word editing')['value'] == 'alpha  beta gamma'
    key('ctrl+z')
    assert record('11-undo-grapheme', 'Word editing')['value'] == words
    assert field('Word editing').queryText().addSelection(0, 5)
    time.sleep(.7)
    assert record('12-at-select', 'Word editing')['selection'] == [[0, 5]]
    key('x')
    assert record('13-replace-at-selection', 'Word editing')['value'] == 'x e\u0301 beta gamma'
    key('ctrl+z')
    assert record('14-undo-at-replacement', 'Word editing')['value'] == words
    key('End', 'F8')
    assert record('15-composition-commit', 'Word editing')['value'] == words + '界'
    key('ctrl+z')
    assert record('16-undo-composition', 'Word editing')['value'] == words
    caret('Read only', 4)
    assert record('17-readonly-caret', 'Read only')['caret'] == 4
    key('BackSpace')
    assert record('18-readonly-edit-guard', 'Read only')['value'] == 'read only text'
    before = state('Disabled')
    accepted = field('Disabled').queryText().setCaretOffset(3)
    time.sleep(.7)
    after = record('19-disabled-caret-guard', 'Disabled')
    report['disabled_request_returned'] = accepted
    assert after['caret'] == before['caret']
    assert after['value'] == before['value']
    password = record('20-password-safe', 'Password')
    assert password['value'] == '•••'
    caret('Password', 1)
    key('BackSpace')
    assert record('21-password-delete', 'Password')['value'] == '••'
    key('ctrl+z')
    assert record('22-password-undo', 'Password')['value'] == '•••'
    caret('UTF16 limit', 3)
    key('X', 'Y', 'Z')
    limited = record('23-utf16-limit-invalid', 'UTF16 limit')
    assert limited['value'] == 'A😀BXY'
    assert limited['description'] == 'At most six UTF-16 units'
    report['invalid_state_exported'] = any(s in limited['states'] for s in ('invalid entry', 'invalid-entry'))
    if os.environ.get('EXPECT_STATE_EXPORTS') == '1':
        assert report['invalid_state_exported']
    if not report['invalid_state_exported']:
        report.setdefault('unsupported_bridge_features', []).append('AccessKit Invalid is not exported as AT-SPI invalid entry')
    key('F7', 'F6')
    assert record('23b-composition-utf16-limit', 'UTF16 limit')['value'] == 'A😀B😀'
    key('ctrl+z')
    assert record('23c-undo-limited-composition', 'UTF16 limit')['value'] == 'A😀B'
    caret('Wrapped notes', 0)
    note = field('Wrapped notes').queryText()
    line, start, end = note.getStringAtOffset(0, pyatspi.TEXT_GRANULARITY_LINE)
    ext = note.getCharacterExtents(0, pyatspi.DESKTOP_COORDS)
    x, y, width, height = ext
    point_offset = note.getOffsetAtPoint(x + max(1, width // 3), y + height // 2,
                                        pyatspi.DESKTOP_COORDS)
    window_geometry = subprocess.run(['xdotool', 'getwindowgeometry', '--shell', window], capture_output=True, text=True, check=True).stdout
    report['geometry'] = {'first_visual_line': line, 'start': start, 'end': end,
                          'x_window_geometry': window_geometry,
                          'window_character_extents': list(note.getCharacterExtents(0, pyatspi.WINDOW_COORDS)),
                          'character_extents': [x, y, width, height],
                          'offset_at_point': point_offset}
    assert width > 0 and height > 0
    assert 0 <= point_offset <= 1
    hard = state('Wrapped notes')['value'].split('\n')[0]
    assert hard.startswith(line)
    assert len(line) < len(hard)
    record('24-wrapped-geometry', 'Wrapped notes')
    origin_check('Wrapped notes', 0, 'initial_screen_origin')
    key('F5')
    long_value = 'a ' * 300
    caret('Word editing', 300)
    long_text = field('Word editing').queryText()
    report['long_metadata'] = {
        'word': list(long_text.getStringAtOffset(300, pyatspi.TEXT_GRANULARITY_WORD)),
        'line': list(long_text.getStringAtOffset(300, pyatspi.TEXT_GRANULARITY_LINE)),
        'character_extents': list(long_text.getCharacterExtents(300, pyatspi.WINDOW_COORDS)),
    }
    assert report['long_metadata']['word'] == ['a ', 300, 302]
    assert report['long_metadata']['line'] == [long_value, 0, 600]
    assert report['long_metadata']['character_extents'][2] > 0
    record('25-long-word-line', 'Word editing')
    assert field('Word editing').queryText().addSelection(250, 520)
    time.sleep(.7)
    record('26-cross-chunk-selection', 'Word editing')
    key('x')
    assert record('27-cross-chunk-replacement', 'Word editing')['value'] == long_value[:250] + 'x' + long_value[520:]
    inserted = list(field('Word editing').queryText().getCharacterExtents(250, pyatspi.WINDOW_COORDS))
    control = list(field('Word editing').queryComponent().getExtents(pyatspi.WINDOW_COORDS))
    report['inserted_grapheme_extents'] = inserted
    report['word_control_extents'] = control
    assert inserted[0] >= control[0]
    assert inserted[0] + inserted[2] <= control[0] + control[2]
    key('ctrl+z')
    assert record('28-cross-chunk-undo', 'Word editing')['value'] == long_value
    subprocess.run(['xdotool', 'windowmove', window, '100', '80'], check=True)
    time.sleep(1)
    origin_check('Wrapped notes', 0, 'moved_screen_origin')
    record('29-window-move', 'Wrapped notes')
    report['passed'] = True
except Exception as error:
    report['passed'] = False
    report['error'] = repr(error)
    raise
finally:
    (out / 'runtime.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
