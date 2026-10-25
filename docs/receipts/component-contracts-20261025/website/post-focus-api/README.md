# Post-review API refresh — 2026-10-25

The website was rebuilt once more from the root-produced strict Rustdoc tree after the
final unchecked-checkbox focus overlay and black/white caret changes. The root reported
all five source gates and 47 tests passing before this refresh. This collector invoked no
Cargo commands, made no source changes and performed no deployment.

```sh
python3 website/build.py --include-api --api-dir /workspace/.wegpui-cloud/target-198/doc
```

`source-sha256.txt` identifies the Cargo, Rust and website source snapshot.
`api-sha256.txt` identifies all 888 copied API files, totaling 17,201,769 bytes.
The 119 additional paths relative to `../final-api/` are Rustdoc search-index shards.
`provenance.json` records both manifest hashes and the timestamp. Observed HEAD
`a3ce3afca6417b7dd1b9d701456ae2d21d2150f4` remains contextual, with a dirty working tree;
this receipt does not attest a final commit hash.

A temporary loopback server exposed the copied API. Chromium requested both
`api/src/wegpui/components/checkbox.rs.html` and `api/src/wegpui/input.rs.html` and received
HTTP 200. The check cloned each Rustdoc code element, removed only line-number anchors
and compared its decoded text with the **entire current Rust source file**. Both matched
exactly. Thus the served source includes the empty overlay that avoids filling an
unchecked checkbox's transparent center and the black/white caret bands for unknown
host surfaces. `source-http-checks.json` records source and served-HTML SHA-256 values;
`source-http-checks.txt` records the passing result.

The existing full browser suite and current API-anchor checks passed in the immediately
preceding [API receipt](../final-api/README.md). The site and public API anchors did not
change, so that full suite was not repeated for this source refresh. The loopback server
was stopped afterward.

This verifies copied documentation/source freshness for the named changes. It is not a
native checkbox/caret rendering check, screen-reader result or release qualification.
Native website images remain historical macOS captures from `2c8ed10`.
