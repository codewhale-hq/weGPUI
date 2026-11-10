# Adaptive layer — 2026-11-09

`wegpui::adaptive` adds `SizeClass`, `SafeArea`, `AdaptiveShell`, `AdaptiveSplit` and
`NavigationBar`. `examples/adaptive.rs` is one application with no platform checks of its
own; the captures below are that one source built for macOS and for the iOS simulator.

Host: macOS 26.1, Xcode 26.2, Rust 1.99.0, Apple silicon. Simulators: `weGPUI iPhone 17`
and `iPad Air 11-inch (M3)`, both iOS 26.2. Sample people and messages are synthetic.

## Captures

| File | What it shows |
| --- | --- |
| [macos-regular-dark.png](captures/macos-regular-dark.png) | 1040×720 window: rail with unread badge, resizable list, placeholder detail |
| [macos-compact-dark.png](captures/macos-compact-dark.png) | 390×800 window: list above a bottom tab bar |
| [iphone-list.png](captures/iphone-list.png) | Phone launch: list clear of the status bar and home indicator, tab bar |
| [iphone-conversation.png](captures/iphone-conversation.png) | After tapping a row: navigation bar with Back, conversation, tab bar hidden |
| [iphone-keyboard-before-tab-bar-hiding.png](captures/iphone-keyboard-before-tab-bar-hiding.png) | Software keyboard up with typed text; the layout shrinks above the keyboard. Taken before `tab_bar_hidden` was added, so the tab bar is still drawn |
| [ipad-two-pane.png](captures/ipad-two-pane.png) | iPad portrait (820 pt): rail, list with the tapped row selected, conversation beside it |

## Observed interaction

iPhone simulator, by injected taps and text:

- Tapping a conversation pushed its detail and cleared that row's unread count (tab badge 3 → 1).
- Tapping the composer focused it and raised the software keyboard; typed text appeared;
  Send appended an outgoing bubble and cleared the draft.
- Back returned to the list and the tab bar; the Settings tab opened and reported
  "Compact: tab bar, one pane".

iPad simulator: tapping a row opened it in the detail pane beside the list; the rail's
Settings button switched destination and reported "Regular: rail, two panes".

macOS: both window sizes rendered as captured. No pointer or keyboard input was sent to the
macOS windows in this session, so rail clicks, divider dragging and keyboard traversal there
are unverified.

## Checks

On the final source, all exit 0:

```sh
cargo fmt --all -- --check
cargo build --all-targets --locked --offline
cargo test --lib --locked --offline              # 55 passed
cargo test --example gallery --locked --offline  # 3 passed
cargo clippy --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline
cargo check --lib --locked --target aarch64-apple-ios-sim
cargo check --lib --locked --target aarch64-linux-android
python3 scripts/tests/test_ios_bundle.py         # 5 passed
bash examples/ios/build-simulator.sh adaptive
cargo +1.95.0 build --all-targets --locked
cargo check --examples --locked --target aarch64-apple-ios-sim
```

`aarch64-apple-ios` was not checked: that Rust target is not installed on this host.

Clippy had failed on `main` since the October 29 desktop commits (nine findings in
`src/desktop/panels.rs` and `choice_picker.rs`). They are fixed here without behaviour
changes: collapsed conditions, two `allow` attributes, one `?`, one needless return.

## Limits

- No pushed-page transition and no edge-swipe back gesture.
- Not exercised: rotation, light/dark and care modes on the simulators, RTL, screen readers,
  a physical device, Linux and Windows.
- The compact tab bar's badge description is the bare count.
- `DesktopComposer`'s default hint reads "⌘ Enter to send" on every platform.
- Android still has no backend; only the library type-check runs there.
