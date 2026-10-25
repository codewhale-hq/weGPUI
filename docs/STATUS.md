# Status — 2026-10-25

weGPUI is an independent Rust library and gallery port of WeUI 2.6.26 for GPUI. GitHub is the
canonical contributor home; CNB is a mirror. All 61 upstream example files have gallery
mappings. Visual, behavioural and accessibility parity is **sampled, not complete**.

The target platforms are macOS, Windows, Linux, iOS and Android. **No platform is supported yet**
in the sense of [PLATFORMS](PLATFORMS.md): every one still lacks at least one of the build,
launch, rendering, interaction, accessibility or packaging receipts. Mobile is not implemented.

Evidence sources: the macOS review of 2026-10-24 ([REVIEW](REVIEW-2026-10-24.md), receipts in
`receipts/review-20261024/`), and the Linux cloud takeover of 2026-10-25 (receipts and their
README in `receipts/takeover-20261025/`). Linux runtime results came from a local build with
`gpui_platform`'s `x11`/`wayland` features enabled. That feature change is not committed and is
pending an owner decision.

## Platform acceptance matrix

| Platform | Build | Launch | Rendering | Interaction | Accessibility | Packaging |
| --- | --- | --- | --- | --- | --- | --- |
| macOS arm64 | Pass at `51be7e3` (Rust 1.99). Not rebuilt after the takeover changes | Pass (earlier gallery runs) | 28 fresh pairs (review) | Mouse/keyboard samples; TextArea Pinyin | AX dumps, 9 routes; no VoiceOver | Missing |
| Linux x86_64 | Pass: build, clippy `-D warnings`, fmt, doc (Rust 1.97 stable; build and clippy also on 1.95.0) | **Fails** with committed manifest; passes with `x11` (Xvfb) | Software-rendered X11; 60 routes swept against Linux Chromium (default state, light) | XTEST click/keys, fcitx5 Pinyin, GTK portal chooser | AT-SPI tree via pyatspi; no Orca | Missing |
| Windows | mingw cross-build links | Missing (Wine 9.0 cannot load; no host) | Missing | Missing | Missing | Missing |
| iOS | Library type-checks (device and simulator targets) | Blocked: no backend at pin | Missing | Missing | Missing | Missing |
| Android | Library type-checks | Blocked: no backend upstream | Missing | Missing | Missing | Missing |

## Done

- **Linux baseline:** `cargo build --all-targets`, `cargo clippy --all-targets -- -D warnings`
  and `cargo fmt --all -- --check` (`--locked --offline`) pass on Linux x86_64 with Rust 1.97.0.
- **Platform-neutral library:** `gpui-pre-platform` is a dev-dependency, so the library depends
  on `gpui` only and `cargo check --lib` passes for `aarch64-apple-ios`, `aarch64-apple-ios-sim`
  and `aarch64-linux-android`. That is a type-check, not support.
- **Platform feasibility:** [PLATFORMS](PLATFORMS.md), source-backed against the pinned crates
  and upstream Zed `347f171a`, with proposals for Linux features, iOS (`gpui-pre-ios 0.3.9`) and
  an upstream Android backend.
- **Measured MSRV:** 1.95.0. Build and clippy pass on 1.95.0. 1.93.0 and 1.94.0 fail
  (`std::hint::cold_path` in `gpui-pre`), and 1.92 fails (`slice::as_array` in `gpui-pre-util`).
  The dependency manifests alone declare at most 1.92 (`oo7`).
- **Defect fixes, verified on Linux unless noted:**
  - Modal accessibility isolation (`ModalScope`): AT-SPI shows only the dialog subtree while a
    dialog, picker or half-screen dialog is open, and the page again when closed.
  - SelectMenu: real-key sequence; Up/Down/Home/End stay within the options and Escape restores
    the trigger focus.
  - Slider: non-finite inputs no longer panic (baseline `range(NaN, 10.)` panicked). Probe
    screenshots before and after.
  - Picker: disabled defaults fall back to an enabled item; all-disabled columns cannot confirm
    (baseline confirmed disabled items). Probe stdout before and after.
  - Presence: an interrupted transition reverses from its current value (frame-sampled; baseline
    jumped to fully open); hide easing follows CSS; panels slide by their own height; hide
    without an opener blurs a focused child (source change, not separately driven).
  - LinkText: each link is a separate focus stop and activates once (probe).
  - Swipe cells: closed actions are hidden from AT-SPI. Article list items are exposed with
    `posinset`.
  - Text input: undo/redo and UTF-16 `maxlength` (real keys); Pinyin commit and cancel in a
    single-line search bar and a TextArea through fcitx5/XIM.
  - Uploader: the native chooser adds picked images; non-images become error items; cancel adds
    nothing; a chooser failure shows a warning tip (GTK portal and no-portal runs).
  - Picker Home/End with disabled edge rows lands on the nearest enabled row (probe; before,
    the selection stayed put).
  - Toasts use a passive `Presence`: focus stays put, Tab and typing keep working behind the
    toast, the page stays in the AT-SPI tree, and the toast is one alert (before: focus moved into
    an empty layer that trapped Tab and hid the page). Toptips reverse mid-fade instead of
    jumping to full opacity (source-only).
  - Source-only (built and linted, not driven): Progress ignores NaN values. A loading-only
    button keeps the "正在操作" name, and an empty-label non-loading button no longer gets it.
    Nested lists inside `ArticleList::none()` drop their markers, as WeUI's `__list_none li`
    rule does. Disabled cells drop the pressed highlight. The Agree shake respects reduced
    motion. Slider values snap to the nearest step, as HTML range does.
- **Tooling:** `scripts/weui_upstream.py` verifies and serves the upstream example, pinning the
  weui.js 1.2.17 and WeChat SDK scripts it loads. `scripts/capture.py` has an X11 path.
  [PARITY-TOOLING](PARITY-TOOLING.md) documents setup and procedures.
- **OSS readiness:** README with screenshot pair, usage example and platform table;
  `examples/minimal.rs`; CONTRIBUTING, Contributor Covenant 2.1, SECURITY, CHANGELOG
  (0.1.0, unreleased), issue/PR templates, and a CI draft ([CI-PROPOSAL](CI-PROPOSAL.md)) that is
  not enabled. Cargo metadata (rust-version, keywords, categories, exclusions).
- **Rustdoc:** every public item is documented, including generated palette fields (the
  generator emits the docs) and text-input actions. See *Documentation status* below for the
  warning count.

## Behaviour by family

"Matches" names the exercised behaviour only. Linux rows used the uncommitted X11 build.

| Family | Result | Evidence and remaining difference |
| --- | --- | --- |
| Action sheet | Partly matches | macOS open states and corner fix. Shares the Presence/ModalScope fixes. Dismissal matrix and timing untested. |
| Article | Partly matches | Linux AT-SPI: headings and list items exposed. Heading level is set, but `accesskit_atspi_common 0.19.1` does not forward it. Nested markers in `list_none` fixed in source. |
| Badge | Untested | Static examples only. |
| Button / Agree | Partly tested | Linux click on 普通型 navigates; Tab focus ring visible. Accessible-name fix is source-only. Loading/disabled/consent matrix not driven. |
| Dialog | Partly matches | Linux AT-SPI modal isolation. Historical macOS Tab/Escape evidence. |
| Flex | Untested | Responsive resizing not exercised. |
| Footer | Untested | Callbacks empty in the gallery. |
| Form / input / textarea | Partly matches; differs | Linux real keys: typing, undo/redo, UTF-16 limit, Pinyin commit/cancel (single line and multiline). macOS TextArea Pinyin. Numeric filtering is not HTML validation. Preedit is not inline on Linux XIM. |
| Form choices | Matches samples | macOS radio/switch samples; Linux AT-SPI switch tree. Checkbox/Agree matrix untested. |
| Form select | Differs (adaptation) | HTML `<select>` is an overlay menu. Linux real-key bounded navigation and Escape focus restore verified. |
| Gallery / image viewer | Matches samples | macOS close/delete. Linux: a thumbnail click opened the viewer. Pinch/zoom untested. |
| Grid | Untested | Example callbacks are empty. |
| Half-screen dialog | Matches samples | macOS open/Tab/Escape/drag. Linux AT-SPI isolation and interrupted-transition frames. |
| Icons | Untested | Assets reproduce from the tarball. |
| Information bar | Matches sample | macOS close. Link/floating variants untested. |
| List / swiped cell | Partly matches | macOS drag. Linux AT-SPI hides closed actions. Delete callback empty upstream and in the gallery. |
| Loading | Untested | Timing and live reduced-motion changes unverified. All spinner kinds draw the same SVG. |
| Loadmore | Untested | Presentation only. |
| Message / LinkText | Partly tested | Probe: two links, separate focus and single activation. Message callbacks empty. |
| Navbar / Tabbar | Matches samples | macOS arrow selection after fix. Not re-driven on Linux. |
| Panel | Untested | CJK punctuation and clamping differ. |
| Picker | Partly matches; differs | Linux AT-SPI option names present; probe disabled-default and all-disabled cases. `today()` is UTC, while the example uses local time. weui.js throws on all-disabled columns; weGPUI disables Confirm. |
| Preview | Untested | Callbacks empty. |
| Progress | Untested | NaN fix is source-only. |
| Search bar | Partly matches | Linux single-line Pinyin commit/cancel. macOS typing/results. |
| Slider | Matches samples | macOS drag/keys. Probe: non-finite inputs, zero step falls back to 1. |
| Steps | Untested | Static rendering. |
| Toast / toptips | Partly matches | macOS success toast. Linux: passive toast keeps focus and the page tree (as upstream's `role="alert"`); uploader error tip. Timing untested. |
| Uploader | Partly matches | Linux chooser lifecycle (picked, error, cancel, no portal). Transport is application-owned, as upstream. |
| Home / layers / index | Untested | Gallery-specific surfaces. |

## Known unresolved defects and gaps

- **Linux windowing** needs `gpui_platform` `wayland`/`x11` (34 added lock packages). Awaiting
  owner decision. Until then the committed gallery panics on Linux.
- **Mobile:** iOS needs a GPUI pin move to 0.3.9 plus `gpui-pre-ios`, and a macOS/Xcode host.
  Android needs a new backend. Neither has an AccessKit adapter.
- **Screen readers:** no VoiceOver, Narrator/NVDA, Orca or TalkBack session has been run. Text
  inside dialogs (title/body) is not exposed as separate nodes; the dialog is named by its title.
- **Local date:** `picker::today()` is UTC. A local-time source needs a dependency (for example
  `chrono`, already in `Cargo.lock` transitively) or platform calls. That needs an owner decision.
- **Linux XIM preedit** is drawn by the input method at the window's top-left, not inline at the
  caret. Not isolated between GPUI and fcitx5.
- **Found during the documentation audit, unfixed** (documented in rustdoc): buttons in cell
  mode ignore `loading`; a clickable small-appmsg panel has no visible focus ring; spinners share
  the default id `weui-loading` unless given an id; all spinner kinds draw the same SVG;
  `ArticleList::new(ListStyle::Inside)` draws no markers while `inside()` numbers inline;
  `ListTips` items have no `List` parent; radio rows reuse ids across
  groups that share a parent; `FormCell::label` does not name its input for assistive technology
  (gallery inputs expose only the placeholder); `VcodeButton::start` restarts while counting.
- Public naming still mixes `Navbar`/`Tabbar` with `SearchBar` and terse `hd`/`bd`/`ft` helpers.
  The existing API is documented. Breaking renames await a versioning policy.
- The input length counter counts UTF-16 units. Numeric `InputKind`s filter characters but do
  not validate.

## Documentation status

`RUSTDOCFLAGS='-W missing-docs' cargo doc --no-deps --locked --offline` on Linux reports **0
warnings**, including broken intra-doc links, and also passes with `-D warnings`. The baseline
review counted 1,019 missing-docs warnings. Palette field docs are emitted by
`scripts/extract_weui.py`, so regeneration preserves them. Doc examples are marked `ignore` and
point to `examples/minimal.rs`, which `cargo build --all-targets` compiles. No doctests were
added.

## Not done / missing receipts

- macOS rebuild, gallery run and AX re-check of the takeover commits. The environment had no Apple
  host.
- Native Windows build/run; iOS/Android anything beyond type-checks; physical devices.
- Hosted CI of any kind (CNB result not recorded; GitHub Actions not approved).
- Packaging (app bundles, installers) on any platform. `cargo package --list` reviewed: 172 files,
  399 KiB compressed. Nothing published. The README's screenshot pair uses repository-relative
  paths that are excluded from the package, so it renders only where the repository is reachable.
- Complete visual recapture of all 264 pairs. Care mode beyond one route. Long pages beyond 812 pt.

## Owner decisions pending

1. Enable `wayland`/`x11` for the gallery (recommended now).
2. Mobile backend and dependency path: GPUI 0.3.9 plus `gpui-pre-ios`; an upstream Android backend.
3. Enabling the GitHub Actions workflow in [CI-PROPOSAL](CI-PROPOSAL.md).
4. Allowing a test suite.
5. crates.io name, publication timing, versioning and the GPUI upgrade policy.
6. Keeping about 34 MB of parity screenshots in Git (they are excluded from the package).
7. Localisation/RTL scope; the WeUI defaults are Chinese strings.
8. Private conduct and security contacts (placeholders explain the interim process).
9. A local-time source for `picker::today()`.
10. Scope of the wider weui.js imperative API. Only behaviour the examples invoke is ported.

[Sources](SOURCES.md) · [Coverage](UPSTREAM-COVERAGE.md) · [Visual evidence](PARITY.md) ·
[Review](REVIEW-2026-10-24.md) · [Platforms](PLATFORMS.md) · [Parity tooling](PARITY-TOOLING.md)
