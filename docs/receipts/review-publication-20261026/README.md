# Final publication review — 26 October 2026

This receipt qualifies the final developer-preview publication candidate. It follows the
[complete change-set review](../../REVIEW-2026-10-26.md), including the owner's latest request
to commit all existing work to canonical GitHub. Earlier landing-hold decisions remain in their
historical receipts; known RTL and platform acceptance gaps remain unresolved.

No new Rust dependency, package version, logical RTL/caret implementation, release or website
deployment was introduced by the review. The four unchanged prototype vendor files still match
the supplied source. `line_layout.rs` retains only its approved trailing-space extension and
`line_wrapper.rs` its approved CJK classifier extension.

## Final source and captures

Gallery SHA-256: `7911fdd09ee94bbf34ac50deec1d1be027edbf05fd175c4ca2e69257e4426bf5`.
The [manifest](metadata/captures/manifest.json) records the final 305 Rust/manifests files and
pinned upstream identities. [Verification](metadata/capture-verification.json) finds no source
or image mismatch, confirms the frozen and regular binary match, and verifies all **2,112 PNGs
in 1,056 pairs**. Every capture succeeds and no reference page error occurs.

The previous continuation differs in exactly one Rust file, `src/components/footer.rs`.
[The A/B summary](metadata/capture-ab.json) finds **1,055 native images byte-identical** and
1,025 browser images byte-identical. The one native difference is the `form_vcode:countdown`
59→60 tick, bounded by device pixels (382,618)–(416,642). The 31 browser differences occur in
Button default, Loading, Loadmore and Toast loading spinner phases. All changed-image crops
were inspected; `visual-review/` retains the crops and coordinates. The single-row gallery
footer appearance is unchanged by the wrapped-row correction.

This reviews the publication delta. It does not accept all remaining native/reference
baseline, typography, behaviour or platform differences. Full raw corpora, the read-only frozen
runner and the generated offline comparison viewer remain generated evidence outside Git.
Selected pairs, manifests and hashes are retained here. The viewer's real Chromium smoke check
passes eight appearances, 132 scenes, image decoding, navigation and a 75% reveal without page
errors ([result](metadata/viewer-check.json)).

## Review fixes and native checks

The original 90px-wide Footer fixture exposed overlapping 45×44 link targets: Help began at
row 372, Terms at row 392, producing 1,080 square pixels of overlap. The corrected Terms target
begins at row 416; overlap is zero. All eight light/dark/care/brand combinations pass **64 edge
clicks and 32 Enter/Space activations**, with exact per-link callback counts. Source, binary/file
identities, before/after source, screenshots, drivers and per-mode results are in `footer/`.
Measurement-only text is now explicitly hidden from accessibility, and the link scopes are stable.

The intermediate attempt to apply a role to a stateless Div failed compilation; a later generic
role was rejected by GPUI at runtime. Both are retained failures. The final named hidden
Paragraph implementation builds and all eight modes pass. No failed attempt is counted as a pass.

Preview verification now rejects portable-archive Windows path escapes. Preview source bundles
include the changelog and project policies. Existing archive content is preserved if a different
base-revision label would otherwise overwrite the same source-derived filename. Regression
checks preserve both the original archive and checksum on rejection.

Capture records now validate canonical identifiers against route/state/appearance metadata.
Reference page errors preserve their evidence but fail the record and command. A deliberately
broken reference fixture throws `reference-error-control` in real Chromium: the command exits
1 with one failed record, as required ([verification](metadata/reference-error-verification.json)).
`negative-control/` retains its manifest, summary and result. It is not an official reference
or a successful visual comparison.

## README consumer

A separate application uses the rewritten README's exact Cargo snippet, all four workspace-root
patches and copies of `minimal.rs` plus its platform helper. After generating the application's
own lockfile, `cargo build --locked --offline` passes. [Lock comparison](metadata/readme-consumer-lock.json)
shows no new third-party package or changed shared version; two root-only development packages
are omitted. The initial attempt seeded the full repository lock and was correctly rejected
under `--locked`; that output is retained. The repository's own lockfile was not changed.

Real X11 pointer/keyboard and independent AT-SPI checks pass: named modal opening, page hiding,
Tab and Shift+Tab containment, confirmation 0→1, restored opener focus, Space reopening,
Escape preserving 1, and pointer reopening/confirming 1→2. `consumer/` retains the exact source,
manifest/lock, binary hash, driver, screenshot and result. This is native consumer evidence,
not reader speech, a fresh-machine installer or other-host acceptance.

## Checks

- Five Rust gates pass: fmt, locked/offline all-target build, 59 library tests with three ignored
  doc examples, strict Clippy and strict rustdoc. Three gallery tests pass separately.
- The four extracted text harnesses pass seven, 16+4, eight and thirteen cases respectively.
  The trailing-space and CJK negative controls retain their expected three failures each.
- After the final Python-only packaging fix, all **33 script tests and five website tests** pass.
- CI YAML parses, desktop jobs include gallery tests, and Linux includes all four extracted
  harnesses with explicit output directories. No hosted run is represented as a local result.
- The existing historical Footer/Layers/SearchBar/Orca receipts retain their earlier source
  identities. Their full results are not retrospectively attributed to this later Footer source.

`logs/` retains commands and outputs. `review-fixes.patch` isolates the review's production,
tooling, regression and CI fixes against the source backed up before the review. README/live
policy changes and all prior work appear in the actual Git change set.

Two full-sweep attempts exhausted artifact/build storage before producing a summary. Their
partial records and errors are retained in `failures/`, and the independent consumer's initial
storage failure is retained in `logs/`. Superseded generated caches were reclaimed while the
active toolchain bytes, source and prior immutable evidence were preserved. The final sweep and
consumer build both complete. No failed storage attempt is used as validation.

## Remaining limits

Linux RTL paragraph clipping/misalignment, step 3, complete Unicode/localisation review, full
component/motion/reader qualification, human speech listening, current-source macOS/Windows
runtime, hardware diversity, signing and distribution remain open. No component or platform is
promoted to supported. Package publication, website deployment and manual workflow dispatch
remain outside this repository-publication instruction.

Publication hygiene: all 170 command logs in the new receipts/proposals were reviewed and
explicitly selected despite the general log ignore rule. Two logs needed local-path redaction;
the original copies remain outside Git. Public checksums identify the redacted copies.
[The preview manifest](metadata/preview-bundle.json) records the regenerated verified source
bundle; its bytes remain generated evidence outside Git.
