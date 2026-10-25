# Contributor CI proposal (draft, not enabled)

Status: **approved for final setup after implementation and local verification** (October 24).
No GitHub Actions workflow is committed or enabled. The YAML below remains a draft; reconcile
it with the eventual manifest and frozen candidate before enabling it. A configuration file is
not evidence that CI passes. The first hosted run will be the first receipt.

## Existing gate

`.cnb.yml` configures a CNB push-to-`main` pipeline in a `rust:1` container: Linux system packages,
`cargo fmt --all -- --check`, `cargo build --all-targets --locked` and
`cargo clippy --all-targets --locked -- -D warnings`. No hosted CNB result has been recorded in
this repository.

## Proposed GitHub workflow

The workflow would be stored at `.github/workflows/ci.yml` at that final step. It mirrors the CNB gates
on Linux and macOS, pins the measured MSRV as a separate Linux build, adds the documentation
check, and grants only read permissions.

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  CARGO_INCREMENTAL: 0

jobs:
  linux:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - name: System dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y --no-install-recommends libxkbcommon-dev \
            libxkbcommon-x11-dev libwayland-dev libvulkan-dev libfontconfig-dev \
            libfreetype-dev libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang
      - name: Toolchain
        run: rustup toolchain install stable --profile minimal -c clippy,rustfmt
      - run: cargo fetch --locked
      - run: cargo fmt --all -- --check
      - run: cargo build --all-targets --locked --offline
      - run: cargo clippy --all-targets --locked --offline -- -D warnings
      - name: Docs
        run: cargo doc --no-deps --locked --offline
        env:
          RUSTDOCFLAGS: -D warnings -W missing-docs

  linux-msrv:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - name: System dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y --no-install-recommends libxkbcommon-dev \
            libxkbcommon-x11-dev libwayland-dev libvulkan-dev libfontconfig-dev \
            libfreetype-dev libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang
      - run: rustup toolchain install 1.95.0 --profile minimal
      - run: cargo +1.95.0 build --all-targets --locked

  macos:
    runs-on: macos-14
    steps:
      - uses: actions/checkout@v4
      - name: Toolchain
        run: rustup toolchain install stable --profile minimal -c clippy
      - run: cargo fetch --locked
      - run: cargo build --all-targets --locked --offline
      - run: cargo clippy --all-targets --locked --offline -- -D warnings

  mobile-typecheck:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4
      - run: rustup toolchain install stable --profile minimal -t aarch64-apple-ios,aarch64-linux-android
      - run: cargo check --lib --locked --target aarch64-apple-ios
      - run: cargo check --lib --locked --target aarch64-linux-android
```

Before enabling, decide:

- whether to pin actions by commit SHA (recommended for a public repository) instead of `@v4`;
- whether the strict documentation job should block. The historical committed receipt uses
  `-W missing-docs`; the separate `-D warnings` result is commit-reported only;
- cache use (`Swatinem/rust-cache`). It adds a third-party action, so it is left out of the draft;
- whether macOS minutes are acceptable on every pull request, or only on `main`.

The `mobile-typecheck` job only proves that the library type-checks for mobile targets, and must
be labelled that way. It is not mobile support.

## Windows qualification plan

1. Add a `windows-2022` job running `cargo build --all-targets --locked` and clippy with the
   MSVC toolchain. That is a compile receipt only.
2. Run the gallery manually on a real Windows 10/11 machine. Record launch, rendering (two scale
   factors), keyboard/focus, IME (Microsoft Pinyin), resizing, and a Narrator or NVDA pass, using
   the platform report template.
3. Record those sampled areas separately. Windows is supported only after its build, launch,
   rendering, interaction, accessibility and packaging columns all have receipts.

## Mobile qualification plan

The iOS dependency path is approved but not implemented (`docs/PLATFORMS.md`). Once it is adopted, add a
macOS job that builds the iOS example for `aarch64-apple-ios-sim` and launches it in the
Simulator with `xcrun simctl`. An Android job needs an Android backend first, then an emulator
runner with KVM. Touch, software keyboard, rotation, lifecycle and VoiceOver/TalkBack results
stay manual receipts, labelled as simulator, emulator or device.
