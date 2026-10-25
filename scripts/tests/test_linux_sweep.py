"""Sweep orchestration with fixture captures; no native or visual acceptance."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class LinuxSweepTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.script = self.root / 'scripts' / 'linux' / 'sweep.sh'
        self.script.parent.mkdir(parents=True)
        shutil.copy(ROOT / 'scripts' / 'linux' / 'sweep.sh', self.script)
        parity = self.root / 'scripts' / 'parity'
        parity.mkdir()
        (parity / 'fixture.json').write_text(json.dumps({'dialog:open': '', 'picker:date': ''}))
        (self.root / 'scripts' / 'capture.py').write_text('''
import json, os, pathlib, sys
with pathlib.Path(os.environ['FIXTURE_CALLS']).open('a') as output:
    output.write(json.dumps({'args': sys.argv[1:], 'pair_dir': os.environ['PAIR_DIR']}) + '\\n')
if sys.argv[1] == os.environ.get('FIXTURE_FAIL'):
    print('fixture capture failure retained', file=sys.stderr)
    sys.exit(42)
''')
        self.out = self.root / 'output with spaces'
        self.calls = self.root / 'calls.jsonl'
        self.env = dict(os.environ, PYTHON=sys.executable, FIXTURE_CALLS=str(self.calls))

    def run_script(self, *args):
        return subprocess.run(['bash', str(self.script), str(self.out), *args],
                              env=self.env, text=True, capture_output=True)

    def captures(self):
        return [json.loads(line) for line in self.calls.read_text().splitlines()]

    def test_care_brand_and_dark_reach_each_registered_state(self):
        result = self.run_script('--states', '--theme', 'dark', '--care', '--brand', 'aB12eF')
        self.assertEqual(result.returncode, 0, result.stderr)
        captures = self.captures()
        self.assertEqual(len(captures), 2)
        for capture, route, state in zip(captures, ('dialog', 'picker'), ('open', 'date')):
            self.assertEqual(capture['args'], [route, '--state', state, '--theme', 'dark',
                                              '--care', '--brand', 'aB12eF', '--out', str(self.out / 'raw')])
            self.assertEqual(capture['pair_dir'], str(self.out / 'pairs'))
        self.assertIn('done: 2 ok, 0 failed', result.stdout)
        self.assertIn('theme=dark care=1 brand=aB12eF only=both states=1', (self.out / 'sweep.log').read_text())

    def test_native_only_is_forwarded_without_reference_capture(self):
        result = self.run_script('--only', 'gpui', '--states')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(self.captures()), 2)
        for capture in self.captures():
            self.assertIn('--only', capture['args'])
            self.assertEqual(capture['args'][capture['args'].index('--only') + 1], 'gpui')
        self.assertIn('only=gpui', (self.out / 'sweep.log').read_text())

    def test_routes_keep_default_theme_without_optional_modes(self):
        result = self.run_script('form_select', 'button')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual([capture['args'] for capture in self.captures()],
                         [[route, '--theme', 'light', '--out', str(self.out / 'raw')]
                          for route in ('form_select', 'button')])

    def test_failed_capture_is_retained_and_later_routes_still_run(self):
        self.env['FIXTURE_FAIL'] = 'dialog'
        result = self.run_script('dialog', 'picker')
        self.assertEqual(result.returncode, 1)
        self.assertEqual(len(self.captures()), 2)
        self.assertIn('fixture capture failure retained', (self.out / 'err-dialog.txt').read_text())
        self.assertIn('FAIL dialog', (self.out / 'sweep.log').read_text())
        self.assertIn('done: 1 ok, 1 failed', result.stdout)

    def test_invalid_arguments_fail_before_capture_or_output_creation(self):
        for args in (('--theme',), ('--theme', 'care'), ('--brand',), ('--brand', ''),
                     ('--brand', '#123456'), ('--brand', '12345'), ('--brand', '12345g'),
                     ('--brand', '--care'), ('--only',), ('--only', 'both'), ('--unknown',),
                     ('--states', 'dialog'), ('../dialog',)):
            with self.subTest(args=args):
                result = self.run_script(*args)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertFalse(self.calls.exists())
                self.assertFalse(self.out.exists())


if __name__ == '__main__':
    unittest.main()
