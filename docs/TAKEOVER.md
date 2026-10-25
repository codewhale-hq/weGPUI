# weGPUI takeover — October 24, 2026

## Prompt for the next model

Take over weGPUI and finish its desktop-first developer release. **The goal is the entirety of Tencent WeUI 2.6.26 implemented as native Rust GPUI components**, including meaningful interaction, state, focus, accessibility, theming and documented APIs. This is a reusable library, not merely a gallery, a screenshot recreation, a website, or a Codewhale-specific UI. Keep upstream licensing and attribution.

**GitHub https://github.com/codewhale-hq/weGPUI is canonical. Push changes to GitHub `main`; CNB is the read-only mirror.** The GitHub About description incorrectly said the reverse and was corrected on October 24. The README and CONTRIBUTING already identify GitHub as canonical. Work from the existing checkout; inspect status before editing and preserve other work.

Read `docs/STATUS.md`, `docs/UPSTREAM-COVERAGE.md`, `docs/RELEASE-READINESS.md`, `docs/PLATFORMS.md`, `CONTRIBUTING.md` and the recent receipts. Read applicable AGENTS.md instructions. Audit current source against the pinned upstream package. All 61 upstream example files map to 60 routes plus the index shell, but route counts and historical screenshots **do not establish functional completeness**.

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

### Handoff state — 2026-10-24 23:35 PDT

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

### First work: two unfinished defects

The previous Mac checkout contains an **uncommitted, uncompiled candidate**, not part of GitHub `main`. If using another machine, do not assume these changes arrived with `git pull`. Either obtain that candidate from the owner or implement these fixes from the verified source:

1. **Focused controls below the viewport are not revealed.** On the gallery `preview` page at 480x650, Tab focuses the upper action, then the lower action stays offscreen. Manual scrolling reveals its focus ring. Reproduced on macOS, with corresponding Windows ring-loss evidence. Implement reusable focus visibility for ordinary GPUI vertical scrollers, integrate built-in controls and the gallery, and test forward/backward Tab, nested scopes if supported, no needless scrolling for visible targets, manual-scroll preservation and inputs. Do not hardcode a preview-page offset.
   - Local candidate adds public `scrolling::{FocusScroll, FocusTarget, RevealFocus}`. `FocusScroll` wraps an existing scroller/ScrollHandle; targets collect focused bounds during prepaint; a focus change minimally adjusts/clamps the offset and refreshes next frame. Built-in semantics/input wrappers and individual control leaves register targets. Gallery PageFrame and component_states use the scope.
   - Review this design carefully: GPUI `AnyElement::prepaint` returns the first newly assigned subtree focus, regular Div does not consume list autoscroll requests, offscreen/nested behavior needs actual runtime proof. A geometry unit test alone is insufficient.
2. **Input-page date/datetime controls are inert.** `examples/gallery/pages/input.rs::picker_field` in committed source has focus styling but no click handler. Upstream uses actual HTML `date` and `datetime-local` inputs. Implement a native editable equivalent with accessible names, confirm/cancel and focus restoration.
   - Local candidate wires existing Picker entities: date selection, then a separate hour/minute sheet for datetime; only final confirmation updates datetime. It uses a 1900–2100 date range and must document that adaptation, verify cancellation/reopening, sequencing and performance. It is not verified or claimed equivalent to every browser date-input behavior.

The candidate also adds keyboard focus to the gallery home-footer link and wraps more raw gallery controls. **Compile and test before committing any of it.** Windows focus-scroll transfer was stopped before any build; a partial guest archive is unusable.

### Finish the entire scope

Build a component-by-component acceptance inventory against upstream CSS, HTML and relevant weui.js behavior. Cover every family in UPSTREAM-COVERAGE, every public variant, controlled callbacks, enabled/disabled/loading/error states, keyboard operation, focus visibility/restoration, accessible roles/names/values and light/dark/care-mode layout. Distinguish intentionally static upstream demos from broken callbacks. A visually matching inert control is not complete. Keep gallery examples usable and reusable library contracts clear.

Prioritize known STATUS findings and runtime failures, then audit untested families. Preserve failed attempts and precise source/OS/toolchain evidence. Add focused regression tests for meaningful contracts, and native interaction checks for behavior that unit tests cannot prove. Run fmt, all example builds, library tests, strict all-target Clippy and strict rustdoc on coherent source batches.

The owner selected **macOS, Windows and Linux for the first release**. Mobile remains experimental: iOS 0.3.9 hosted compilation has upstream API mismatches; Android lacks a backend. Do not quietly shrink WeUI feature coverage, falsely claim mobile support, patch the Cargo registry, or introduce an unapproved dependency fork.

Remaining desktop qualification includes full family/state testing, IME/selection/clipboard/undo, actual screen-reader acceptance, Windows x64, Linux Wayland/Orca, scaling/resizing and fresh-machine distribution. VoiceOver must remain off under the owner's existing instruction. An accessibility tree is not a screen-reader pass.

Finish developer onboarding, examples, package contents/licenses, versioned API docs, compatibility/release notes and the shareable website. Rebuild docs and website from the same reviewed source; current local website captures are historical, not current native parity acceptance. Re-run website checks including API links. No public website deployment, crates.io publication, signing/notarization, infrastructure spend or final CI dispatch occurred in this continuation. Obtain the required release authorization before those actions. Keep the label developer preview until the actual gates pass.

The owner authorized a dedicated Windows subagent so Windows setup does not block core work. Use separate file ownership and preserve the main agent's integration responsibility. Continue the work autonomously; don't stop after an audit or claim all of WeUI is finished from route coverage alone.
