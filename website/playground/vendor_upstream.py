#!/usr/bin/env python3
"""Vendor the verified pinned browser example without its SDK/analytics shell."""
import argparse
import hashlib
from html import escape
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import shutil
import tarfile

HERE = Path(__file__).resolve().parent
VOID = {'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta', 'param', 'source', 'track', 'wbr'}


def local_images(value):
    return re.sub(r'(?<![\w/])(?:\./)?images/', 'vendor/images/', value)


class Template(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=False)
        self.output = []
        self.script = False
        self.skip = 0
        self.changes = dict(scriptsRemoved=0, inlineHandlersRemoved=0, externalReferencesRemoved=0,
                            registrationFootersRemoved=0)

    def handle_starttag(self, tag, attrs):
        if self.script:
            return
        if self.skip:
            self.skip += tag not in VOID
            return
        values = dict(attrs)
        if tag == 'script':
            self.script = True
            self.changes['scriptsRemoved'] += 1
            return
        if 'page_copyright' in (values.get('class') or '').split():
            self.skip = 1
            self.changes['registrationFootersRemoved'] += 1
            return
        cleaned = []
        for name, value in attrs:
            if name.lower().startswith('on'):
                self.changes['inlineHandlersRemoved'] += 1
                continue
            if name in ('target', 'action', 'formaction'):
                continue
            if name == 'href' and value:
                if re.match(r'(?i)(?:javascript:|(?:https?:)?//|mailto:|tel:)', value):
                    cleaned.append(('data-original-href', value))
                    if not value.lower().startswith('javascript:'):
                        self.changes['externalReferencesRemoved'] += 1
                    value = '#'
            if name in ('src', 'srcset') and value and re.match(r'(?i)(?:https?:)?//', value):
                cleaned.append(('data-original-src', value))
                self.changes['externalReferencesRemoved'] += 1
                continue
            if name in ('src', 'srcset', 'style') and value:
                value = local_images(value)
            cleaned.append((name, value))
        attributes = ''.join(' ' + name + ('' if value is None else '="' + escape(value, quote=True) + '"')
                             for name, value in cleaned)
        self.output.append('<' + tag + attributes + '>')

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in VOID:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        if self.script:
            if tag == 'script':
                self.script = False
            return
        if self.skip:
            if tag not in VOID:
                self.skip -= 1
            return
        if tag not in VOID:
            self.output.append('</' + tag + '>')

    def handle_data(self, data):
        if not self.script and not self.skip:
            self.output.append(data)

    def handle_entityref(self, name):
        self.handle_data('&' + name + ';')

    def handle_charref(self, name):
        self.handle_data('&#' + name + ';')

    def handle_comment(self, data):
        if not self.script and not self.skip:
            self.output.append('<!--' + data + '-->')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def package_archive(package, filename, sha256):
    archive = package.parents[1] / 'downloads' / filename
    if digest(archive) != sha256:
        raise ValueError('Pinned tarball digest differs: ' + str(archive))
    with tarfile.open(archive) as tar:
        return {member.name: tar.extractfile(member).read() for member in tar.getmembers() if member.isfile()}


def vendor(weui, weui_js, zepto_license):
    assert json.loads((weui / 'package.json').read_text())['version'] == '2.6.26'
    assert json.loads((weui_js / 'package.json').read_text())['version'] == '1.2.17'
    source = weui / 'src/example'
    archives = {
        weui: package_archive(weui, 'weui-2.6.26.tgz', 'f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435'),
        weui_js: package_archive(weui_js, 'weui.js-1.2.17.tgz', '62aa1d4538bb81ac5d77aeee2b3cdbe03bfca1d28e619ed0641caa74e791182a'),
    }

    def verify(path):
        for package, members in archives.items():
            if path.is_relative_to(package):
                assert path.read_bytes() == members['package/' + str(path.relative_to(package))], path
                return

    verify(source / 'index.html')
    imports = re.findall(r'<link rel="import" href="\./([^"]+)"', (source / 'index.html').read_text())
    assert len(imports) == 60 and len(set(imports)) == 60
    output = HERE / 'vendor'
    output.mkdir(exist_ok=True)
    files = {
        'weui.css': weui / 'dist/style/weui.css',
        'example.css': weui / 'dist/example/example.css',
        'zepto.min.js': source / 'zepto.min.js',
        'weui.min.js': weui_js / 'dist/weui.min.js',
        'LICENSE-WEUI.txt': weui / 'LICENSE.txt',
        'LICENSE-WEUI-JS.txt': weui_js / 'LICENSE.txt',
        'LICENSE-ZEPTO.txt': zepto_license,
    }
    provenance = dict(weuiVersion='2.6.26', weuiJsVersion='1.2.17', zeptoVersion='1.2.0',
                      packages={
                          'weui': dict(url='https://registry.npmjs.org/weui/-/weui-2.6.26.tgz', sha256='f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435'),
                          'weui.js': dict(url='https://registry.npmjs.org/weui.js/-/weui.js-1.2.17.tgz', sha256='62aa1d4538bb81ac5d77aeee2b3cdbe03bfca1d28e619ed0641caa74e791182a')},
                      zeptoLicenseUrl='https://raw.githubusercontent.com/madrobby/zepto/v1.2.0/MIT-LICENSE',
                      omitted=['Official index shell and contact/registration footer', 'example.js', 'wah.js', 'WeChat JS SDK', 'All inline example scripts and on* handlers', 'External demo navigation/transport'],
                      files={}, routes=[])
    for name, path in files.items():
        verify(path)
        shutil.copyfile(path, output / name)
        provenance['files'][name] = dict(sha256=digest(path), source=str(path.relative_to(weui)) if path.is_relative_to(weui) else ('weui.js/' + str(path.relative_to(weui_js)) if path.is_relative_to(weui_js) else 'Zepto v1.2.0 MIT-LICENSE'))
    shutil.copytree(source / 'images', output / 'images', dirs_exist_ok=True)
    for path in sorted((output / 'images').rglob('*')):
        if path.is_file():
            verify(source / str(path.relative_to(output)))
            provenance['files'][str(path.relative_to(output))] = dict(sha256=digest(path), source='src/example/' + str(path.relative_to(output)))
    routes = {}
    states = {}
    for path in sorted((HERE.parents[1] / 'scripts/parity').glob('*.json')):
        for key in json.loads(path.read_text()):
            route, state = key.split(':', 1)
            states.setdefault(route, []).append(state)
    for relative in imports:
        path = source / relative
        verify(path)
        route = path.stem
        template = Template()
        original = path.read_text()
        template.feed(original)
        template.close()
        inert = HERE / 'source' / (route + '.html.txt')
        inert.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, inert)
        routes[route] = dict(html=''.join(template.output), source='src/example/' + relative,
                             sha256=digest(path), changes=template.changes,
                             states=['default'] + states.get(route, []))
        provenance['routes'].append(dict(id=route, source='src/example/' + relative, sha256=digest(path), changes=template.changes))
    (output / 'routes.js').write_text('window.WEGPUI_ROUTES = ' + json.dumps(routes, ensure_ascii=False) + ';\n')
    provenance['files']['routes.js'] = dict(sha256=digest(output / 'routes.js'), source='Sanitized route markup generated by vendor_upstream.py')
    (output / 'PROVENANCE.json').write_text(json.dumps(provenance, ensure_ascii=False, indent=2) + '\n')
    print(f'Vendored {len(routes)} routes and {len(provenance["files"])} local assets/license files')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--weui', type=Path, required=True)
    parser.add_argument('--weui-js', type=Path, required=True)
    parser.add_argument('--zepto-license', type=Path, required=True)
    args = parser.parse_args()
    vendor(args.weui, args.weui_js, args.zepto_license)
