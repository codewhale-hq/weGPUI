# Status — 2026-10-25

weGPUI is an independent Rust library and gallery port of WeUI 2.6.26 for GPUI. GitHub is the
canonical contributor home; CNB is a mirror. All 61 upstream example files have gallery
mappings. Visual, behavioural and accessibility parity is **sampled, not complete**.

The target platforms are macOS, Windows, Linux, iOS and Android. **No platform is supported yet**
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

## Platform acceptance matrix

| Platform | Build | Launch | Rendering | Interaction | Accessibility | Packaging |
| --- | --- | --- | --- | --- | --- | --- |
| macOS arm64 | Hosted arm64 build, seven tests and Clippy pass at `05cae89`; historical strict docs at `e4a2c87` | Historical minimal/gallery launches at `e4a2c87`; no current runtime requalification | Earlier 28 review pairs; new diagnostic images, paired recapture incomplete | Minimal dialog open/close/OK and focus restore; select-key attempts inconclusive | Modal isolation, option AXTitle labels, closed swipe actions hidden, article headings/lists; VoiceOver incomplete | Missing |
| Linux x86_64 | Fresh build, clippy `-D warnings`, fmt, strict doc pass (Rust 1.99.1); 1.95 build and seven tests pass | Both examples pass with committed windowing features under isolated Xvfb | Software-rendered X11; 192 pairs swept against Linux Chromium (60 routes light and dark, 72 states light; contact-sheet review) | XTEST click/keys, fcitx5 Pinyin, GTK portal chooser | AT-SPI tree via pyatspi; no Orca | Missing |
| Windows | Historical mingw cross-build links | Missing; Wine failure reported without a committed run log | Missing | Missing | Missing | Missing |
| iOS | Fresh library type-checks pass; hosted app build fails in upstream iOS/core APIs | 0.3.9 scaffold wired; upstream compilation blocker, no launch | Missing | Missing | Missing; backend lacks AccessKit | Missing |
| Android | Fresh 0.3.9 library type-check; no app build | Blocked: no backend upstream | Missing | Missing | Missing | Missing |

## Done

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

"Matches" names the exercised behaviour only. Linux rows used the uncommitted X11 build.

| Family | Result | Evidence and remaining difference |
| --- | --- | --- |
| Action sheet | Partly matches | macOS open states and corner fix. Shares the Presence/ModalScope fixes. Dismissal matrix and timing untested. |
| Article | Partly matches | Linux AT-SPI and fresh macOS AX: headings and list items exposed. Heading level is set, but `accesskit_atspi_common 0.19.1` does not forward it. Nested markers in `list_none` fixed in source. |
| Badge | Untested | Static examples only. |
| Button / Agree | Partly tested | Linux click on 普通型 navigates; Tab focus ring visible. Accessible-name fix is source-only. Loading/disabled/consent matrix not driven. |
| Dialog | Partly matches | Linux AT-SPI and fresh macOS minimal-dialog isolation. Minimal real-input OK increments the counter and restores focus; gallery `--state 1` was an invalid probe, not an open-dialog check. |
| Flex | Untested | Responsive resizing not exercised. |
| Footer | Untested | Callbacks empty in the gallery. |
| Form / input / textarea | Partly matches; differs | Linux real keys: typing, undo/redo, UTF-16 limit, Pinyin commit/cancel (single line and multiline). macOS TextArea Pinyin. Numeric filtering is not HTML validation. Preedit is not inline on Linux XIM. |
| Form choices | Matches samples | macOS radio/switch samples; Linux AT-SPI switch tree. Checkbox/Agree matrix untested. |
| Form select | Differs (adaptation) | HTML `<select>` is an overlay menu. Linux real-key bounded navigation and Escape focus restore verified. Fresh macOS AX isolation and option titles pass; attempted key sequence did not open the menu and is inconclusive. |
| Gallery / image viewer | Matches samples | Historical macOS close/delete. Linux committed captures show `uploader:viewer` startup state; they do not prove a thumbnail click. Pinch/zoom untested. |
| Grid | Untested | Example callbacks are empty. |
| Half-screen dialog | Matches samples | Historical macOS open/Tab/Escape/drag; fresh macOS AX isolation. Linux AT-SPI isolation and interrupted-transition frames. |
| Icons | Untested | Assets reproduce from the tarball. |
| Information bar | Matches sample | macOS close. Link/floating variants untested. |
| List / swiped cell | Partly matches | Historical macOS drag. Linux AT-SPI and fresh macOS AX hide closed actions. Delete callback empty upstream and in the gallery. |
| Loading | Untested | Timing and live reduced-motion changes unverified. All spinner kinds draw the same SVG. |
| Loadmore | Untested | Presentation only. |
| Message / LinkText | Partly tested | Probe: two links, separate focus and single activation. Message callbacks empty. |
| Navbar / Tabbar | Matches samples | macOS arrow selection after fix. Not re-driven on Linux. |
| Panel | Untested | CJK punctuation and clamping differ. |
| Picker | Partly matches; differs | Linux AT-SPI option names present; fresh macOS raw AXTitle contains the names omitted by the old dump. Disabled-default/all-disabled interaction evidence remains Linux-only. `today()` is UTC, while the example uses local time. weui.js throws on all-disabled columns; weGPUI disables Confirm. |
| Preview | Untested | Callbacks empty. |
| Progress | Untested | NaN fix is source-only. |
| Search bar | Partly matches | Linux single-line Pinyin commit/cancel. macOS typing/results. |
| Slider | Matches samples | Historical macOS drag/keys; Linux non-finite-input probe. Zero-step fallback is implemented and included in probe source, without a committed interaction receipt. |
| Steps | Untested | Static rendering. |
| Toast / toptips | Partly matches | Fresh macOS tree keeps page controls and one named toast node; alert subrole and typing not verified. Linux: passive toast keeps focus and the page tree (as upstream's `role="alert"`); uploader error tip. Timing untested. |
| Uploader | Partly matches | Linux chooser lifecycle (picked, error, cancel, no portal). Transport is application-owned, as upstream. |
| Home / layers / index | Untested | Gallery-specific surfaces. |

## Known unresolved defects and gaps

- **Linux windowing:** approved `wayland`/`x11` features applied (34 added lock packages).
  Both examples launch under isolated Xvfb. Wayland runtime and Orca remain untested.
- **Mobile:** the approved GPUI 0.3.9 plus target-specific `gpui-pre-ios` path and simulator
  bundle scaffold are implemented. Hosted simulator compilation found six missing upstream
  core APIs; see [the blocker](proposals/gpui-ios-039-incompatibility.md). No application linked.
  Linux checks do not qualify an Apple app. Historically, Xcode 26.2
  and an iOS 26.2 Simulator runtime were available during setup; no app was built or launched.
  Android still needs a backend decision. Neither path has a verified mobile AccessKit adapter.
- **Screen readers:** VoiceOver was attempted, but announcement retrieval timed out and the
  pass was stopped; it ended disabled. No usable VoiceOver transcript or Narrator/NVDA, Orca
  or TalkBack session exists. Text
  inside dialogs (title/body) is not exposed as separate nodes; the dialog is named by its title.
- **Local date:** `picker::today()` is UTC. A local-time source needs a dependency (for example
  `chrono`, already in `Cargo.lock` transitively) or platform calls. That needs an owner decision.
- **Linux XIM preedit** is drawn by the input method at the window's top-left, not inline at the
  caret. Not isolated between GPUI and fcitx5.
- **Found during the documentation audit, unfixed** (rustdoc is not uniformly accurate): buttons in cell
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
- Native Windows build/run; iOS/Android anything beyond type-checks; physical devices.
- Hosted CI: Linux stable/MSRV, mobile library checks and macOS native checks pass;
  simulator build fails inside published GPUI 0.3.9. See [CI receipt](receipts/ci-20261024/README.md).
  Default CodeQL also runs; no hosted CNB result is recorded.
- Packaging (app bundles, installers) on any platform. `cargo package --list` reviewed: 172 files,
  399 KiB compressed. Nothing published. The README's screenshot pair uses repository-relative
  paths that are excluded from the package, so it renders only where the repository is reachable.
- Complete macOS recapture of all 264 pairs (Linux has 192 sweep pairs, not comparable with macOS). Care mode beyond one route. Long pages beyond 812 pt.

## Owner decisions

Approved October 24:

- Linux `wayland`/`x11` features applied and verified under Xvfb (34 added lock packages).
- GPUI 0.3.9 pins and target-specific `gpui-pre-ios` are applied with an example bundle
  scaffold. Further Apple verification needs a separately authorized host.
- Seven focused Rust tests and five bundle-script fixture tests pass. The approved final
  GitHub workflow is configured after local verification; hosted results remain separate.

Still pending:

- An upstream Android backend plan.
- crates.io name, publication timing, versioning and ongoing GPUI upgrade policy.
- Keeping the historical roughly 34 MB parity corpus in Git (excluded from the package).
- Localisation/RTL scope; private conduct/security contacts; a local-time source for
  `picker::today()`; the wider weui.js imperative API scope.

No platform support or publication decision follows from the approvals above.

## Picking up the work

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
- **Next verification:** extend the sampled isolated X11 checks to Wayland and Orca; finish the remaining macOS input, IME, reduced-motion and paired-capture
  work on a separate Apple session; verify the implemented iOS scaffold on that host and label every Simulator
  result; obtain native Windows and Android evidence separately. A Linux container cannot
  supply Apple Metal tooling or macOS/iOS runtime acceptance.
- **Open defects:** the component audit findings above and the mobile work in
  [PLATFORMS](PLATFORMS.md) remain unfinished. The October 24 Mac follow-up did not change
  Rust component code or dependency pins.

[Sources](SOURCES.md) · [Coverage](UPSTREAM-COVERAGE.md) · [Visual evidence](PARITY.md) ·
[Review](REVIEW-2026-10-24.md) · [Platforms](PLATFORMS.md) · [Parity tooling](PARITY-TOOLING.md)
