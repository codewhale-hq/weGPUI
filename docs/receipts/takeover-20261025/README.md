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
