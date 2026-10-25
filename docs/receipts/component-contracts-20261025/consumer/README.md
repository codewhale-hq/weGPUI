# Separate consumer build and X11 runtime — 2026-10-25

A separate Cargo application at `/workspace/.wegpui-cloud/consumer-contracts` builds and
runs the minimal component example against weGPUI through a **path dependency**. Its own
workspace-root patch table selects the approved local compatibility sources, proving that
the application can explicitly select the same dependencies. This is not a crates.io,
Git-dependency, packaged-crate, installer or fresh-machine qualification.

## Build and artifact provenance

The archived Cargo.toml, Cargo.lock, main source and platform helper exactly matched the
separate consumer files when this runtime was prepared. The successful root-controlled
build log and reverse dependency trees are in `build-retry.txt`. They identify
`wegpui 0.1.0`, `gpui-pre` / `gpui-pre-platform` 0.3.9, `accesskit_atspi_common` 0.19.1
and `accesskit_unix` 0.22.1, including the selected local paths. `source-sha256.txt`
identifies the archived fixture and collectors.

The executed immutable copy was:

```text
/workspace/.wegpui-cloud/artifacts/consumer-contracts/wegpui-preview-consumer
SHA-256 cefd3ea1d4187670c68c17907b39d1f9325ecfee098dae93ac1df001b6817c43
```

The root built that artifact before the later focus/contrast source edits. The runtime
collector did not invoke Cargo or rebuild it. `runtime/provenance.txt` records the
observed HEAD `a3ce3afca6417b7dd1b9d701456ae2d21d2150f4` and dirty status as context only;
neither is an attestation that the binary contains the final working tree. Later
styling changes require their own source gates and targeted evidence.

The first consumer build omitted `src/support/platform.rs`, so Cargo failed to read that
fixture helper. `build-first-attempt.txt` preserves the failure. Copying the unchanged
repository platform helper into the consumer fixed that setup error, and the retry
completed in 2.41 seconds. It was a consumer-fixture setup failure, not an accepted
library build or runtime result.

## Actual runtime result

The collector started a private D-Bus session, Xvfb display `:111`, Openbox and AT-SPI.
The app used X11 with `GPUI_X11_SCALE_FACTOR=1` and no WAYLAND_DISPLAY, on Debian 13.5
x86_64, Rust/Cargo 1.98.1 and Mesa 25.0.7 software Vulkan. Native client dimensions were
375×640 pixels. `runtime/provenance.txt` records exact tool and package versions.
ImageMagick captured the actual application window; XTEST supplied real pointer and
keyboard events. Pointer targets were normalized from AT-SPI desktop bounds to the
client frame, then passed to `xdotool mousemove --window`.

| Capture | Recorded observation |
| --- | --- |
| `initial` | Confirmed is visibly 0 and its accessible description is `0`. |
| `opened` | A real pointer click opens weGPUI dialog. Its title, body and buttons are exposed; the background opener is absent from AT-SPI. |
| `cancel`, `ok`, `wrap`, `back` | Tab focuses Cancel, then OK, then wraps to Cancel. Shift+Tab returns to OK. |
| `confirmed` | Return invokes confirmation, counter changes 0→1, dialog closes and focus returns to Open dialog. |
| `reopen-keyboard`, `close-keyboard` | Return reopens from the focused opener. Escape dismisses, restores opener focus and retains counter 1. |
| `reopen-pointer`, `confirmed-again` | Pointer reopens again. Two Tabs then Return confirm once, change counter 1→2 and restore opener focus. |

The focused OK, first confirmation and second confirmation screenshots were opened and
visually reviewed: the rings are visible and the displayed counter agrees with queried
AT-SPI descriptions. No application panic occurred. These screenshots show the
pre-contrast artifact's colors; they do not establish the later focus-color correction.

`python3 docs/receipts/component-contracts-20261025/consumer/verify.py` checks the saved
callback, focus and modal-tree contracts. Its passing result is in `verification.txt`.
Raw launch/input logs were archived losslessly as `.log.gz` after collection. The exact
run driver and AT-SPI collector are preserved under `runtime/`.

This receipt contains no screen-reader session and does not establish screen-reader
acceptance, cross-platform acceptance, complete component/theme parity or release
readiness. It is one actual separate-consumer X11 runtime sample on the identified binary.
