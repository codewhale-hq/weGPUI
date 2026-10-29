import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('website_inventory', ROOT / 'website/inventory.py')
inventory = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(inventory)


class RouteInventoryTests(unittest.TestCase):
    def test_native_languages_page_does_not_require_a_browser_demo(self):
        source = '("desktop-form", desktop_form::build), ("languages", languages::build), ("dialog", dialog::build), ("form", form::build)'
        source += ''.join(f'("{route}", macos::build),' for route in inventory.NATIVE_ONLY_ROUTES if route not in {'languages', 'desktop-form'})
        self.assertEqual(inventory.browser_route_map(source), {'dialog': 'dialog', 'form': 'form'})

    def test_missing_native_only_exception_is_reported(self):
        with self.assertRaisesRegex(ValueError, 'not registered'):
            inventory.browser_route_map('("dialog", dialog::build)')

    def test_other_new_routes_are_not_silently_excluded(self):
        source = '("desktop-form", desktop_form::build), ("languages", languages::build), ("new-route", extra::build)'
        source += ''.join(f'("{route}", macos::build),' for route in inventory.NATIVE_ONLY_ROUTES if route not in {'languages', 'desktop-form'})
        self.assertIn('new-route', inventory.browser_route_map(source))

    def test_all_real_browser_routes_match_the_upstream_inventory(self):
        routes = inventory.browser_route_map((ROOT / 'examples/gallery/pages/mod.rs').read_text())
        provenance = json.loads((ROOT / 'website/playground/vendor/PROVENANCE.json').read_text())
        expected = {route['id'] for route in provenance['routes']}
        self.assertEqual(set(routes), expected)
        self.assertEqual(len(routes), 60)

    def test_isolated_python_builds_the_complete_real_site(self):
        with tempfile.TemporaryDirectory() as output:
            result = subprocess.run(
                [sys.executable, '-I', str(ROOT / 'website/build.py'), '--out', output],
                text=True, capture_output=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            data = (Path(output) / 'data.js').read_text()
            records = json.loads(data.removeprefix('window.WEGPUI = ').removesuffix(';\n'))
            self.assertEqual(len(records['routes']), 71)
            languages = next(route for route in records['routes'] if route['id'] == 'languages')
            self.assertNotIn('playground', languages)
            self.assertEqual((Path(output) / languages['rustSource']).read_bytes(),
                             (ROOT / languages['source']).read_bytes())
            evidence = records['nativeEvidence']
            self.assertEqual(evidence['capturedPairs'], 1056)
            self.assertEqual(evidence['scenes'], 132)
            panel = next(route for route in records['routes'] if route['id'] == 'panel')
            self.assertEqual(panel['captureEvidence']['default']['light']['platform'], 'Linux ARM64')
            self.assertEqual(len(list((Path(output) / 'captures').glob('*.png'))), 862)


if __name__ == '__main__':
    unittest.main()
