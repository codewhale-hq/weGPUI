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
- [ ] Resolve the known component defects in STATUS: cell-button loading, focus
  indication for clickable panels, spinner IDs and variants,
  list semantics, radio-group IDs, form-label association and vcode restart.
- [x] Align explicit and shorthand inside-list constructors; constructor and
  nested-marker contracts pass. Visual/screen-reader acceptance is separate.
- [ ] Exercise enabled/disabled/loading/error states and keyboard operation for
  each interactive family. Preserve failed attempts and specify the tested SHA.
- [ ] Verify modal isolation, names, values and focus restoration with an actual
  screen reader on each supported desktop OS. Tree snapshots alone do not pass.
- [ ] Verify input selection, clipboard, undo/redo, UTF-16 limits and IME composition.
- [ ] Reconcile public rustdoc with observed behavior; document intentional differences.

## 2. Desktop acceptance

Use the acceptance matrix in PLATFORMS; each result needs source/host versions.

- [ ] macOS: current native rendering, input, resizing, IME and VoiceOver on a
  separately authorized Apple host. Hosted compilation is not runtime acceptance.
- [ ] Windows: native MSVC build followed by a real Windows window, rendering at
  multiple scales, keyboard/input/IME and Narrator or NVDA.
- [ ] Linux: X11 and Wayland runtime checks, resizing/scaling, IME and Orca.
- [ ] Package a runnable gallery for the supported desktop platforms and verify
  fresh-machine installation. Define signing/notarization requirements before release.

No personal Mac access, infrastructure purchase or public package publication is
implied by this checklist. The existing authorization for cloud work remains in force.

## 3. Developer experience and distribution

- [ ] Confirm the crates.io package name, versioning and dependency-upgrade policy.
- [ ] Test the README installation flow in a new consumer project with locked pins.
- [ ] Ship complete small examples: application setup, forms, controlled state,
  overlays/focus, theming and application-owned upload handling.
- [ ] Generate versioned API documentation from the release source.
- [ ] Verify cargo package contents, licenses/attribution and documentation assets.
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
