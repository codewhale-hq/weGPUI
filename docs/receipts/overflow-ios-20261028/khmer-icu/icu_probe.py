#!/usr/bin/env python3
import argparse
import ctypes as c
import hashlib
import json
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--data', choices=('chromium', 'stock'), required=True)
parser.add_argument('--icu-lib-dir', default='/opt/homebrew/opt/icu4c@78/lib')
parser.add_argument('--container', default='wegpui-rtl-qualification')
args = parser.parse_args()
uc = c.CDLL(args.icu_lib_dir + '/libicuuc.78.dylib')
lib = c.CDLL(args.icu_lib_dir + '/libicui18n.78.dylib')
version = (c.c_uint8 * 4)()
get_version = uc.u_getVersion_78
get_version.argtypes = [c.POINTER(c.c_uint8)]
get_version(version)
info = {'engine_version': '.'.join(map(str, version)), 'data_mode': args.data}
if args.data == 'chromium':
    blob = subprocess.check_output(['docker', 'exec', args.container, 'cat', '/usr/lib/chromium/icudtl.dat'])
    digest = hashlib.sha256(blob).hexdigest()
    expected = '02a01b3188b6a41a756f7322fdb5e6b5ca6573b327b01dfc8ed662d571a6f769'
    if digest != expected:
        raise SystemExit(f'Unexpected Chromium data hash: {digest}')
    buf = c.create_string_buffer(blob)
    status = c.c_int32()
    setter = uc.udata_setCommonData_78
    setter.argtypes = [c.c_void_p, c.POINTER(c.c_int32)]
    setter(c.cast(buf, c.c_void_p), c.byref(status))
    if status.value != 0:
        raise SystemExit(f'udata_setCommonData failed: {status.value}')
    info.update(data_sha256=digest, data_bytes=len(blob), set_common_data_status=status.value)
text = 'ភាសាខ្មែរជាភាសាផ្លូវការរបស់ប្រទេសកម្ពុជា ភាសាខ្មែរជាភាសាផ្លូវការរបស់ប្រទេសកម្ពុជា'
encoded = text.encode('utf-16-le')
chars = (c.c_uint16 * (len(encoded) // 2)).from_buffer_copy(encoded)
open_iterator = lib.ubrk_open_78
open_iterator.argtypes = [c.c_int, c.c_char_p, c.POINTER(c.c_uint16), c.c_int32, c.POINTER(c.c_int32)]
open_iterator.restype = c.c_void_p
next_break = lib.ubrk_next_78
next_break.argtypes = [c.c_void_p]
next_break.restype = c.c_int32
close = lib.ubrk_close_78
close.argtypes = [c.c_void_p]
info['locale_results'] = {}
for locale in ('zh_CN', 'km', 'root'):
    status = c.c_int32()
    iterator = open_iterator(2, locale.encode(), chars, len(chars), c.byref(status))
    if not iterator or status.value > 0:
        raise SystemExit(f'ubrk_open {locale} failed: {status.value}')
    offsets = []
    index = next_break(iterator)
    while index != -1:
        offsets.append(len(encoded[:index * 2].decode('utf-16-le').encode('utf-8')))
        index = next_break(iterator)
    close(iterator)
    info['locale_results'][locale] = {'status': status.value, 'utf8_breaks': offsets}
info['text'] = text
print(json.dumps(info, ensure_ascii=False, indent=2))
