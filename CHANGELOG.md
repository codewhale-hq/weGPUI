# Changelog

All notable changes to weGPUI are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Version numbers will follow semantic
versioning once a release policy is decided. No version has been published.

## [0.1.0] - Unreleased

### Added

- `components::date_field::{DateField, DateFieldMode, DateValue, DateFieldEvent}`: reusable
  date and date-time fields (HTML `date`/`datetime-local` equivalent) built on `Picker`,
  with value/placeholder/disabled/year-range API, Change/Cancel events, Enter/Space opening,
  focus restoration and reopen-at-current-value. The gallery input page uses it.
- `docs/ACCEPTANCE.md`: per-component acceptance inventory.
- `scrolling::{FocusScroll, FocusTarget, RevealFocus}` keep keyboard focus visible in ordinary
  vertical scrollers: a focus change scrolls the minimum needed, visible targets and manual
  scrolling are left alone. Built-in focusable controls register as targets; the gallery page
  frame, `BottomFixedPage` and `component_states` use the scope.
- Gallery input page date (`日期`) and datetime (`时间`) fields open native pickers: a
  1900–2100 date cascade, then an hour/minute sheet for datetime. Only the final confirm writes
  the value; cancel keeps the previous value; focus returns to the field.

### Fixed

- Information-bar link and close focus indicators use the bar text colour (2px), visible on
  every bar colour.
- `TextInput`/`TextArea` undo: paste and cut are separate undo steps, and moving the caret ends
  typing coalescing.
- Primary and warn buttons show a visible outer brand focus ring, not only a white inner border.
- Accessible set positions were one too high (AccessKit positions are zero-based) for radios,
  select options, article lists, form-preview items and list tips.
- Bottom-fixed pages reveal focused fields above the toolbar fade (`FocusScroll::inset_bottom`).
- Picker arrow/Page/Home/End keys pressed during a snap animation no longer panic debug builds.

- `FormCell::label_for` associates a visible label with a text input and its row-focus target.
- `component_states` example demonstrates loading/disabled actions, associated form labels,
  independent radio choices and a verification-code countdown.
- Website checks cover every recorded state in both themes and decode all 528 capture assets.

- Final GitHub CI configuration with Linux/MSRV checks, mobile library type-checks, and
  Apple build-only jobs. Hosted results remain separate from runtime acceptance.

- Approved GPUI 0.3.9 dependency path, target-specific iOS example entry points, and an
  evidence-preserving simulator bundle script. Apple build/runtime acceptance remains pending.

- Seven focused regression tests for UTF-16 offsets, disabled picker defaults, Gregorian
  month lengths, and slider snapping/invalid bounds and steps.

- Enabled the approved Linux X11/Wayland windowing features for both examples; isolated
  X11 build, rendering, input and AT-SPI receipts retain failed probe attempts.

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

- Inline-link accessibility targets use shaped, wrapped text bounds. Pointer clicks focus the
  actual clicked link, and assistive activation uses its handler directly.

- Cell-style buttons honor loading state, show a spinner and reject repeat activation.
- Verification-code countdowns start at their full duration and ignore duplicate starts.
- Clickable small-app panels retain a visible keyboard-focus ring without a layout border.
- Radio rows include their group ID; loading indicators and tip/preview lists use callsite
  defaults with explicit IDs available for repeated helper/loop instances.
- Standard and primary loading indicators use their corresponding upstream artwork.
- Navigation arrows wrap within their own tab bar, with only the selected tab in Tab order.
- Dialog headings/body text and individual form-preview/tip rows expose semantic nodes.
- Website API documentation resolves Cargo’s configured target directory or an explicit path.

- macOS AX dumps include raw `AXTitle`, retaining picker/select option names that System Events
  does not return through its `name` property.

- Evidence scripts fail on capture/traversal errors and invalid image dimensions. X11 capture
  restricts its window search to the launched PID; contact sheets validate their arguments.

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

See `docs/STATUS.md`. Mobile platforms are not supported, Windows runtime and screen-reader evidence are missing,
and visual/behavioural parity is
sampled rather than complete.
