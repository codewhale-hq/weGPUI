# Approved X11 grab-mode focus filter

The owner explicitly approved this additional compatibility fix on 2026-10-25.
The exact locked `gpui-pre-linux 0.3.9` archive is now copied to
`vendor/gpui_pre_linux`, with precisely `compat.patch` applied. The root Cargo path
patch selects that same version; only its registry source/checksum entry was removed
from Cargo.lock. Parsed package versions and dependency lists are unchanged.
`applied-source-checks.json` records those source checks. Registry files were not edited.
No publication, upstream issue, message or external post is authorized.

The earlier observation and proposal text below preserves the reason for the change.
Application-root patch guidance is in `vendor/README.md`. Current weGPUI now also
requires the separately approved patched GPUI action-scope API to compile; the matching
workspace patch table is mandatory, not an optional accessibility enhancement.

## Concrete problem and evidence

On the immutable `49268de` Orca controls probe, sampled native-app AT-SPI focus flags continue to expose
`icon-0` while reader keypad commands produce same-client X11 FocusOut/FocusIn events
with NotifyGrab/NotifyUngrab. The locked Linux backend handles every such event as
window deactivation/reactivation, forwards that state to AccessKit and clears/resets
keyboard, compose, preedit and XIM state. Orca receives repeated window/focus pulses
and quits/recreates its flat-review context.

The [passive observer receipt](../../receipts/orca-controls-20261025/x11-focus-modes-20261025/README.md)
records actual modes and the corresponding reader session. For held Insert+KP_Begin:

| UTC on 2026-10-25 | Observation |
| --- | --- |
| 10:11:37.437198 | Reader driver starts held Insert+KP_Begin. |
| 10:11:37.440443 / .440472 | Same client `0xa00001`: FocusOut / FocusIn, NotifyGrab, NotifyNonlinear. |
| 10:11:37.641732 / .641817 | Same client: FocusOut / FocusIn, NotifyUngrab, NotifyNonlinear. |
| 10:11:37.475835 / .768796 | Orca logs window deactivate / activate. |
| 10:11:37.471599 / .633601 | Orca quits flat review / creates a new context. |
| 10:11:37.767629 / .782329 | Orca queues icon-0 focused false / true. |

No NotifyNormal occurs within that command interval. NotifyNormal appears only at
session teardown, at 10:11:46.508, including a NotifyPointer detail. Queried app AT-SPI snapshots
before and after the reader command expose icon-0 as focused; no continuous native
FocusHandle or XGetInputFocus trace was taken. The observer timestamps are
receive times; Orca queue/process timestamps overlap events from preceding KP_Right
grabs. These are correlated observations with a direct source path, not a claimed
one-to-one mapping of each queued reader event. They do not establish all reader
problems as resolved by this candidate.

## Exact proposed change

`compat.patch` adds the same early mode guard to both `Event::FocusIn` and
`Event::FocusOut` in `src/linux/x11/client.rs`. NotifyGrab and NotifyUngrab return
`Some(())` before window activation, keyboard-focus mutation or IME work. Existing
NotifyNormal, NotifyWhileGrabbed, detail and unknown-mode behavior remains unchanged.
The existing `xproto` import supplies `NotifyMode`; no new import or dependency is needed.

The Xlib protocol documentation describes Grab/Ungrab events as generated **as if**
focus changed from the current focus to the grab window, then back. It separately
describes normal focus changes and changes while grabbed. Thus the candidate filters
the temporary grab activation/deactivation modes rather than suppressing every event
while a grab exists. NotifyWhileGrabbed continues through the existing handler so an
actual underlying focus change during a grab is still processed.
`protocol-excerpts.txt` preserves the relevant specification text and response hash.
The locked `x11rb-protocol 0.13.2` also exposes the four mode constants distinctly.
This patch deliberately introduces no detail filter or broader X11 focus redesign.

The component layer cannot prevent the failure: the native client dispatcher calls
`window.set_active` and clears backend keyboard/IME state before component key handlers
run. `X11Window::set_active` immediately invokes the active-status callback and
`accesskit_adapter.update_window_focus_state`. Filtering Escape or reader keys in
weGPUI components would leave these earlier backend state changes intact.

## Approved integration and required validation

1. Copy the **exact locked** `gpui-pre-linux 0.3.9` crate into
   `vendor/gpui_pre_linux`, retaining its manifests, source provenance and Apache-2.0
   license. Apply only `compat.patch`; verify every other file against the locked archive.
2. Add `gpui-pre-linux = { path = "vendor/gpui_pre_linux" }` to the repository's existing
   root `[patch.crates-io]`. Refresh the lockfile only to select that same-version local
   source. No package version, dependency requirement, feature or resolved dependency
   edge is proposed to change.
3. Add provenance/notices and matching application-workspace patch guidance, including
   the separate consumer. Dependency patch tables are not inherited by consumers.
   No forked crate is published and no release/distribution approval is implied.
4. Run all five required source gates with locked dependencies. Rebuild immutable
   native probes and retain their exact source/artifact hashes.
5. Repeat the same real Orca commands with the passive observer. X11 Grab/Ungrab events
   should still be observable, while spurious AT-SPI window/focus pulses and flat-review
   resets should disappear. Review speech/audio and reader behavior independently.
6. Exercise a genuine normal window-focus transfer and return, plus an underlying
   focus change while a keyboard grab is active. Both must still update GPUI/AT focus.
   Verify compose/preedit/commit and cancellation across ordinary focus changes and
   temporary reader grabs; confirm no stale focused input or composition remains.

## Validation and limits

`checksums.txt` identifies the original crate/archive, source files, license, proposed
modified client and exact patch. The archive SHA-256 matches the prior registry entry; the original
client, window, manifests and license match that archive byte-for-byte.
`git apply --check` passed against a temporary copy of the original file.
`validation.txt` records the original proposal checks. The approved source was then
compared against the archive: client.rs is the sole modified upstream file, with its
expected proposed checksum; the root lockfile differs only in source/checksum for
that package. `applied-source-checks.json` records this result.

The root subsequently reported all five combined source gates passing, including
47 tests. The [new runtime receipt](../../receipts/x11-grab-focus-20261025/README.md)
verifies actual Normal/WhileGrabbed focus transfers, temporary grabs without false AT
pulses, real Fcitx composition across grabs and actual Orca retaining icon-2 review and
emitting its disabled state. Exact combined-artifact hashes and remaining listening,
reader/IME, host/theme and release limits are explicit there. The earlier linked mode
receipt remains evidence of the **unpatched failure**.

Risks include changing how temporary grabs interact with XIM and active-window callbacks.
Retaining Normal/WhileGrabbed handling narrows that risk, but requires the explicit
native regressions above. Wayland and other host backends are outside this two-handler
change. Full screen-reader, IME, platform and release acceptance remains incomplete.
