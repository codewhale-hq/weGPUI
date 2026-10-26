# Status — 2026-10-26

weGPUI is an independent Rust library and gallery port of WeUI 2.6.26 for GPUI. GitHub is the
canonical contributor home; CNB is a mirror. All 61 upstream example files have gallery
mappings. Visual, behavioural and accessibility parity is **sampled, not complete**. The
[publication review](REVIEW-2026-10-26.md) records the final review scope, fixes and checks.

Current upstream is still WeUI 2.6.26. The [parity plan](WEUI-PARITY-PLAN.md) inventories
61 HTML examples and 59 widget styles, and executes the eight-appearance, 132-scene reference
matrix. The [new receipt](receipts/weui-parity-20261025/README.md) separates capture completeness,
manual review and contract acceptance. The owner separately approved a monotonic shaped-row
trailing-space trial; this changes one guard in `line_layout.rs` and fixes the article's early
word break. On October 26 the owner requested complete review and publication of all current
work to GitHub, superseding the earlier landing hold for repository publication. The Linux RTL
visual failure remains an acceptance blocker and step 3 remains unapproved. The supplied
macOS prototype images do not qualify the new guard. The [October 26 continuation](receipts/weui-parity-20261026/README.md)
adds upstream 500-weight corrections, responsive transformed Layers labels and direct named
accessibility actions, and the separately approved twenty-character CJK closing-mark trial.
All eight Layers appearances pass 48 native interaction steps, including selected focus and
hidden inactive controls. A further 80 normal-motion interruption and stable-endpoint checks
pass in all eight appearances. A current-source English/light Orca sample passes twelve
state steps and eight name/role speech windows, with real synthesized PCM retained; human
listening and full reader acceptance remain open. Current source gates pass with 59 root and three gallery tests.
The earlier apparent 16px half-screen text gap was a measurement error; no padding change was made.

The first public release targets macOS, Windows and Linux; iOS and Android remain
experimental. See the [desktop release checklist](RELEASE-READINESS.md) and
[website build guide](../website/README.md). **No platform is supported yet**
in the sense of [PLATFORMS](PLATFORMS.md): every one still lacks at least one of the build,
launch, rendering, interaction, accessibility or packaging receipts. The iOS backend/bundle
scaffold is implemented but unqualified; Android has no backend.

Evidence sources: the macOS review of 2026-10-24 ([REVIEW](REVIEW-2026-10-24.md), receipts in
`receipts/review-20261024/`), and the Linux cloud takeover of 2026-10-25 (receipts and their
README in `receipts/takeover-20261025/`). Linux runtime results came from a local build with
`gpui_platform`'s `x11`/`wayland` features enabled. That feature change is now applied; fresh isolated X11 receipts are in
[cloud-linux-20261024](receipts/cloud-linux-20261024/README.md). The later
[macOS follow-up](receipts/2026-10-24-macos/README.md) adds fresh build and sampled runtime evidence,
including failed/incomplete attempts. It does not complete platform acceptance.

## Localisation and right-to-left — 2026-10-25

The independent [completion continuation](COMPLETION.md) adds minimum footer targets and an
application-supplied safe-area API, readable neutral swipe actions, bounded agreement layout,
a complete application example and a reproducible preview source bundle. It also repairs the
website route-inventory regression; five website regressions and 42 full Chromium checks pass
across root/project mounts (360 route/viewport observations). Its source and
verification are separate from the held text prototype:
[receipt](receipts/completion-20261025/README.md). This does not complete a component's desktop
acceptance matrix or authorize publication.

The owner's instruction that the library "work in English and all other languages" settles the
scope question that was open here. Every string the library supplies comes from a message catalog
(14 built-in; any other language by supplying a `Messages`), layouts mirror under a right-to-left
locale, and `TextInput` follows bidirectional runs. The gallery opens in English with a Language
menu; `--lang zh-Hans` still reproduces the upstream wording. Guide: [I18N](I18N.md); receipt:
[i18n-20261025](receipts/i18n-20261025/README.md) (macOS only, five gates pass with 59 tests).

Not done, and said plainly: no native-speaker review of the 12 starter catalogs; no screen-reader
check in another language; no real keyboard/IME input or caret capture in a right-to-left field;
Linux and Windows not run; `TextArea` does not right-align; dates stay ISO and the picker's
column order stays year-month-day. Arabic, Hebrew, Devanagari and Greek now wrap at spaces
(owner-approved step 1 of the [GPUI proposal](proposals/gpui-text-i18n/README.md), applied to
`vendor/gpui_pre`; [receipt](receipts/gpui-word-break-20261025/README.md)); the vendored GPUI
has an approved step 2 prototype for logical-order wrapping. Its supplied macOS captures and
all five Linux source gates pass, but Linux Xvfb still clips and misaligns Arabic/Hebrew
paragraphs (also present in the A/B baseline). **The owner initially chose to hold landing on 2026-10-25**:
[receipt](receipts/gpui-logical-wrap-20261025/README.md). Step 3 (caret x and partial selection
in wrapped right-to-left rows) remains unapproved. Thai and Hangul still break between any two
characters; Hangul was left alone by decision. This is a sampled Linux check, not completion
of localisation acceptance on Linux. The later
[publication review](REVIEW-2026-10-26.md) records the owner's request to commit the existing
prototype with these limits; it does not authorize step 3 or another wrapping-logic revision.

## Linux continuation — 2026-10-25

The first cloud continuation fetched clean GitHub main `52650f3` and was committed as
`9c4515c`. The later local Mac candidate was unavailable here and supplies no cloud validation.
The separately green vertical-form label fix `a3ce3af` was then integrated into the second
Linux batch. Every runtime receipt identifies its own source and executable; later dirty
source is not retrospectively covered by an earlier executable.

The first batch's Rust 1.98.1 checks and artifacts remain in
[the continuation receipt](receipts/linux-continuation-20261025/README.md). It added focus
reveal targets, text readback and semantic fixes, and depth-ranked nested-modal isolation.
It also captured and visually reviewed 216 dark/care/brand startup images and ran actual GPUI
Wayland clients under nested Weston. The pinned SDK fetch blocked a new paired upstream-shell
sweep; native-only captures are explicitly labelled.

The second batch implements and drives reusable APIs rather than counting gallery routes:

- [Controls](receipts/control-contracts-20261025/README.md): button sizes/kinds/loading/disabled
  callbacks in eight theme combinations, consent propagation/wrapping, verification-code timer
  guards and sampled continuous switch reversal.
- [Forms and dates](receipts/form-contracts-20261025/README.md): validated optional ISO entry,
  range/owner-update rules, DateTime staging, read-only/disabled behavior and mode-change focus.
  The final 15-step native rerun and actual Orca invalid/retry/correct/clear speech checks pass
  separately; the earlier duplicate error speech remains preserved.
- [Overlays](receipts/overlay-contracts-20261025/README.md): thirteen native cases cover dismissal,
  exact callback counts during close, reentrant reopening, long selects, lazy full-range dates,
  sheet dragging and inline focus reveal. The initial repeated-confirm failure is retained.
- [Data and content](receipts/data-contracts-20261025/README.md): controlled navigation/Slider,
  actual row/grid/footer callbacks, live events and viewport notice/footer occlusion. Semantic
  article-image paint regressions are preserved; the final light/dark correction passes semantic
  and native visual review with 184px images, 24px following gaps and no paint spill.
- [Media](receipts/media-contracts-20261025/README.md): actual thumbnail previews and deletion,
  uploader capacity/status APIs, controlled swipe requests, focus return and AT default actions.
- [Search](receipts/search-contracts-20261025/README.md): actual Homepage/Outlined/Filled callbacks,
  public focus, read-only clearing guards and input refocus. Disabled transitions/FilledGrey callback matrix and complete
  theme/state qualification remain open.
- [Input](receipts/input-contracts-20261025/README.md): native word editing, selection/undo,
  composition-commit contracts and 31-step shaped-text/AT matrices at scales 1/2. Inserted-glyph
  visibility and moved X11 origins pass; held-modifier Orca readback is recorded separately,
  with service-teardown failures preserved.

The owner approved local compatibility copies of GPUI/AccessKit without changing their package
versions. [Actual Orca followups](receipts/linux-bridge-20261025/README.md) now read repeated
reopened dialogs and restore their opener after the Cache AddAccessible correction. The
[bridge/state/origin followup](receipts/wayland-20261025/bridge-followup/README.md) verifies
Disabled, ReadOnly, Busy, InvalidEntry, Expanded/Collapsed, heading levels and X11 native origin
updates; old unsupported-state/cache observations describe their old artifacts. Cargo patch
propagation to downstream workspace roots is documented in [vendor/README](../vendor/README.md).

The final source, including the checkbox/caret review corrections, passed all five gates on
Rust 1.98.1: formatting, all-target build, 47 unit tests (one ignored doctest), Clippy with
warnings denied and strict rustdoc, using locked/offline Cargo resolution where applicable.
[The batch receipt](receipts/component-contracts-20261025/README.md) records the final source
qualification. Article light/dark review and corrected checkbox eight-mode callback/interior
pixel checks pass. Final Outlined/FilledGrey caret surfaces and FilledGrey action-focus pixels
also pass their eight-mode native/visual checks
([focus receipt](receipts/focus-contrast-20261025/README.md)). The earlier 2.226:1 Picker
ring and the first contrast sweep's unchecked-checkbox fill defect remain historical failures.
Actual Orca speech/PCM was collected, but audio intelligibility has not been accepted by
listening. Other desktop hosts, hardware, complete theme/state/reference sweeps and full reader
qualification remain open. **No component row is accepted.**

### Runtime follow-up after `49268de`

The preceding batch was committed as `49268de`. The five-source continuation is committed
as `3840b47` and passes all five gates again on Rust 1.98.1 with 47 unit tests and one ignored
doctest; its exact source,
artifacts and log are in [the runtime follow-up receipt](receipts/linux-runtime-followup-20261025/README.md).
Runtime observations retain their own executable/source provenance:

- [Actual XIM/Pinyin](receipts/ime-x11-20261025/README.md) exposes and fixes cancellation that
  deleted selected committed text. The fixed observer passes 35 snapshots: real forward/reversed
  cancellation emits zero Changed, commit/Undo/Redo each emit one, and multiline, state changes,
  UTF-16/grapheme/suffix limits pass. Direct event-order protocol checks are labeled separately.
  fcitx's UseOnTheSpot=True enables inline preedit and caret-adjacent candidates; default False
  uses an external preedit window. The old external-only observation is not a blanket GPUI limit.
- [Responsive Article](receipts/article-responsive-20261025/README.md) passes 375/480/700/900px
  light and 480px dark native/static-CSS comparisons, including the 690px image cap and 24px
  following-heading gap. All ten raw captures were reviewed. Earlier size/gap failures and a
  disclosed one-pixel Contain rounding difference remain; full SDK/JS or exact raster parity
  is not established.
- [Swiped Escape](receipts/overlay-contracts-20261025/swiped-escape-followup/README.md) now lets
  Escape on a collapsed row reach its Modal. An open row still closes itself first, restores row
  focus and lets the next Escape dismiss the modal; native callback counts and trees pass.
- [Agreement warning](receipts/orca-controls-20261025/README.md) adds/removes InvalidEntry through
  Semantics::invalid and Agree::warn. Actual Orca says invalid entry only while warning is true,
  for both checked and unchecked cases. Checkbox/terms callbacks are unchanged by flag updates.

Additional real Orca samples on the separate committed `49268de` consumer cover checked/pressed
choices, independent radio positions, Agreement/terms, Vcode countdown/restoration and disabled
default/`Button::cell()` grayed speech plus loading descriptions. Busy is a tree state in these
samples. No audio has been accepted by human listening. The original disabled IconButton
readback failed because X11 reader grabs reset Orca's review context; its later approved-source
result is recorded below.

### Approved modal/cache and X11 focus compatibility

This native compatibility batch is pushed and remote-verified as
`f7d41a534b02c15c6dbf3d0b216c9becc3925bcd`; its fresh five source gates pass with
47 root tests and one ignored doctest. The browser follow-up is separate.

The owner approved and applied the [central dispatch/cached-frame guard](proposals/modal-a11y-action-guard/README.md)
and [X11 grab-focus filter](proposals/x11-grab-focus/README.md). Their additional scope is now
implemented: three GPUI framework files plus library Semantics/Modal integration, and a fourth
same-version local `gpui-pre-linux 0.3.9` copy. Combined root fmt, locked all-target build,
47 tests/one ignored doctest, strict Clippy and strict rustdoc pass on Rust 1.98.1
([gate log](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/gates.log)).

- [Seven saved-action timing cases](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/)
  preserve closed owner state/restored focus, including requested 0/1ms Click and Focus.
  Original immediate-close failures remain in the historical actual-request receipt.
- [The fixed cached/live fixture](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/cached/README.md)
  re-renders both children on parent-only notify while AT is active, retains both nodes in
  13 fresh trees and keeps saved Click/Focus proxies usable. The earlier missing-node failure
  and the retry's failed first window-manager startup remain preserved.
- [Actual focus-mode, Fcitx, Orca and Wayland controls](receipts/x11-grab-focus-20261025/README.md)
  preserve Normal/WhileGrabbed activation and suppress false AT pulses during Grab/Ungrab.
  Real preedit survives native grabs; commit/undo/reversed cancel pass. Actual Orca retains
  disabled icon-2 flat review and emits `icon-2 button grayed.`; transcript/dispatcher/PCM
  were checked independently without listening or ASR. Rebuilt nested-Weston modal isolation,
  keyboard trap and focus restoration pass.

The 47 root tests do not execute the two added vendor unit tests. Their explicit attempt
stops before execution at 21 missing upstream test-asset/optional-dependency errors
([log](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/vendor-unit-attempt.log));
no new dependency was added. A separate extracted NodeBuilder harness passes the two metadata
regressions with existing locked rlibs ([scope](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/metadata-extraction.json));
that is supplemental and does not mean the full vendor suite or 49 root tests passed.

Six extended native modes also pass raw-caller callbacks, multiple-listener close,
hide/reopen epoch invalidation, passive overlay/page independence, closed-ancestor handling
with a still-open child, and two-window isolation (`contracts-*` in the fixed-guard receipt).
Actual counters remain unchanged for closed targets even when AT accepts the queued request;
focus/tree controls verify restoration. Cache performance, arbitrary external topmost ordering,
full reader/theme/host matrices and human audio listening remain open.

**Matching patched GPUI is a compilation requirement for every consumer.** Semantics/Modal
calls `Window::with_a11y_action_guard`, which registry GPUI 0.3.9 lacks. Consumers must adopt
workspace-root patches selecting `vendor/gpui_pre`, `vendor/gpui_pre_linux`,
`vendor/accesskit_unix` and `vendor/accesskit_atspi_common` from the matching checkout;
dependency-manifest patches are not inherited. See [vendor setup](../vendor/README.md).
No component row is accepted.

The [interactive browser gallery](../website/README.md) is built with 60 live routes and
132 state presets from pinned WeUI 2.6.26 / WeUI.js 1.2.17. The full `final-candidate1-rerun`
suite passes at site root and `/weGPUI/`: 40 browser checks, including 360 route/viewport
observations. Final candidate 3 passes three targeted checks per mount plus the complete
legacy checker: each mount verifies 264 historical state/theme views and 528 decoded images.
The theme/native-theme checks cover 375/390/768/1280px, retained palette routes and live iframe
pointer/Space interaction; the final 390px theme capture was reviewed. Candidate 3 changes
only the palette handler and CSS minimum width relative to candidate 1; all other built
files match. The complete suite remains attributed to candidate 1
([exact scope and receipts](receipts/web-interactive-20261025/README.md)). It uses a Python-only normal build with
checked-in assets and local-only upload previews. This separate HTML/CSS/JavaScript
implementation does not run native GPUI; historical native captures use `?view=native`,
and native setup remains at `#native-guide`. All 40 native component rows remain unaccepted.

The owner configured Pages and dispatched the manual-only workflow. Build and deploy both
succeeded for `e5b6996` in [run 40211664701](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701),
returning <https://codewhale-hq.github.io/weGPUI/>. See the
[publication receipt](receipts/pages-publication-20261025/README.md) and
[hosting steps](../website/HOSTING.md). The agent's checks were read-only. Direct access
to the public origin is blocked by the cloud proxy, so deployed-origin browser qualification
is still open. Further workflow dispatches require owner authorization; source pushes do
not publish automatically.

## Platform acceptance matrix

| Platform | Build | Launch | Rendering | Interaction | Accessibility | Packaging |
| --- | --- | --- | --- | --- | --- | --- |
| macOS arm64 | October 24 candidate examples, nine tests, strict Clippy/rustdoc and packaged consumer pass; current Linux continuation unqualified | Historical component/gallery/minimal launches | Historical native dark sample and parity pairs | Historical loading/countdown/disabled/panel/choice samples | Historical input/spinner/tip AX checks; VoiceOver off and incomplete | Historical crate/consumer pass; signed/fresh-machine distribution missing |
| Linux x86_64 | Combined approved guard/cache/Linux-focus source passes all five gates on Rust 1.98.1, 47 root tests/one ignored doctest; vendor-unit attempt blocked separately; earlier source/MSRV receipts stay separate | X11 and real Wayland clients under nested Weston | Historical 192 pairs; 216 native-only states and targeted eight-mode control/data/media reviews; complete reference/theme parity open | Reusable callback/state/focus/transition matrices and Wayland regression samples; hardware/reduced-motion/IME coverage incomplete | Four approved compatibility copies pass sampled action/cache/state/text/X11-origin and focus-mode checks; real Fcitx and Orca reopen/input/date/control/disabled-icon speech exist; full reader/listening acceptance missing | Separate path-consumer build and X11 runtime pass with explicit root patches; published-package/fresh-machine/distribution unqualified |
| Windows ARM64 | Native MSVC gallery/component sample build, 9 tests and strict rustdoc pass on the October 24 component candidate | Gallery and component sample launch in Windows 11 ARM VM | Native VirtIO GPU rendering; sampled light components and dark dialog | Text entry, loading/disabled actions, countdown, panel focus, independent radios and navbar wrap | Screen-reader and native accessibility-tree acceptance still missing | Package archive built and used for guest compilation; distributable installation unqualified |
| iOS | Historical library type-checks pass; hosted app build fails in upstream iOS/core APIs | 0.3.9 scaffold wired; upstream compilation blocker, no launch | Missing | Missing | Missing; backend lacks AccessKit | Missing |
| Android | Historical 0.3.9 library type-check; no app build | Blocked: no backend upstream | Missing | Missing | Missing | Missing |

## Historical completed slices

The entries below retain their original source/date scope. Current second-batch behavior and
remaining gaps are recorded above and in the reconciled family table; historical source-only
claims are superseded only by the specifically linked later receipts.

- **Component continuation:** native macOS and Windows ARM builds, 9 tests each, strict Clippy
  and rustdoc pass; a fresh packaged macOS consumer compiles. Runtime checks cover loading,
  countdown, disabled focus, panel focus and independent choices. macOS AX confirms input
  naming and tip-list structure. [Receipts](receipts/component-fixes-20261024/README.md).
  The subsequently implemented focus-reveal fix has Linux X11 evidence; macOS/Windows
  reruns of current source remain open (see ACCEPTANCE).

- **Article list constructor:** `new(ListStyle::Inside)` now selects inline decimal
  markers, matching `inside()`. Focused constructor/inheritance contracts pass;
  this source-level fix does not establish new visual or screen-reader acceptance.

- **Approved iOS path:** exact 0.3.9 pins, shared target-specific example entry point, and
  simulator bundle scaffold. Linux builds/tests, three mobile library type-checks, and
  bundle-script fixture checks pass. Apple application build/runtime remains unqualified.
  See [receipts](receipts/ios-path-20261024/README.md).

- **macOS follow-up:** fetch, fmt, all-target build, Clippy `-D warnings`, and rustdoc with
  `-D warnings -W missing-docs` passed at `e4a2c87`. Rust sources and Cargo files remained
  unchanged for the later minimal/gallery launches. Minimal dialog OK increments the count
  and restores trigger focus. AX dumps confirm modal isolation in minimal, picker, select and
  half-screen dialog, hidden closed swipe actions, and article headings/list containers. Raw
  `AXTitle` contains picker/select option labels; the dump script now includes it. Further
  interaction, IME, reduced-motion and screen-reader acceptance remain incomplete.

- **Linux baseline:** `cargo build --all-targets`, `cargo clippy --all-targets -- -D warnings`
  and `cargo fmt --all -- --check` (`--locked --offline`) pass on Linux x86_64 with Rust 1.97.0.
- **Platform-neutral library:** `gpui-pre-platform` is a dev-dependency, so the library depends
  on `gpui` and `unicode-segmentation`, without a platform crate. `cargo check --lib` passed for
  `aarch64-apple-ios`, `aarch64-apple-ios-sim` and `aarch64-linux-android` at the `0637c25` change.
  That is a historical type-check, not support or a check of later HEADs.
- **Platform feasibility:** [PLATFORMS](PLATFORMS.md), source-backed against the pinned crates
  and upstream Zed `347f171a`, with proposals for Linux features, iOS (`gpui-pre-ios 0.3.9`) and
  an upstream Android backend.
- **Measured MSRV at `1ea95a5`:** 1.95.0. Build and clippy passed on 1.95.0. Later source was not
  rechecked on that compiler. 1.93.0 and 1.94.0 fail
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
  - Picker Home/End with disabled edge rows: the source fix and probe are committed. `b563ed2`
    reports Home→B and End→C, but there is no committed stdout receipt of that sequence.
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
  now implemented after local verification. Cargo metadata (rust-version, keywords, categories, exclusions).
- **Rustdoc:** every public item is documented, including generated palette fields (the
  generator emits the docs) and text-input actions. See *Documentation status* below for the
  warning count.

## Behaviour by family

Each result is a sampled contract, not whole-component acceptance. Historical macOS/Windows
samples predate the current Linux source. New Linux callbacks, native captures, AT-SPI trees,
bridge events and actual reader speech are separate evidence classes.

| Family | Current sampled evidence | Remaining difference / qualification |
| --- | --- | --- |
| Action sheet | iOS/Android item callbacks, Cancel/mask/Escape and rapid-close guard; inline leaf reveal ([overlays](receipts/overlay-contracts-20261025/README.md)) | Full theme/timing/reader/host matrix; pinned upstream has no disabled-item state. |
| Article | Semantic text/heading/list helpers and repeated h6 level; responsive gallery images at four light widths and 480px dark, 690px cap and 24px following gap pass native/static-CSS review ([responsive](receipts/article-responsive-20261025/README.md)) | Earlier paint/gap failures and one-pixel Contain rounding retained; full typography/list/reader parity incomplete. |
| Badge | Historical light/dark/care rendering and accessible row names | Complete themes/states/readers/hosts. |
| Button / Agree | Eight-mode callback/guards and focus samples; real Orca disabled default/`Button::cell()` grayed and loading descriptions; Agree warning invalid-entry additions/clears preserve checkbox/terms counts ([controls](receipts/control-contracts-20261025/README.md), [reader](receipts/orca-controls-20261025/README.md)) | No human audio listening; Busy tree-only in these speech samples; full motion/theme/state/reader/host acceptance remains open. |
| Dialog / Modal | Native five-click/disabled/focus guards and depth-ranked isolation; approved guard passes seven timing cases, fixed cached/live proxies and six raw/listener/epoch/passive/ancestor/two-window modes ([fixed receipts](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/)) | Cache performance, arbitrary external topmost ordering and full timing/reader/theme/host matrix. |
| Flex | Historical light/dark/care layout; broader window resize samples | Complete responsive-layout/host coverage. |
| Footer | Actual link callbacks, viewport placement, click occlusion and modal hidden/restored tree ([data](receipts/data-contracts-20261025/README.md)) | Host-supplied safe-area padding and minimum 44px targets are implemented in the independent continuation; full focus/reader/host acceptance remains open. |
| Form / DateField | Strict validation, optional ISO typing/clearing, range/owner/DateTime/focus/availability contracts; final 15/15 native steps and actual Orca invalid/retry feedback once each with valid/clear removing error ([forms](receipts/form-contracts-20261025/README.md)) | Default range 1900–2100 and UTC today unchanged; locale/seconds, full theme/reader/host coverage. |
| Input / TextArea | Scale 1/2 native/AT text/selection/geometry and held-modifier Orca; real inline XIM selected/reversed cancellation fix, callback/undo/state/maxlength and multiline contracts in 35-snapshot observer ([input](receipts/input-contracts-20261025/README.md), [IME](receipts/ime-x11-20261025/README.md)) | Pinned adapter lacks EditableText; full IME/reader/theme/host matrix and preedit cursor modes remain open. Historical reader teardown failures retained. |
| Form choices | Controlled/disabled/propagation/motion samples; real Orca checkbox checked/unchecked, Switch pressed/not-pressed and independent radio positions with disabled skip and callback counts ([controls](receipts/control-contracts-20261025/README.md), [reader](receipts/orca-controls-20261025/README.md)) | Complete motion/reader/theme/host matrix and human audio listening remain open. |
| Form select | Bounded long menu, disabled skip, initial selected reveal, commit/reopen/dismiss and restore ([overlays](receipts/overlay-contracts-20261025/README.md)) | Native overlay adaptation; complete state/reader/host matrix. |
| Gallery / image viewer | Actual thumbnail open; image dismissal, full-bar deletion, focus restoration; dark+care/brand samples ([media](receipts/media-contracts-20261025/README.md)) | Pinned upstream has no pinch/zoom/paging requirement. Complete reader/theme/host coverage remains open. |
| Grid / Cells / Panel | Actual keyboard callbacks, title/tips group semantics and selected panel focus capture ([data](receipts/data-contracts-20261025/README.md)) | Generic Cell state/reader coverage and complete visual parity remain open; the approved CJK closing-mark trial corrects the sampled Panel clamp; caller generic slots own semantics. Button::cell() reader samples are recorded in the Button family. |
| Half-screen dialog | Short/long drag, returning animation, interrupted reopen, short-window sizing, body/subtitle semantics and inline header focus ([overlays](receipts/overlay-contracts-20261025/README.md)) | Complete size/theme/touch/reader/host matrix. |
| Icons | Historical named assets and native enabled/disabled callbacks; approved X11 filter lets actual Orca retain disabled icon-2 flat review and emit grayed speech ([fixed reader](receipts/x11-grab-focus-20261025/README.md)) | No human audio listening; full IconButton/theme/state/reader/host coverage open. Combined approved artifact, no isolated one-patch A/B qualification. |
| Information bar | Viewport fixed placement while parent scrolls, native link/close, occlusion and live text events ([data](receipts/data-contracts-20261025/README.md)) | Bridge events are not speech; full variant/reader/theme/host matrix. |
| List / swiped cell | Controlled drag/AT callbacks and Expanded/Collapsed; native collapsed-row Escape reaches Modal, open-row Escape closes row first and restores focus ([media](receipts/media-contracts-20261025/README.md), [Escape](receipts/overlay-contracts-20261025/swiped-escape-followup/README.md)) | Approved Modal dispatch guard has separate bounded receipts; neutral action uses the theme foreground in the independent continuation; full touch/reader/theme/host matrix remains open. |
| Loading / Loadmore | Historical spinner artwork/light/dark/care; later loading text announcement event | Complete animation/live reduced-motion and actual reader qualification. |
| Message / LinkText | Historical short/wrapped real-input links and shaped bounds; title weight corrected to 500 ([inline links](receipts/inline-links-20261024/README.md), [data](receipts/data-contracts-20261025/README.md)) | Full typography/theme/reader/host coverage. |
| Navbar / Tabbar | Controlled native/AT selections and roving offscreen reveal ([data](receipts/data-contracts-20261025/README.md), [focus](receipts/focus-reveal-20261025/README.md)) | Caller wraps tab_panel in FocusScroll; full theme/state/reader/host matrix. |
| Picker | Lazy full-i32 dates, exact Home/End, leap/day clamp, disabled/reentrant/wheel contracts and focused-column ring ([overlays](receipts/overlay-contracts-20261025/README.md)) | UTC policy unchanged; all-disabled Confirm intentionally disabled; full focus/reader/theme/touch/host coverage. |
| Preview / Steps | Historical static light/dark/care trees and focus; title weights corrected | Full typography/state/reader/host coverage. |
| Progress / Slider | Controlled value/cancel removal/live events; numeric AT/key updates, snapping and availability guards ([data](receipts/data-contracts-20261025/README.md)) | Complete pointer/geometry/live-reader/theme/host matrix. |
| Search bar | Native Homepage/Outlined/Filled callback counts, public focus, read-only clear guards, keyword label, clearing/refocus and cancel rules ([search](receipts/search-contracts-20261025/README.md)) | Eight-mode FilledGrey caret/action focus pixels pass separately. The October 26 fixture adds 24 FilledGrey/Filled/Outlined runs and 120 callback/inner-input disabled/read-only transition steps; complete motion/reader/host coverage remains open. |
| Toast / toptips | Historical timed passive focus/page-tree behavior; explicit live metadata added | Full actual announcement/reader, reduced-motion/timing/theme/host matrix. |
| Uploader | Actual capacity/add/status/preview/delete lifecycle and focus; historical native chooser lifecycle ([media](receipts/media-contracts-20261025/README.md)) | Transport/compression/retry application-owned; complete chooser/reader/theme/host coverage. |
| Verification code | Native timer/activation guards; real Orca rounded/plain countdown names, grayed state, reset/restoration/restart and callback counts ([controls](receipts/control-contracts-20261025/README.md), [reader](receipts/orca-controls-20261025/README.md)) | Complete timing/theme/reader/host matrix; no read-only API exists. |
| Home / layers / index | Focus-reveal samples and native startup captures | Gallery-specific; Layers now has matched responsive projection/labels and eight-mode native keyboard/pointer/AT-action checks (October 26 receipt). Full reader, motion and other-host checks remain open. |

## Known unresolved defects and gaps

- **Focus appearance and target sizes:** shared contrast styling now has native eight-mode
  samples. The earlier 2.226:1 Picker ring, filled unchecked-checkbox centre and insufficient
  FilledGrey caret strategy remain preserved failures. Corrected checkbox callbacks/interiors,
  16 final caret surfaces and 24 FilledGrey action-focus checks pass. Complete focus appearance,
  arbitrary host palettes/images and minimum targets remain unqualified. Footer links remain
  at least 44×44 in the independent continuation; neutral swipe actions explicitly use the
  theme foreground. Complete target/contrast/reader checks on other hosts remain open.
- **Complete parity:** per-component dark/care/brand/state reference checks, actual reader
  announcements/order and current-source macOS/Windows runtime matrices are incomplete.
  All inventory rows remain unaccepted. The final article image-wrapper light/dark correction
  passes targeted screenshot/tree review; earlier visually failed evidence is retained.
- **Separate consumer:** a new external path consumer explicitly selects all approved root
  patches and passes actual X11 modal/counter/focus checks
  ([receipt](receipts/component-contracts-20261025/consumer/README.md)). Its earlier styling
  artifact does not qualify final focus colours or a published/fresh-machine package.
- **Input:** numeric kinds filter characters rather than implement HTML validation. Real XIM
  inline preedit works with fcitx UseOnTheSpot=True; its default False selects external preedit.
  The selected-text cancellation defect is fixed with actual callback/undo/state evidence
  ([IME receipt](receipts/ime-x11-20261025/README.md)); complete IME/host/preedit-cursor coverage
  remains open. The pinned AT-SPI adapter does not expose EditableText; textual Value handling
  exists for adapters that deliver it.
  The [EditableText proposal](proposals/linux-editable-text/README.md) records why advertising
  only that interface would not supply its atomic range/replacement and clipboard contracts.
  Geometry/replacement and held-modifier reader samples now pass their recorded contracts,
  with Speech Dispatcher/Orca teardown failures retained; they do not establish clean shutdown
  or complete reader acceptance.
- **Reader acceptance:** VoiceOver remains off. Actual Orca reopen/input/date/control speech
  samples work on their named artifacts, including Agree invalid-state changes. Audio has not
  been accepted by listening; full Orca/NVDA/Narrator matrices remain missing. The approved
  [X11 mode filter](proposals/x11-grab-focus/README.md) repairs the sampled flat-review reset:
  actual Orca retains disabled icon-2 and emits grayed speech, with native mode/AT-event controls
  ([receipt](receipts/x11-grab-focus-20261025/README.md)). The original reset and setup failures
  remain preserved. These local results do not establish resolution of the upstream AccessKit
  issue or full IconButton acceptance.
  Linux tree/event checks alone do not establish reader acceptance.
- **Modal/cache qualification:** the approved guard repairs the bounded saved-action closing
  defect and active-AT cache omission. Seven timing cases and the actual cached/live fixture
  pass ([fixed receipt](receipts/overlay-contracts-20261025/action-guard-fixed-20261025/));
  the [original immediate-close failures](receipts/overlay-contracts-20261025/custom-actions-followup/README.md)
  remain historical evidence. Six extended raw/listener/epoch/passive/ancestor/two-window
  modes pass their sampled contracts. Cache performance and full reader/theme/host coverage remain open;
  the guard's lifecycle scope does not authorize arbitrary external topmost ordering.
- **Date policy:** the owner still decides the source of local today. `picker::today()` remains
  UTC; DateField defaults 1900–2100 with configurable Gregorian years 1–9999. Typed ISO entry
  exists; locale display and seconds are not implemented.
- **Windowing/hardware:** X11 scale/resize/origin and nested-Weston Wayland samples exist, using
  software rendering. Hardware/compositor diversity, live scale changes and monitor moves are
  unqualified. Wayland has no global window-origin hook.
- **Mobile:** approved GPUI 0.3.9 iOS scaffolding still fails hosted application compilation on
  missing core APIs ([blocker](proposals/gpui-ios-039-incompatibility.md)). No application linked;
  Linux checks do not qualify Apple runtime. Android has no backend decision. Neither mobile
  path has a verified AccessKit adapter.
- **Composition/identity:** caller-supplied generic children own their semantics; plain-text
  helpers are available for many slots. Repeated helper/loop instances need explicit stable IDs.
  Footer has an application-supplied bottom safe-area API; automatic host inset discovery remains application-owned.
- **API/release policy:** public naming mixes Navbar/Tabbar and SearchBar with hd/bd/ft helpers.
  Breaking naming, package/version policy and distribution authorization remain owner decisions.
  Approved Cargo path patches must be adopted at downstream workspace roots; current Linux
  qualification is for the recorded patched graph, not unmodified registry packages. A fresh
  package listing includes licenses/NOTICE but excludes nested vendor packages, so patch
  distribution remains unresolved for publication.

## Documentation status

GitHub private vulnerability reporting was verified enabled on October 24, 2026;
[SECURITY](../SECURITY.md) now links to the private report form. A separate conduct-enforcement
contact and maintainer/versioning policy still need owner decisions.

The macOS follow-up ran `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps
--locked --offline` at `e4a2c87`: exit 0, no rustdoc diagnostics. Cargo separately reports the
existing dependency `block 0.1.6` future-incompatibility warning. See the retained rustdoc log.

`RUSTDOCFLAGS='-W missing-docs' cargo doc --no-deps --locked --offline` after the `b563ed2`
documentation work reported **0 warnings**. That commit also reports a `-D warnings` pass, but
the committed log covers only the `-W` run and does not identify a final HEAD. The baseline
review counted 1,019 missing-docs warnings. Palette field docs are emitted by
`scripts/extract_weui.py`, so regeneration preserves them. Doc examples are marked `ignore` and
point to `examples/minimal.rs`, which `cargo build --all-targets` compiles. No doctests were
added.

## Not done / missing receipts

- Remaining macOS real-input checks: select navigation/focus, toast typing, undo/redo, UTF-16
  limits, Pinyin, edge probes, reduced motion, and an uninterrupted screen-reader pass.
  Build/launch and sampled AX checks are now recorded; a complete overlay recapture is not.
- Current-source Windows reruns beyond the historical Windows ARM component/inline-link
  samples; Windows x64; iOS/Android anything beyond type-checks; physical devices.
- Historical hosted CI: Linux stable/MSRV, mobile library checks and macOS native checks pass;
  simulator build fails inside published GPUI 0.3.9. See [CI receipt](receipts/ci-20261024/README.md).
  Default CodeQL also runs; no hosted CNB result is recorded.
- Packaging (app bundles, installers) on any platform. The historical package review listed
  172 files, 399 KiB compressed; that count does not describe the current patched graph.
  The new path-consumer build/runtime is separate from published-package qualification.
  The current package-list check passes but nested vendor packages are excluded; that does not
  supply a distributable patched crate. Nothing published. The README's screenshot pair uses repository-relative
  paths that are excluded from the package, so it renders only where the repository is reachable.
- Complete macOS recapture of all 264 pairs (the earlier Linux run has 192 sweep pairs, not comparable with macOS). Care mode beyond one route. Long pages beyond 812 pt.

## Owner decisions

Approved October 24 (historical):

- Linux `wayland`/`x11` features applied and verified under Xvfb (34 added lock packages).
- GPUI 0.3.9 pins and target-specific `gpui-pre-ios` are applied with an example bundle
  scaffold. Further Apple verification needs a separately authorized host.
- Seven focused Rust tests and five bundle-script fixture tests pass. The approved final
  GitHub workflow is configured after local verification; hosted results remain separate.

Approved October 25:

- Exact local AccessKit cache compatibility and additional state forwarding; GPUI 0.3.9 X11
  origin-hook repairs. The owner also approved and applied the modal action/cache guard and
  `gpui-pre-linux 0.3.9` Grab/Ungrab focus filter. Combined source gates and bounded native,
  actual Fcitx/Orca and Wayland results are recorded separately above. The matching GPUI
  workspace-root patch is required for compilation; all four paths are in vendor guidance.
  Package versions remain pinned.

Still pending:

- An upstream Android backend plan.
- crates.io name, publication timing, versioning and ongoing GPUI upgrade policy.
- Keeping the historical roughly 34 MB parity corpus in Git (excluded from the package).
- Private conduct/security contacts; a local-time source for `picker::today()`; the wider
  weui.js imperative API scope; resolution of the Linux step 2 rendering failure and approval for step 3 of the [GPUI text-breaking proposal](proposals/gpui-text-i18n/README.md).

No platform support or publication decision follows from the approvals above.

## Picking up the work

Read ACCEPTANCE, RELEASE-READINESS and recent receipts for current acceptance and source-gate evidence. The historical
October 24 checkout details below are retained as provenance, not current main.

At the October 24 macOS audit, the checkout was clean at `0c6555d` and GitHub advertised only
`main`. The handover named `claude/youthful-fermi-y23wn0` at `e23436c`; that commit is an ancestor
of `main`, but the branch was absent from `git ls-remote --heads origin` during this audit.
No branch was deleted here. Recheck current refs and working state before editing.

- **Run Linux locally or in a cloud container:** the approved windowing patch is applied.
  Run the Linux checks and use `scripts/linux/session.sh`, `scripts/linux/sweep.sh` and
  `scripts/contact_sheets.py` as described in [PARITY-TOOLING](PARITY-TOOLING.md).
- **Reproduce behaviour probes:** `receipts/takeover-20261025/probe.rs.txt` and
  `probe-edges.rs.txt` are the manual probe apps behind the before/after receipts. Copy one to
  `examples/probe.rs` locally to run it; they are not part of the crate.
- **Next verification:** qualify rendering, deep links and interactions on the published
  gallery from a host that can reach its origin. Further CI dispatch/deployment requires owner
  authorization. Preserve the separately verified native
  batch and its extended guard receipts; qualify cache performance and remaining matrices. Further
  vendor changes beyond the approved scope require approval. Extend sampled theme/state,
  nested-Weston and Orca checks to full desktop/component qualification;
  finish the remaining macOS input, IME, reduced-motion and paired-capture
  work on a separate Apple session; verify the implemented iOS scaffold on that host and label every Simulator
  result; obtain native Windows and Android evidence separately. A Linux container cannot
  supply Apple Metal tooling or macOS/iOS runtime acceptance.
- **Open defects:** the component audit findings above and the mobile work in
  [PLATFORMS](PLATFORMS.md) remain unfinished. The October 24 Mac follow-up did not change
  Rust component code or dependency pins.

[Sources](SOURCES.md) · [Coverage](UPSTREAM-COVERAGE.md) · [Visual evidence](PARITY.md) ·
[Review](REVIEW-2026-10-24.md) · [Platforms](PLATFORMS.md) · [Parity tooling](PARITY-TOOLING.md)
