# Ordinary source follow-up API refresh — 2026-10-25

The local website was rebuilt from the root's current strict Rustdoc output after the
ordinary follow-up changes to Input cancellation, Agree/Semantics invalid state,
SwipedCell Escape documentation/behavior and the responsive gallery article. The root
reported all five gates and 47 tests passing before this refresh. This collector ran
no Cargo command and changed no website, library, vendor or manifest source.

```sh
python3 website/build.py --include-api --api-dir /workspace/.wegpui-cloud/target-198/doc
```

`source-sha256.txt` identifies Cargo files, current library Rust source, the responsive
gallery article source and website source. `api-sha256.txt` identifies all 1,014 copied
API files, totaling 18,272,754 bytes. `provenance.json` records the timestamp and manifest
hashes. Observed HEAD `49268de7bb78a69cb56b96f6cff02a2d9284e71d` is contextual, with
`source_dirty: true`; these checks identify a source/API snapshot, not a final commit.

A temporary loopback-only server exposed `website/dist` on port 4180. Chromium
154.0.8037.57 verified that the new `Semantics::invalid` API page returns HTTP 200 and
contains its method anchor. It also requested the Rustdoc source pages for:

- `src/input.rs`
- `src/components/agree.rs`
- `src/components/swiped.rs`
- `src/accessibility.rs`

All four source pages returned HTTP 200. After stripping only Rustdoc line-number
anchors, each complete decoded source text matched the current Rust file exactly.
`http-checks.json` records each source/served-HTML SHA-256 and the new method-anchor
result; `http-checks.txt` records the passing output. The server was stopped afterward.

The website source and existing public anchors were otherwise unchanged. The earlier
[complete browser suite](../../component-contracts-20261025/website/final-api/README.md)
was not repeated; this refresh checks the changed API/source output. Native website
images remain historical macOS captures from `2c8ed10`. No native capture was refreshed,
and no runtime, screen-reader, deployment, publication or final release qualification
is implied. The unapplied X11 focus-mode and modal-guard compatibility proposals remain
separate from this ordinary source/API result.
