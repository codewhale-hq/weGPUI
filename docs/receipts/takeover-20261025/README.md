# Takeover receipts — 2026-10-25

Environment: Linux x86_64 cloud container (Ubuntu 24.04), Rust/Cargo 1.97.0, Xvfb with Mesa
software rendering, no GPU, KVM, Apple host or physical devices. Compiler progress lines
(`Compiling`/`Checking`/`Downloaded`) were stripped from the logs. Every other line is verbatim.

| File | Command and result |
| --- | --- |
| `build-linux-baseline.txt` | `cargo build --all-targets --locked --offline` at `e23436c`: exit 0 |
| `clippy-linux-baseline.txt` | `cargo clippy --all-targets --locked --offline -- -D warnings`: exit 0 |
| `fmt-linux-baseline.txt` | `cargo fmt --all -- --check`: exit 0 |
| `run-linux-baseline-xvfb.txt` | `gallery` under Xvfb: panic, no `wayland`/`x11` feature |
| `check-lib-<target>.txt` | `cargo check --lib --target <target>` before the dev-dependency move: Windows needs `windres` (mingw); wasm needs nightly; iOS/Android fail with E0308 in `gpui_platform` |
| `check-gpui-core-<target>.txt` | `cargo check -p gpui-pre --target <target>` for iOS/Android: exit 0 |
| `check-lib-devdep-<target>.txt` | library check with `gpui_platform` as a dev-dependency (scratch copy, now on `main`): exit 0 for `aarch64-apple-ios`, `aarch64-apple-ios-sim`, `aarch64-linux-android` |
| `build-windows-gnu-cross.txt` | `cargo build --all-targets --target x86_64-pc-windows-gnu`: exit 0 (cross-compiled; not run natively) |
| `check-gpui-wgpu-android.txt` | probe crate using `gpui-pre-wgpu =0.3.7` for `aarch64-linux-android`: exit 0 |
| `check-wegpui-on-gpui-039.txt` | scratch copy pinned to `=0.3.9`: `cargo check --all-targets` exit 0 |
| `check-gpui-pre-ios-039*.txt` | probe crate using `gpui-pre-ios =0.3.9` for `aarch64-apple-ios-sim`: fails, Metal shader build needs an Apple host |
| `build-linux-x11wayland-exp.txt`, `linux-features-lock-delta.txt` | scratch copy with `wayland`+`x11`: gallery build exit 0; 34 added lock packages, no version changes |
| `run-linux-x11-exp.txt`, `linux-x11-xvfb-*.png` | X11 gallery under Xvfb: window opened; `button` route; after an `xdotool` click on 普通型 and two `Tab` presses |
| `atspi-before-*.txt` | AT-SPI tree via `pyatspi` (scratch X11 build, `e23436c` sources) before accessibility fixes |

## Defect-fix verification (Linux X11 scratch build, fixed sources)

Runtime checks used the scratch copy with `wayland`/`x11` enabled, rebuilt from the committed
`src/` and `examples/`. Input came from `xdotool` (XTEST) under Xvfb without a window manager, so
apps receive keyboard focus only after a real click inside the window. That is why `--state`
startup routes were not driven by keyboard. accesskit reports the focused state only for an active
window, so focus was judged from screenshots.

| File | What it shows |
| --- | --- |
| `atspi-after-{dialog-ios1,picker-single,half-screen-dialog-1}.txt` | With a modal open, only the dialog subtree is exposed. Compare with the `atspi-before-*` files, where background buttons and links stayed exposed. |
| `atspi-after-dialog-default.txt` | No modal: the page is exposed under an unnamed `panel` (the `ModalScope` group). |
| `linux-x11-select-keyboard-sequence.png`, `atspi-after-form_select-keyboard.txt` | `form_select` opened by click, then Down, Down×3, Up×5, End, Home, Escape. Focus stays within the three options, Home/End reach the ends, and Escape closes the menu and restores the trigger focus ring. |
| `probe.rs.txt` | Manual probe app (not part of the crate) with two LinkText links, non-finite Slider inputs, an all-disabled picker and a disabled default value. |
| `probe-baseline-e23436c.txt`, `linux-x11-probe-baseline-e23436c.png` | Baseline: `Slider::range(NaN, 10.)` panics in `f32::clamp`; a NaN value renders "NaN" with no handle; the all-disabled picker confirms `["B"]`; the disabled default confirms `"Y (disabled)"`; only the first link is keyboard-reachable. |
| `probe-fixed-stdout.txt`, `linux-x11-probe-fixed-pickers.png`, `linux-x11-linktext-two-targets.png` | Fixed: no panic, NaN value at the midpoint, invalid ranges ignored; the all-disabled picker has a disabled confirm, Return emits nothing and `selected()` is empty; the disabled default falls back to Z; each link has its own focus stop, highlight and single activation. |
| `presence-interrupt-frames.txt`, `presence-interrupt-int-{base,fixed}.png` | Escape 20 ms after opening a half-screen dialog. Baseline jumps to fully open (mask gray 160 → 119), then fades. Fixed reverses from the current value. The fixed panel also enters sooner because it travels its own height rather than the viewport height. |
| `atspi-{before,after}-list-closed.txt` | A closed swipe cell's 删除 button was exposed before and is hidden after. |
| `atspi-{before,after}-article.txt` | List items are now exposed with `posinset`. Heading `level` is set on the AccessKit node, but `accesskit_atspi_common 0.19.1` does not forward it to AT-SPI. macOS/Windows exposure is untested. |
