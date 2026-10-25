"""Download, verify, extract and serve the pinned upstream WeUI example.

Usage:
  python3 -I scripts/weui_upstream.py fetch [--dir DIR]
  python3 -I scripts/weui_upstream.py serve [--dir DIR] [--port 7193]

fetch downloads three pinned resources, checks their SHA-256, and extracts the two npm tarballs
into DIR (default: $CARGO_TARGET_DIR/weui-upstream, else target/weui-upstream):

  weui@2.6.26     the CSS package and its dist/example pages
  weui.js@1.2.17  the script the example shell loads from res.wx.qq.com
  jweixin-1.0.0   the WeChat JS SDK the example shell also loads from res.wx.qq.com

DIR/weui is the verified, unmodified package. DIR/site is a copy whose dist/example/index.html
loads the two scripts from DIR/site/vendor instead of res.wx.qq.com; nothing else differs.
serve serves DIR/site on 127.0.0.1 so that http://127.0.0.1:PORT/dist/example/index.html is the
page scripts/capture.py compares against. Only the Python standard library is used.
"""

import argparse
import functools
import hashlib
import http.server
import os
import pathlib
import shutil
import sys
import tarfile
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parent.parent
DEFAULT_DIR = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')) / 'weui-upstream'

WEUI = ('https://registry.npmjs.org/weui/-/weui-2.6.26.tgz',
        'f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435')
WEUI_JS = ('https://registry.npmjs.org/weui.js/-/weui.js-1.2.17.tgz',
           '62aa1d4538bb81ac5d77aeee2b3cdbe03bfca1d28e619ed0641caa74e791182a')
JWEIXIN = ('https://res.wx.qq.com/open/js/jweixin-1.0.0.js',
           '9e1a67f12789a5fde0ade6157154dc9bdfd17a5399b654f73740e0edaa95d94e')

SCRIPT_URLS = {
    'https://res.wx.qq.com/t/wx_fed/weui.js/res/1.2.17/weui.min.js': 'vendor/weui.min.js',
    'https://res.wx.qq.com/open/js/jweixin-1.0.0.js': 'vendor/jweixin-1.0.0.js',
}


def download(url, sha256, dest):
    if dest.exists() and hashlib.sha256(dest.read_bytes()).hexdigest() == sha256:
        return dest
    print(f'downloading {url}')
    with urllib.request.urlopen(url, timeout=60) as response:
        data = response.read()
    digest = hashlib.sha256(data).hexdigest()
    if digest != sha256:
        sys.exit(f'SHA-256 mismatch for {url}\n  expected {sha256}\n  received {digest}')
    dest.write_bytes(data)
    return dest


def safe_extract(archive, dest):
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    root = dest.resolve()
    with tarfile.open(archive, 'r:gz') as tar:
        for member in tar.getmembers():
            target = (dest / member.name).resolve()
            if not (member.isfile() or member.isdir()):
                sys.exit(f'refusing non-regular archive member {member.name!r}')
            if target != root and root not in target.parents:
                sys.exit(f'refusing archive member outside {dest}: {member.name!r}')
        if hasattr(tarfile, 'data_filter'):
            tar.extractall(dest, filter='data')
        else:
            tar.extractall(dest)


def fetch(directory):
    downloads = directory / 'downloads'
    downloads.mkdir(parents=True, exist_ok=True)
    weui = download(*WEUI, downloads / 'weui-2.6.26.tgz')
    weui_js = download(*WEUI_JS, downloads / 'weui.js-1.2.17.tgz')
    jweixin = download(*JWEIXIN, downloads / 'jweixin-1.0.0.js')
    safe_extract(weui, directory / 'weui')
    safe_extract(weui_js, directory / 'weui.js')

    site = directory / 'site'
    if site.exists():
        shutil.rmtree(site)
    shutil.copytree(directory / 'weui/package', site)
    (site / 'vendor').mkdir()
    shutil.copyfile(directory / 'weui.js/package/dist/weui.min.js', site / 'vendor/weui.min.js')
    shutil.copyfile(jweixin, site / 'vendor/jweixin-1.0.0.js')
    index = site / 'dist/example/index.html'
    html = index.read_text(encoding='utf-8')
    for url, local in SCRIPT_URLS.items():
        if html.count(f'src="{url}"') != 1:
            sys.exit(f'expected exactly one script tag for {url} in {index}')
        html = html.replace(f'src="{url}"', f'src="../../{local}"')
    index.write_text(html, encoding='utf-8')
    print(f'verified and extracted into {directory}')
    print(f'serve with: python3 -I scripts/weui_upstream.py serve --dir {directory}')


def serve(directory, port):
    site = directory / 'site'
    if not (site / 'dist/example/index.html').exists():
        sys.exit(f'{site} is missing; run the fetch command first')
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(site))
    with http.server.ThreadingHTTPServer(('127.0.0.1', port), handler) as server:
        print(f'serving http://127.0.0.1:{port}/dist/example/index.html (Ctrl-C to stop)')
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest='command', required=True)
    for name in ('fetch', 'serve'):
        p = sub.add_parser(name)
        p.add_argument('--dir', type=pathlib.Path, default=DEFAULT_DIR)
        if name == 'serve':
            p.add_argument('--port', type=int, default=7193)
    args = parser.parse_args()
    if args.command == 'fetch':
        fetch(args.dir)
    else:
        serve(args.dir, args.port)


if __name__ == '__main__':
    main()
