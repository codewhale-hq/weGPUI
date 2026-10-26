# Platform feasibility and acceptance — 2026-10-25

The target is macOS, Windows, Linux, iOS and Android. This document records what the pinned
dependencies can do, what was actually built or run, and the smallest maintainable path to the
rest. A platform is **supported** only when every column in the acceptance matrix has a receipt
from that platform. Nothing below reaches that bar yet.

## Historical dependency investigation (0.3.7 and proposed 0.3.9)

Sources: crates.io `gpui-pre =0.3.7` (a snapshot of `zed@1a28cff`) and `gpui-pre-platform =0.3.7`,
read from the locked Cargo registry; upstream Zed `main` at
`347f171a3fd00a3b672382d78348b37bd969d17c` (2026-10-24), read from a sparse clone of
`crates/gpui*`.

| Fact | Source |
| --- | --- |
| `gpui_platform::current_platform` has `macos`, `windows`, `linux`/`freebsd` and `wasm` branches only. On iOS/Android it has no tail expression, so the crate fails with `E0308` (expected `Rc<dyn Platform>`, found `()`). | `gpui-pre-platform-0.3.7/src/gpui_platform.rs:57`; receipts `check-lib-aarch64-*.txt` |
| weGPUI's library code does not call `gpui_platform`; the gallery and minimal example do. | `rg gpui_platform src examples` |
| `gpui_platform` depends on `gpui_linux` with `default-features = false`. With only `font-kit` enabled, Linux compiles, but the windowed constructor reaches `unreachable!("At least one of the \"wayland\" or \"x11\" features must be enabled …")`. | `gpui-pre-linux-0.3.7/src/linux.rs:56`; receipt `run-linux-baseline-xvfb.txt` |
| GPUI core has `TouchEvent`; its gesture arena delivers taps as synthesized mouse down/up and pans as `ScrollWheelEvent`. At 0.3.7 the arena has no pinch recognizer, despite the module's introductory rustdoc. `PinchEvent` exists separately. Core also has `WindowInsets { safe_area, ime }`, virtual-keyboard requests and `AppLifecyclePhase`. | `gpui-pre-0.3.7/src/gestures.rs`, `window.rs::dispatch_recognized_touch_gesture`, `platform.rs` |
| GPUI core type-checks for `aarch64-apple-ios` and `aarch64-linux-android`. | receipts `check-gpui-core-*.txt` |
| AccessKit adapters are wired for macOS, Windows and Linux (`accesskit_macos`, `accesskit_windows`, `accesskit_unix`). No mobile adapter is wired in the pinned crates. | `gpui-pre-{macos,windows,linux}-0.3.7/Cargo.toml` |
| `gpui-pre-wgpu 0.3.7` (wgpu renderer plus cosmic-text/swash text system) type-checks for `aarch64-linux-android`. | receipt `check-gpui-wgpu-android.txt` |
| Upstream Zed `main` has `crates/gpui_ios` (UIKit, CoreText, Metal through `gpui_apple`; lifecycle, safe-area and keyboard insets, native text input, raw touches) and `gpui_platform` now dispatches `target_os = "ios"` to it. It has no AccessKit adapter, and `prompt_for_paths`/save dialogs return "not yet implemented for iOS". | `zed/crates/gpui_ios/src/ios/platform.rs:313,328`; `zed/crates/gpui_platform/src/gpui_platform.rs:68` |
| crates.io `gpui-pre-ios 0.3.9` (snapshot of `zed@b4ecc45`) requires `gpui-pre =0.3.9`. `gpui-pre-platform 0.3.9` does not yet contain the iOS branch. | crate manifests |
| `gpui-pre-ios 0.3.9` cannot be type-checked from Linux: `gpui-pre-apple`'s build script produces Metal shader files only on an Apple host, with or without `runtime_shaders`. | receipts `check-gpui-pre-ios-039*.txt` |
| Upstream Zed has no Android platform crate. | sparse clone listing of `zed/crates/gpui*` |
| `gpui-pre-mobile 0.1.0` (`longbridge/gpui-mobile`) implements iOS and Android outside Zed, about 46,700 lines, pinned to `gpui-pre =0.3.4`. It has no AccessKit integration, and its README states IME composition, accessibility and lifecycle hooks are incomplete. | crate source and README |
| weGPUI compiles unchanged against `gpui-pre =0.3.9`/`gpui-pre-platform =0.3.9` (`cargo check --all-targets`, Linux, in a scratch copy). | receipt `check-wegpui-on-gpui-039.txt` |
| The `wasm32` path needs nightly: `wasm_thread` uses `#![feature(stdarch_wasm_atomic_wait)]`. | receipt `check-lib-wasm32-unknown-unknown.txt` |

## Historical Linux environment

Linux x86_64 cloud container, Rust/Cargo 1.97.0, no GPU, no KVM, no Apple host and no physical
devices.

- **Linux, historical manifest before windowing patch:** `cargo build --all-targets --locked --offline`,
  `cargo clippy --all-targets --locked --offline -- -D warnings` and `cargo fmt --all -- --check`
  all exited 0. Launching the gallery under Xvfb panicked with the `unreachable!` above.
- **Linux, scratch copy with `features = ["font-kit", "wayland", "x11"]`:** the build added 34
  packages to `Cargo.lock` (listed in `linux-features-lock-delta.txt`) and changed no existing
  version. The gallery opened a real X11 window under Xvfb, rendered through Mesa's software
  driver, navigated on an `xdotool` click and moved focus with `Tab` (screenshots in the receipts).
  `accesskit_unix` exposed the tree to AT-SPI (`atspi-before-*.txt`, read with `pyatspi`). No Orca
  session, Wayland compositor or physical display was used.
- **Windows:** `cargo build --all-targets --target x86_64-pc-windows-gnu --locked --offline` exited
  0 with mingw-w64 and linked `gallery.exe`. Commit `0637c25` reports a Wine 9.0 startup failure
  involving `icuuc.dll`, but no Wine stdout/stderr receipt was committed. No native Windows host was available.
- **iOS/Android:** before `0637c25`, `cargo check --lib` failed in `gpui_platform` (E0308).
  With `gpui_platform` as a dev-dependency (applied in `0637c25`, see below), `cargo check --lib`
  succeeds for `aarch64-apple-ios`, `aarch64-apple-ios-sim` and `aarch64-linux-android`. That only
  proves the library type-checked at that change, not at every later revision. Nothing was linked,
  packaged, launched or touched.

## macOS follow-up (October 24 local time)

[Receipts](receipts/2026-10-24-macos/README.md) identify the exact checked source and incomplete
runtime attempts. The build and strict documentation checks passed. Minimal dialog interaction
and selected AX trees were observed. Select keyboard attempts did not establish an interaction
pass. VoiceOver was attempted without a captured announcement transcript and ended disabled.
iOS setup reached an installed runtime only. No platform has completed acceptance.

## Historical Linux follow-up — October 24

The approved patch is applied with exactly 34 added lock packages and no removed versions.
Both examples build and render under isolated Xvfb on Debian 13 with Rust 1.99.1.
XTEST confirms the minimal dialog counter and gallery select change/dismissal; AT-SPI
confirms sampled modal isolation. Earlier failed probe attempts are retained.
See [receipts](receipts/cloud-linux-20261024/README.md). No Wayland or Orca pass is claimed.

## Current Linux continuation — October 25

Rust/Cargo 1.98.1 and GPUI 0.3.9 are used for the current cloud source. Actual X11 component
interaction/geometry and Orca transcript/audio samples are recorded in the October 25
component receipts linked from [ACCEPTANCE](ACCEPTANCE.md). Actual Wayland clients run with
DISPLAY absent under nested Weston; protocol, input, nested modal and text checks are in
`receipts/wayland-20261025/`. The owner-approved compatibility copies in `vendor/` repair
sampled cache/state export and X11 screen-origin failures. Root Cargo patches are required
in downstream consumers; see [vendor guidance](../vendor/README.md).

These samples do not complete a platform: hardware GPU/compositor, physical monitor/live
scale changes, fresh installation, the complete state matrix and human speech listening
remain open. Wayland does not expose a global desktop origin. Current-source macOS/Windows
reruns require those hosts; their historical samples do not qualify the Linux changes.

## Current RTL continuation

The [RTL continuation](receipts/rtl-completion-20261025/README.md) adds current macOS ARM64
and Linux ARM64 X11 software-renderer samples for paragraph wrapping and native input. These
are bounded samples. The existing Windows ARM64 VM also passes the current source bundle’s
all-target build, 59 library/three gallery tests, strict Clippy/docs and standalone consumer.
Visible Windows runtime remains blocked at sign-in. Linux x86_64/hardware, real RTL IME, complete reader matrices
and fresh installation still need their own evidence. VoiceOver remains off.

## Acceptance matrix

Each cell needs a receipt from that platform. Simulators and emulators count only when labelled
as such. Current state:

| Platform | Build | Launch | Rendering | Interaction | Accessibility | Packaging |
| --- | --- | --- | --- | --- | --- | --- |
| macOS | Current local ARM64 all-target build, 59 library/three gallery tests, strict Clippy/docs at `f26610b` | Current sampled Languages/input and ten route launches; full matrix open | Earlier 28 review pairs; new diagnostic images, no new parity verdict | Minimal dialog open/close/OK and focus restore; select-key attempts inconclusive | Modal isolation in minimal/picker/select/half-screen; closed swipe actions absent; article headings/lists; option labels in AXTitle; VoiceOver incomplete | Not done |
| Linux | Rust 1.98.1 current-source gates recorded in continuation receipts | X11 plus actual Wayland clients under nested Weston | Historical 192 pairs; 216 reviewed native-only theme/state images and later targeted component reviews | Native keyboard/pointer callbacks, scale/move text geometry, sampled Wayland input; historical XIM/portal | Actual Orca transcript/audio samples and AT-SPI contracts; full reader/listening acceptance open | Historical consumer/package checks; current patched-consumer and distribution qualification separate |
| Windows | Current ARM64/MSVC source-bundle all-target build, 59 library/three gallery tests, strict Clippy/docs and standalone consumer at `f26610b` | Historical Windows 11 ARM samples; current visible UI blocked at sign-in | Historical native VirtIO samples | Historical sampled controls/inline links; current-source rerun missing | Missing (Narrator/NVDA) | Historical package build; installation qualification missing |
| iOS | Library type-checks pass; hosted simulator app compilation fails | Upstream 0.3.9 API mismatch; no launch | Missing | Missing (touch, keyboard, IME, rotation, lifecycle) | Missing; upstream backend has no AccessKit | Not done |
| Android | Library type-checks | Blocked: no Android backend anywhere upstream; no KVM for an emulator here | Missing | Missing | Missing (TalkBack) | Not done |

Mobile interaction acceptance means touch taps, drags and scrolling; the software keyboard and
IME composition; safe-area and keyboard insets; orientation changes; background/foreground
lifecycle; and a screen-reader pass. Desktop acceptance means real windows, keyboard and focus,
resizing and a screen-reader pass.

## weGPUI work required for mobile (independent of the backend choice)

A backend alone would not make the components mobile-ready. Read against `gpui-pre 0.3.7`:

| Area | What GPUI provides | What weGPUI does today | Needed |
| --- | --- | --- | --- |
| Taps | Recognised taps are delivered as synthesised mouse down/up (`gestures.rs`, `RecognizedTouchGesture::Tap`) | Click handlers | Likely works as is; unverified |
| Drags | A touch becomes a pan (`ScrollWheelEvent` with `TouchPhase`) unless an element claims `TouchDragEvent` at `Started` with `prevent_default` | Swipe cells, slider and half-screen grab use mouse events; picker columns also handle `on_scroll_wheel` already | Handle touch drags where needed and verify the picker's existing pan path on a phone |
| Long press, pinch | `LongPressEvent`, `capture_long_press`; `on_pinch` exists but the 0.3.7 touch arena does not recognize pinches | Not used | Backend/recognizer work before any optional gallery pinch-zoom (upstream has none) |
| Software keyboard | `Window::request_virtual_keyboard`/`dismiss_virtual_keyboard`; the platform may require a user gesture | Never requested | Request on tap-to-focus in `TextInput` and dismiss on blur |
| Safe area and keyboard occlusion | `Window::fully_visible_bounds`, `visual_viewport_bounds` (`WindowInsets::safe_area`/`ime`) | Not applied (bottom-fixed areas, tabbar, sheets, toasts) | Pad or position against the visible bounds; scroll the focused input into view |
| Lifecycle | `Platform::on_app_lifecycle` (`Background`/`Foreground`) exists on the platform trait only; at 0.3.7 no `App` API forwards it | Not observed | Upstream plumbing or a newer GPUI, then pause timers and animations (toast, vcode countdown, spinners) |
| Accessibility | AccessKit tree built in core | Roles, names and states as on desktop | Needs a mobile AccessKit adapter in the backend |

These changes are safe on desktop, where insets are zero and the soft-keyboard calls are no-ops.
They were not made in this pass because no mobile run could verify them.

## Applied without new dependencies

`gpui_platform` moved from `[dependencies]` to `[dev-dependencies]`. The library never used it,
`Cargo.lock` is unchanged, and the gallery and examples still use it. Consumers now pick their own
platform crate (`gpui_platform` on desktop, a mobile backend on a phone). On macOS they must enable
`gpui_platform`'s `font-kit` feature themselves, as the README shows. This is what makes the
mobile library type-check above.

## Dependency proposals and decisions

### 1. Linux windowing features (approved and applied)

Approved October 24 for both examples. The first slice set their shared dev-dependency to
`gpui_platform = { package = "gpui-pre-platform", version = "=0.3.7", features = ["font-kit", "wayland", "x11"] }`.
That first slice changed no pins; the subsequent iOS slice moves to 0.3.9. It added 34
transitive packages, all already depended on by the pinned
`gpui-pre-linux` crate's optional features (`wayland-client`, `x11rb`, `xkbcommon`, `zed-xim` and
their helpers). Effect: the Linux gallery launches. Public API: none. Maintenance: CI needs
`libxkbcommon-dev`, `libxkbcommon-x11-dev`, `libwayland-dev` and `libx11-xcb-dev`, which
`.cnb.yml` already installs.

### 2. iOS through upstream `gpui_ios` (implemented; upstream build blocker)

Approved October 24, following the historical macOS verification. Both GPUI pins are now `=0.3.9` with a target-specific dev-dependency,
`[target.'cfg(target_os = "ios")'.dev-dependencies] gpui_ios = { package = "gpui-pre-ios", version = "=0.3.9" }`.
Both examples call `gpui_ios::current_platform(false)` on iOS through a shared helper
(until a published `gpui-pre-platform` carries Zed's iOS branch). The desktop dependency
is excluded on iOS. [The bundle script and Info.plist](../examples/ios/README.md) follow
the published upstream example and preserve failed builds. It does not boot or launch a
simulator. [Linux and script-test receipts](receipts/ios-path-20261024/README.md)
are separate from Apple app build/runtime evidence. The first hosted Apple build
[failed inside the published backend](proposals/gpui-ios-039-incompatibility.md):
six calls reference core APIs absent from the exact 0.3.9 dependency. No newer published
iOS release was available. No dependency fork or registry patch has been applied.

- Public API: none expected. The library compiled unchanged on 0.3.9.
- Renderer: Metal through `gpui_apple`, which is shared with macOS.
- Input: UIKit touches feed GPUI's gesture arena. Native text input exists, but weGPUI's
  `TextInput`/`TextArea` must also call `request_virtual_keyboard` on focus and honour
  `WindowInsets::ime`. That is weGPUI work.
- Accessibility: upstream has no AccessKit adapter for iOS. VoiceOver acceptance needs an
  upstream contribution (`accesskit` has no UIKit adapter either, so this is substantial).
- Maintenance: follow upstream; the `gpui-pre` snapshots move weekly.
- Needs a macOS host with Xcode for building, the iOS Simulator, and a device for physical
  receipts. The later macOS session installed the iOS 26.2 runtime and created an iPhone 17
  Simulator, but did not boot it or build an app. Cloud Linux cannot replace Apple build tooling.

### 3. Android through an upstream `gpui_android` contribution

No backend exists in Zed. The smallest maintainable path is to contribute one upstream, in the
shape of `gpui_ios`/`gpui_linux`. Window, input and lifecycle would come from `android-activity`
(GameActivity, for IME `InputConnection`). Rendering and text would reuse `gpui_wgpu`, which
already type-checks for Android. Accessibility would use `accesskit_android` (0.9.0 on crates.io).
That is a new crate outside weGPUI and a substantial project: multiple person-months judging by
`gpui_ios` (about 3,500 lines without accessibility) and `gpui-pre-mobile` (about 46,700 lines).

Alternatives considered and **not recommended**:

- Adopting `gpui-pre-mobile` means a third-party fork-style platform pinned to `gpui-pre =0.3.4`,
  with no accessibility. It would force a pin downgrade or a fork.
- Running `gpui_web` in a WebView needs nightly Rust and a web host. It amounts to replacing the
  native platform with a web runtime, which the brief excludes.
- Forking GPUI is excluded unless approved.

### Decision summary

| Decision | State / recommendation |
| --- | --- |
| Enable `wayland`/`x11` for both examples | Applied; isolated X11 build/rendering/input/AT-SPI receipts |
| Move to `gpui-pre =0.3.9` and add `gpui-pre-ios` | Implemented scaffold; separate authorized Apple host needed for app build/runtime acceptance |
| Fund or contribute `gpui_android` upstream | Required for any honest Android claim; scope separately |
| iOS AccessKit/VoiceOver | Upstream contribution; without it, iOS fails the accessibility column |

weGPUI has historical macOS samples, Linux X11 experiment receipts, a Windows cross-build and
mobile library type-checks. None of these qualifies all desktop platforms or mobile support.
