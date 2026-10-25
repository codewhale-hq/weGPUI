# Passive X11 focus-mode observation — 2026-10-25

The actual Orca controls session exposed same-window deactivation/reactivation while
the app focus snapshots continued to expose icon-0 as focused. This follow-up reads the locked backend source
and passively records X11 FocusIn/FocusOut modes during the reader's existing commands.
No production, registry, vendored or Cargo source was changed. No Cargo build was run.

## Method and exact session

The companion [observed Orca attempt](../icons-attempt4-observed/session/) ran the immutable
controls probe `efaea6abfdb20a155230aa3b0501830dfa84b182a6ab463deaad648334dea1d7`,
from the `49268de` continuation. Its own provenance, fixture source, native/AT snapshots,
Orca debug transcript and speech audio identify that run. This observer adds no new
screen-reader acceptance verdict to the reader agent's results.

`observe.py` attached at 10:11:18.139015 UTC to the already existing GPUI X11 client
`10485761` (`0xa00001`) on display `:110`:

```text
stdbuf -oL -eL xev -id 10485761 -event focus
```

This selects focus events on that existing window. It creates or maps no window,
grabs no input, changes no focus and starts/stops no reader, audio or D-Bus service.
The reader agent retained control of the session. Python Xlib was unavailable in the
examined interpreters, so the existing xev tool supplied the requested passive fallback.
`icons-attempt4/xev.txt` preserves raw events; `events.jsonl` timestamps each received
line with UTC and monotonic nanoseconds. Focus events do not carry an X server time:
these are **observer receive times**. The host/container clocks share the same system.

The first attach missed the preceding short session while the observer was prepared.
`icons-attempt3/missed-attach.txt` preserves that diagnostic; it supplies no mode evidence.
For attempt4 the observer was polling before session launch and attached successfully.
The xev connection ended when the reader session shut down its X server. Its recorded
return code 1 and `X connection ... broken` line describe that teardown, after the
reader observations, rather than an early loss of observation.

## Actual result

| UTC on 2026-10-25 | Native/reader observation |
| --- | --- |
| 10:11:37.437198 | Reader driver begins held Insert+KP_Begin. |
| 10:11:37.440443 / .440472 | Client `0xa00001`: FocusOut / FocusIn, NotifyGrab, NotifyNonlinear. |
| 10:11:37.641732 / .641817 | Same client: FocusOut / FocusIn, NotifyUngrab, NotifyNonlinear. |
| 10:11:37.475835 / .768796 | Orca logs window deactivate / activate. |
| 10:11:37.471599 / .633601 | Orca quits flat review / creates a new context. |
| 10:11:37.767629 / .782329 | Orca queues icon-0 focused false / true. |

The raw X events say `synthetic NO`. No NotifyNormal occurs in the held-command
interval. Earlier reader keypad commands also produce Grab/Ungrab pairs. Queried native-app AT-SPI
snapshots before the held command, after it and at the end expose icon-0 as focused.
These are sampled accessibility flags, not a continuous XGetInputFocus or GPUI
FocusHandle trace.

The only captured NotifyNormal pair appears during session teardown at 10:11:46.508:
FocusOut/NotifyNonlinear followed by FocusIn/NotifyPointer. This distinguishes those
normal/detail notifications from the reader-command Grab/Ungrab events. It is **not**
a controlled normal-window-focus-transfer regression test.

`correlation.json`, `focus-events.json`, `reader-actions.txt` and
`orca-event-excerpt.txt` under `icons-attempt4/` preserve the selected data.
Orca processes queued events asynchronously, including preceding KP_Right grabs.
The timestamps establish co-occurrence and ordering at the two observers, not a
one-to-one causal assignment of every queued reader event to a specific X notification.

## Source path and proposed next step

The locked, unmodified `gpui-pre-linux 0.3.9` source handles FocusIn at client.rs:1060
and FocusOut at :1071 without testing `event.mode` or `event.detail`:

- Every FocusIn calls `set_active(true)`, records keyboard focus and enables IME.
- Every FocusOut calls `set_active(false)`, clears keyboard/compose/preedit state,
  resets IME and deletes marked input.
- `X11Window::set_active`, window.rs:1335, invokes the active-status callback and
  forwards `update_window_focus_state` to AccessKit.

The source excerpts and hashes are preserved here. The original files were compared
against the locked crate archive by the separately prepared
[unapplied proposal](../../../proposals/x11-grab-focus/README.md).
The Xlib specification distinguishes normal/while-grabbed focus changes from events
generated *as if* focus moved when a keyboard grab activates/deactivates. That supports
reviewing a narrow Grab/Ungrab filter before backend activation/IME work; components
cannot intercept these earlier native state changes.

The proposal is not applied, compiled or runtime-verified. A new local compatibility
copy of the Linux package requires owner approval. Complete reader, IME, normal focus,
while-grabbed focus, host and release qualification remains open.
