# Radio option-set accessibility proof — 2026-10-28

The additive grouped-row design was evaluated in two isolated, copied weGPUI crates on Linux/X11. These are artifact probes, not tests of the final product checkout. The copied product radio, picker and select inputs matched the corresponding `d053fd2` blobs; each build manifest records 445 product/overlay/fixture input hashes, the build command and resulting binary hash. The final product helper was independently checked after formatting against the second overlay helper and matched exactly. Product gallery compilation and captures are separate.

Both probes retain `RadioGroup::rows` and render the options in a named, option-only RadioGroup with an explicit size; the decorative Add more action is a sibling. The first probe covered default FormGroup `(16,16)` and Cells `(16,0)` logical separator insets. Its LTR and RTL legacy/grouped screenshots were byte-identical, row/button accessibility bounds matched, radio options reported AT-SPI positions 1–3 and set size 3 including the disabled option, and decorative buttons had no inherited set size. Down-arrow selection skipped the disabled item, and both decorative actions fired.

The second probe changed the helper to return `None` for an empty group, letting callers omit the row. It covered empty and three-option cases in LTR and RTL with custom FormGroup `(32,16)` and Cells `(36,12)` insets. All eight live AT-SPI/action runs and four legacy-versus-grouped screenshot and accessible-bounds comparisons passed. The empty variant exposed no empty radio group and preserved the decorative actions. The complete outputs are in the local `radio-set-probe/` and `radio-empty-probe/` artifact directories.

The first execution of the second probe is **not** a pass: its final geometry comparison failed because the script saved an incomplete AT tree and separately validated a later complete tree. The failed snapshots, screenshots, logs and build manifest are retained in `radio-empty-probe/failed-attempt-01/`. The corrected checker validates the exact serialized snapshot and requires all expected controls and bounds before saving it; its bounded retry fails on timeout. The corrected, hash-bound build and complete rerun passed.

| Artifact | SHA-256 |
| --- | --- |
| First probe binary | `f0fdd5f36425f37e5e18d691c4a3cc924b19ba68ae3a296fbeab9e902ab2d243` |
| First probe patch | `3fe453113944993b43f9e37168721e3c8cfc4cfc2f991c4b6614444a7ec0f417` |
| First probe runtime log | `e51f4681923fd947da25e163f5b3a97a87561b2e77894a9b05bfc399c94a6041` |
| Empty-group probe binary | `3c0b8cb90d2985c1fd84c087974f46f85ae26e9a0cfcd4d9db99ac2c92d2b3ed` |
| Empty-group probe patch | `76484d2835cf1177c435f6bbb8beaec079e6096cccf37bed4275a07da13391e9` |
| Corrected AT-SPI checker | `4c6314fb30ba2e1bbe5c033266fe4d96d68036081d576e1b3fa7084bd994c9cf` |
| Empty-group runtime log | `a833a59f3e3a4dbc07eb697d22b8606a0969cfe94fe78544e9793557d5ccaf7d` |

Each probe was built with the shared build lock via its `./build.sh`, followed by `docker exec wegpui-rtl-qualification bash /work/full-parity/<probe>/run.sh`. Build and capture abort if the snapshotted product, overlay or binary changes.

These checks do not establish screen-reader announcement quality, Tab/Shift-Tab or horizontal RTL navigation, dynamic or nested groups, browser parity, Windows runtime, or macOS/iOS behavior. They do not establish broader weGPUI parity.

The final product diff was reviewed independently and its source hashes are in
`product-source.json`. On macOS ARM64, 54 library tests, three gallery tests
and strict all-target Clippy passed. The website inventory suite passed five
tests, and selected shell and theme layout browser checks passed at root and
project-prefix hosting. These checks do not change the scope limitations above.

The recorded patches are stored as gzip files to preserve their exact bytes, including diff-context whitespace.
