import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    "weui_parity_capture", Path(__file__).resolve().parents[1] / "weui_parity_capture.py"
)
capture = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(capture)


class CaptureEvidenceValidationTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.reference = self.root / "reference"
        self.metadata = {}
        for path, key in [("dist/style/weui.css", "css_sha256"), ("vendor/weui.min.js", "weui_js_sha256"),
                          ("dist/example/index.html", "reference_index_sha256")]:
            target = self.reference / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(path.encode())
            self.metadata[key] = hashlib.sha256(target.read_bytes()).hexdigest()
        (self.reference / "REFERENCE.json").write_text(json.dumps(self.metadata))
        self.inventory = self.root / "inventory.json"
        self.inventory.write_text(json.dumps({"reference": self.metadata, "pair_count": 8,
                                              "upstream_routes": ["footer"], "replays": {}}))
        self.matrix = self.root / "matrix.jsonl"
        self.record = {"id": "light/footer", "route": "footer", "state": None,
                       "theme": "light", "care": False, "brand": None}
        self.matrix.write_text(json.dumps(self.record) + "\n")
        self.binary = self.root / "binary"
        self.binary.write_bytes(b"fixture; never executed")
        self.binary.chmod(0o755)
        self.output = self.root / "output"
        self.argv = ["capture", "--reference", str(self.reference), "--inventory", str(self.inventory),
                     "--matrix", str(self.matrix), "--binary", str(self.binary), "--out", str(self.output)]

    def rejected(self, *extra):
        with patch("sys.argv", self.argv + list(extra)), contextlib.redirect_stderr(io.StringIO()):
            with self.assertRaises(SystemExit) as error:
                capture.main()
        self.assertEqual(error.exception.code, 2)
        self.assertFalse(self.output.exists())

    def test_changed_reference_asset_is_rejected_before_output(self):
        (self.reference / "dist/style/weui.css").write_text("changed")
        self.rejected()

    def test_reference_metadata_drift_is_rejected_before_output(self):
        (self.reference / "REFERENCE.json").write_text("{}")
        self.rejected()

    def test_writable_binary_cannot_be_reused_as_frozen(self):
        self.rejected("--reuse-frozen-binary")

    def test_parent_scene_path_is_rejected_before_output(self):
        self.matrix.write_text(json.dumps({"id": "../outside"}) + "\n")
        self.rejected()

    def test_duplicate_scenes_cannot_overwrite_evidence(self):
        self.matrix.write_text(self.matrix.read_text() * 2)
        self.rejected()

    def test_inconsistent_scene_metadata_is_rejected_before_output(self):
        for change in [{"route": "unknown"}, {"state": "missing"}, {"theme": "dark"},
                       {"care": 1}, {"care": True}, {"brand": "#7C3AED"},
                       {"id": "light"}, {"id": "light/footer/extra"}, {"id": "light/.."}]:
            with self.subTest(change=change):
                self.matrix.write_text(json.dumps(self.record | change) + "\n")
                self.rejected()

    def test_registered_state_and_custom_brand_can_be_captured(self):
        inventory = {"upstream_routes": ["footer"], "replays": {"footer:active": "replay"}}
        capture.validate_records([self.record | {"id": "dark-care-brand/footer-active", "theme": "dark",
                                 "care": True, "brand": "ABC123", "state": "active"}], inventory)


if __name__ == "__main__":
    unittest.main()
