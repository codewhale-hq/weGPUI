# Cached GPUI accessibility baseline — Linux X11, 2026-10-25

This is an actual runtime reproduction on the existing approved GPUI source,
before the proposed modal action-guard/cache-bypass patch. The fixture has no
Modal, Presence, or guard. Missing targets/actions here therefore demonstrate
cache replay loss, not a successful guard.

The immutable external fixture executable has SHA-256
`26eda739b3a8444f33c688a9a76c466a4ef37817fe067fbb4642ef44d57d53fb`.
The exact external consumer source, platform module, Cargo files, vendor hashes,
source diff, source HEAD/status and Rust 1.98.1 provenance are retained per run.
The root-built source matches proposal `cached-probe.rs.txt`, SHA-256
`688be4bf6989b0ea91e9b26fe1071ea9a6c99a6be839518ca6e63c5c69ec61a8`.
The initial compile error from passing `cx.listener` to a custom AT callback and
the successful corrected external build are retained as separate logs. No
production source or vendor patch was applied by this diagnostic.

`cache-run.sh` starts Xvfb :103, openbox and a private a11y-enabled D-Bus session,
with GPUI_X11_SCALE_FACTOR=1. `cache-drive.py` retains the same AT action/component
proxies while taking trees through separate fresh clients. F3 snapshots log native
model values, render counters and focused handles without notifying the view.
The window is activated once; after a parent-only F1 the fresh tree is captured
before any action or focus change. This avoids concealing cache reuse through
focus-triggered full refreshes.

## Observed results

`run-1` and `run-2` exited 0 as diagnostic collectors. Both intentionally retain
the reproduced accessibility failures and D-Bus errors; exit 0 is not component
acceptance. In both runs cached pixels remain visibly present after parent-only
redraws while the cached accessible button disappears.

`run-2` adds independent fresh-client rescue controls that the first collector
lacked. `summary.json` derives and checks these native/tree observations:

| Step | Cached child | Live control |
| --- | --- | --- |
| Initial custom AT Click and Focus | Callback increments to 1; native cached focus true | Later native focus moves to live child |
| First parent-only F1 | Render count stays 7; fresh tree omits cached target; saved Click raises `Unknown object`; no callback | Render count 7→8; target remains; custom Click increments 0→1 |
| F2 direct cached notify | Render count 7→8; target reappears; fresh-client custom Click increments 1→2 | Remains available |
| Second parent-only F1 | Render count stays 9; target disappears again; saved Focus errors; native focus remains live | Render count 11→12; native focus true |
| F4 full window refresh | Target reappears; fresh-client Focus succeeds; native cached focus true | Native live focus false |

The cached/live paint captures, including parent-only loss and final restored
focus, were visually reviewed. The first run's and second run's original saved
proxies remain defunct even after a target reappears and report `The application
no longer exists`. This does not mean the process crashed: fresh clients see the
same live application and successfully invoke the restored cached target. Both
old-proxy errors and fresh-client successful controls are preserved separately.

The runtime agrees with the source finding: active accessibility frames clear
node/listener metadata, while cached prepaint/paint replay does not rebuild that
metadata. The paired proposed view-cache bypass addresses an established existing
gap. These baseline runs do not validate that unapplied patch, its performance,
modal lifecycle guards, or any other host.

This is Linux AT-SPI interface/native state evidence. No screen reader or speech
acceptance is claimed. DISPLAY :103 and the diagnostic processes were cleaned up.
