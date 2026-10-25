# SwipedCell Escape propagation — 2026-10-25

Native input exposed a component bug: a focused, already collapsed SwipedCell consumed
Escape, so its enclosing Modal could not dismiss. The earlier observed failure is
preserved in [the custom-action attempt](../custom-actions-followup/initial-20ms/), where
Escape left the modal open after the row had been collapsed. That attempt's saved-AT
action results remain separate from this keyboard fix.

The narrow source change returns from the row's key handler when Escape finds its live
logical state already closed. The event then reaches the enclosing modal. Escape while
the row is open keeps its existing close request and row-focus restoration; Right is
unchanged. The public component documentation records the propagation rule. No new unit
test or dependency was added; the actual nested-control behavior is checked below.

## Exact executable and source

The root rebuilt the separate `wegpui-closing-actions-probe` consumer after the fix.
This collector copied and executed an immutable artifact:

```text
/workspace/.wegpui-cloud/artifacts/swiped-escape/probe-fixed
SHA-256 ca7d02cb6385bec9395b79778a20e900dcc24ebd1944ce9fae6451aa995375e9
```

`after/provenance.txt` records parent HEAD
`49268de7bb78a69cb56b96f6cff02a2d9284e71d`, source/fixture hashes, versions and timestamp.
The fix was uncommitted during the run; `after/swiped-source.diff` records its exact
delta. `after/git-status.txt` is context, not a claim that this binary represents a
future final working tree or commit. Consumer manifest, lock and full probe/platform
sources are archived in `after/`. The collector ran no Cargo command except reading its
version. Root source gates are recorded separately.

The consumer uses weGPUI 0.1.0 through a local path, GPUI/platform 0.3.9 and the approved
unchanged-version compatibility sources (`accesskit_atspi_common` 0.19.1,
`accesskit_unix` 0.22.1). This is a separate path-consumer fixture, not a package, Git
dependency or installer qualification.

## Native result

The runtime used Debian 13.5 x86_64, Rust/Cargo 1.98.1, Mesa 25.0.7 software Vulkan,
Xvfb 21.1.16, Openbox 3.6.1 and a private D-Bus/AT-SPI session on display `:114`.
`GPUI_X11_SCALE_FACTOR=1`; WAYLAND_DISPLAY was absent. ImageMagick captured the actual
480×640 client window. XTEST sent real keys and pointer events; the controlled row
was inside a Modal with a two-second presence animation.

| Capture | Verified observation |
| --- | --- |
| `initially-collapsed` | Pointer opens Modal, then Tab focuses the collapsed row. |
| `dismissed-from-collapsed` | Native Escape actually hides Modal and restores Open slow modal focus. |
| `expanded`, `action-focused` | Return reopens Modal; Tab and Left open the row through its controlled callback. Tab focuses Archive. |
| `row-closed-modal-open` | Escape from Archive closes only the row, emits the false callback and restores row focus. Modal is still exposed and the page remains hidden. |
| `dismissed-after-row` | The next Escape dismisses Modal and restores opener focus. |

`after/app.txt` contains exactly two modal opens, two actual hides and one true/false
callback pair. Both callbacks report `presence_open=true`; no redundant closed-row
callback occurs. The saved JSON trees confirm focus, Expanded/Collapsed states and
modal isolation. The initial collapsed row, closed row with modal still open and final
opener-focus screenshots were opened and visually reviewed. No application panic occurred.

Run `python3 docs/receipts/overlay-contracts-20261025/swiped-escape-followup/verify.py`
to check the recorded contracts. `verification.txt` retains the passing result.
The driver, collector and fixture hashes are in `source-sha256.txt`; all processes
were cleaned up after collection.

This receipt qualifies the named native X11 keyboard/callback sequence only. Queried
AT-SPI state is not screen-reader acceptance, and it does not establish every theme,
host, input method or component state as accepted.
