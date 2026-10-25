# Actual Orca control samples — 2026-10-25

These are real Orca 48.1 / Speech Dispatcher 0.12.0 / eSpeak NG 1.52.0 runs in the prepared Debian 13 Linux container, with Xvfb :110, Openbox, `GPUI_X11_SCALE_FACTOR=1` and a private D-Bus/accessibility/audio session. The private PulseAudio 17.0 null sink records 16 kHz mono signed 16-bit PCM. No speech labels or pre-recorded audio are injected. The driver sends ordinary XTEST pointer/keyboard actions; held Insert invokes Orca's own flat-review “current accessible object” command.

The agent reviewed Orca's `SPEECH OUTPUT` transcripts, native callback/state observations and nonzero PCM measurements. It did not listen to the audio. Pronunciation, intelligibility, Chinese speech quality and human screen-reader usability sign-off remain open. These are partial samples, not full screen-reader acceptance for any inventory row or other operating system.

## Source and artifacts

The eight baseline collection sessions use committed GitHub source `49268de` and its approved vendored GPUI/AccessKit compatibility stack, compiled with Rust 1.98.1. The exact external consumer main, Cargo manifest/lock and executable digests are in `probe.rs.txt`, `consumer-Cargo.toml.txt`, `consumer-Cargo.lock.txt` and `artifact-sha256.json`. `source-commit.txt` and `committed-source-tree.txt` identify the library/vendor source tree. The immutable executable is `/workspace/.wegpui-cloud/orca-controls-49268de/wegpui-orca-controls-probe`; later library edits do not alter it.

The external fixture comes from `../control-contracts-20261025/final-control_contracts.rs.txt`. Its accessible button names intentionally use the builder's existing `title` override, such as `button-0-false-1`. They are actual application labels supplied by the fixture, not labels synthesized by the reader harness.

Per-session provenance records launch-time Git status/diff, executable hash, source/tool versions and exact command. Launch-time dirty state is context, not an attestation that subsequent edits were compiled into the earlier binary. Post-warning source/artifact snapshots are recorded separately below.

## Baseline samples

`review-speech.py` independently checks the retained requested Orca phrases, native callback counts, collection exit codes and actual PCM; `review-summary.json` records the results and hashes. A passing collection/assertion is not an acceptance oracle. Each session retains speech transcript/audio, full losslessly compressed reader debug log, actual actions/timestamps, native trees/states, screenshots and application callbacks.

| Run | Actual observation | Audio |
| --- | --- | --- |
| `choices-attempt1/` | Checkbox/CheckboxCell checked-state speech; Switch pressed/not-pressed speech; independent radio groups and set positions | 81.97 s, nonzero PCM |
| `buttons-attempt1/session/` | Normal buttons skip disabled/loading during forward/reverse Tab. Real flat review reads the disabled and loading objects/descriptions | 70.77 s, nonzero PCM |
| `agree-attempt1/session/` | Agreement checkbox state, separate terms links/callbacks, disabled-parent terms link; warning/shake baseline has no invalid cue | 57.59 s, nonzero PCM |
| `vcode-attempt1/session/` | Rounded/plain send, disabled countdown name/state speech, Tab skip, restore/restart/reset | 53.59 s, nonzero PCM |
| `cells-attempt1/session/` | Cell-button disabled/loading readback, enabled icon button; intended final disabled-icon object readback did not reach that object | See measured summary, nonzero PCM |
| `icons-attempt2/session/` | Flat-review line/item navigation reaches `icon-2`, but held Insert triggers a focus pulse and current-object readback returns to `icon-0` | Nonzero PCM; availability not qualified |
| `icons-attempt3-observed/session/` | Same preserved icon retry; the passive observer missed attachment | Nonzero PCM; no mode evidence from this attempt |
| `icons-attempt4-observed/session/` | Same retry with successful passive X11 focus-mode observer | 31.26 s, nonzero PCM; mode evidence linked below |

The first choices run kept its exact driver/launcher beside the output. Later run roots keep a copied `harness/` and output under `session/`, so later harness edits cannot change the recorded driver.

### Choices

Orca actually says `Standalone checkbox check box not checked.`, then `checked.`, then the full checked and unchecked summaries. CheckboxCell similarly changes from not checked to checked. Switch is presented as `Switch toggle button not pressed.`, then `pressed.`, then not pressed again.

The first radio group contains three entries, including a disabled middle row. Its summaries say `Radio A selected radio button.` / `1 of 3`, then Down skips the disabled row and says `Radio C selected radio button.` / `3 of 3`. The independent second group says `Second A` / `1 of 2`, then `Second B` / `2 of 2`. Returning to each group reads C / 3 of 3 and B / 2 of 2 respectively. Separate native state assertions also confirm both selections remain selected.

Actual callback counts are checkbox 2, checkbox-row 1, switch 2, radio-2 1 and second-radio-1 1. Disabled-control and checkbox-parent callbacks do not fire in this sequence. This is sampled navigation/state speech, not every disabled-state or stale-proxy interaction.

### Buttons and cell buttons

Forward/reverse Tab goes between active kind-0 and active kind-1 buttons, skipping the disabled/loading controls in between. Orca flat review reads the actual disabled line names; held Insert + KP_Begin then says `button-0-false-1 button grayed.` followed by `不可用.`. The corresponding cell sample says `cell-0-1 button grayed.` followed by `不可用.`.

For loading, flat-review current-object speech says `button-0-false-2 button.` / `cell-0-2 button.` followed by `正在加载.`. The separate native tree reports Busy; this sample does not establish a spoken English “busy” cue. The disabled/loading controls are inspected through real reader navigation without being activated. No disabled/loading activation is requested in these reader-navigation samples; their activation guards are verified separately by the native control-contract matrix.

This qualifies these downstream samples on the owner-approved compatibility stack. It does not assert that the upstream AccessKit disabled-button issue is resolved or filed, nor that every size/kind/overlay/icon variant has complete screen-reader acceptance.

### Agreement and Vcode

Agreement 0 announces unchecked/checked/unchecked states. Tab announces `Terms 0 link.` and Return activates that link once. Agreement-0 toggles exactly twice; the link activation does not toggle it. The disabled Agreement 1 checkbox is skipped, while its deliberately active Terms 1 link is announced and activates once; Agreement-1's toggle callback stays zero.

Warning Agreement 2 and shake Agreement 3 in the **49268de baseline** are read as ordinary unchecked checkboxes, with no invalid cue. This exposed the warning-state gap addressed below. The pinned WeUI `form_vcode.html` uses `.weui-agree_warn` when submission finds consent unchecked (lines 145–147) and removes it after a checked submission (line 137). Its separate caller-owned hidden alert is at line 76. The native warning therefore represents failed consent validation.

Vcode says `Send rounded button.`, changes to `重新发送(3s) button grayed.` and `重新发送(2s) button grayed.`, and returns to the send label. Tab during countdown reaches `Start twice 0`. Restarting and pressing Return again during countdown emits only one new send callback; Reset cancels it and restores the label. The plain variant announces 3s, 2s, 1s and `Send plain button.` while focus remains on that control. Native counts are rounded send 2, reset 1 and plain send 1, with both timers idle at the end. Vcode has a disabled/countdown state; it has no read-only API, so no invented Vcode read-only acceptance is claimed.

## Preserved reader focus limitation

The intended disabled-icon current-object readback was not accepted. Flat review speaks the line `icon-0 icon-1 icon-2 icon-3` and can speak item `icon-2`, but the held Insert command causes deactivation/reactivation of the unchanged client and resets Orca's review context. Current-object speech consequently reads the original `icon-0` instead. In the first cell run, the file labelled `flat-disabled-icon` instead read `icon-1`; labels describe intended actions, and the retained transcript determines the observation.

The independent observer in `../x11-focus-modes-20261025/icons-attempt4/` connected passively with `xev -id ... -event focus`, without creating a window, grabbing input or changing focus. It records NotifyGrab at 10:11:37.440 and NotifyUngrab at .641 around held Insert at .437; no NotifyNormal occurs in that command interval. Orca logs the corresponding window deactivate/activate and icon-0 focus false/true events. The unmodified Linux platform crate's `client.rs` FocusIn/Out handlers forward every event to activation/IME handling without checking mode. The reader transcript and passive mode evidence support a Linux platform limitation; no new platform fork or registry edit is made by this receipt.

The same short-lived frame/original-control announcement is visible in some button/cell current-object sequences. Fresh native snapshots retain the original focused active control; they do not prove that Orca's separate flat-review context remained intact.

## Warning-state implementation and follow-up

The later change adds `Semantics::invalid(bool)` and forwards it as AccessKit `Invalid::True`; `Agree::warn` now applies it to the checkbox. It changes no colours, callbacks or checkbox/link event guards. The owner clears the controlled validation flag with `warn(false)`. It does not inject a live alert or silently clear warning state when the checked value changes.

`agree-warn-source.diff`, `agree-warn-accessibility.rs.txt`, `agree-warn-agree.rs.txt`, `agree-warn-tested-probe.rs.txt`, `agree-warn-consumer-Cargo.lock.txt` and `agree-warn-artifact-sha256.json` record the exact tested warning code, external fixture and artifact. The immutable binary is `/workspace/.wegpui-cloud/agree-warn-current-20261025/wegpui-agree-warn-consumer`. The dynamic fixture uses the same actual label throughout; F1 sets warn, F2 clears it and F8 records owner state. `agree-warn-final2/session/` completes the actual follow-up: six phase-aligned reader summaries cover warning false → true → false with an unchecked checkbox, then with a checked checkbox. On warning true, Orca adds `invalid entry.`; after warning false, the corresponding full summary omits that cue. Fresh native snapshots match the InvalidEntry flag in all six states and keep the checkbox enabled and focusable. Warning flips emit no change/link callbacks; one Space activation changes the checkbox once, and one Terms Return activates the link once. Final owner state is `warn=false checked=true changes=1 links=1`.

`review-agree-warn.py` checks actual Orca speech only between each recorded KP_Enter command and its named capture, checks the independent native flags and callbacks, and measures the 41.93 s nonzero PCM. `agree-warn-review.json` records those results. Selected checked/unchecked warning screenshots were visually reviewed; the same label and layout remain mounted throughout. This is a reader-state cue; no automatic live alert or spoken explanation of the particular consent error is claimed.

`agree-warn-final1/session/` is preserved as a failed harness run. A missing `in` in the new Python assertion caused a SyntaxError before any driver actions. The real app/reader pipeline started and produced 5.64 s of audio, but no warning-state acceptance is attributed to that run. The driver was corrected and AST syntax-checked before rerunning the same immutable binary.

Selected baseline raw screenshots visually reviewed were `choices-attempt1/final.png`, `agree-attempt1/session/agreement-checked.png` and `vcode-attempt1/session/plain-countdown-readback.png`. They corroborate the visible selected values and countdown; screenshots and AT trees alone do not establish the speech results above.

Human listening, cross-platform reader runs, all component/theme/state combinations and complete acceptance remain open.
