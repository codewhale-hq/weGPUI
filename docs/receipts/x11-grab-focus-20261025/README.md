# Approved X11 focus-mode runtime follow-up — 2026-10-25

This receipt verifies the owner-approved `gpui-pre-linux 0.3.9` filter after the root
reported combined source gates passing, including 47 tests. The original unpatched
reader failure remains in
[the passive mode receipt](../orca-controls-20261025/x11-focus-modes-20261025/README.md).
The exact applied source/lock provenance is in
[the approved proposal](../../proposals/x11-grab-focus/README.md).

## Normal, while-grabbed and temporary-grab controls

`normal-while-grabbed/` executes a new minimal artifact under private D-Bus, Xvfb and
Openbox on display `:115`, at X11 scale 1, with no reader or IME service. It creates a
separate native X11 control window using installed libX11 through Python ctypes.
All focus changes and keyboard grabs are deliberate controls in that isolated session;
the existing reader sessions are untouched.

The driver queries actual `XGetInputFocus`, frame Active state through AT-SPI and
records a passive xev mode trace plus a separate AT-SPI event stream. This provides
native protocol evidence and accessibility observations, rather than inferring
native focus solely from a tree snapshot.

| Native operation | Actual X mode | Verified application behavior |
| --- | --- | --- |
| Focus away/back normally | NotifyNormal FocusOut/FocusIn | Actual input focus changes to control/app; frame deactivates/reactivates, with corresponding AT window/focus events. |
| Temporary keyboard grab/release | NotifyGrab FocusOut / NotifyUngrab FocusIn | Actual input focus remains on app and frame remains Active; no spurious AT window activation/deactivation or focused-state events occur in those intervals. |
| Change underlying focus away/back while keyboard grabbed | NotifyWhileGrabbed FocusOut/FocusIn | Actual focus changes and frame/AT activation updates still occur. |

`controls/states.json`, `xevents/focus-events.json`, `atspi-events.jsonl` and
`mode-state-correlation.json` retain raw and correlated evidence. Event timestamps
are observer receive times; the driver waits for settled state between controls.
The private control window owns/relinquishes its real keyboard grab explicitly.
Cleanup releases grabs and terminates only this session's own processes.

`normal-while-grabbed/provenance.txt` records the exact executable SHA-256, parent
source context, Cargo.lock and affected compatibility-source hashes. The minimal
executable was copied before the later disk-saving strip instruction and remains
unaltered. The new reader/IME artifacts distinguish source-build and
stripped-runtime hashes in `artifact-build-strip.txt`. No Cargo command was run by this
collector beyond reading its version.

`python3 docs/receipts/x11-grab-focus-20261025/verify-focus.py` verifies the saved mode,
native focus and AT-event contracts. Its output is in `focus-verification.txt`.
No application panic occurred. This sample establishes the intended focus-mode
policy on identified Linux artifacts; it is not IME, Orca, cross-platform or complete
screen-reader acceptance. Reader and real XIM follow-ups follow below.

## Real Fcitx preedit, commit and cancellation

`xim-real-grabs/` runs the preserved input observer against the new approved source,
with real Fcitx 5.1.12 / Pinyin 5.1.8 and XIM on-the-spot mode in a private display
`:116` session. Actual native typing replaces a selected Chinese range with marked
`shi jie` preedit. Two real libX11 keyboard-grab/release cycles keep `XGetInputFocus`
on the GPUI client, retain the text and marked range `1..8`, and emit no Changed event.

Space commits `世界` exactly once; undo restores the original Unicode text, including
combining and ZWJ content. A second real composition over the reversed selection
survives grab/release, then Escape restores original text, selection `[1,3]`, reversed
direction and no extra Changed event. These are actual XIM events, not synthetic
preedit/commit API calls. F12 only reads native input state for the fixture log.

The after-grab preedit and canceled-selection captures were visually reviewed.
Underlining, selection and Chinese text render; emoji missing-glyph boxes remain in
this installed font environment. No complete glyph, candidate-popup geometry, locale
or IME matrix acceptance is inferred. `verify-xim.py` and `xim-verification.txt` check
the preserved text, marked range, callbacks, native focus and cancellation results.

## Actual Orca command after the fix

The valid `orca-icons-cells/` run uses the same reader command sequence and cells page
as the original unpatched mode receipt. `CONTROL_CASE=icons` selects the reader driver;
`FIXTURE_PAGE=cells` selects the native fixture containing the icon row. The newly built
and immediately stripped runtime artifact is identified in `artifact-build-strip.txt`.
No Cargo output or old tested artifact was stripped.

Held Insert+KP_Begin begins at 11:21:53.372069 UTC. The passive observer still records
same-client FocusOut/In NotifyGrab at .375289/.375510 and NotifyUngrab at
.569454/.569553. Unlike the original artifact, there are no AT window activation/focus
pulses or flat-review reset lines in the command interval. Orca retains the review
target and emits **`icon-2 button grayed.`** at 11:21:53.497264. Sampled app AT-SPI focus
remains on icon-0, while flat review visits disabled icon-2; the driver does not spoof
native focus or inject speech labels.

The actual Orca debug transcript, Speech Dispatcher output and nonzero PCM are retained.
The audio is mono 16-bit 16 kHz, 487,898 frames / 30.493625 seconds, RMS 4349.887939.
An independent agent checked the retained transcript, dispatcher phrase, mode
correlation and PCM sample statistics. No listening or ASR was performed; pronunciation,
intelligibility and per-word PCM alignment remain unqualified. This is a named actual
reader/synthesis sample, not full IconButton or screen-reader acceptance.

`orca-correlation.json`, `verify-orca.py` and `orca-verification.txt` preserve/check that
result. This is a combined approved artifact, not an isolated one-patch A/B build.

Two setup failures are preserved: `orca-icons/` passed the nonexistent native `icons`
page and the fixture panicked before showing a window; `orca-icons-buttons/` used the
buttons page, which lacks icon-0, so its driver stopped. The corrected cells route was
verified against the original session's exact command before the passing run. These
failures do not qualify component behavior or hide a runtime regression. Their passive
observation attempts are retained separately.

## Wayland nested regression and remaining scope

`wayland-nested/` runs the rebuilt nested-modals example under nested Weston on a private
X display `:117`, with DISPLAY absent from the actual GPUI process. Inner modal tree
isolation, Tab/Shift+Tab trapping and sequential Escape focus restoration pass.
`verify-wayland.py` and `wayland-verification.txt` check the saved result. The target
example was executed directly to avoid another large binary copy; its hash before
and after the run is identical in `wayland-nested-artifact.txt`.

Every run retains its own environment and artifact/source hashes. Raw `.log` files
are archived losslessly as `.log.gz` after collection. Approved source snapshots are
identified in `approved-native-source-sha256.txt`; external fixture manifests/locks and
build/strip provenance are retained. No publication, deployment or CI dispatch occurred.
These focused Linux samples do not close complete themes/states, hardware desktop,
scaling, cross-host, accessibility or release acceptance.
