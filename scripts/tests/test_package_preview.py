import importlib.util
import io
import json
import os
import pathlib
import shutil
import tarfile
import tempfile
import unittest


SCRIPT = pathlib.Path(__file__).resolve().parents[1] / "package_preview.py"
SPEC = importlib.util.spec_from_file_location("package_preview", SCRIPT)
preview = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(preview)


class PreviewBundleTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)
        self.source = self.root / "source"
        self.source.mkdir()
        for name in preview.FILES:
            (self.source / name).write_text("Example\n", encoding="utf-8")
        (self.source / "Cargo.toml").write_text(
            '[package]\nname="wegpui"\nversion="0.1.0"\n', encoding="utf-8"
        )
        (self.source / "Cargo.lock").write_text(
            'version=4\n[[package]]\nname="wegpui"\nversion="0.1.0"\n',
            encoding="utf-8",
        )
        for vendor in preview.VENDORS:
            path = self.source / "vendor" / vendor
            path.mkdir(parents=True)
            (path / "Cargo.toml").write_text('[package]\nname="fixture"\nversion="0.1.0"\n')
            (path / "LICENSE").write_text("Vendor license\n")
        (self.source / "src").mkdir()
        (self.source / "src/lib.rs").write_text("pub fn sample() {}\n")
        (self.source / "assets").mkdir()
        (self.source / "assets/image.png").write_bytes(b"image fixture")
        (self.source / "examples/support").mkdir(parents=True)
        (self.source / "examples/application.rs").write_text("fn main() {}\n")
        (self.source / "examples/support/platform.rs").write_text("pub fn application() {}\n")

    def make_bundle(self, output="out"):
        return preview.create_bundle(self.source, self.root / output, "base-revision")

    def test_bundle_is_reproducible_across_location_and_mtime_changes(self):
        first = self.make_bundle()
        moved = self.root / "different-location"
        shutil.copytree(self.source, moved)
        for path in moved.rglob("*"):
            if path.is_file():
                os.utime(path, (1, 1))
        self.source = moved
        second = self.make_bundle("second-out")
        self.assertEqual(first.read_bytes(), second.read_bytes())
        self.assertEqual(preview.verify_bundle(first), preview.verify_bundle(second))

    def test_separate_consumer_uses_all_workspace_patches(self):
        archive = self.make_bundle()
        with tarfile.open(archive) as tar:
            root = pathlib.PurePosixPath(tar.getnames()[0]).parts[0]
            workspace = tar.extractfile(f"{root}/Cargo.toml").read().decode()
            self.assertIn('members = ["wegpui", "consumer"]', workspace)
            for name in ("LICENSE-MIT", "LICENSE-APACHE", "LICENSE-WEUI", "LICENSE-UNICODE-3", "NOTICE",
                         "CHANGELOG.md", "CODE_OF_CONDUCT.md", "SECURITY.md"):
                self.assertIsNotNone(tar.getmember(f"{root}/wegpui/{name}"))
            for vendor in preview.VENDORS:
                self.assertIn(f'path = "wegpui/vendor/{vendor}"', workspace)
                self.assertIsNotNone(tar.getmember(f"{root}/wegpui/vendor/{vendor}/LICENSE"))
            self.assertEqual(
                tar.extractfile(f"{root}/consumer/src/main.rs").read(),
                (self.source / "examples/application.rs").read_bytes(),
            )
            manifest = json.load(tar.extractfile(f"{root}/SOURCE.json"))
            self.assertEqual(manifest["base_revision"], "base-revision")
            self.assertNotIn(str(self.source), json.dumps(manifest))

    def test_an_existing_artifact_is_not_overwritten_by_different_provenance(self):
        archive = self.make_bundle()
        original = archive.read_bytes()
        checksum = archive.with_name(archive.name + ".sha256").read_bytes()
        with self.assertRaisesRegex(ValueError, "already exists with different provenance"):
            preview.create_bundle(self.source, self.root / "out", "another-base-revision")
        self.assertEqual(archive.read_bytes(), original)
        self.assertEqual(archive.with_name(archive.name + ".sha256").read_bytes(), checksum)

    def test_missing_vendor_copy_is_rejected(self):
        shutil.rmtree(self.source / "vendor/gpui_pre")
        with self.assertRaisesRegex(ValueError, "Missing required vendor"):
            self.make_bundle()

    def test_source_symlink_is_rejected(self):
        (self.source / "assets/linked.png").symlink_to(self.source / "assets/image.png")
        with self.assertRaisesRegex(ValueError, "symlinks"):
            self.make_bundle()

    def test_modified_archive_content_is_rejected(self):
        original = self.make_bundle()
        changed = self.root / "tampered.tar.gz"
        with tarfile.open(original) as source, tarfile.open(changed, "w:gz") as destination:
            for entry in source:
                data = source.extractfile(entry).read()
                if entry.name.endswith("/wegpui/src/lib.rs"):
                    data += b"pub fn changed() {}\n"
                    entry.size = len(data)
                destination.addfile(entry, io.BytesIO(data))
        with self.assertRaisesRegex(ValueError, "CHECKSUMS"):
            preview.verify_bundle(changed)

    def test_unsafe_archive_paths_are_rejected(self):
        for name in ["bundle/../escape", r"bundle/..\escape", "C:/bundle/file", "bundle/file:stream"]:
            with self.subTest(path=name):
                unsafe = self.root / "unsafe.tar.gz"
                with tarfile.open(unsafe, "w:gz") as tar:
                    entry = tarfile.TarInfo(name)
                    tar.addfile(entry, io.BytesIO(b""))
                with self.assertRaisesRegex(ValueError, "Invalid archive entry"):
                    preview.verify_bundle(unsafe)


if __name__ == "__main__":
    unittest.main()
