# Overlay API/state contracts — Linux X11, 2026-10-25

The current thirteen-case X11 run passes callback/state/focus assertions and the final owned-text,
inline-focus and picker-column indicator checks ([validation](current-validation.log)). Its immutable
executable is `6ec795d52f060b9bb328dfa60baed82d7ea3076483f34b040a943c31a5dc04e6`.
Twenty key captures were visually reviewed in [current-review.png](current-review.png), including
full-size inline controls and a focused month column. Full component acceptance remains open;
[picker ring contrast](picker-focus-contrast.txt) is 2.226:1 in the captured default theme.

The batch started at GitHub main `9c4515c`; main subsequently gained the separately verified
vertical-label fix `a3ce3af`. Each runtime uses the recorded source/binary snapshot from this
integrated continuation, not a claim that every final working-tree byte was retested. Exact owned-file changes are
[source.patch](source.patch). Upstream references are WeUI 2.6.26 and weui.js 1.2.17 already in
the pinned cache; [upstream hashes](upstream-sha256.txt) identify the inspected files. Runtime
uses the shared `wegpui-198` container, Rust 1.98.1, gpui-pre 0.3.9, Debian 13.5 x86_64,
Xvfb :103 with Openbox, `GPUI_X11_SCALE_FACTOR=1`, real XTEST input and AT-SPI snapshots.
The integrating agent records dependency compatibility patches and complete Cargo gates
separately; this receipt makes no claim about an unmodified dependency graph.

## Source changes

- Picker dates no longer build a year × month × day tree. A scalar year range generates at most
  nine nearby year rows, with only the selected year's months and selected month's days.
  An exact integer year index is separate from animation offsets, so the full inclusive i32
  range and Home/End remain exact on supported 64-bit desktop hosts. Date event rows retain
  labels/numeric values but omit eager child trees; ordinary cascade rows retain theirs.
- Date month/year changes clamp the remembered day to the last valid day rather than falling
  back to the middle of a shorter month. UTC `today()` policy is unchanged.
- Closed pickers ignore confirm/navigation/drag/wheel input. Wheel-idle callbacks check opening
  and column generations, including after a close/reopen or rebuild. Disabled picker rows now
  carry disabled semantics and set positions.
- Long SelectMenu lists scroll within the available window height, reveal keyboard focus and
  retain a visible Cancel row. Deferred initial focus cannot steal focus from another layer.
  SelectCell triggers now register for offscreen focus reveal.
- Short half-screen drags return with the upstream 300ms ease transition (immediate under
  reduced motion). Completed drags retain their offset while closing; interrupted close/reopen
  returns the drag offset smoothly. The short-screen height rule allows height minus 16px at
  window heights up to 558px. Bottom-fixed body focus stays above its 80px fade.
- Presence shares an immediate input guard across render clones. Closing overlays block input
  before redraw, and built-in dialog/action-sheet/select callbacks check the same live guard.
  Rendering state remains caller-owned; item callbacks still do not automatically close sheets.
- Inline action-sheet cells, dialog footer buttons and half-screen header icon buttons register
  for focus reveal in a caller scroller; raw `hd_link` keeps its builder return type and documents
  caller wrapping. The picker selected-row ring now follows the actual focused column rather
  than applying `focus_visible` to an unfocusable decorative div.
- Android action-sheet cell colour overrides the warn modifier, matching upstream selector
  specificity. iOS warn cells remain red.
- Semantics forwards explicit read-only and live-region states. A stable per-window background
  hidden flag also lets deferred background elements participate in ModalScope isolation;
  window-close cleanup removes registry entries. Existing depth-based modal ordering is retained.

## Initial runtime and preserved failure

[initial-closing-race](initial-closing-race/) contains all nine initial case runs and their
screenshots, actual event counts and accessibility trees. [Initial validation](initial-validation.log)
failed: five rapid pointer clicks on Dialog Confirm caused five confirm callbacks during the
one-second closing fade. The first layer-only capture checked a hitbox occluded by its content
and used state copied at render time. The shared live guard plus content capture and component
activation guards address that concrete failure.

The initial run also observed bounded/exact full-range dates, disabled-option skipping,
long-select focus reveal, reentrant picker reopening, dismissal focus restoration and short-drag
return frames. These observations were repeated in the twelve-case rerun below.
The [initial source hashes](runtime-source-sha256.txt) and [executable identifier](initial-binary-location.txt)
are retained. The initial debug executable was rebuilt before an archive copy was made; logs
and captures are retained, but the old executable itself is not archived.

## Historical twelve-case rerun

The [immutable binary hash](final-binary-sha256.txt), [compiled source hashes](final-source-sha256.txt)
and [successful probe build](final-probe-build.log) identify this run. The executable is retained
locally under `/workspace/.wegpui-cloud/evidence/overlay-immutable/probe`; it is not included in
Git or the package. [Sixteen key frames](final-review.png) were visually reviewed.

| Case | Actual observations |
| --- | --- |
| iOS/Android action sheet | Menu/tip/warn callbacks once per activation; menu activation stays open by contract; Cancel, mask and Escape dismissal restore opener. Android omits Cancel and keeps warn text in the Android cell colour. |
| Dialog | Mask and disabled button keep dialog open; Tab skips disabled; keyboard confirm and a five-click burst each emit one confirm and restore opener. |
| Raw Modal | An ordinary Button child receives one confirm from five rapid clicks; five mask clicks produce one dismissal. This exercises foundation capture as well as specialized callback guards. |
| SelectMenu | Selected option20 opens in view; End reaches30 and skips disabled31; Home reaches1 and skips0; Down skips5; commit6, reopen6, Cancel/Escape/mask and focus return pass. |
| Disabled SelectCell | Click ignored, Tab skipped, disabled semantics present. |
| Picker | Up/Down/Home/End skip disabled rows; confirm/close order, Escape and close/reopen with queued wheel input pass. |
| Reentrant Picker | Confirm callback reopens it; the prior Close event does not close the new session; subsequent Escape restores opener. |
| All-disabled Picker | Enter cannot confirm; disabled option/button semantics retained; Escape closes. |
| Date Picker | Full i32 range opens with nine year rows and fewer than100 tree lines. March31→February29 in2000, max year→February28, exact2147483647/index4294967295, previous year and Home−2147483648/index0, confirm and focus return pass. |
| Grab sheet | 30px drag returns; 70px drag dismisses; rapid close/reopen returns to its original position. Sampled panel tops385→415→intermediate→385; long-held455. |
| Short sheet | 480×420 viewport permits a404px sheet starting at16px, matching the short-window adaptation. |

The callback matrix is a pass for this artifact. Its picker/date images did **not** have a
visible selected-column ring. Source inspection and an independent DateField runtime sample
confirmed the decorative-div focus style was ineffective; followup source and smoke evidence
are tracked separately rather than relabelling those images as passing visual evidence.

## Current thirteen-case run

The [current binary hash](current-binary-sha256.txt), [source hashes](current-source-sha256.txt)
and [integrated build/test log](current-integrated-build.log) identify the approved patched
GPUI/AccessKit graph used here. The build passed all targets and 45 tests at that snapshot;
complete final five-gate qualification belongs to the integrating agent's final receipt.
The immutable executable is retained at
`/workspace/.wegpui-cloud/evidence/overlay-immutable/probe-current` and is excluded from Git/package.

All twelve earlier cases were rerun under [current/](current/), plus the inline case. The
[full validator](validate.py) passed. New observations are:

- Action-sheet cell, dialog footer and half-screen header icon reveal into view on forward and
  backward Tab inside an ordinary `FocusScroll`, with visible rings.
- Year, month and day navigation show exactly one outlined selected-row segment, following
  the actual focused column. The earlier missing-ring captures are retained under `final/`.
- The public HalfScreenDialog body/subtitle and Picker description appear as named Label nodes
  in actual AT-SPI trees. Arbitrary caller-supplied children still own their semantics.
- Arc-based immediate interaction guards preserve Presence's Send/Sync auto-traits; the compile
  contract is part of the integrated test run.

The default green focus ring's measured contrast remains below 3:1 against the selected-row
surface. This is a broader theme/focus-contrast gap; this receipt passes ring placement and
visibility, not WCAG contrast acceptance. Additional dark/care/brand combinations, other OSes,
Wayland repetition of this complete matrix, actual screen readers and hardware remain open.

## Later focus-contrast correction

The 2.226:1 measurement above describes this receipt's immutable thirteen-case executable,
not the later integrated focus styling. The subsequent
[focus-contrast receipt](../focus-contrast-20261025/README.md) verifies the shared surface-aware
helper, eight-mode Picker/other focus samples, corrected Checkbox interiors and final
Outlined/FilledGrey caret/action pixels. The complete thirteen-case callback matrix was not
relabelled as a rerun of that later executable; each receipt retains its own source and hashes.
Complete component, host and focus-appearance acceptance remains open.

## Reproduction and evidence limits

[Probe source](probe.rs.txt) is copied temporarily to `examples/overlays_contracts_probe.rs` for
building; the transient example was removed after capture and its exact source retained here. [Driver](run-case.sh) records one case per fresh D-Bus/X11 session; [validator](validate.py)
checks callback counts, tree states, focus and sampled panel positions. Image checks require
Pillow, already used by parity tooling.

```sh
cargo build --example overlays_contracts_probe --locked --offline
# Cases: action android dialog raw-modal select select-disabled picker reentrant all-disabled date sheet sheet-short inline
dbus-run-session -- bash docs/receipts/overlay-contracts-20261025/run-case.sh date /tmp/overlay-date
```

These are sampled native API, interaction, rendering and accessibility-tree checks. They are not
screen-reader acceptance, complete theme/state acceptance, or qualification of another backend
or OS. VoiceOver remains off. Arbitrary external deferred wrappers/priorities remain outside
the documented modal-ordering contract.
