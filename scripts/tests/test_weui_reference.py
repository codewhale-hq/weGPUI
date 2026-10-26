import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock


SPEC = importlib.util.spec_from_file_location(
    "weui_reference", Path(__file__).resolve().parents[1] / "weui_reference.py"
)
reference = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(reference)


class WeuiReferenceTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.source = self.root / "source"
        (self.source / "dist/style").mkdir(parents=True)
        (self.source / "dist/example").mkdir()
        (self.source / "package.json").write_text(json.dumps({"version": reference.VERSION}))
        (self.source / "LICENSE.txt").write_text("MIT fixture\n")
        (self.source / "dist/style/weui.css").write_bytes(b"body { color: black; }\n")
        self.index = self.source / "dist/example/index.html"
        self.index.write_text(
            '<html><body>Example</body>' + reference.SDK_TAG
            + f'<script src="{reference.JS_URL}"></script></html>'
        )
        self.javascript = self.root / "weui.min.js"
        self.javascript.write_bytes(b"window.weui = {};\n")
        self.addCleanup(mock.patch.stopall)
        mock.patch.object(reference, "CSS_SHA256", reference.sha256(
            (self.source / "dist/style/weui.css").read_bytes())).start()
        mock.patch.object(reference, "JS_SHA256", reference.sha256(self.javascript.read_bytes())).start()
        mock.patch.object(reference.subprocess, "check_output",
                          side_effect=[reference.REVISION + "\n", b""]).start()
        self.output = self.root / "reference"

    def prepare(self):
        return reference.prepare(self.source, self.output, self.javascript)

    def test_reference_retains_assets_and_records_exact_html_changes(self):
        metadata = self.prepare()
        html = (self.output / "dist/example/index.html").read_text()
        self.assertNotIn(reference.SDK_TAG, html)
        self.assertIn('src="../../vendor/weui.min.js"', html)
        self.assertEqual((self.output / "vendor/weui.min.js").read_bytes(), self.javascript.read_bytes())
        self.assertEqual((self.output / "dist/style/weui.css").read_bytes(),
                         (self.source / "dist/style/weui.css").read_bytes())
        self.assertNotEqual(metadata["original_index_sha256"], metadata["reference_index_sha256"])
        self.assertEqual(len(metadata["changes"]), 2)
        self.assertNotIn(str(self.root), json.dumps(metadata))

    def test_used_wechat_sdk_cannot_be_silently_removed(self):
        self.index.write_text(self.index.read_text() + "<script>wx.config({});</script>")
        with self.assertRaisesRegex(ValueError, "uses the WeChat SDK"):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_changed_ui_runtime_is_rejected(self):
        self.javascript.write_bytes(b"changed runtime")
        with self.assertRaisesRegex(ValueError, "weui.js differs"):
            self.prepare()

    def test_existing_evidence_is_not_overwritten(self):
        self.output.mkdir()
        marker = self.output / "old-evidence.txt"
        marker.write_text("keep")
        with self.assertRaisesRegex(ValueError, "empty reference"):
            self.prepare()
        self.assertEqual(marker.read_text(), "keep")

    def test_other_upstream_revision_is_rejected(self):
        reference.subprocess.check_output.side_effect = ["different-revision\n"]
        with self.assertRaisesRegex(ValueError, "Expected upstream"):
            self.prepare()


if __name__ == "__main__":
    unittest.main()
