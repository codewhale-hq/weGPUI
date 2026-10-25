# Form and DateField contracts — Linux, 2026-10-25

This continuation starts at GitHub main `9c4515c`; canonical main later advanced to
`a3ce3af` (vertical-label fix), which was integrated while preserving this work. The
pinned upstream input HTML uses
native `date`/`datetime-local` controls; weui.js datePicker supplies year/month/day
selection. The exact inspected HTML/JS hashes are in `upstream-source-hashes.txt`.
`before-date-field.rs.txt` and `before-form.rs.txt` retain the original source.

## Source findings addressed

- DateValue/set_value/parse_date previously accepted impossible dates and times. New
  `validate`, strict numeric `parse` and `try_set_value` reject malformed, calendar,
  time and configured-range errors without replacing the committed value. Date mode
  normalizes time to midnight. Year bounds are reordered/clamped to supported years
  1–9999; narrowing a range preserves an existing out-of-range value and exposes its
  error instead of silently changing it. Default 1900–2100 and UTC today are unchanged.
- set_value/set_year_range previously dropped live picker entities/subscriptions,
  left datetime pending state behind and could lose focus cleanup. Disabling left
  sheets interactive. Explicit edit stages and generation guards now cancel a pending
  sequence once, close its sheets, and prevent a queued time sheet from opening after
  an owner setter. Disabled controls leave the tab order; read-only inputs remain
  focusable. A fresh picker starts from committed value after cancellation.
- Optional `set_typed_entry(true)` adds editable ISO text plus a calendar button.
  Enter/blur commits valid text, invalid drafts retain the old value with visible
  error/accessible description, Escape discards drafts, and Alt+Down opens sheets.
  Default picker-button interaction is preserved. The gallery input page opts in.
- The original DateFieldEvent variants are unchanged. New DateFieldValueChanged
  carries Option<DateValue>, including typed clearing, without adding a variant to
  the old enum. Owner setters emit no value-change event; interruption emits Cancel.
  Existing `set_value` still returns `()`; callers needing an error use the new
  `try_set_value`. Existing call sites therefore need no return-type migration.
- FormCell adds explicit `validation_error(Option<SharedString>)` and
  `label_for_date`. Validation updates the associated input's semantic invalid flag
  and description without mutating its value/disabled/read-only state. The existing
  visual-only warn/state builders retain their ownership rules. The status gallery
  focuses the card on failed submission and gives errors accessible descriptions.
- ClearableInput previously exposed a clear action on focused read-only fields and
  called programmatic clear(), bypassing read-only editing protection. It now hides
  and guards clear actions for read-only/disabled inputs.

These are source changes, not a claim of full component acceptance. API unit tests
cover leap-century/calendar/format/time/range boundaries; native receipts below
separately exercise state, callbacks, focus and accessibility observations.

## Native collection

`probe.rs.txt` is the exact temporary fixture source. `run.sh` starts Xvfb :105 plus
openbox and a private a11y-enabled D-Bus session, with GPUI_X11_SCALE_FACTOR=1. The
43-step `steps.json` drives actual xdotool keys/pointer events; every step records
an ImageMagick window capture, a fresh-client accessibility JSON tree, a native
owner-API snapshot and callback assertions. Fresh clients avoid confusing native
state with a long-running screen reader's cache. The runner refuses existing OUT
paths, retains failed steps and completes the sequence even after an assertion fails.

```sh
dbus-run-session -- docs/receipts/form-contracts-20261025/run.sh \
  NEW-OUT /absolute/path/to/forms_contracts_probe
```

No tree observation establishes screen-reader acceptance. The initial artifact used
the original AccessKit AT-SPI common 0.19.1 behavior, which omitted Invalid::True from
Linux STATE_INVALID_ENTRY. Its historical steps preserve that absence. The owner
approved an integrated bridge correction; the final artifact now exports and verifies
the invalid-entry state. This does not by itself qualify every invalid-state reader
interaction. Complete cross-platform, locale and theme/state acceptance remain open.
VoiceOver stays off. No dependency/registry changes were made by this subtask; the
approved integrated build's dependency provenance is root-owned.

## Recorded native results

| Run | Result | Limit |
| --- | --- | --- |
| `attempt1` | 13 recorded assertions passed, 2 failed; collection exited 1 when a later clear-button lookup found no target | Driver treated exported desktop extents as screen coordinates, while the bridge supplied window-local bounds; pointer clicks hit the preceding row. Keyboard-only date assertions remain valid. Full failure note/logs/trees/screenshots retained. |
| `attempt2` | **43/43 assertion steps passed**, exit 0 | Corrected pointer targeting anchors child bounds to the fresh frame and uses native X window coordinates. This validates control behavior, not absolute accessibility screen-coordinate accuracy. |

Both used the exact fixture binary SHA-256
`652a91e6cd62e39805d10f321daae7ad4545e52bc8017cbc818d46a041fc6abf`.
The per-run provenance records source HEAD/status/diff hash, binary hash, Rust and
Debian versions. The final source additionally improves availability styling and
announcements; targeted final captures are recorded separately rather than silently
reusing this artifact as final-source evidence.

The complete sequence verified:

- Bad typed dates and hours retain committed value; valid typed values commit once;
  repeated Enter adds no callback; clearing emits the optional-value event without
  inventing a nonempty value; Escape restores text and emits Cancel once.
- Invalid owner values are rejected; narrower ranges preserve the old value and
  expose the range error. Error descriptions and warning text were visually/tree
  inspected; this initial artifact still omitted AT-SPI invalid-entry. The final
  artifact below verifies the approved bridge correction.
- Visible label clicks focus the associated read-only control; disabled labels do
  not focus disabled controls. Clear is absent for read-only/disabled inputs,
  present when editable, and clears/refocuses exactly that input.
- Form validation supplies the combined hint/error description and clears it back
  to the hint. It does not replace the input's text or enabled/read-only state.
- The default picker-button remains keyboard operable; cancellation/reopening uses
  committed date rather than cancelled wheel state. Disabling an open date field
  cancels once, closes its sheet and moves focus to an enabled control.
- Datetime's first sheet does not commit. Final time confirmation changes once;
  Escape in the second sheet cancels once. Owner replacement, disabling during the
  time stage and an armed owner update during date→time transition leave no later
  time sheet. The owner update preserves its authoritative replacement value.

Selected screenshots from this run were reviewed: invalid date shows red draft and
explicit calendar error, cleared/read-only controls retain their intended values,
and modal transitions expose the active picker/focus. Assertions are backed by
owner-API snapshots and callback logs as well as fresh trees; screenshots alone do
not establish these behavior results.

## Additional mode-switch defect found during final state verification

`final-states` preserves a real failure in the first optional-typed-entry implementation:
9 of 13 steps passed, while 4 later steps produced 11 failed assertions after changing
from a focused read-only picker button back to typed entry. The two presentations
used different focus handles; the old focused handle disappeared, so the next owner
shortcuts never reached the view. The input consequently stayed read-only, causing
the later disabled/format expectations to fail. These are not discarded as a driver
problem.

The correction shares the underlying input's focus handle between the default button
and typed input, with a separate handle only for the calendar action. One leaf tracks
that field handle per frame. The final targeted matrix explicitly checks both mode
changes before any pointer-assisted refocus, plus blocked read-only activation.
The earlier 43-step run did not exercise that mode change and is not presented as
verification of this correction.

## Focus lifecycle corrections

`final-fixed` preserves the next real lifecycle failure: both mode-focus assertions
passed after the shared-handle correction, but disabling an already focused input
with no open sheet still left focus on the frame. GPUI invokes next-frame callbacks
before drawing the new frame, so a single callback navigated the old tab map and
selected the disappearing calendar action. Four later cases consequently failed.
The correction waits for one disabled frame to finish before advancing, checks the
current disabled flag to avoid a stale move after re-enabling, and respects focus on
another control. The failed captures and callback logs remain unchanged.

`final-verified` then completed **15/15 targeted steps, zero failed assertions,
exit 0** on that intermediate source, using the read-only copied fixture binary SHA-256
`108aed141e443622b1bc2d437b72ed512fba522ba9de4d151aec4edac607f14d`.
HEAD was `a3ce3afca6417b7dd1b9d701456ae2d21d2150f4` plus the recorded source changes;
Rust 1.98.1. The root-owned build passed all targets and 45 unit tests before this
run. The final source was subsequently checked again below after the reader and
focus-contrast corrections.

- Confirming an unchanged picker value emits the original Change once but no
  optional-value-change event; changing the year from leap 2024 to 2025 clamps
  February 29 to February 28 and emits one change in each relevant channel.
- Read-only cancellation closes the active sheet and preserves focus/value. Typed
  and default-button read-only activation cannot mutate the value. Both mode changes
  preserve field focus before any pointer refocus. Disabling an idle focused field
  advances to the next enabled control; subsequent re-enable shortcuts work and
  clear the unavailable description.
- The date-only format error displays `Use YYYY-MM-DD.`, retains the committed value,
  exports that description and **STATE_INVALID_ENTRY=true**, and clears on Escape.
- The picker owner's column-ring fix is visibly present on the focused hour and moves
  to the minute after Tab. Both fresh trees identify the corresponding focused list
  box. The armed owner update during date→time transition preserves authoritative
  replacement text and restores field focus with no late sheet.

The read-only/disabled appearances, format error and both column-ring screenshots
were visually reviewed. `window-info-raw.txt` reports the real X client origin
(21,40), matching the final AT-SPI frame origin. xdotool's separate geometry command
adds the relative frame offset again (22,60); it is not the authoritative comparison.
These input-contract tests use frame-relative native coordinates; the independent
input geometry receipt covers detailed screen/text bounds.

## Actual Orca error feedback and final-source rerun

`orca-date` retains the first actual reader run on the preceding artifact. It
recorded a real duplicate: one invalid Return produced the error at 09:07:38.367
from `object:announcement`, then again at 09:07:38.374 from the focused field's
`accessible-description` change. Its 38.69-second nonzero PCM, full debug log,
transcript, screenshots and trees are preserved as failure evidence.

The narrow correction keeps the error description and Invalid state. On Linux,
when the field itself is focused, a changed description supplies the initial
feedback and the accompanying alert has Live::Off. An identical invalid resubmit
has no description change, so it creates a fresh assertive alert instead. Other
hosts and unfocused errors retain assertive live feedback. Repeating an unchanged
owner error setter returns early; ordinary redraws do not create fresh alert IDs.
This platform behavior is based on the observed Orca event paths; other screen
readers remain unqualified.

`orca-date-deduped` ran actual Orca on the final copied artifact SHA-256
`0823b696c37f2a861a4ea9da4ae5e291614dafb95823b8f2d5c3de56cf981326`.
`speech-review.json` counts the exact speech-output records between actions:

| Action | Error speech outputs | Observation |
| --- | --- | --- |
| First invalid Return | 1 | The changed description reports the calendar error. |
| Same invalid Return again | 1 | Fresh live alert reports the unchanged error. |
| Third identical invalid Return | 1 | Feedback remains available on another retry. |
| Explicit Orca summary | 1 | Reads the invalid draft, `invalid entry`, and error description. |
| Corrected value and summary | 0 | Reads `2024-03-01` with no stale error. |
| Clear and summary | 0 | Reads the empty field's placeholder with no stale error. |

This reader run exited 0 and captured **43.47 seconds of nonzero synthesized PCM**
(mono 16 kHz, RMS 5129). The transcript, event paths, screenshots and tree/text
observations were reviewed. The model cannot listen to audio in this environment;
audible pronunciation/intelligibility and human screen-reader acceptance remain
open. The recording is available for that later review; nonzero PCM alone does
not establish that acceptance.

`final-deduped` reran the complete targeted matrix on the **same final artifact**:
**15/15 steps passed, zero failed assertions, exit 0**. The format error, disabled
appearance, and hour/minute focus-ring captures were visually reviewed after the
shared contrast-style changes. The preserved earlier 43-step run remains earlier
artifact evidence. Final root-owned compilation passed all targets and 47 tests
before these captures; final pre-commit gates are recorded by the root task.
HEAD was `a3ce3af` plus the recorded pending changes, Rust 1.98.1, X11/Xvfb plus
openbox, scale 1. `date-field-final.rs.txt` and `form-final.rs.txt` retain the final
owned sources. The temporary executable fixture was removed from `examples/`
after collection; its exact source remains `probe.rs.txt`.
