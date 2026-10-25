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
| `probe-input-stdout.txt`, `linux-x11-textarea-limit-undo.png` | TextArea with `set_max_len(Some(5))`: `ab cd` fills it and `XYZ` is rejected. Ctrl+Z gives `ab`, Ctrl+Z gives empty, Ctrl+Shift+Z gives `ab`. 😀😀 counts 4/5 and a third emoji is rejected (UTF-16 units, as HTML `maxlength`). `ab😀é` fills 5/5. |
| `linux-x11-fcitx5-pinyin-xim.png` | fcitx5 Pinyin over XIM (`XMODIFIERS=@im=fcitx`). Single-line search bar (left) and TextArea (right): focus, compose `ni hao`, Space commits `你好`, then `shijie` plus Escape cancels with nothing inserted. fcitx5 drew the preedit and candidates in its own window at the top-left, not inline at the caret. The cause (GPUI XIM spot location or the fcitx5 frontend) was not isolated. |
| `linux-x11-uploader-no-portal-error.png` | Uploader add with no D-Bus session: `prompt_for_paths` fails and the gallery shows the error in a warning Toptips. No sample file is added. |
| `linux-x11-uploader-portal-chooser.png`, `linux-x11-uploader-picked-error-cancel.png` | With `xdg-desktop-portal-gtk` in a D-Bus session: the GTK "Open File" chooser shows the prompt `选择图片`. Picking `photo.png` adds a thumbnail loaded from disk, picking `notes.txt` adds an error item, and Escape in the chooser adds nothing. Upload transport stays application-owned. |
| `linux-x11-capture-pair-dialog-ios1-light.png` | `scripts/capture.py dialog --state ios1` on Linux: X11 native capture at `GPUI_X11_SCALE_FACTOR=2` (Xvfb 1600×3600) beside Playwright 1.63 Chromium against `scripts/weui_upstream.py serve`. Linux fonts differ from the macOS baseline, so this pair validates the pipeline, not macOS parity. |
| `msrv-*.txt` | Clean snapshot of `1ea95a5`, `cargo +V build --all-targets --locked --offline`. 1.89 is refused (`oo7` requires 1.92); 1.92 fails (`slice::as_array`, `gpui-pre-util`); 1.93.0 and 1.94.0 fail (`std::hint::cold_path`, `gpui-pre`); 1.95.0 builds, and `clippy --all-targets -D warnings` on 1.95.0 passes. |
| `rustdoc-missing-docs.txt` | `RUSTDOCFLAGS='-W missing-docs' cargo doc --no-deps --locked --offline` after the rustdoc work: 0 warnings (baseline review: 1,019). Also passes with `-D warnings`. |
| `cargo-package-list.txt` | `cargo package --list --allow-dirty`: 172 files, with screenshots, receipts, scripts, `.cnb.yml` and `.github/` excluded. `cargo package --no-verify` produced a 399 KiB `.crate`, which was inspected and deleted. Nothing was published. |
| `linux-x11-minimal-example.png` | `examples/minimal.rs` on X11: start, click "Open dialog", Tab twice (focus stays in the dialog, on OK), Enter (count 1, dialog closed). |
| `probe-edges.rs.txt` | After the audit fixes, a picker `[A disabled, B, C, D disabled]`: Home selects B, End selects C, Return confirms C. |
