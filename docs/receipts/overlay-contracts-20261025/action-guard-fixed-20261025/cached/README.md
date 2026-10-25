# Approved cached-view accessibility runtime

Run 2 passes the focused Linux X11 AT-SPI fixture after the approved AT-active
cache bypass. Each parent-only redraw re-renders both children, every fresh tree
retains the cached and ordinary controls, and retained Click/Focus proxies remain
usable. The existing fixture has no Modal or action guard scope, so this receipt
qualifies cache behavior only.

## Source and environment

- Source HEAD at observation: `3840b47370f8defa925cbff7d3bf12ccf6250722`, with
  the approved compatibility changes still uncommitted. `run-2/source.diff`,
  `git-status.txt`, root/consumer manifests and `vendor-source.sha256` preserve the
  observed source and all four local compatibility patches.
- Existing-API fixture SHA256:
  `688be4bf6989b0ea91e9b26fe1071ea9a6c99a6be839518ca6e63c5c69ec61a8`.
  The copied fixture matches the earlier baseline; no Cargo command or source
  change was made for this runtime task.
- Original build SHA256:
  `7e5690d4d2a28eb04f5bf82041576417c6fc1dac206441e69ddd2f77416b05de`.
  Runtime executable SHA256:
  `c6b2ba7e82c0241424d0ddc01383baba4c993dc54374b06022555c30e180333d`.
  The build owner preserved the executable with GNU strip 2.44 using
  `--strip-debug`; see `run-2/binary-build-and-strip-lineage.txt`. This task checked
  the runtime hash and used that immutable executable directly.
- Shared `wegpui-198` Debian 13.5 container, Rust 1.98.1, Xvfb
  `:103 -screen 0 1280x1000x24`, Openbox, private D-Bus session and AT-SPI enabled,
  `GPUI_X11_SCALE_FACTOR=1`, no Wayland display. Package versions are preserved in
  `environment-packages.txt`.

Command from the repository root, using the existing managed Docker daemon:

```sh
env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS \
  -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH \
  docker --host=unix:///var/run/docker.sock exec wegpui-198 \
  bash /workspace/weGPUI/docs/receipts/overlay-contracts-20261025/action-guard-fixed-20261025/cached/cache-run.sh \
  /workspace/weGPUI/docs/receipts/overlay-contracts-20261025/action-guard-fixed-20261025/cached/run-2
```

Choose a new output path when repeating; the runner refuses to overwrite an
existing directory. `BIN` may select an already-built executable. The driver is
copied unchanged from the baseline receipt. Fresh subprocesses dump each tree;
the long-running client retains the original node/action proxies.

## Observations

| Step | Native evidence | AT-SPI evidence |
| --- | --- | --- |
| Initial controls | Cached Click increments `0→1`; cached then live native focus is confirmed | Initial cached Click/Focus and live Focus return true |
| First F1, parent notify only | Cached renders `7→8`; live renders `7→8` | The first fresh tree, before another action or focus request, retains both buttons |
| Retained cached Click | Cached clicks `1→2` | Original action proxy returns true |
| Live Click control | Live clicks `0→1` | Original live action proxy returns true |
| F2, child notify | Cached renders `10→11`; subsequent cached clicks `2→3→4` | Retained and fresh-process cached Click both return true |
| Second F1, parent notify only | Cached renders `13→14`; live renders `13→14` | The first fresh tree again retains both buttons |
| Retained cached Focus | `cached_focused=true`, `live_focused=false` | Original Component proxy's Focus returns true |
| F4, full refresh | Cached renders `15→16`; final native focus remains cached | Retained and fresh-process Focus both return true |

All 13 independently collected trees contain both named controls. There are no
action/proxy errors in run 2. `summary.json` maps each tree label to its native
snapshot and preserves the exact operation results. The app's F3 snapshot does
not notify or refresh the window. The F1 trees are therefore observed before
any later action, activation or focus request can force a full refresh.

The reviewed `parent-only-first.png` shows both controls still painted with live
focus; `final-focus-control.png` shows cached focus and the final click counts.
These screenshots support the visible state only; `app.log` provides native
callback/focus results and the JSON trees provide accessibility presence.

The [unchanged-GPUI baseline](../../custom-actions-followup/cached-baseline/README.md)
preserves the original failure: cached renders stayed constant on F1 while the
cached node disappeared, retained Click failed, and explicit child/full refresh
was required for fresh-process recovery. The approved bypass now re-renders the
cached child while accessibility is active, retaining its node and dispatch data.

## Preserved failure and limits

Run 1 exited 1 before the interaction controls. Openbox attempted to connect
before Xvfb was ready, so activation failed with missing `_NET_ACTIVE_WINDOW`.
Its logs and driver exception remain in `run-1/`. The runner now waits for X
display geometry and the window manager's `_NET_ACTIVE_WINDOW` support before
launching the fixture; run 2 exited 0.

This is an AT-SPI/native callback fixture, not an Orca screen-reader result. It
does not establish inactive-accessibility cache performance, other native hosts,
full component acceptance or Modal lifecycle behavior. The separate closing
overlay receipts qualify the approved action guard.
