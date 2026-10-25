# Component continuation — October 24, 2026

This receipt covers the component source carried by this commit. `source-hashes.json`
records 153 Rust/assets/lockfile hashes, all matched against the Windows package.
Its original Cargo.toml also matched the checkout. Windows package SHA-256:
`f74f376a7742ac138e6465edc955228dfdc3ca481c4ca426b9b1d46cded9d03b`.
Docs changed after packaging; no Rust source changed during these checks.

## macOS

macOS 26.1 (25B78), Apple Silicon, Rust 1.99.0, Xcode 26.2 (17C52).
All commands used the shared SSD cache and --locked --offline.

- `cargo build --examples`: pass.
- `cargo test --lib`: 9 passed.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps`: pass.
- A separate consumer compiled the packaged minimal example: pass (`macos-consumer.txt`).

The component sample was bundled locally only for CUA discovery. Native dark-mode
checks: text entry; Save profile three Enter presses yielded Saved 1 while showing
loading; Tab skipped Unavailable; explicit duplicate countdown start showed 3s and
Sent 1; panel focus ring and Enter yielded Details 1; SMS and Weekly both visibly
selected independently. AX exposes Display name on the input, separate named
spinner images, and a List containing the two tips. The diagnostic radio rows share
one parent to test ID isolation; they do not qualify grouped radio semantics.
VoiceOver remained off. CUA's reported focused element stayed at the window, so
focus evidence is the visible ring and observed activation, not that AX field.
The purple UTM overlay in the screenshots is host chrome, not sample content.

## Windows ARM64

Existing Windows 11 ARM VM, Rust 1.99.0, VS Build Tools 2022/MSVC14.44.
See `windows-environment.json`. Native gallery/component build, nine library tests,
strict rustdoc and strict clippy passed. The missing official Clippy component was
added to the existing Rust toolchain; the compiler was not upgraded.

Runtime: text entry, disabled Tab skip, busy-save duplicate suppression, full 3s
countdown and duplicate-start suppression, visible panel focus/activation, and
independent radio selection passed. Counts: `windows-states.stderr.txt`.
Navbar Right/Left wrap and Home/End stayed in the bar. A dark dialog cycled focus
within its buttons and dismissed/reopened through keyboard actions. Full retained
screenshots/logs live in the local Windows artifact directory.

A preview check found a further issue: Tab can move beyond the visible viewport
without scrolling the focused action into view. This receipt does not resolve it.

## Limits

No screen-reader acceptance, Windows x64, Linux requalification, mobile support,
IME/scaling matrix, signed distribution or publication is established here.
Form/dialog/preview semantics still need broader runtime and screen-reader checks.
Visual parity screenshots remain historical; no new complete capture sweep occurred.
