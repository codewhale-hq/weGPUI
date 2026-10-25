# Approved accessibility compatibility copies

The owner approved these local compatibility copies on 2026-10-25. Package versions
remain `accesskit_unix 0.22.1`, `accesskit_atspi_common 0.19.1`, `gpui-pre 0.3.9` and
`gpui-pre-linux 0.3.9`. Copies came from the exact locked crate archives; licenses and
package provenance are retained. Registry files were not edited. Root Cargo patches
select these copies without changing dependency requirements or feature edges.

## Compilation and consumer setup

**The matching patched GPUI copy is required to compile current weGPUI source.**
Semantics/Modal integration calls the added `Window::with_a11y_action_guard` API.
Unpatched registry GPUI 0.3.9 does not provide that API. All four patches below belong
in the consuming application's workspace-root Cargo.toml, using a matching checkout:

```toml
[patch.crates-io]
gpui-pre = { path = "/path/to/weGPUI/vendor/gpui_pre" }
gpui-pre-linux = { path = "/path/to/weGPUI/vendor/gpui_pre_linux" }
accesskit_unix = { path = "/path/to/weGPUI/vendor/accesskit_unix" }
accesskit_atspi_common = { path = "/path/to/weGPUI/vendor/accesskit_atspi_common" }
```

Dependency-manifest patch sections are not inherited. A Git or path dependency on
weGPUI alone does not apply these workspace patches. Versions stay unchanged, but
this preview therefore depends on its documented local compatibility sources.

## Scope and provenance

The original deltas and checksums are under `docs/proposals/accesskit-linux-compat/`,
`a11y-state-and-bounds/`, `x11-grab-focus/` and `modal-a11y-action-guard/`.
The approved changes repair AT-SPI cache signal encoding; forward disabled/read-only,
invalid, busy, expanded/collapsed and heading-level metadata; update X11 window bounds;
guard accessibility action dispatch for hidden/closing/stale modal targets, including
cached views; and ignore temporary X11 Grab/Ungrab focus notifications before backend
activation/IME changes. Normal and WhileGrabbed handling is retained.

Modified upstream source files are:

- `accesskit_unix/src/atspi/bus.rs`
- `accesskit_atspi_common/src/node.rs`
- `gpui_pre/src/window.rs`, `src/window/a11y.rs`, `src/view.rs`
- `gpui_pre_linux/src/linux/x11/client.rs`

Earlier Linux runtime receipts are in `docs/receipts/linux-bridge-20261025/`,
`input-contracts-20261025/`, `data-contracts-20261025/` and
`wayland-20261025/bridge-followup/`. Their exact artifact hashes distinguish those
runs from the later guard and focus-mode changes. Cache/reopen and X11 bounds results
remain sampled checks, not full screen-reader acceptance. The new focus-mode controls,
real Fcitx and Orca samples are in `docs/receipts/x11-grab-focus-20261025/`; exact new
artifact hashes and remaining listening/reader/IME limits are recorded there. Modal
action-guard checks remain separate. EditableText is unavailable in the bridge.

No forked package publication, GPUI version change or release distribution is
authorized by these compatibility approvals. Full accessibility/platform
qualification remains separate.
