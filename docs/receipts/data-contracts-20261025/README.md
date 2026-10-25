# Data, navigation and content contracts — 2026-10-25

This receipt separates source parity, native interaction, AT-SPI trees/events and screen-reader
acceptance. **No component row is accepted by this receipt.** macOS, Windows, complete theme/state
coverage and actual screen-reader qualification remain open.

## Implemented contracts

- `Progress::cancelable(false)` removes the cancel icon, hit target, focus stop and accessibility
  node. `Progress::announcement(text)` exposes caller-controlled assertive live text independently
  of the numeric value. Callers control milestone/throttling policy, matching the upstream
  example's application-owned updates. The gallery now uses this library API once, on its first
  bar, in place of the separate implicit-live status node.
- `InformationBar::floating(true)` uses viewport coordinates (8px top/left/right), deferred above
  ordinary modals, independently of a parent's inset or scrolling. Its content has explicit live
  semantics; its link/close controls opt out of inherited live announcements. Fixed notice/footer
  layers occlude content beneath them so one pointer/AT click cannot activate an underlying row.
- `Footer::fixed_bottom()` uses the viewport bottom and full width with 16px vertical padding.
  A shared `ModalScope` flag hides deferred footer links from accessibility while a modal covers the page. New
  `Footer::id` gives repeated helpers a stable scope; `text_line` exposes plain footer text.
  Safe-area insets and expanded link hit areas remain limitations.
- `Navbar::label` names a tab list, matching the existing Tabbar naming API. Selection remains
  controlled by the application. The fixture records pointer, arrow/Home and AT action callbacks.
- `Cells::title_text` / `tips_text` add accessible plain-text alternatives to the compositional
  slots. The Label title uses its value, as required by the pinned AccessKit bridge.
- Article adds `paragraph_text`, `h6`, `heading_with_id` and `ArticleList::id`. Default heading/list
  ids derive from caller location rather than content, so identical siblings are not dropped and
  changing text does not change their identity. Loops/helpers still require distinct explicit ids.
  The gallery article now exposes its body paragraph and both images' upstream `配图` labels.
- Article h1–h3, panel headers, step titles and message titles use weight 500 (`MEDIUM`), matching
  the pinned CSS rather than the previous weight 600 (`SEMIBOLD`). This source correction does
  not establish full visual parity or cross-font equivalence.

Existing MediaBox, Cell, Grid and FooterLink callbacks are exercised with application-owned
state. Static upstream demo callbacks were not given invented behavior. The fixture also
exercises the separately implemented controlled Slider actions, snapping and edit guards.

## Source references

Compared against verified WeUI 2.6.26 package files under
`target-198/weui-upstream/weui/package` (the npm tarball SHA-256 is
`f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435`):

- `src/example/progress/progress.html`, `src/style/widget/weui-progress/weui-progress.less`:
  separate operation element and visually hidden alert; application-owned announcement timing.
- `src/style/widget/weui-tips/weui-information-bar.less`: fixed viewport insets and z-index 5500.
- `src/style/widget/weui-footer/weui-footer.less`: fixed viewport bottom and 16px padding.
- `src/style/widget/weui-page/weui-article.less`: h1–h6, paragraph margins and weight 500 headings.
- `src/example/article/article.html`: body text and image alternative labels.
- `src/style/widget/weui-panel/weui-panel.less`, `weui-steps/weui-steps.less`, and
  `weui-page/weui-msg.less`: weight 500 titles.

Native diagnostic fixture: [data_contracts_probe.rs](data_contracts_probe.rs), SHA-256
`fdb6d445e8eb08e3f68ca439914d09cf6e9b37b29375b8a3b8e1e18191363493`. Its temporary copy under
`examples/` was removed after exact archive verification; no public diagnostic target is shipped.
To rebuild it, copy this archive to `examples/data_contracts_probe.rs`, build that example with the
documented locked toolchain and remove the temporary copy after verification. Existing captured
binaries can be rerun without restoring the example; the session records the archived fixture path.
[run.sh](run.sh), [session.sh](session.sh), [action.py](action.py), [events.py](events.py) and
[validate.py](validate.py) reproduce the procedure in the configured Linux container. Each
invocation makes a new directory and preserves its helper copies, source diff, probe source,
binary hashes, Rust/OS versions, screenshots, tree snapshots, callback log and announcement events.
Rust is 1.98.1 and GPUI's version remains 0.3.9. Later integrated-backend runs use the owner's
approved `vendor/gpui_pre` path patch coordinated by the integrating agent; they are distinguished
below from the first two runs. New runs also preserve Cargo manifests and per-file source/vendor
SHA-256 manifests, including untracked source helpers, since the original `src`/`examples` diff
alone does not describe all build inputs. X11 interactions use scale 1 and Xvfb **with Openbox**.

## Preserved failed runs

| Run | Result and correction |
| --- | --- |
| `run-20261025T081914Z-QzkQ3n` | Initial assertion found the new Cells title unnamed. Fixed Role::Label to export its value. No runtime acceptance from this attempt. |
| `run-20261025T081949Z-zp4oNh` | Same failure with tree/image preservation enabled. Tree shows `label ''`; live-event log confirms the need to opt notice action labels out of inherited announcements. |
| `run-20261025T082500Z-PQGWOH` | Static/geometry assertions passed. Pointer driver wrongly treated GPUI's client-local AT bounds as X root coordinates, causing native keys to lose OS focus. Partial AT action/value observations are retained; native callback claims are not taken from this run. |
| `run-20261025T082712Z-94kMKg` | Corrected X root offset/activation produced real pointer/key callbacks. It found a real fixed-footer click-through: activating the footer also activated the covered “Open data modal” button. Added pointer occlusion to fixed footer and floating notice; this run is before that fix. |

The corrected driver used for the two complete runs below adds the actual X client origin from
`xdotool getwindowgeometry` and activates the window before driving focus/keys. This compensates
for that binary's GPUI/AT-SPI DESKTOP-coordinate origin limitation; those runs do not qualify
desktop accessibility geometry. The current helper instead subtracts the AT Frame origin and
drives `mousemove --window`, so it also works after the integrating agent's origin-hook repair.
Each run retains its own helper copy. Each AT operation uses a fresh client to avoid conflating
callbacks with the separately recorded AccessKit cache-signal/reopen blocker.

## Verification results

Both post-occlusion Linux X11 sequences completed with exit 0 and passed
[validate.py](validate.py):

| Theme | Complete run | Callback events |
| --- | --- | --- |
| Light | [run-20261025T084009Z-Xdmtch](run-20261025T084009Z-Xdmtch/) | 24 |
| Dark | [run-20261025T084217Z-Q2m5Gh](run-20261025T084217Z-Q2m5Gh/) | 24 |

Each run proves progress 25→50 and cancel visibility/removal; controlled Navbar pointer,
arrow/Home and AT selection; controlled Tabbar keyboard/AT selection; Slider End snapping
0..10/step 3 to 9, fractional AT/keyboard updates and disabled/read-only guards; actual
MediaBox/Cell/Grid/FooterLink keyboard callbacks; floating notice link/close callbacks; and
fixed-footer pointer occlusion, modal AT isolation and restoration. In the occlusion regression,
only `fixed-footer-open` fires when the footer overlays the modal trigger; `modal-open` fires
once after that trigger is separately revealed and activated. The notice stays at (8,8), width
584 in the 600px client while its inset parent moves from y80 to y−204.

The event stream contains `Floating data notice`, `Upload 25 percent`, `Loading data records`,
`Upload 50 percent` and `Upload canceled`, and contains no separate `Notice details` or
`Close notice` live notifications. Restoring hidden page content after the modal closes also
re-emits live text; this receipt does not claim exactly-once announcements across modal lifetime.
The gallery tree exposes its body paragraph and both `配图` images.

Both runs used the same binaries: data probe SHA-256
`2c5454e47fef195e30eb0d79930a583a0127b55599c8c13e44563333fa11b48c`, gallery SHA-256
`6b2f6f8ca0b10815fb55583dc4f6c4f0b291b79eca9c77f46329f7d8b006595c`. They were built by the
integrating agent from baseline `9c4515c788bbacc42a166534e39dc393ddbcb30b` plus the integrated
working source. The light snapshot's `src`/`examples` diff SHA-256 is
`bcfbf81a494a6bea4bfa10dc2a566e6f0e56951bac17656444e826fe21cf1f70`. The dark snapshot hash
is `cc40c7826ecf3173e0c5b3958f1a23899daf2aa9c6260a3d07f322da25e62329`: only a concurrent
Picker focus-style change differs, and that change is absent from these unchanged binaries.
No Picker runtime claim is taken from these runs. Exact provenance, helper copies, diff and
fixture source are retained in each directory. Environment: Rust 1.98.1, GPUI 0.3.9,
Debian 13.5 x86_64, Xvfb :106 1280×1200×24, Openbox 3.6.1 and X11 scale 1.

A later integrated-backend light replay,
[run-20261025T085656Z-BNpbsD](run-20261025T085656Z-BNpbsD/), also completed with exit 0 and
passed the full data-probe validator with the same 24 callbacks. It explicitly skips the gallery,
which at that time awaited the image correction's separate rebuild. Probe SHA-256:
`d8c77dca812fc307cae9bb1f5e72537b892258ad3cb060ce6a00b2add8465186`; source snapshot diff:
`a3153c36795b947c593a609a200ff07b53eaefa8b224062bd57dde48e8db3f6f`. Its current AT tree adds
`level:6` for each identical h6 heading, explicitly marks the read-only slider `read only`, and
omits `enabled`/`sensitive`/`focusable` from the disabled slider. The repaired DESKTOP origin is
observed at Frame (340,157), notice (348,165); the frame-relative pointer driver reaches the
cancel button at local (572,156) and Navbar Details at local (300,325), with actual callbacks.
This replay's source snapshot includes the pending gallery correction, but no gallery runtime
claim is taken from it. Reviewed its `panel-key.png` and `modal-isolation.png`; appearance and
focus/modal placement remain consistent with the earlier light captures.

### Visual review

Reviewed `initial.png`, `panel-key.png`, `fixed-footer-at-action.png`, `modal-isolation.png`,
`footer-key.png`, `notice-link-key.png` and `gallery-article.png` from both complete runs.
The native fixture captures show the expected light/dark surfaces, selected
navigation and numeric values, visible panel keyboard outline, underlined notice keyboard focus,
viewport notice position and modal dimming. The fixture deliberately overlaps its fixed footer
and an underlying button to prove occlusion; applications still need bottom content space.
These earlier FooterLink captures show the former tint on its text-sized target. They predate
the integrating agent's final contrast-style migration and do not validate that newer focus
paint. The target remains text-sized; minimum-target acceptance is not established here.

The gallery review found an actual regression in the added semantic image wrappers: the second
image paints behind following list headings, especially visibly in dark mode. Tree validation
alone did not catch it. These gallery screenshots are retained as failed visual evidence; a source
correction and targeted screenshot rerun were required before accepting that gallery change.

The first correction put the labels directly on native `Img` elements and restored their
original clean visual layout. Targeted runs `article-20261025T085206Z-U0hnLv` and
`article-20261025T085314Z-YyoYLd` found that the pinned native image element never registers an
accessibility node, despite exposing role/id builder traits. The second run retains both themes'
trees and initial/scrolled screenshots; image names are absent. Its gallery binary SHA-256 is
`54426c7fdfdb16b4917fbcf9c24bd3bf3f0e54bfba29bbbc953bf378ce69d678`.
The next correction gave each semantic wrapper the original image's fixed-height slot and used
a percentage height for its image. Run `article-20261025T090610Z-1hYyYx` passed both themes'
tree and wrapper-bounds assertions, but screenshot review still failed: the native image painted
beyond the percentage-height slot and behind following headings. Gallery SHA-256:
`b1a298dab9ffb924ac087e9ce6b26275b4696244e5fefcb6a98fdae1fbfbcd11`. Its baseline HEAD is
`a3ce3afca6417b7dd1b9d701456ae2d21d2150f4`; its integrated source-diff hash is
`b424db6676f583362990fd6f41dee0533f2e58eccdbc729c1729ba9db8654fc6`. This is retained as
failed visual evidence despite green tree/geometry assertions. The current source instead gives
both wrapper and child image the original definite pixel height.

Run `article-20261025T092044Z-y6cB6A` tested that definite-height correction on build 7 with the
new contrast styles. It again passed both themes' tree/wrapper-bounds checks but failed screenshot
review with image paint behind following headings. Binary SHA-256:
`1d495e3ffd1e187bc74097fd2b91f665b3aa309214aafb6705d539e46c077226`; source-diff hash:
`e0569079894886fc0abd60cf14aaa832135cf8693cb9fa14f971673b48329958`. This run also records exact
Cargo files and per-file source/vendor hashes. The final correction makes the semantic
wrappers flex columns, restoring the image's original flex-child layout path rather than the
default block wrapper.

That run also records nested article-list bounds: each nested list is 20px to the right of its
parent (x464 versus x444 for the native unordered/ordered lists). An initial screenshot suspicion
of missing indentation was not substantiated; no list-layout change is made.
[article-rerun.sh](article-rerun.sh) preserves snapshots even when its tree/geometry checks fail
and copies the gallery binary outside the repository before starting. Visual review remains
required; the binary path and hash are recorded without committing the binary.

### Final article result

[article-20261025T092554Z-pnwv0k](article-20261025T092554Z-pnwv0k/) completed with exit 0 on the
final flex-wrapper source. Both themes pass named body/image and ordered image-bounds assertions.
Independently reviewed all four `light.png`, `dark.png`, `light-scrolled.png` and
`dark-scrolled.png`: both images now paint within their original 184px-high slots, the following
heading has a clear 24px gap, and the visible nested list has its expected indent. No image/text
overlap remains in these reviewed captures. The semantic wrapper bounds match the previous
failed run; independent paint review was necessary to distinguish the correction.

Gallery SHA-256: `ed6fe92eab26d0d9eb9e6c286340ac2022f57c5180838ea5924a58676440053f`.
Baseline HEAD: `a3ce3afca6417b7dd1b9d701456ae2d21d2150f4`; source-diff hash:
`020192b2283d21c663c017cb539613749dec031317f0a107cd845ca6d3ad841a`.
The run preserves Cargo manifests and exact source/vendor hash lists. SHA-256 of
`source-files.sha256`: `1f32d39c9548fe1fc7061a197f49c65953a97ea23c299bbac0612b2e1933bfda`;
of `vendor-gpui-pre.sha256`: `fccf0736b3fcb8cf12b928000592fae684fa58da8ac91f47e5862079a5c0ce42`.
This is a Linux light/dark article regression check, not full reference parity or screen-reader
qualification; the earlier failed attempts remain intact.

`object:announcement` is bridge-event evidence, **not Orca speech or screen-reader acceptance**.
AT SetCurrentValue may return true for a disabled/read-only slider even though the value is
unchanged; the fixture checks actual values and callback counts. Full-source gate results are
recorded by the integrating root agent; this receipt does not substitute screenshots for those gates.
