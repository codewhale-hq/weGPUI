import contextlib
import hashlib
import io
import json
import pathlib
import re
import runpy
import sys

ROOT = pathlib.Path(__file__).resolve().parent
BASE = ROOT.parent / 'chromium-icu-78-oracle/oracle.py'
if len(sys.argv) != 2 or not pathlib.Path(sys.argv[1]).is_absolute():
    raise SystemExit('usage: holdout-oracle.py /absolute/path/to/icu-build-run')
run = pathlib.Path(sys.argv[1])
sys.argv = [str(BASE), str(run)]
out = io.StringIO()
with contextlib.redirect_stdout(out):
    module = runpy.run_path(str(BASE), run_name='__main__')
result = json.loads(out.getvalue())
reference = module['boundaries']
source_root = run / 'source/test/testdata/cldr/personNameTest'
scripts = {'km': (0x1780, 0x17ff), 'lo': (0x0e80, 0x0eff), 'th': (0x0e00, 0x0e7f), 'my': (0x1000, 0x109f)}
source_commit = '19dfb44a62ae60ce6b0060de8c0cef5d5c527177'
seeds = {}
for lang, (low, high) in scripts.items():
    path = source_root / (lang + '.txt')
    data = path.read_bytes()
    found = []
    seen = set()
    for no, line in enumerate(data.decode('utf-8').splitlines(), 1):
        match = re.fullmatch(r'name\s*;\s*(?:given|given2|surname|surname-core)\s*;\s*(.+?)\s*', line)
        if not match:
            continue
        text = match.group(1)
        if text in seen or len(text) < 2 or not all(low <= ord(ch) <= high for ch in text):
            continue
        seen.add(text)
        found.append({'text': text, 'line': no, 'source_file': str(path),
                      'source_sha256': hashlib.sha256(data).hexdigest(),
                      'source_url': f'https://chromium.googlesource.com/chromium/deps/icu/+/{source_commit}/source/test/testdata/cldr/personNameTest/{lang}.txt'})
        if len(found) == 8:
            break
    assert len(found) == 8, (lang, found)
    seeds[lang] = found
old_texts = set()
for name in ('expanded-oracle.json', 'transition-oracle-expanded.json'):
    old_texts.update(case['text'] for case in json.loads((ROOT / name).read_text())['cases'])
cases = []
def add(id, text, sources, kind, lang):
    assert text not in old_texts, id
    offsets, status = reference(text)
    cases.append({'id': id, 'language': 'km', 'text': text, 'exact_icu4c_boundaries': offsets,
                  'status': status, 'holdout_language': lang, 'kind': kind, 'sources': sources})
contexts = {
    'semicolon': lambda s: s + ';' + s,
    'slash': lambda s: s + '/' + s,
    'en-dash': lambda s: s + '\u2013' + s,
    'curly-quotes': lambda s: '\u201c' + s + '\u201d',
    'zwnj': lambda s: s + '\u200c' + s,
    'thin-space': lambda s: s + '\u2009' + s,
    'crlf': lambda s: s + '\r\n' + s,
    'tab': lambda s: s + '\t' + s,
    'combining-acute': lambda s: s + '\u0301' + s,
    'directional-isolate': lambda s: '\u2068' + s + '\u2069' + s,
    'variation-selector': lambda s: s + '\ufe0f' + s,
}
for lang, entries in seeds.items():
    for i, source in enumerate(entries):
        add(f'{lang}-{i:02}-base', source['text'], [source], 'base', lang)
        for label, transform in contexts.items():
            add(f'{lang}-{i:02}-{label}', transform(source['text']), [source], label, lang)
for lang, entries in seeds.items():
    for other, others in seeds.items():
        if lang == other:
            continue
        for i, (a, b) in enumerate(zip(entries, others)):
            add(f'{lang}-{i:02}-adjacent-{other}', a['text'] + b['text'], [a, b], 'cross-script-adjacent', lang + '+' + other)
result['cases'] = cases
result['holdout_corpus'] = {'sources': 'new CLDR person-name test fields from pinned ICU source',
                            'seeds_per_script': 8, 'contexts': list(contexts),
                            'note': 'Corpus not used to select candidate; some texts are short names, not natural sentence samples; language km is harness grouping.'}
print(json.dumps(result, ensure_ascii=False, indent=2))
