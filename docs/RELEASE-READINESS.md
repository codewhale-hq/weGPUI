# Desktop-first public release

Decision: the first release targets **macOS, Windows and Linux**. iOS and Android
remain explicitly experimental. This scope was selected by the owner on October
24, 2026 (America/Los_Angeles). It does not waive desktop acceptance criteria.

“Ready to share” means a new developer can discover the project, understand its
limits, install a versioned dependency, run an example, find the API they need,
and rely on the supported behavior. A website is one part of that promise.

## 1. Correctness and accessibility

- [x] Enable Linux windowing and verify sampled native X11 interactions.
- [x] Add focused regression tests and stable/MSRV/strict-doc CI.
- [x] Implement the listed component source fixes: cell-button loading, panel focus,
  spinner IDs/artwork, list semantics, scoped radio IDs, form labels and vcode restart.
  Sampled Linux/macOS/Windows evidence is linked from STATUS and ACCEPTANCE; full
  cross-platform state and screen-reader acceptance remains open below.
- [x] Align explicit and shorthand inside-list constructors; constructor and
  nested-marker contracts pass. Visual/screen-reader acceptance is separate.
- [ ] Exercise enabled/disabled/loading/error states and keyboard operation for
  each interactive family. Preserve failed attempts and specify the tested SHA.
  Tracked per module in [ACCEPTANCE](ACCEPTANCE.md). October 25 adds reviewed native
  dark/care/brand startup states; the pinned reference fetch remains blocked, so paired
  parity is still open.
- [ ] Verify modal isolation, names, values and focus restoration with an actual
  screen reader on each supported desktop OS. Tree snapshots alone do not pass. Actual
  Orca samples now exist. Approved local compatibility fixes repair the sampled cache/reopen
  failure; audio listening review and complete reader acceptance remain open. See
  [original receipt](receipts/orca-20261025/README.md),
  `receipts/linux-bridge-20261025/` and `receipts/input-contracts-20261025/`.
- [x] Apply the owner-approved modal action/cache and X11 keyboard-grab focus fixes.
  Combined Rust 1.98.1 gates pass with 47 root tests/one ignored doctest; seven saved-action
  timing cases and fixed cached/live controls pass
  ([guard receipts](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/)).
  Actual X11 focus-mode, Fcitx, Orca disabled-icon and nested-Weston samples pass
  ([focus receipts](receipts/x11-grab-focus-20261025/README.md)). These close their bounded
  reproduced failures, while the original failures remain preserved.
- [x] Verify six extended raw-caller/listener/epoch/passive/ancestor/two-window native modes
  in the fixed-guard receipt. Callback, restored-focus and tree controls pass; queued AT
  request success alone is not treated as guard acceptance.
- [ ] Qualify cache performance and the full ordering/reader/host matrix. The 47 root tests
  do not run the two new GPUI vendor unit tests; their separate attempt stops before execution
  with 21 missing upstream test-asset/optional-dependency errors
  ([attempt](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/vendor-unit-attempt.log)).
  The extracted NodeBuilder harness passes two supplemental metadata regressions with
  existing locked rlibs ([scope](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/metadata-extraction.json));
  this does not establish a full vendor-suite or 49-root-test pass.
- [ ] Verify input selection, clipboard, undo/redo, UTF-16 limits and IME composition.
- [ ] Reconcile public rustdoc with observed behavior; document intentional differences.

## 2. Desktop acceptance

Use the acceptance matrix in PLATFORMS; each result needs source/host versions.

- [ ] macOS: current native rendering, input, resizing, IME and accessibility on the
  authorized local Apple host. VoiceOver stays off; macOS screen-reader acceptance
  remains incomplete. Hosted compilation is not runtime acceptance.
- [ ] Windows: native MSVC build (CI configured; result pending) followed by a real Windows window, rendering at
  multiple scales, keyboard/input/IME and Narrator or NVDA.
- [ ] Linux: complete X11/Wayland, resizing/scaling, IME and Orca qualification. Nested
  Weston runtime and partial Orca samples now exist; hardware desktop and complete matrix open.
- [ ] Package a runnable gallery for the supported desktop platforms and verify
  fresh-machine installation. Define signing/notarization requirements before release.

Local macOS work and the existing Windows VM are authorized for this continuation.
Infrastructure purchases and public package publication still require approval.
VoiceOver remains off under the owner’s existing instruction.

## 3. Developer experience and distribution

- [ ] Confirm the crates.io package name, versioning and dependency-upgrade policy.
- [ ] Test the README installation flow in a new consumer project with locked pins.
  Linux X11 pass on 2026-10-24 ([receipt](receipts/release-gates-20261024/README.md));
  current source requires all four approved root Cargo patches in a new consumer.
  The matching patched GPUI is a compilation requirement: Semantics/Modal calls the added
  `Window::with_a11y_action_guard` API. The paths are `vendor/gpui_pre`,
  `vendor/gpui_pre_linux`, `vendor/accesskit_unix` and `vendor/accesskit_atspi_common`
  in the matching checkout. Dependency-manifest patches are not inherited; see
  [vendor guidance](../vendor/README.md).
  macOS and Windows consumers still needed.
- [ ] Ship complete small examples: application setup, forms, controlled state,
  overlays/focus, theming and application-owned upload handling.
- [ ] Generate versioned API documentation from the release source.
- [ ] Verify cargo package contents, licenses/attribution and documentation assets.
  `cargo package` list and verify build pass on Linux at `f5fe0b6`; README screenshots stay
  repository-relative (see STATUS).
- [ ] Prepare release notes, compatibility table and migration notes.
- [ ] Select maintainers and working security/conduct contact routes.
- [ ] Publish the versioned crate and signed/downloadable examples only after the
  release candidate and publication details have been reviewed.

## 4. Public website

The website source belongs in this repository. It must offer:

- A clear explanation of native Rust/GPUI components and their WeUI origin.
- A searchable catalog of all 60 gallery routes and their recorded states.
- Light/dark native captures, source links and reproducible gallery commands.
- Honest capture provenance; screenshots are not an interactive native web port.
- Getting started, API docs, theming tokens, examples and contribution guidance.
- A current platform/support matrix and a visible release roadmap.
- Accessible keyboard navigation, responsive layouts, useful empty/error states,
  tested links and no required analytics/accounts for reading the docs.
- A stable public URL and a repeatable deployment from the reviewed repository.

- [x] Build the interactive browser gallery: 60 live routes and 132 state presets using
  pinned WeUI 2.6.26 / WeUI.js 1.2.17, with local-only upload previews and a Python-only
  normal build from checked-in assets.
- [x] Verify full `final-candidate1-rerun` at site root and `/weGPUI/`: 40 browser checks,
  including 360 route/viewport observations ([receipts](receipts/web-interactive-20261025/)).
- [x] Verify final candidate 3's three affected-surface checks per mount and complete legacy
  suite at both mounts; each legacy run checks 264 historical state/theme views and 528
  decoded images. Theme/native-theme pages pass at 375/390/768/1280px with palette retention
  and live iframe pointer/Space interaction. The final 390px theme capture was reviewed.
  Candidate 3 changes only the palette handler and CSS minimum width relative to candidate 1;
  the full 40-check result remains attributed to candidate 1.
- [ ] Qualify the live deployed host, additional browser engines and actual screen-reader
  behavior on the website. Local Chromium checks do not establish these results.
- [x] Prepare the manual-only Pages workflow and [hosting steps](../website/HOSTING.md).
- [x] Publish the owner's reviewed source on GitHub Pages. The owner configured the source
  and dispatched [run 40211664701](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701)
  for `e5b6996`; build and deploy succeeded and returned <https://codewhale-hq.github.io/weGPUI/>.
  [The receipt](receipts/pages-publication-20261025/README.md) distinguishes this GitHub
  deployment result from public-origin browser qualification, blocked by the cloud proxy.
  Further dispatches/publications still require owner authorization; source pushes do not deploy.

The browser uses a separate HTML/CSS/JavaScript implementation. Historical native captures
remain available with `?view=native`, and native setup at `#native-guide`. Browser checks do
not establish native GPUI acceptance; all 40 component rows remain unaccepted. The separately
verified native batch is committed as `f7d41a534b02c15c6dbf3d0b216c9becc3925bcd`.

## 5. Experimental mobile work

The `gpui-pre-ios =0.3.9` / core `=0.3.9` pair fails to compile because of missing
upstream APIs. Keep that result visible in an experimental CI workflow and the
[upstream report](proposals/gpui-ios-039-incompatibility.md). Do not silently patch
the Cargo registry or introduce an unapproved GPUI fork. Android needs an upstream
platform backend. Neither mobile platform blocks the desktop-first release, but
neither may be described as supported.

## Release gate

The desktop release is ready only when sections 1–3 have evidence for the promised
scope, the website passes its checks, and publication details are settled. Until
then the project and site should say **developer preview**, not production-ready.
There is no honest “everything is finished forever” milestone: subsequent GPUI
upgrades and OS releases still require compatibility maintenance.
