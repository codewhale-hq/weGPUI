# Changelog

All notable changes to weGPUI are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Version numbers will follow semantic
versioning once a release policy is decided. No version has been published.

## [0.1.0] - Unreleased

### Added

- WeUI 2.6.26 tokens generated from `dist/style/weui.css`: four 180-field palettes (light,
  dark, care-light, care-dark) with per-field rustdoc, plus 32 SVG icons and the example images.
- `Theme` global with system/light/dark modes, care mode and brand-colour override.
- Components for every WeUI widget family. The 61 upstream example files map to 60 gallery
  routes plus the index shell (`docs/UPSTREAM-COVERAGE.md`).
- `TextInput`/`TextArea` adapted from GPUI's input example, with IME marked-text support.
- Accessibility helpers: `Semantics` (disabled, busy, modal, hidden) and `ModalScope`, which
  hides application content from assistive technology while a modal is open.
- `Picker::can_confirm`. Confirm is disabled when any column has no enabled current item.
- Undo/redo in text inputs (ctrl/cmd-z, ctrl/cmd-shift-z, ctrl-y).
- `examples/minimal.rs`, a small consumer application.
- Tooling: `scripts/weui_upstream.py` (verified download/serve of the upstream example and its
  pinned weui.js/WeChat SDK scripts), an X11 path in `scripts/capture.py`, the Linux session and
  sweep helpers, `scripts/contact_sheets.py`, and a proposed Linux windowing patch.
- Contributor documentation: platform feasibility (`docs/PLATFORMS.md`), parity tooling guide,
  contributing guide, code of conduct, security policy, issue and PR templates.

### Changed

- `gpui-pre-platform` is now a dev-dependency. The library depends on `gpui` and
  `unicode-segmentation`, without a platform crate. The library type-checked for iOS and Android
  at the `0637c25` change; applications choose their own platform crate.
- Documentation distinguishes historical checks, source inspection and committed runtime
  receipts; corrected mobile gesture descriptions and contributor reproduction instructions.
- Text length limits count UTF-16 code units, as HTML `maxlength` does.
- `UploaderFile::src` and `Gallery::src` take `ImageSource` (asset, URI or file path).
- The gallery uploader opens the native file chooser instead of inserting a bundled image.
- Overlay transitions reverse smoothly when interrupted, play CSS `ease` forward when hiding, and
  slide by the panel's own height.
- `LinkText` gives every actionable link its own keyboard focus stop and accessibility node.
- `Cargo.toml` declares `rust-version = "1.95"` (measured), keywords, categories, and excludes
  parity screenshots, receipts and scripts from the package.

### Fixed

- `Slider` no longer panics on non-finite ranges. Invalid steps and values fall back to the
  HTML defaults.
- `SelectMenu` arrow keys stay within its options (Up/Down/Home/End stop at the ends).
- Pickers never select disabled items from remembered or default values.
- `Presence::hide` without an opener blurs a focused child inside the overlay.
- Closed swipe-cell actions are hidden from assistive technology. Article headings carry their
  level, and list items are list items.
- Navbar/tabbar arrow keys select the focused tab. Action-sheet child corners are rounded.
- Toasts no longer take focus or hide the page from assistive technology (`Presence::passive`).
  Toptips reverse smoothly when hidden mid-fade.
- Picker Home/End skip disabled edge rows. Slider values snap to the nearest step. Progress
  ignores NaN. Disabled cells drop the pressed highlight. The Agree shake respects reduced
  motion.

### Known gaps

See `docs/STATUS.md`. Mobile platforms are not supported, Linux windows need a pending feature
decision, Windows runtime and screen-reader evidence are missing, and visual/behavioural parity is
sampled rather than complete.
