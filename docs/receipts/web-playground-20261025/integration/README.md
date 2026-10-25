# Interactive website integration — 2026-10-25

This gallery runs separate HTML/CSS/JavaScript WeUI examples. It does not run
Rust GPUI in the browser or establish native component acceptance.

The main shell defaults to live demonstrations, with 60 routes and 132 state
presets; native screenshots remain under `?view=native`, and native setup at
`#native-guide`. Theme, care, custom brand and preview width persist between
components. Appearance changes preserve the actual input DOM/draft/focus.
Native-source links still describe the exact build checkout and dirty status.

The normal Python build verifies and copies checked-in pinned assets without
Cargo, npm or network access. Optional prebuilt Rust API documentation is a
separate input. Source and package provenance is in
`website/playground/vendor/PROVENANCE.json`; generated assets retain licenses.
The iframe blocks network connections and form submissions; file previews use
local object URLs. No account, upload service, analytics or SDK is configured.

## Review and checks

The four adjacent PNGs are the main-shell candidate1 at desktop/mobile sizes.
They were opened and visually reviewed: homepage, component controls, mobile
layout and live theme page. They precede the small native-theme navigation
correction and mobile theme-grid correction, whose before/after evidence is in
[the independent browser receipt](../../web-interactive-20261025/README.md).
That receipt qualifies the exact browser artifacts, both mounting paths,
preserved failures, final checks and source hashes. The
[bridge receipt](../bridge/README.md) separately qualifies 132 initialization
cases, six interaction checks and selected theme captures.

The full candidate1 suite passes 40 browser checks across both mounts, including
360 route/viewport observations. Candidate3 changes only the palette navigation
handler and theme-grid minimum width. Its targeted themes, shell and theme-page
layout checks all pass at both mounts; the latter covers 375/390/768/1280px,
palette-route retention and actual pointer/Space use inside the live preview.
The legacy checker also passes all 264 historical state/theme views and decodes
528 images at each mount. Candidate3 manifests prove unchanged remaining files;
the full route suite is not represented as rerun against those two small fixes.
The final 390px theme capture was opened and reviewed after the grid correction.

`source-sha256.json` identifies the final authored integration files.
`environment.json` records the actual browser, Playwright, Python, Node and OS versions.
JavaScript syntax checks passed for app.js, bridge.js and router.js; Python
compile checks passed for the builder, both browser checkers and vendor generator.

[gates.log](gates.log) records the five required Rust 1.98.1 checks before this
website commit: fmt, locked/offline all-target build, 47 unit tests, one ignored
doctest, strict all-target Clippy and strict rustdoc. The native code remains
that of `f7d41a534b02c15c6dbf3d0b216c9becc3925bcd`.
The [final rerun](gates-final.log) after the theme-grid correction passes again.
Both use the same ordered commands and container environment documented in the
[native final-gate receipt](../../overlay-contracts-20261025/action-guard-fixed-20261025/README.md).

## Hosting status

The manual-only GitHub Pages workflow is prepared with pinned actions, a
main-only build, and limited deployment permissions. No Pages setting, CI
dispatch or deployment was performed. The expected default destination is
`https://codewhale-hq.github.io/weGPUI/`; this is not a verified live URL.
See [hosting instructions](../../../../website/HOSTING.md).

Publication remains a final owner action under the original takeover
instruction against deployment/CI dispatch without authorization. The available
GitHub connector cannot enable Pages or dispatch Actions, shell GitHub API
authentication is unavailable, and no connected local-browser tool is exposed.
Routine source pushes are authorized. No Cloudflare account or project was
created. All 40 native component rows remain unaccepted; browser reader,
Safari/Firefox, touch hardware and the deployed origin remain unqualified.
