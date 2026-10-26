# Changelog

All notable changes to weGPUI are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Version numbers will follow semantic
versioning once a release policy is decided. No version has been published.

## [0.1.0] - Unreleased

### Changed (breaking default)

- **Library defaults are now English.** Every default label, accessible name, hint and validation
  message the library supplies comes from the active `I18n` catalog, and `I18n` starts as English.
  Applications that relied on the original WeUI wording (多选, 开关, 确定, 取消, 正在加载 …) call
  `I18n::set_locale(Locale::zh_hans(), cx)`, which reproduces it exactly. The date field's
  validation messages and placeholders were previously English-only or WeUI-browser style and
  are now localised with the rest.

### Added

- Pinned WeUI source/style inventory and a complete paired-capture worklist, a verified local
  UI reference, and a Linux capture runner covering themes, care, custom brand, states,
  responsive widths and scroll positions. Captures remain distinct from parity acceptance.

- `examples/application.rs`: a complete draft form with validation, controlled state, themes,
  modal confirmation and application-owned local image selection/preview/deletion.
- `scripts/package_preview.py`: reproducible, integrity-checked desktop preview source bundles
  containing a separate consumer, all four Cargo compatibility copies and project licences.

- `wegpui::i18n` (`Locale`, `Direction`, `Messages`, `I18n`, `fill`): BCP 47 locale tags,
  text direction inferred from script/language, runtime language switching,
  and 14 built-in catalogs (English, Simplified and Traditional Chinese, Japanese, Korean,
  Spanish, French, German, Portuguese, Russian, Arabic, Hebrew, Hindi, Thai; all but English and
  Simplified Chinese are starter translations awaiting native review). Any other language is
  supplied as a `Messages` value. See `docs/I18N.md`.
- `TextInput`/`TextArea` caret, click placement and selection painting follow bidirectional
  text on unwrapped lines containing right-to-left glyphs.
- Right-to-left layout: components read `I18n::direction` and mirror (rows, padding and margins,
  arrows, switch and slider travel, swipe actions and keys, steps, picker columns, half-screen
  header slots, forms, search, dialogs and more; the list is in `docs/I18N.md`).
  `wegpui::logical::Logical` gives application code the same `row`/`ps`/`pe`/`ms`/`me`/
  `inset_s`/`inset_e`/`text_start`/`text_end` vocabulary. The *Limits* section of the guide lists
  what is not mirrored.
- Gallery: opens in English (`--lang zh-Hans` reproduces the upstream wording and is what the
  parity scripts pass), `--lang <tag>`, `--dir ltr|rtl`, a Language menu, English text for every
  page next to the upstream Chinese, and a Languages page with samples in every built-in script,
  mixed-direction lines, the active catalog and editable fields with scripted selection states
  (`--state type:<field>:<ops>`).
- `scripts/i18n_sweep.py` captures every route per language and, where macOS Automation is
  allowed, scans the accessibility tree for Chinese that survived translation.
- Vendored `gpui-pre` (owner-approved 2026-10-25): `LineWrapper::is_word_char` treats Arabic,
  Hebrew, Devanagari and Greek as word characters, so wrapping and ellipsis truncation stop
  splitting words of those scripts. Thai and Hangul breaking are unchanged. Delta, checksums
  and tests: `docs/proposals/gpui-text-i18n`.
- Approved step 2 prototype for logical-order wrapping of right-to-left/mixed paragraphs in
  vendored GPUI, explicit logical row starts and Languages page paragraph samples. Five Linux gates pass, but native Linux Arabic/Hebrew
  captures remain clipped and misaligned. Supplied macOS evidence and Linux A/B captures:
  `docs/receipts/gpui-logical-wrap-20261025`. No new dependency; step 3 remains unapproved.
- Interactive browser gallery with 60 WeUI examples, 132 state presets, local file previews,
  theme controls and portable Python-only builds, published on GitHub Pages through a manual workflow.
- DateField typed ISO entry, read-only and validation APIs, optional-value change events and
  form-label/error association. Full-i32 year ranges use bounded lazy picker rows.
- Semantic article paragraphs/headings, cell titles/tips, configurable progress announcements,
  uploader capacity and controlled swipe-open callbacks.
- Theme helpers for composited surface and focus contrast; two-colour focus indicators for
  controls on inherited backgrounds and images.
- `Semantics::invalid(bool)` exposes controlled validation state; `Agree::warn` uses it.

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

- Dialog, half-screen dialog and form-link emphasis use upstream 500 weight.
- Layers follows upstream responsive plane sizes and flattened transforms; labels and keyboard
  focus outlines follow the planes. Named accessibility actions select the intended layer,
  focus follows selection and hidden layers leave the accessibility/tab order.
- The separately approved CJK trial keeps twenty closing marks with preceding CJK text,
  correcting the sampled Panel line break and two-row ellipsis. No dependency was added.
- Wrapped footer rows keep 44px targets apart; invisible measurement text is hidden from
  accessibility. Preview verification rejects Windows drive, backslash and stream paths;
  capture validation rejects inconsistent scenes and reference page errors. Preview bundles
  retain the changelog and project policies.
- Footer links retain invisible 44px targets while restoring the upstream visible line height
  and separator placement. The gallery's registration link now has keyboard/click activation.
- Form page titles use the upstream medium weight. The separately approved GPUI trailing-space
  guard trial keeps a fitting last word on its row; known RTL rendering limits remain.
- Scrolled reference captures find the last page even when a form appends script siblings.

- Restore website builds and browser verification after the native-only Languages route was
  added; retain strict coverage checks for the 60 upstream browser routes and add CI regressions.

- Footer links have at least 44×44 logical-pixel targets and can wrap in narrow rows.
  `Footer::safe_area_bottom` accepts host-provided bottom padding, including for fixed footers.
- Neutral swipe actions use the theme foreground instead of inheriting white text.
- Long agreement text can shrink and wrap while its checkbox keeps its width.

- Canceling inline IME composition restores selected committed text and selection direction
  without a Changed event. State/maxlength changes settle preedit without stale restoration.
- Escape on an already collapsed SwipedCell reaches its containing modal; an open row still
  closes first.
- Gallery article images resize proportionally up to their intrinsic width and preserve the
  following content gap in wide windows.
- Vertical FormCell labels fill their header; horizontal labels retain the 85px cap.
- Guard closing overlay callbacks, picker confirmation and delayed wheel work; reveal long
  select choices, preserve short-sheet layout and interrupted drag/animation behavior.
- Preserve Agree link/checkbox callback independence, Switch interruption continuity and
  read-only clear-input values; expose outlined search keywords and the homepage focus handle.
- Keep keyboard focus visible across light/dark/care/custom-brand fills, and announce each
  invalid typed-date submission once in the sampled Linux reader sequence, including retries.
- Export input visual-line text geometry, AT selection, word boundaries and current screen
  origin; preserve IME suffixes, bounded undo and newly inserted text visibility.
- Apply owner-approved local compatibility copies of the existing AccessKit crates and
  GPUI 0.3.9 for cache signal encoding, disabled/read-only/state/heading export and the existing
  window-bounds hook. Versions stay pinned; consumers need equivalent workspace-root patches.
- Guard queued accessibility actions against closed/reopened overlay lifecycles, including
  native Focus and custom callbacks. Keep cached nodes/actions available while assistive
  technology is active. The scoped GPUI hook makes matching root patches mandatory to compile.
- Ignore temporary X11 keyboard-grab focus notifications while retaining real focus changes;
  real Orca review no longer resets to the wrong icon during the sampled command.

- Register focus-reveal targets on navigation tabs, clear-input actions and remaining raw
  gallery controls; add the article link keyboard ring.
- Expose TextInput/TextArea text runs and caret/selection to assistive technology with masked
  password text; expose dialog/form-preview labels and cell value/descriptions correctly.
- Bind Ctrl+Home/End and Shift variants to existing document-boundary editing actions.
- Give the two independent radio groups in `component_states` separate named group parents.
- Hide covered modal layers from accessibility while retaining keyboard/visual restoration.
  The later approved cache compatibility fix repairs the sampled Orca reopen/restoration failure;
  full screen-reader acceptance remains incomplete.

- Swiped-cell actions: the warn (删除) action's focus ring uses theme `fg_0` and default actions
  use brand. The white ring next to a white row was not visible as focus.

- Search bar: the homepage 搜索 entry and camera button, and the back, clear and 取消 buttons, show a 2px brand focus ring instead of a
  faint grey fill; the unfocused layout is unchanged.

- Nested modals: the Tab trap ran in the capture phase, so an outer modal took Tab from a modal
  rendered inside it and focus reached controls behind the inner mask. The trap now runs in the
  bubble phase and the innermost modal handles Tab first. New `examples/nested_modals.rs`.
- Overlay primary buttons (white fill on brand) show a near-black focus ring instead of a brand
  ring that was hard to see against the fill.

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
