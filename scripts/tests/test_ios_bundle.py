"""Bundle-script contract tests using fake Apple tools; never Apple acceptance."""
import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class BundleScriptTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.ios = self.root / 'examples' / 'ios'
        self.ios.mkdir(parents=True)
        for name in ('build-simulator.sh', 'Info.plist'):
            shutil.copy(ROOT / 'examples' / 'ios' / name, self.ios / name)
        self.bin = self.root / 'bin'
        self.bin.mkdir()
        self.tool('uname', '#!/bin/sh\necho Darwin\n')
        self.tool('xcrun', '#!/bin/sh\necho /fixture/SDK\n')
        self.tool('codesign', '#!/bin/sh\necho fixture-signing-only\n')
        self.tool('cargo', '''#!/usr/bin/env python3
import json, os, pathlib, sys
if os.environ.get('FIXTURE_FAIL'):
    print('fixture compiler failure', file=sys.stderr)
    sys.exit(42)
name = sys.argv[sys.argv.index('--example') + 1]
exe = pathlib.Path.cwd() / 'fixture executable'
exe.write_text('fixture binary, not executable Apple code')
event = {'reason': 'compiler-artifact', 'target': {'name': name, 'kind': ['example']}, 'executable': str(exe)}
print(json.dumps(event))
if os.environ.get('FIXTURE_DUPLICATE'):
    print(json.dumps(event))
''')
        self.env = dict(os.environ, PATH=f'{self.bin}:{os.environ["PATH"]}',
                        CARGO_TARGET_DIR='output with spaces',
                        IOS_SIMULATOR_TARGET='aarch64-apple-ios-sim')

    def tool(self, name, source):
        path = self.bin / name
        path.write_text(source)
        path.chmod(0o755)

    def run_script(self, *args):
        return subprocess.run(['bash', str(self.ios / 'build-simulator.sh'), *args],
                              cwd=self.root, env=self.env, text=True, capture_output=True)

    def receipts(self):
        return list((self.root / 'output with spaces' / 'ios-simulator').glob('*'))

    def test_relative_target_directory_and_both_examples(self):
        for example in ('gallery', 'minimal'):
            result = self.run_script(example)
            self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(self.receipts()), 2)
        for receipt in self.receipts():
            app = receipt / 'WeGPUI.app'
            info = plistlib.loads((app / 'Info.plist').read_bytes())
            self.assertTrue((app / info['CFBundleExecutable']).is_file())
            self.assertTrue((receipt / 'codesign.txt').is_file())
            self.assertIn('not launched', result.stdout)

    def test_build_failure_is_retained_without_a_bundle(self):
        self.env['FIXTURE_FAIL'] = '1'
        result = self.run_script()
        self.assertEqual(result.returncode, 42)
        receipt, = self.receipts()
        self.assertIn('fixture compiler failure', (receipt / 'build.stderr').read_text())
        self.assertFalse((receipt / 'WeGPUI.app').exists())

    def test_ambiguous_artifacts_fail_without_a_bundle(self):
        self.env['FIXTURE_DUPLICATE'] = '1'
        result = self.run_script()
        self.assertNotEqual(result.returncode, 0)
        receipt, = self.receipts()
        self.assertTrue((receipt / 'build.jsonl').is_file())
        self.assertFalse((receipt / 'WeGPUI.app').exists())

    def test_linux_host_is_rejected_before_build(self):
        self.tool('uname', '#!/bin/sh\necho Linux\n')
        result = self.run_script()
        self.assertEqual(result.returncode, 1)
        self.assertIn('authorized Apple host', result.stderr)
        self.assertEqual(self.receipts(), [])

    def test_device_target_is_rejected(self):
        self.env['IOS_SIMULATOR_TARGET'] = 'aarch64-apple-ios'
        self.assertEqual(self.run_script().returncode, 2)
        self.assertEqual(self.receipts(), [])


if __name__ == '__main__':
    unittest.main()
