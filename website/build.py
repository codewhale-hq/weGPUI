#!/usr/bin/env python3
"""Build the static browser gallery and separate recorded native showcase."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
_inventory_spec = importlib.util.spec_from_file_location('website_inventory', ROOT / 'website/inventory.py')
_inventory = importlib.util.module_from_spec(_inventory_spec)
_inventory_spec.loader.exec_module(_inventory)
browser_route_map = _inventory.browser_route_map
FAMILY_INFO = {
    'button': ('Buttons', 'Actions', 'Primary, secondary, warning and consent actions, in regular and bottom-fixed layouts.'),
    'actionsheet': ('Action sheets', 'Overlays', 'Contextual actions presented in an iOS- or Android-style sheet.'),
    'dialog': ('Dialogs', 'Overlays', 'Modal decisions with named controls, keyboard dismissal and managed presence.'),
    'half-screen-dialog': ('Half-screen dialogs', 'Overlays', 'Bottom sheets for richer tasks, with several header and content arrangements.'),
    'form': ('Forms', 'Inputs', 'Form layouts, validation feedback, choices, text areas and verification-code examples.'),
    'input': ('Inputs', 'Inputs', 'Native text editing, selection, clipboard, undo/redo and IME integration.'),
    'picker': ('Pickers', 'Inputs', 'Single, multiple, cascading and date selections in a wheel-style sheet.'),
    'searchbar': ('Search', 'Inputs', 'Search fields with several visual treatments and populated example states.'),
    'slider': ('Sliders', 'Inputs', 'Controlled values with pointer dragging, keyboard steps and a value readout.'),
    'uploader': ('Uploaders', 'Media', 'Native file selection and preview. The application owns upload transport.'),
    'gallery': ('Image gallery', 'Media', 'Full-window image viewing with close and delete controls.'),
    'navbar': ('Navigation bars', 'Navigation', 'Horizontal navigation with selection and keyboard operation.'),
    'tabbar': ('Tab bars', 'Navigation', 'Bottom navigation with icons, labels, selected states and badges.'),
    'steps': ('Steps', 'Navigation', 'Progress through a sequence in horizontal and vertical arrangements.'),
    'toast': ('Toasts', 'Feedback', 'Temporary success, error, loading and text feedback.'),
    'msg': ('Messages', 'Feedback', 'Outcome pages with action areas, supporting details and optional previews.'),
    'progress': ('Progress', 'Feedback', 'Upload-progress presentation and its example state.'),
    'loading': ('Loading', 'Feedback', 'Loading indicators and their documented variants.'),
    'loadmore': ('Load more', 'Feedback', 'Loading, empty and continuation markers inside a page.'),
    'badge': ('Badges', 'Feedback', 'Small notification indicators composed with list cells.'),
    'information-bar': ('Information bars', 'Feedback', 'Inline notices and information tips.'),
    'list': ('Lists', 'Layout', 'Cell groups, accessories and swipe actions.'),
    'panel': ('Panels', 'Layout', 'Structured text and media previews for content lists.'),
    'preview': ('Form previews', 'Layout', 'Read-only label/value summaries with an operation area.'),
    'flex': ('Flex layouts', 'Layout', 'Flexible rows and proportional content arrangements.'),
    'grid': ('Grids', 'Layout', 'Icon-and-label navigation arranged in a regular grid.'),
    'article': ('Articles', 'Content', 'Headings, body copy, lists and images for long-form content.'),
    'icons': ('Icons', 'Content', 'WeUI icon assets and their semantic visual styles.'),
    'footer': ('Footers', 'Content', 'Supporting text and link arrangements at the bottom of a page.'),
    'home': ('Gallery home', 'Gallery', 'The native gallery’s grouped component navigation.'),
    'layers': ('Layers', 'Gallery', 'A gallery-specific explanation of content, navigation and overlay layers.'),
}
TITLES = {'button_default': 'Button styles', 'button_bottom_fixed': 'Bottom-fixed buttons',
          'form_select': 'Select fields', 'form_primary': 'Primary form', 'form_textarea': 'Text areas',
          'steps_horizonal': 'Horizontal steps', 'half-screen-dialog': 'Half-screen dialogs'}


def build(output: Path, include_api: bool, api_dir: Path | None = None):
    # The script owns only this generated directory, never source or the whole target tree.
    if output.resolve() in [ROOT.resolve(), (ROOT / 'target').resolve()]:
        raise ValueError('Choose a dedicated website output directory')
    output.mkdir(parents=True, exist_ok=True)
    for name in ('index.html', 'style.css', 'app.js', 'favicon.svg'):
        shutil.copyfile(ROOT / 'website' / name, output / name)
    playground_source = ROOT / 'website/playground'
    provenance = json.loads((playground_source / 'vendor/PROVENANCE.json').read_text())
    route_bundle = (playground_source / 'vendor/routes.js').read_text()
    browser_routes = json.loads(route_bundle.removeprefix('window.WEGPUI_ROUTES = ').removesuffix(';\n'))
    for name, info in provenance['files'].items():
        if hashlib.sha256((playground_source / 'vendor' / name).read_bytes()).hexdigest() != info['sha256']:
            raise ValueError(f'Vendored browser asset changed without provenance update: {name}')
    for required in ('index.html', 'router.js', 'bridge.js', 'bridge.css', 'shell.css',
                     'vendor/routes.js', 'vendor/weui.css', 'vendor/example.css',
                     'vendor/zepto.min.js', 'vendor/weui.min.js'):
        if not (playground_source / required).is_file():
            raise ValueError(f'Missing interactive browser asset: {required}')
    shutil.copytree(playground_source, output / 'playground', dirs_exist_ok=True,
                    ignore=shutil.ignore_patterns('vendor_upstream.py', '.gitattributes', '__pycache__', '*.pyc'))
    licenses = output / 'licenses'
    licenses.mkdir(exist_ok=True)
    for name in ('LICENSE-MIT', 'LICENSE-APACHE', 'LICENSE-WEUI', 'NOTICE'):
        shutil.copyfile(ROOT / name, licenses / name)
    captures = output / 'captures'
    captures.mkdir(exist_ok=True)
    route_source = (ROOT / 'examples/gallery/pages/mod.rs').read_text()
    route_map = browser_route_map(route_source)
    records = []
    for line in (ROOT / 'docs/UPSTREAM-COVERAGE.md').read_text().splitlines():
        if not line.startswith('| `'):
            continue
        columns = [x.strip() for x in line.split('|')[1:-1]]
        if len(columns) != 4 or columns[0] == '`index.html`':
            continue
        match = re.fullmatch(r'`([\w-]+)`', columns[2])
        if not match:
            continue
        route = match.group(1)
        family = route.split('_')[0]
        label, category, description = FAMILY_INFO[family]
        states = ['default'] + ([] if columns[3] == '—' else [x.strip() for x in columns[3].split(',')])
        images = {}
        for state in states:
            images[state] = {}
            for theme in ('light', 'dark'):
                stem = route + ('' if state == 'default' else '-' + state) + '-' + theme
                images[state][theme] = {}
                for kind in ('gpui', 'weui'):
                    filename = stem + '-' + kind + '.png'
                    source = ROOT / 'docs/parity' / filename
                    if not source.is_file():
                        raise ValueError(f'Missing recorded capture: {filename}')
                    shutil.copyfile(source, captures / filename)
                    images[state][theme][kind] = 'captures/' + filename
        module = route_map[route]
        page_source = f'examples/gallery/pages/{module}.rs'
        if not (ROOT / page_source).exists():
            page_source = f'examples/gallery/pages/{module}/mod.rs'
        assert (ROOT / page_source).exists(), page_source
        modules_match = re.search(r'components::\{([^}]+)\}', columns[1])
        modules = [x.strip() for x in modules_match.group(1).split(',')] if modules_match else []
        # input is a crate-level module, not components::input.
        api_modules = [('input' if x == 'input' else f'components/{x}') for x in modules]
        title = TITLES.get(route, label if route == family else route.replace('_', ' ').replace('-', ' ').capitalize())
        records.append(dict(id=route, family=family, familyLabel=label, category=category,
                            title=title, description=description, states=states, images=images,
                            source=page_source, modules=api_modules,
                            playground='playground/index.html#' + route,
                            browserSource='playground/source/' + route + '.html.txt'))
    assert {r['id'] for r in records} == set(route_map), 'Coverage differs from the Rust gallery'
    assert {r['id'] for r in provenance['routes']} == set(route_map), 'Browser inventory differs from the 60 routes'
    assert len(provenance['routes']) == 60
    assert set(browser_routes) == set(route_map), 'Browser route bundle differs from the inventory'
    for record in records:
        assert set(browser_routes[record['id']]['states']) == set(record['states']), record['id']
        source = playground_source / 'source' / (record['id'] + '.html.txt')
        assert hashlib.sha256(source.read_bytes()).hexdigest() == browser_routes[record['id']]['sha256'], record['id']
    manifest = tomllib.loads((ROOT / 'Cargo.toml').read_text())
    data = dict(routes=records, tokens=json.loads((ROOT / 'docs/tokens.json').read_text()),
                sourceCommit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                sourceDirty=bool(subprocess.check_output(
                    ['git', 'status', '--porcelain', '--untracked-files=no'], cwd=ROOT, text=True).strip()),
                captureCommit='2c8ed10', gpuiVersion=manifest['dependencies']['gpui']['version'].removeprefix('='),
                rustVersion=manifest['package']['rust-version'], apiIncluded=include_api,
                playground=dict(entry='playground/index.html', engine='WeUI HTML/CSS/JavaScript browser examples',
                                weuiVersion=provenance['weuiVersion'], weuiJsVersion=provenance['weuiJsVersion'],
                                zeptoVersion=provenance['zeptoVersion'], routeCount=len(provenance['routes']),
                                states={record['id']: record['states'] for record in records},
                                provenance='playground/vendor/PROVENANCE.json'),
                compatibilityPatches={name: entry['path'] for name, entry in
                    manifest.get('patch', {}).get('crates-io', {}).items() if 'path' in entry})
    if include_api:
        if api_dir is None:
            metadata = json.loads(subprocess.check_output(
                ['cargo', 'metadata', '--format-version=1', '--no-deps', '--locked', '--offline'],
                cwd=ROOT, text=True))
            source = Path(metadata['target_directory']) / 'doc'
        else:
            source = api_dir.resolve()
        if not (source / 'wegpui/index.html').exists():
            raise ValueError('Run cargo doc --no-deps --locked before --include-api')
        shutil.copytree(source, output / 'api', dirs_exist_ok=True, ignore=shutil.ignore_patterns('.lock'))
    (output / 'data.js').write_text('window.WEGPUI = ' + json.dumps(data, ensure_ascii=False) + ';\n')
    (output / '.nojekyll').write_text('')
    (output / 'robots.txt').write_text('User-agent: *\nAllow: /\n')
    print(f'Built {len(records)} routes, {sum(len(r["states"]) for r in records)} recorded states; API docs: {include_api}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', type=Path, default=ROOT / 'website/dist')
    parser.add_argument('--include-api', action='store_true')
    parser.add_argument('--api-dir', type=Path, help='Rustdoc directory; defaults to Cargo target_directory/doc')
    args = parser.parse_args()
    build(args.out, args.include_api, args.api_dir)
