"""Build or verify a reproducible desktop preview source bundle with its Cargo patches."""

import argparse
import gzip
import hashlib
import io
import json
import pathlib
import subprocess
import tarfile
import tomllib


ROOT = pathlib.Path(__file__).resolve().parent.parent
VENDORS = ("gpui_pre", "gpui_pre_linux", "accesskit_unix", "accesskit_atspi_common")
FILES = ("Cargo.toml", "Cargo.lock", "README.md", "CONTRIBUTING.md", "CHANGELOG.md", "CODE_OF_CONDUCT.md", "SECURITY.md", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE-WEUI", "LICENSE-UNICODE-3", "NOTICE")
DIRECTORIES = ("src", "assets", "examples", "vendor", "scripts", "docs")
OMIT = {".git", "target", "__pycache__", ".DS_Store"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def json_bytes(value):
    return (json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()


def source_files(source):
    for name in FILES:
        if not (source / name).is_file():
            raise ValueError(f"Missing source file: {name}")
    for name in VENDORS:
        if not (source / "vendor" / name / "Cargo.toml").is_file():
            raise ValueError(f"Missing required vendor copy: {name}")
    selected = [source / name for name in FILES]
    for directory in DIRECTORIES:
        selected.extend((source / directory).rglob("*"))
    result = {}
    for path in sorted(selected):
        relative = path.relative_to(source)
        if relative.parts[0] == "docs" and len(relative.parts) > 1 and relative.parts[1] in {"receipts", "parity"}:
            continue
        if any(part in OMIT or part.startswith("._") for part in relative.parts):
            continue
        if path.is_symlink():
            raise ValueError(f"Source symlinks are not supported: {relative}")
        if path.is_file():
            result[relative.as_posix()] = path.read_bytes()
    return result


def bundle_files(source, revision):
    files = source_files(source)
    required_example = "examples/application.rs"
    if required_example not in files:
        raise ValueError(f"Missing source file: {required_example}")
    source_hashes = {name: digest(data) for name, data in files.items()}
    identity = digest(json_bytes(source_hashes))
    package = tomllib.loads(files["Cargo.toml"].decode())["package"]
    workspace = '''[workspace]
members = ["wegpui", "consumer"]
resolver = "3"

[patch.crates-io]
gpui-pre = { path = "wegpui/vendor/gpui_pre" }
gpui-pre-linux = { path = "wegpui/vendor/gpui_pre_linux" }
accesskit_unix = { path = "wegpui/vendor/accesskit_unix" }
accesskit_atspi_common = { path = "wegpui/vendor/accesskit_atspi_common" }
'''
    consumer = '''[package]
name = "wegpui-preview"
version = "0.0.0"
edition = "2024"
rust-version = "1.95"
publish = false

[dependencies]
wegpui = { path = "../wegpui" }
gpui = { package = "gpui-pre", version = "=0.3.9" }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.9", features = ["font-kit", "wayland", "x11"] }
'''
    lock = files["Cargo.lock"]
    if any(p["name"] == "wegpui-preview" for p in tomllib.loads(lock.decode())["package"]):
        raise ValueError("Source lockfile already contains the preview consumer")
    lock += b'''\n[[package]]
name = "wegpui-preview"
version = "0.0.0"
dependencies = [
 "gpui-pre",
 "gpui-pre-platform",
 "wegpui",
]
'''
    result = {f"wegpui/{name}": data for name, data in files.items()}
    result.update({
        "Cargo.toml": workspace.encode(),
        "Cargo.lock": lock,
        "consumer/Cargo.toml": consumer.encode(),
        "consumer/src/main.rs": files[required_example],
        "consumer/src/support/platform.rs": files["examples/support/platform.rs"],
        "SOURCE.json": json_bytes({
            "format": 1,
            "package": package["name"],
            "version": package["version"],
            "base_revision": revision,
            "source_sha256": identity,
            "files": source_hashes,
        }),
        "README.md": b'''# weGPUI desktop developer preview

This is a source bundle, not a signed installer or a published crate. It includes all four
required same-version vendor copies and a separate consumer with workspace-root Cargo patches.
SOURCE.json identifies the exact on-disk source, including uncommitted changes; base_revision
identifies its base and does not imply a clean checkout or platform acceptance.

Install Rust 1.95+ and the Linux system libraries described in wegpui/README.md, then run:

```sh
cargo build -p wegpui-preview --locked
cargo run -p wegpui-preview --locked
cargo test -p wegpui --locked
```

The application demonstrates validation, controlled state, light/dark switching, a focus-managed
confirmation dialog and local image selection/preview/deletion. It sends no data. Files and
drafts remain application-owned; this example keeps its state in memory.

Cargo needs registry access on the first build or an existing dependency cache. With that cache,
append --offline. Upstream licenses/provenance and the library's licenses/NOTICE are retained.
Historical screenshots and receipts stay in the canonical GitHub repository. macOS/Windows
runtime, signing, hardware and screen-reader acceptance remain separate. Mobile is experimental.
''',
    })
    result["CHECKSUMS.json"] = json_bytes({name: digest(data) for name, data in result.items()})
    return identity, result


def create_bundle(source, output, revision):
    identity, files = bundle_files(source, revision)
    prefix = f"wegpui-preview-{identity[:12]}"
    output.mkdir(parents=True, exist_ok=True)
    archive = output / f"{prefix}.tar.gz"
    temporary = archive.with_suffix(".tmp")
    try:
        with temporary.open("wb") as raw:
            with gzip.GzipFile(fileobj=raw, mode="wb", filename="", mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as tar:
                    for name, data in sorted(files.items()):
                        info = tarfile.TarInfo(f"{prefix}/{name}")
                        info.size = len(data)
                        info.mode = 0o755 if name.endswith(".sh") else 0o644
                        tar.addfile(info, io.BytesIO(data))
        verify_bundle(temporary)
        if archive.exists():
            if archive.read_bytes() != temporary.read_bytes():
                raise ValueError("Archive already exists with different provenance; choose another output directory")
        else:
            temporary.replace(archive)
    finally:
        temporary.unlink(missing_ok=True)
    archive.with_name(archive.name + ".sha256").write_text(
        f"{digest(archive.read_bytes())}  {archive.name}\n", encoding="utf-8"
    )
    return archive


def verify_bundle(archive):
    with tarfile.open(archive) as tar:
        files = {}
        prefixes = set()
        for entry in tar:
            name = pathlib.PurePosixPath(entry.name)
            if (not entry.isfile() or name.is_absolute() or ".." in name.parts
                    or "\\" in entry.name or ":" in entry.name
                    or len(name.parts) < 2 or entry.name != name.as_posix()):
                raise ValueError(f"Invalid archive entry: {entry.name}")
            prefixes.add(name.parts[0])
            relative = pathlib.PurePosixPath(*name.parts[1:]).as_posix()
            if relative in files:
                raise ValueError(f"Duplicate archive entry: {relative}")
            files[relative] = tar.extractfile(entry).read()
        if len(prefixes) != 1:
            raise ValueError("Expected one archive root")
        checksums = json.loads(files.pop("CHECKSUMS.json"))
        if checksums != {name: digest(data) for name, data in files.items()}:
            raise ValueError("Archive contents do not match CHECKSUMS.json")
        source = json.loads(files["SOURCE.json"])
        hashes = source["files"]
        if digest(json_bytes(hashes)) != source["source_sha256"]:
            raise ValueError("Source identity does not match its file manifest")
        actual_source = {name.removeprefix("wegpui/"): digest(data)
                         for name, data in files.items() if name.startswith("wegpui/")}
        if hashes != actual_source:
            raise ValueError("Bundled library differs from SOURCE.json")
        for vendor in VENDORS:
            if f"wegpui/vendor/{vendor}/Cargo.toml" not in files:
                raise ValueError(f"Missing vendor copy: {vendor}")
    return source["source_sha256"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=pathlib.Path, default=ROOT)
    parser.add_argument("--out", type=pathlib.Path, default=ROOT / "target/preview")
    parser.add_argument("--revision", help="Base revision; exact contents are hashed separately")
    parser.add_argument("--verify", type=pathlib.Path, metavar="ARCHIVE")
    args = parser.parse_args()
    try:
        if args.verify:
            print(f"Verified source SHA-256: {verify_bundle(args.verify)}")
        else:
            revision = args.revision
            if revision is None:
                result = subprocess.run(["git", "rev-parse", "HEAD"], cwd=args.source,
                                        capture_output=True, text=True, check=False)
                revision = result.stdout.strip() if result.returncode == 0 else "unversioned"
            print(create_bundle(args.source.resolve(), args.out.resolve(), revision))
    except (ValueError, KeyError, OSError, tarfile.TarError) as error:
        parser.exit(1, f"Preview bundle failed: {error}\n")


if __name__ == "__main__":
    main()
