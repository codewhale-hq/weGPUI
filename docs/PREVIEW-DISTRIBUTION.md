# Reproducible desktop preview source bundle

Current weGPUI needs four matching Cargo compatibility copies. A dependency's `[patch]` table
is not inherited, so a standalone crate archive does not provide a complete consumer setup.
`scripts/package_preview.py` builds a local source bundle containing the library, assets,
examples, all four vendor copies with their provenance/licences, and a separate consumer under
a workspace that applies the patches at its root.

This is a developer preview source distribution. It is not a crates.io publication, an
installer, a signed binary or evidence that a desktop platform has completed acceptance.

## Build and inspect

Use Python 3.11 or newer:

```sh
python3 -I scripts/package_preview.py --out target/preview
python3 -I scripts/package_preview.py --verify target/preview/wegpui-preview-<source-hash>.tar.gz
```

The filename is derived from the exact selected source bytes. Rebuilding identical source
with the same base revision produces identical archive bytes, independent of checkout location
and file timestamps. An existing archive with different provenance is rejected rather than
overwritten; choose a fresh output directory when changing only the revision label.
The adjacent `.sha256` file hashes the complete archive. `CHECKSUMS.json` hashes every bundled
file; `SOURCE.json` records the library's exact file hashes and base revision. A revision does
not imply the working tree was clean: uncommitted source is included as it exists on disk.
Verify the review source before sharing a bundle. A held candidate is not approved for landing
or publication merely because it packages successfully.

`--source DIR` packages an explicit source snapshot, and `--revision REV` labels its base.
The completion receipt uses a snapshot of canonical main plus only the independent continuation;
the held logical-wrap prototype is excluded from that artifact. Registry downloads and build
caches, environment files outside the selected source directories, Git metadata, historical
screenshots and receipts are not bundled. No network service or credential is needed to create
or inspect an archive.

## Run a new consumer

Extract the archive into an empty directory. From its top-level directory:

```sh
cargo build -p wegpui-preview --locked
cargo run -p wegpui-preview --locked
cargo test -p wegpui --locked
```

The new consumer has a path dependency on `wegpui` and uses the root patches for `gpui-pre`,
`gpui-pre-linux`, `accesskit_unix` and `accesskit_atspi_common`. Its lockfile adds only the
consumer package to the supplied locked graph. Building with `--locked` verifies that graph;
no dependency version is changed to make a build pass. With the registry sources already
cached, append `--offline` to the commands above. A first build needs registry access plus
the operating system's [GPUI system libraries](../CONTRIBUTING.md).

The application shows a validated name and bounded notes field, controlled agreement and
notification state, live light/dark switching, an application-owned image chooser with a
three-image limit, preview/deletion, and a confirmation dialog. Draft state remains in memory;
the example sends no data or writes selected images. A working desktop portal/chooser is
needed for Linux file selection. A failed or cancelled chooser preserves the existing images.

The source bundle retains `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-WEUI`, `NOTICE`, the
changelog, code of conduct and security policy, along with the vendor copies' licence and
provenance files. Historical captures and receipts remain
in the canonical GitHub repository.

## Release work still required

Linux consumer compilation, tests and native samples do not qualify macOS, Windows, a fresh
machine, signing, hardware rendering or a screen reader. Public crate naming/versioning,
distribution of the patched dependency graph, installers and publication remain maintainer
decisions. See [the remaining-work checklist](COMPLETION.md) and
[desktop release gates](RELEASE-READINESS.md).
