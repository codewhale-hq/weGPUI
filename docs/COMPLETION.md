# Work remaining for the first desktop release

The release scope is macOS, Windows and Linux. iOS and Android are experimental and do not
block this release. A source build, a gallery route, or a screenshot alone does not complete
component acceptance. The criteria in [ACCEPTANCE](ACCEPTANCE.md),
[PLATFORMS](PLATFORMS.md) and [RELEASE-READINESS](RELEASE-READINESS.md) still apply.

## Current continuation

The [RTL continuation](receipts/rtl-completion-20261025/README.md) follows the owner’s step-3
approval. It implements row-aware wrapped input selection/carets and TextArea alignment,
normalises platform glyph order, and records fresh macOS/Linux samples. Complete desktop
acceptance still requires the host, reader, IME and distribution work below.

### Earlier completion work

The later [WeUI parity execution](WEUI-PARITY-PLAN.md) extends this work with an exact upstream
inventory, complete appearance/state paired capture sweeps, a footer visible-geometry correction
and the separately approved trailing-space guard trial. Its current-source checks and open
differences have their own [receipt](receipts/weui-parity-20261025/README.md). The preview bundle
below remains frozen at its earlier source; it is not silently replaced by the trial. The
[October 26 follow-up](receipts/weui-parity-20261026/README.md) records further typography,
Layers and approved CJK corrections, plus expanded native interaction checks.

The new work is independent of the held logical-wrap prototype. It uses canonical `b1ddcae`
plus the footer, agreement, neutral swipe-action, application-example and preview-bundle
changes, plus a website route-inventory repair. The exact source, checks and limitations are recorded in
[the completion-work receipt](receipts/completion-20261025/README.md).

- Footer links have at least 44×44 logical-pixel targets and wrap in narrow rows. Applications
  can supply bottom safe-area padding with `Footer::safe_area_bottom`.
- Neutral swipe actions explicitly use the theme foreground instead of inheriting white text.
- Long agreement text can shrink and wrap while the check keeps its width.
- `examples/application.rs` demonstrates validation, controlled state, theme changes, image
  selection/preview/deletion and confirmation with modal focus restoration.
- [The preview source bundle](PREVIEW-DISTRIBUTION.md) includes all four same-version Cargo
  compatibility copies, a separate consumer, source hashes, licences and an integrity manifest.
  It supplies a reproducible way to try the preview; it is not a signed installer or a
  publication of replacement crates.
- The website builder and checker now account for the native-only Languages route. Five
  regression checks and both full Chromium mounts pass: 42 checks and 360 route/viewport
  observations. Deployed-origin, other-engine and actual-reader acceptance remain open.

## Remaining work, in dependency order

| Work | Completion evidence needed | Where it can be done |
| --- | --- | --- |
| Qualify right-to-left wrapping | Fresh macOS and Linux ARM64 X11 samples are readable; re-run Linux x86_64/hardware and Windows; extend the passing styled Hebrew sample to complete complex-script/break cases | Current receipt identifies the sampled hosts; full matrix remains open |
| Qualify wrapped caret and partial selection | Step 3 is approved; row-aware input mapping is implemented. Complete real RTL IME, reader and desktop matrices; shared text-owning GPUI mapping is implemented | macOS and Linux samples are in the RTL continuation |
| Localisation details | Locale-aware numeric date display/entry/column order and host-local today are implemented with the approved locked crates; qualify translations and dictionary-breaking scripts | Native locale review; CLDR, alternate calendars and dictionary segmentation remain outside this bounded policy |
| Review translations | Native-speaker review of the twelve starter catalogs and keyboard/reader checks for the promised locales | Qualified language reviewers and native hosts |
| Complete component behaviour | Each interactive family's enabled/disabled/loading/error, callbacks, keyboard, reduced-motion, focus and state-transition cases | Linux automation can extend coverage; current-source macOS/Windows runs are still required |
| Complete rendering parity | Full light/dark/care/custom-brand/state/reference review, responsive widths and long pages | Each desktop renderer, with fonts and source provenance recorded |
| Qualify accessibility | Actual Orca, NVDA/Narrator and the authorized macOS reader process; full reading/navigation/modal/announcement checks and human review | Native hosts and a human listener; the existing instruction keeps VoiceOver off |
| Qualify input/windowing | Clipboard, undo/redo, UTF-16 limits, RTL and real IME; X11/Wayland, hardware compositors, monitor moves and scale changes | Native desktop sessions and hardware |
| Qualify distribution | Fresh-machine consumer and installer runs on all three desktop systems; signing/notarization requirements; a distributable dependency policy for the patched GPUI graph | Native build/signing hosts and maintainer decisions |
| Set release policy | Package name/version, upgrade/support policy, maintainers, conduct contact, publication timing and review | Owner/maintainer decisions; do not invent contacts or publish to settle them |
| Qualify the public website | Reachable deployed-origin checks, additional browser engines, keyboard and actual reader use | A host that can reach the deployed origin; another publication is not required for source-only checks |

Every component row remains unaccepted until its full desktop matrix passes. The preview
source bundle does not waive those gates. Application-owned upload transport, storage, retry
policy and authentication are integration responsibilities, rather than missing library APIs.

## RTL prototype publication

The owner initially chose **Hold landing** for step 2 on 2026-10-25. On October 26 the owner
requested complete review and publication of all current work to GitHub. The
[publication review](REVIEW-2026-10-26.md) records that later authorization; the original
[receipt](receipts/gpui-logical-wrap-20261025/README.md) remains a historical record of the hold.
Linux clipping/misalignment is still a release-acceptance blocker. The logical implementation
was not revised by that publication review. The later RTL continuation has step-3 approval;
new library dependencies remain a separate decision. Repository publication does
not authorize a package release, website deployment or another platform-support claim.

## Experimental mobile work

iOS still fails in the pinned upstream application APIs and has no runtime qualification.
Android has no selected upstream backend. Both need backend/API work, real device or simulator
builds, rendering, input, accessibility and distribution evidence before support can be claimed.
These are distinct projects after the desktop release scope, not boxes a Linux build can close.

The [current code/parity receipt](receipts/code-parity-20261026/README.md) records shared
Unicode bidi mapping, styled painting, punctuation, shaped clamping and locale dates, with
Windows held to build/tests. The table above tracks the remaining qualification work.
