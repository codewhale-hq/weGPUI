# Saved accessibility actions during modal close — Linux X11, 2026-10-25

**A closing-input defect is reproduced.** On the immutable `49268de` executable, saved
custom Click actions at requested 0/1ms invoke the controlled row callback with
`Presence::is_open()==false`. Saved built-in Component.grabFocus at 0/1ms also steals the
restored focus. Clicks at 20/100/500ms instead encounter removed AT-SPI objects. These later
rejections do not establish an immediate model guard.

## Source and method

The root built a separate path consumer using the existing approved compatibility packages;
[consumer-build.log](consumer-build.log) is its successful Rust 1.98.1 build. No new production
dependency or package version was introduced by this runtime subtask. The executed copy is:

```text
/workspace/.wegpui-cloud/artifacts/closing-actions/probe-49268de
SHA-256 36555b7ae634701e14c0ed394220e1b6ad224758ff698bd54be66f44c1699bd6
```

The root attested clean production source at GitHub main `49268de7bb78a69cb56b96f6cff02a2d9284e71d`
when building it. `compiled-source-git-tree.txt` identifies that committed Cargo/source/vendor
tree; the three `compiled-*-49268de.rs.txt` files preserve the relevant source. Every runtime
copies the actual consumer manifest, lock, main source and platform helper. `probe.rs.txt` is
the originally prepared fixture; the run's `consumer-main.rs.txt` identifies the compiled one.
The later Swiped Escape correction was edited while these runs used the immutable old binary;
live workspace hashes/diffs in later run directories are diagnostic context, not this binary's
build attestation. No production Rust or vendored file was changed by this collector.

Environment: Debian 13.5 x86_64, Rust/Cargo 1.98.1, Xvfb 21.1.16 on display 103, Openbox 3.6.1,
AT-SPI 2.56.2, python3-pyatspi 2.46.1 and real XTEST input, scale 1. Exact package versions are in
[runtime-packages.txt](runtime-packages.txt). Each case starts a fresh D-Bus/Xvfb/Openbox/app
session; no Orca or audio session is used. AT-SPI observations are not screen-reader acceptance.

The fixture uses an ordinary controlled `SwipedCell` inside a two-second closing `Modal`.
Its normal owner callback assigns `row_open` and logs the current Presence state; it deliberately
adds no caller-side guard. The driver opens by native pointer, invokes AT Click twice while
open to prove the callback path, and retains that exact object/interface across a native mask
click. The Focus cases additionally prove Component.grabFocus moves initial focus from Dialog
to the row while open. A fresh tree and native window capture accompany each phase.

The native mask command uses separate mousedown/up, avoiding xdotool click's 100ms trailing
wait. Requested delay is measured from the native command's return, not compositor delivery.
Both command start/return and action invocation use `time.monotonic_ns`; the app logs logical
hide/callback order with its own elapsed milliseconds. Each case also invokes the same saved
proxy after more than two seconds and then reopens through real pointer input and activates
via a fresh AT interface. `drive.py` and `run-session.sh` preserve the current collector, and
per-run `drive-used.py` / `run-used.sh` preserve what each case actually executed.

## Recorded results

| Saved route | Requested delay | After command return / start | Raw result and callback/focus observation |
| --- | --- | --- | --- |
| Custom Click | 0ms |0.002 / 3.469ms | Returns true; `3655 hide` then `3655 request=3 open=true presence_open=false`; after close only Frame is focused. |
| Custom Click | 1ms |1.083 / 4.198ms | Returns true; `3767 hide` then `3769 request=3 open=true presence_open=false`; after close only Frame is focused. |
| Custom Click | 20ms |20.088 / 23.074ms | Unknown object; no closed-model callback; opener remains focused. |
| Custom Click | 100ms |100.089 / 103.474ms | Unknown object; no closed-model callback; opener remains focused. |
| Custom Click | 500ms |500.164 / 503.286ms | Unknown object; no closed-model callback; opener remains focused. |
| Built-in Focus | 0ms |0.002 / 3.162ms | grabFocus returns true after hide; no extra model callback, but after close only Frame is focused. |
| Built-in Focus | 1ms |1.096 / 8.553ms | grabFocus returns true after hide; no extra model callback, but after close only Frame is focused. |

The directories are `mask-fast-{delay}ms/` and `focus-fast-{delay}ms/`. All seven baseline
Click controls emit true/false once each while Presence is open. The four failing early cases
lose restored opener focus; the three later Click cases keep it. All seven saved-proxy calls
at 2.36–2.91 seconds report a stale-object error (`The application no longer exists`). That
string does not describe an application crash: the same launched process reopens and its
fresh AT Click works in every case. Callback logs, queried focus and AT return/error values
are independent observations.

[observations-summary.json](observations-summary.json) records exact times, raw errors,
closed-state callbacks and focused targets. [verify-observations.py](verify-observations.py)
checks those archived controls and failures; [verification.txt](verification.txt) explicitly
reports the four reproduced closing-safety failures rather than claiming the bug fixed.
Representative 0ms/20ms during-close and 0ms after-close captures were opened and reviewed;
the actual closing panel and later page render. Screenshots do not substitute for callback
or focus assertions.

## Preserved earlier attempts

- `initial-20ms/` attempted Escape, but the already-collapsed row consumed it; there is no hide
  log, so this is not evidence about closing AT actions. The driver later failed to find the
  still-hidden page opener. This exposed the separate collapsed-row Escape propagation defect;
  its corrected native sequence is in [the Escape followup](../swiped-escape-followup/README.md).
- `mask-20ms/` completed with object rejection and opener restoration, but xdotool click's
  trailing wait made invocation approximately 129ms after command start. It remains a real
  observation with limited timing scope; the fast 20ms case replaces it in the primary matrix.
- `drive.py.txt` preserves the original unexecuted plan. The executable collector evolved as
  above; the original text file is not relabelled as a driver that ran these cases.

## Reproduction and remaining work

Run inside the prepared container, with the immutable binary or a newly qualified build:

```sh
dbus-run-session -- bash run-session.sh BIN 0 NEW_OUT click
dbus-run-session -- bash run-session.sh BIN 1 NEW_OUT focus
```

The central GPUI dispatcher invokes saved custom listeners directly and handles Focus through
its saved focus-id map, before a new hidden tree removes the nodes. Mouse capture does not
cover either route. The bounded [unapplied hook proposal](../../../proposals/modal-a11y-action-guard/README.md)
is ready separately; it would introduce a required patched-GPUI build API and needs additional
owner approval. No extra vendored compatibility change is applied by these receipts. Any correction must rerun early
Click and Focus plus normal open controls, nested/deferred/passive/reopen behavior and source
gates. This sample does not qualify every custom handler, timing window, host or reader.
