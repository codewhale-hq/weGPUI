# weGPUI takeover — October 25, 2026

## Prompt for the next model

Take over weGPUI and finish its desktop-first developer release. **The goal is the entirety of Tencent WeUI 2.6.26 implemented as native Rust GPUI components**, including meaningful interaction, state, focus, accessibility, theming and documented APIs. This is a reusable library, not merely a gallery, a screenshot recreation, a website, or a Codewhale-specific UI. Keep upstream licensing and attribution.

**GitHub https://github.com/codewhale-hq/weGPUI is canonical. Push changes to GitHub `main`; CNB is the read-only mirror.** The GitHub About description incorrectly said the reverse and was corrected on October 24. The README and CONTRIBUTING already identify GitHub as canonical. Work from the existing checkout; inspect status before editing and preserve other work.

Read `docs/STATUS.md`, `docs/UPSTREAM-COVERAGE.md`, `docs/RELEASE-READINESS.md`, `docs/PLATFORMS.md`, `CONTRIBUTING.md` and the recent receipts. Read applicable AGENTS.md instructions. Audit current source against the pinned upstream package. All 61 upstream example files map to 60 routes plus the index shell, but route counts and historical screenshots **do not establish functional completeness**.

### Handoff state — 2026-10-25 component-contract continuation

The canonical checkout is `/workspace/weGPUI` on GitHub `main`. The first cloud continuation
was pushed as `9c4515c`. The separately requested vertical FormCell label-width fix was
verified and pushed as `a3ce3afca6417b7dd1b9d701456ae2d21d2150f4`: all five Rust 1.98.1 gates
passed, 24 tests passed and one doctest was ignored. No Mac build was requested or run here.
The temporary checkout used to isolate that small fix was removed; recovery bundle, gate log
and the path-only form stash are retained outside the repository at
`/workspace/.wegpui-cloud/backups/vertical-integration/`. No further development lane is needed.
The temporary branch reference remains only because branch deletion is prohibited.

The following larger component batch is independent of the unavailable uncommitted Mac work.
Use the receipts' exact artifact/source hashes: intermediate failures and old-artifact controls
are preserved, and runtime evidence is not automatically validation of every later source edit.

Mac coordination update from the app owner: the app pins reviewed `a3ce3af` using immutable
fetched dependency bytes. The canonical local `/Volumes/VIXinSSD/CW/weGPUI` is still at
`52650f3` with twelve dirty paths; the owner reports its Rust focus-reveal behavior is now
upstream and four dirty docs still need reconciliation. Exact patch/hash recovery inventory:
`/Volumes/VIXinSSD/CW/artifacts/whales-folder-20261025/local-wegpui-inventory.md`.
This cloud host has not inspected those local bytes. Preserve/reconcile that checkout before
future Mac builds and coordinate the single native build slot with the app/Engine owners.

- Controls: real callback/state checks cover Button/IconButton, Vcode, choices, Agree link
  independence and interrupted Switch motion across light/dark, care and custom brand combinations.
  Exact callback counts, failures/fixes, CSS samples and reviewed images are in
  `receipts/control-contracts-20261025/`.
- Input: native and AT selection/word/line behavior, long text-run chunks, password masking,
  UTF-16 limits, IME/undo contracts and scale/move geometry are in
  `receipts/input-contracts-20261025/`. The Linux bridge has no EditableText interface; native
  replacement of an AT-selected range is distinct from assistive text replacement.
- Forms: typed DateField, read-only/disabled/range/error behavior, optional-value events,
  label association, mode-switch focus, delayed disable focus and stale-sheet guards are in
  `receipts/form-contracts-20261025/`. `today()` remains UTC. Full-i32 year ranges use bounded
  lazy picker rows rather than allocating billions of items.
- Overlays: exact-once closing-frame callbacks, nested/passive layers, long Select navigation,
  picker key/wheel guards, actual focus rings, inline controls and short-sheet drag/viewport
  behavior are in `receipts/overlay-contracts-20261025/`.
- Content/media/search: controlled callbacks and accessibility, fixed notice/footer occlusion
  and modal hiding, progress announcements, article semantics, upload capacity/statuses,
  image viewer keys/delete target, swipe state, search clear/cancel/submit and homepage focus
  have separate receipts in `data-contracts-20261025/`, `media-contracts-20261025/` and
  `search-contracts-20261025/`.
- The owner explicitly approved **two sets of local dependency compatibility fixes**. Exact
  packages remain `accesskit_unix 0.22.1`, `accesskit_atspi_common 0.19.1` and `gpui-pre 0.3.9`.
  See `../vendor/README.md`, `proposals/accesskit-linux-compat/` and
  `proposals/a11y-state-and-bounds/`. Registry source was not modified; no package was added.
  Cache signal struct encoding, disabled/read-only and omitted state/heading export are
  repaired. GPUI calls its existing bounds hook; X11 SCREEN minus WINDOW extents now agree
  with direct client-origin translation at scales 1/2 and after moving. `xdotool` geometry
  double-counts decorations here; use raw `xwininfo`/XTranslateCoordinates for origin proof.
- Actual Orca repeated close/reopen no longer hits malformed cache signatures in the sampled
  sequence (`receipts/linux-bridge-20261025/`). Actual reader followups and real Wayland
  clients are separately recorded. Synthesized audio still needs human listening review;
  no component or desktop platform has full screen-reader acceptance. VoiceOver stays off.
- Cargo patches apply only at the application workspace root and are **not inherited** from
  this dependency. A downstream app must adopt equivalent configuration to use the fixes.
  Publication or hosted forks remain unauthorized. The small `a3ce3af` app pin does not
  include the larger compatibility/component batch.

Final integrated gates are all green on Rust/Cargo 1.98.1: fmt, locked all-target build,
47 tests (one doctest ignored), strict all-target Clippy and strict rustdoc. Exact commands,
preserved failures and final source hashes are in `receipts/component-contracts-20261025/`.
All transient probes were archived into receipts and removed from shipped examples.
The shared focus-contrast helpers preserve sufficient brand colours and use contrasting
fallbacks or dual bands for unknown backgrounds; per-artifact pixel review is separate.
DateField's actual Orca sequence now speaks each invalid attempt once, including repeated
submissions. Keep current-source macOS/Windows, full per-state paired parity, reader listening,
hardware desktop and fresh-consumer/package qualification before acceptance. The managed
network still blocks the pinned WeChat SDK; verified static CSS samples do not replace the
complete pinned JavaScript reference. Keep HUMAN-TASKS, ACCEPTANCE and release readiness current.

### Historical handoff — first 2026-10-25 Linux cloud continuation

GitHub main was fetched and safely checked at `52650f3`. The checkout was clean; the old
`work` branch was preserved and `main` tracks GitHub. No applicable AGENTS.md was found.
The later uncommitted Mac changes/results described by the owner were unavailable here;
these Linux source changes and receipts were produced independently.

- Rust/Cargo 1.98.1, gpui-pre 0.3.9, locked dependency graph unchanged. All five gates and
  24 tests pass (one doctest ignored); exact final commands/source are in
  `receipts/linux-continuation-20261025/`. Failed setup/build/probe attempts are retained.
- Filled missing RevealFocus registrations and the article link ring, with reusable-control
  and five-page short-viewport checks (`receipts/focus-reveal-20261025/`).
- Actual Orca speech revealed missing input Text runs, unlabeled Label values, cell value/desc
  omission and incorrectly grouped example radios. Fixed and tested; Unicode/grapheme,
  password-mask, multiline and reversed-selection contracts are covered. See
  `receipts/a11y-text-20261025/` and `receipts/orca-20261025/` for exact limits.
- Fixed lower-modal accessibility exposure; nested and sibling/deeper modal ordering,
  keyboard restoration and passive toast behavior have source/runtime evidence in
  `receipts/nested-modal-a11y-20261025/`. External application deferred priorities are outside
  the documented ordering contract.
- Real Wayland client runtime checks ran with DISPLAY absent under nested Weston, with
  protocol, keyboard, clipboard/undo, tree and resize evidence (`receipts/wayland-20261025/`).
  Hardware GPU/compositor, live scale/monitor moves and complete desktop qualification open.
- Reran and visually reviewed all 72 registered states in dark, light+care and light+brand
  7C3AED: 216 native-only images. No capture failures or confirmed native visual defect.
  Exact source patch, binary hash and review manifests are in `receipts/native-states-20261025/`.
  **No new paired parity pass:** the configured proxy blocked the pinned res.wx.qq.com SDK
  download. The earlier 61/72 run was not reused. `sweep.sh` now accepts --care/--brand/--only.
- Actual Orca collection now exists, but **no screen-reader/component acceptance is complete**.
  Reopened/restored modal objects become defunct in Orca; the pinned AccessKit stack emits
  malformed cache Add/Remove signal signatures. See `receipts/orca-20261025/cache-investigation.md`.
  Disabled-button export also remains unresolved upstream. No dependency patch/fork or
  upstream issue was made. Speech transcripts/PCM were inspected; listening/intelligibility
  review is still needed. VoiceOver stays off; NVDA/Narrator/current-source macOS/Windows open.

Next: finish the full acceptance matrix, obtain authorized resolution of the dependency/cache
blocker, review real synthesized speech with a human, unblock the pinned reference fetch and
rerun paired theme/state comparisons, then qualify remaining desktop/platform/release work.
Keep ACCEPTANCE and HUMAN-TASKS current; no row is accepted from these samples.
Publication/versioning/local date/website/signing decisions and release authorization remain
with the owner. DateField still uses UTC today. Never push to CNB or file issues without
approval. Continue useful work under the constraints below.

The older sections below are historical evidence/context, not the current source backlog.

### Verified stopping point

Source through `0039001` is committed and pushed. `b1e94e1` adds the completed Windows inline-link receipt and corrects the README platform summary.

- `5eabf41`: showcase checks cover 60 routes, 264 state/theme selections and 528 decoded images; API path resolution respects Cargo target configuration.
- `e5a8921`: loading-button activation/focus guards, panel focus indication, scoped radio IDs, vcode countdown/restart guard, spinner IDs/artwork, input label association, list/preview/dialog semantics and navbar/tabbar keyboard behavior. Added `examples/component_states.rs`.
- `3cf5bf5`: private GitHub vulnerability reporting verified enabled; SECURITY.md points to the private report form.
- `0039001`: inline links use shaped text bounds with separate keyboard/accessibility activation. Corrected a real bug where clicking the short link then Enter activated the adjacent wrapped link.
- macOS and Windows ARM: component builds, nine unit tests, strict Clippy/docs passed for the component slice; corrected inline-link builds/lints/docs and pointer/keyboard checks passed. A separate packaged macOS consumer built. Read exact source hashes and limits in `docs/receipts/component-fixes-20261024` and `docs/receipts/inline-links-20261024`.

### Update — 2026-10-24 late (Linux cloud agent)

Both defects below are implemented from verified source on GitHub `main` (the Mac candidate was
not available and was not used). See `docs/receipts/focus-date-20261024/` for the Linux X11
runtime sequences, gates and the picker key-during-animation panic found and fixed on the way.
Still open from this section: macOS/Windows reruns, `navbar::tab_panel` (caller-wrapped) (the home footer link focus is now done). `DateField` is now a library component. The acceptance inventory is
`docs/ACCEPTANCE.md`; work its P1 rows first. Human/other-machine items are in
`docs/HUMAN-TASKS.md`. Release-gate receipts done on Linux: `docs/receipts/release-gates-20261024/`.

### Update — 2026-10-24 later (Linux cloud agent)

P1 pass done on Linux: set positions fixed (off by one), filled-button focus ring, paste/undo
grouping, information-bar focus contrast, footer link focus, bottom-fixed fade inset; select,
dialog, forms, navbar/tabbar and a 15-page keyboard sweep recorded. Use openbox on Xvfb (see
PARITY-TOOLING) or inputs never get focus events. Next: nested modals, toast timing, zero-step
slider receipt, search-bar homepage variant, screen readers, other OSes.

### Historical handoff state — 2026-10-24 23:35 PDT

- Gates: fmt check, build all targets, tests (15 pass, 1 doc test ignored), strict clippy
  and strict rustdoc. All green at the last pushed commit (see `git log -1 origin/main`).
- Remaining work, in priority order:
  1. macOS/Windows reruns of the Linux fixes (HUMAN-TASKS 8).
  2. Screen readers: Orca/NVDA/Narrator (VoiceOver stays off).
  3. Wayland.
  4. Dark/care/brand state parity sweep. One was started and stopped at 61/72 dark states with
     0 FAIL, unreviewed and not committed. Rerun:
     `DISPLAY=:95 PYTHON=<venv with playwright+pillow> scripts/linux/sweep.sh OUT --theme dark --states`
     after `scripts/weui_upstream.py fetch` and `serve`, on `Xvfb :95 -screen 0 1600x3600x24`.
  5. Owner decision on a local date source.
  6. Release items in RELEASE-READINESS.
- Test setup (Debian box):
  - Packages: Xvfb, openbox, xdotool, ImageMagick, dbus, at-spi2-core and python3-pyatspi.
  - Start the display: `Xvfb :96 -screen 0 1280x1000x24 &` then `DISPLAY=:96 openbox &`.
    Without a window manager there are no focus events.
  - Run with `GPUI_X11_SCALE_FACTOR=1` and capture with `import -window <id>`.
  - AT-SPI: `dbus-run-session -- scripts/linux/session.sh BIN`.
  - Never `pkill -f` a pattern that also matches your own shell command line.
- Rules:
  - GitHub main is canonical; never push to CNB.
  - No force-push, history rewrite, branch deletion, tags, releases or deploys.
  - Gates green before every commit; unsigned commits say so; use `[skip ci]`.
  - No new dependencies, forks or registry patches without approval.
  - No external posts, including the AccessKit issue.
  - Keep failed evidence.

### Update — 2026-10-24 night (Linux cloud agent)

Fixed and verified on Linux: the nested-modal Tab trap (it was capture phase, so the outer modal
stole Tab; now bubble phase), the overlay-primary ring, search-bar focus (homepage, back,
clear, 取消) and swiped-action rings. Linux receipts cover button overlay/cell, form
validation, toast and toptips timing (2s), the zero-step slider (unit test; the API sanitises
the step), all P3 families (light/dark/care, AT-SPI and keyboard), the DateField AT-SPI tree,
scaling 1.5/2 and resize, and the STATUS defect rechecks. `component_states` has no initial
focus, so click a field before driving it with keys. Next: Orca/Wayland where a host allows,
macOS/Windows reruns (HUMAN-TASKS 8), and the owner decision on a local date source.

### Historical candidate: two defects subsequently implemented on GitHub main

This section preserves the original candidate design for context. The Linux implementations
are now on GitHub main (see the later receipts and ACCEPTANCE); these are no longer two
unimplemented source defects. macOS/Windows current-source reruns remain incomplete. The
owner also reported a later, locally validated but uncommitted Mac continuation on top of
`52650f3`; it was not available to this cloud session and supplies no cloud validation. The
October 25 cloud focus-registration changes were implemented independently from source.

1. **Focused controls below the viewport are not revealed.** On the gallery `preview` page at 480x650, Tab focuses the upper action, then the lower action stays offscreen. Manual scrolling reveals its focus ring. Reproduced on macOS, with corresponding Windows ring-loss evidence. Implement reusable focus visibility for ordinary GPUI vertical scrollers, integrate built-in controls and the gallery, and test forward/backward Tab, nested scopes if supported, no needless scrolling for visible targets, manual-scroll preservation and inputs. Do not hardcode a preview-page offset.
   - Local candidate adds public `scrolling::{FocusScroll, FocusTarget, RevealFocus}`. `FocusScroll` wraps an existing scroller/ScrollHandle; targets collect focused bounds during prepaint; a focus change minimally adjusts/clamps the offset and refreshes next frame. Built-in semantics/input wrappers and individual control leaves register targets. Gallery PageFrame and component_states use the scope.
   - Review this design carefully: GPUI `AnyElement::prepaint` returns the first newly assigned subtree focus, regular Div does not consume list autoscroll requests, offscreen/nested behavior needs actual runtime proof. A geometry unit test alone is insufficient.
2. **Input-page date/datetime controls are inert.** `examples/gallery/pages/input.rs::picker_field` in committed source has focus styling but no click handler. Upstream uses actual HTML `date` and `datetime-local` inputs. Implement a native editable equivalent with accessible names, confirm/cancel and focus restoration.
   - Local candidate wires existing Picker entities: date selection, then a separate hour/minute sheet for datetime; only final confirmation updates datetime. It uses a 1900–2100 date range and must document that adaptation, verify cancellation/reopening, sequencing and performance. It is not verified or claimed equivalent to every browser date-input behavior.

The candidate also adds keyboard focus to the gallery home-footer link and wraps more raw gallery controls. The original Windows focus-scroll transfer was stopped before a build; its partial guest
archive is unusable. A later dispatched Windows ARM rerun has no established completed result.

### Finish the entire scope

Build a component-by-component acceptance inventory against upstream CSS, HTML and relevant weui.js behavior. Cover every family in UPSTREAM-COVERAGE, every public variant, controlled callbacks, enabled/disabled/loading/error states, keyboard operation, focus visibility/restoration, accessible roles/names/values and light/dark/care-mode layout. Distinguish intentionally static upstream demos from broken callbacks. A visually matching inert control is not complete. Keep gallery examples usable and reusable library contracts clear.

Prioritize known STATUS findings and runtime failures, then audit untested families. Preserve failed attempts and precise source/OS/toolchain evidence. Add focused regression tests for meaningful contracts, and native interaction checks for behavior that unit tests cannot prove. Run fmt, all example builds, library tests, strict all-target Clippy and strict rustdoc on coherent source batches.

The owner selected **macOS, Windows and Linux for the first release**. Mobile remains experimental: iOS 0.3.9 hosted compilation has upstream API mismatches; Android lacks a backend. Do not quietly shrink WeUI feature coverage, falsely claim mobile support, patch the Cargo registry, or introduce an unapproved dependency fork.

Remaining desktop qualification includes full family/state testing, IME/selection/clipboard/undo, actual screen-reader acceptance, Windows x64, Linux Wayland/Orca, scaling/resizing and fresh-machine distribution. VoiceOver must remain off under the owner's existing instruction. An accessibility tree is not a screen-reader pass.

Finish developer onboarding, examples, package contents/licenses, versioned API docs, compatibility/release notes and the shareable website. Rebuild docs and website from the same reviewed source; current local website captures are historical, not current native parity acceptance. Re-run website checks including API links. No public website deployment, crates.io publication, signing/notarization, infrastructure spend or final CI dispatch occurred in this continuation. Obtain the required release authorization before those actions. Keep the label developer preview until the actual gates pass.

The owner authorized a dedicated Windows subagent so Windows setup does not block core work. Use separate file ownership and preserve the main agent's integration responsibility. Continue the work autonomously; don't stop after an audit or claim all of WeUI is finished from route coverage alone.
