#!/usr/bin/env python3
"""Build the native Rust showcase and its upstream browser references."""
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


def native_evidence():
    config = json.loads((ROOT / 'website/native-evidence.json').read_text())
    receipt = (ROOT / config['receipt']).resolve()
    if not receipt.is_relative_to(ROOT / 'docs/receipts'):
        raise ValueError('Native evidence must belong to a repository receipt')
    checksums = json.loads((receipt / 'CHECKSUMS.json').read_text())
    required = ['metadata/verification.json', 'metadata/qualified-matrix.jsonl', 'metadata/ci-jobs-final.json']
    required += [sample[key] for sample in config['samples'] for key in ['native', 'reference']]
    for name in required:
        path = (receipt / name).resolve()
        if not path.is_relative_to(receipt) or not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != checksums.get(name):
            raise ValueError(f'Native evidence differs from its receipt: {name}')
    verification = json.loads((receipt / required[0]).read_text())
    matrix = [json.loads(line) for line in (receipt / required[1]).read_text().splitlines()]
    rows = {row['id']: row for row in matrix}
    if len(rows) != len(matrix) or len(rows) != verification['pairs'] or any(row['status'] != 'captured' for row in rows.values()):
        raise ValueError('Native matrix is incomplete')
    if len({sample['id'] for sample in config['samples']}) != len(config['samples']):
        raise ValueError('Native samples must have unique identifiers')
    samples = []
    for sample in config['samples']:
        row = rows[sample['id']]
        if row['theme'] not in ['light', 'dark'] or row['care'] or row['brand']:
            raise ValueError('The native component viewer requires an ordinary light/dark sample')
        for key, hash_key in [('native', 'native_sha256'), ('reference', 'reference_sha256')]:
            if checksums[sample[key]] != row[hash_key]:
                raise ValueError('Native sample differs from its qualified matrix')
        corpus = row['corpus']
        manifest_name = f'metadata/{corpus}/manifest.json'
        manifest_path = (receipt / manifest_name).resolve()
        if not manifest_path.is_relative_to(receipt) or hashlib.sha256(manifest_path.read_bytes()).hexdigest() != checksums.get(manifest_name):
            raise ValueError('Native capture manifest differs from its receipt')
        manifest = json.loads(manifest_path.read_text())
        if manifest['host']['system'] != 'Linux' or manifest['host']['architecture'] != 'aarch64':
            raise ValueError('Native sample platform differs from its recorded host')
        samples.append(dict(sample, route=row['route'], state=row['state'] or 'default', theme=row['theme'],
                            commit=manifest['base_commit'], platform='Linux ARM64', viewport=row['viewport']))
    return dict(receipt=config['receipt'] + '/README.md', sourceCommit=verification['final_source_commit'],
                capturedPairs=verification['pairs'], scenes=len({(row['route'],row['state']) for row in rows.values()}),
                platformJobs=json.loads((receipt / required[2]).read_text())['jobs'], samples=samples)


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
    for name in ('LICENSE-MIT', 'LICENSE-APACHE', 'LICENSE-WEUI', 'LICENSE-UNICODE-3', 'NOTICE'):
        shutil.copyfile(ROOT / name, licenses / name)
    evidence = native_evidence()
    captures = output / 'captures'
    captures.mkdir(exist_ok=True)
    sources = output / 'source'
    sources.mkdir(exist_ok=True)
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
        shutil.copyfile(ROOT / page_source, sources / f'{route}.rs')
        modules_match = re.search(r'components::\{([^}]+)\}', columns[1])
        modules = [x.strip() for x in modules_match.group(1).split(',')] if modules_match else []
        # input is a crate-level module, not components::input.
        api_modules = [('input' if x == 'input' else f'components/{x}') for x in modules]
        title = TITLES.get(route, label if route == family else route.replace('_', ' ').replace('-', ' ').capitalize())
        capture_evidence = {}
        for sample in evidence['samples']:
            if sample['route'] != route:
                continue
            state, theme = sample['state'], sample['theme']
            if state not in states:
                raise ValueError('Native sample state differs from the gallery')
            capture_evidence.setdefault(state, {})[theme] = dict(commit=sample['commit'], platform=sample['platform'], viewport=sample['viewport'], receipt=evidence['receipt'])
            for key, kind in [('native','gpui'),('reference','weui')]:
                filename = 'verified-' + sample['id'].replace('/','-') + '-' + kind + '.png'
                shutil.copyfile(ROOT / Path(evidence['receipt']).parent / sample[key], captures / filename)
                images[state][theme][kind] = 'captures/' + filename
        records.append(dict(id=route, family=family, familyLabel=label, category=category,
                            title=title, description=description, states=states, images=images,
                            source=page_source, rustSource=f'source/{route}.rs', modules=api_modules,
                            playground='playground/index.html#' + route,
                            browserSource='playground/source/' + route + '.html.txt', captureEvidence=capture_evidence))
    assert {r['id'] for r in records} == set(route_map), 'Coverage differs from the Rust gallery'
    assert {r['id'] for r in provenance['routes']} == set(route_map), 'Browser inventory differs from the 60 routes'
    assert len(provenance['routes']) == 60
    assert set(browser_routes) == set(route_map), 'Browser route bundle differs from the inventory'
    for record in records:
        assert set(browser_routes[record['id']]['states']) == set(record['states']), record['id']
        source = playground_source / 'source' / (record['id'] + '.html.txt')
        assert hashlib.sha256(source.read_bytes()).hexdigest() == browser_routes[record['id']]['sha256'], record['id']
    language_receipt = ROOT / 'docs/receipts/rtl-completion-20261025'
    language_hashes = json.loads((language_receipt / 'evidence-sha256.json').read_text())
    language_commit = json.loads((language_receipt / 'final/results.json').read_text())['source_commit']
    language_images = {}
    language_evidence = {}
    samples = [('default', 'light', 'macos/paragraphs-en-final-light-gpui.png'),
               ('arabic-selection', 'dark', 'macos/textarea-ar-dark-final-dark-gpui.png')]
    for state, theme, name in samples:
        source = language_receipt / name
        if hashlib.sha256(source.read_bytes()).hexdigest() != language_hashes[name]:
            raise ValueError('Native language capture differs from its receipt')
        filename = f'languages-{theme}-gpui.png'
        shutil.copyfile(source, captures / filename)
        language_images[state] = {theme: dict(gpui='captures/' + filename)}
        language_evidence[state] = {theme: dict(commit=language_commit, platform='macOS',
                                               viewport=[375, 812], receipt='docs/receipts/rtl-completion-20261025/README.md')}
    language_source = 'examples/gallery/pages/languages.rs'
    shutil.copyfile(ROOT / language_source, sources / 'languages.rs')
    records.append(dict(id='languages', family='languages', familyLabel='Languages', category='Content',
                        title='Languages & typography', description='Multilingual text, bidirectional layout and native text editing.',
                        states=['default', 'arabic-selection'], images=language_images, source=language_source,
                        rustSource='source/languages.rs', modules=['input', 'i18n'],
                        nativeArgs={'default': '--lang en --scroll 2310', 'arabic-selection': '--lang ar --state type:7:all'},
                        captureEvidence=language_evidence))
    registry = set(re.findall(r'\("([\w-]+)",\s*\w+::build\)', route_source))
    assert {record['id'] for record in records} == registry, 'Website differs from the complete native gallery'
    manifest = tomllib.loads((ROOT / 'Cargo.toml').read_text())
    evidence.pop('samples')
    data = dict(nativeEvidence=evidence, routes=records, tokens=json.loads((ROOT / 'docs/tokens.json').read_text()),
                sourceCommit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                sourceDirty=bool(subprocess.check_output(
                    ['git', 'status', '--porcelain', '--untracked-files=no'], cwd=ROOT, text=True).strip()),
                captureCommit='2c8ed10', gpuiVersion=manifest['dependencies']['gpui']['version'].removeprefix('='),
                rustVersion=manifest['package']['rust-version'], apiIncluded=include_api,
                playground=dict(entry='playground/index.html', engine='WeUI HTML/CSS/JavaScript browser examples',
                                weuiVersion=provenance['weuiVersion'], weuiJsVersion=provenance['weuiJsVersion'],
                                zeptoVersion=provenance['zeptoVersion'], routeCount=len(provenance['routes']),
                                states={record['id']: record['states'] for record in records if record.get('playground')},
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
