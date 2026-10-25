# Contributing to weGPUI

Thank you for helping. weGPUI ports WeUI 2.6.26 to native GPUI components, and the useful
contributions are the ones that bring it closer to upstream behaviour with evidence:
bug reports with screenshots, parity comparisons, accessibility findings, platform reports and
focused fixes.

[GitHub](https://github.com/codewhale-hq/weGPUI) is the home for issues and pull requests.
[CNB](https://cnb.cool/codewhale.net/weGPUI) is a read-only mirror. Participation is governed by
the [Code of Conduct](CODE_OF_CONDUCT.md). Security problems go through [SECURITY.md](SECURITY.md),
not public issues.

## Build and check

You need Rust 1.95 or newer (the measured minimum, `rust-version` in `Cargo.toml`) and the
platform libraries GPUI needs. On Debian/Ubuntu these are the same packages as `.cnb.yml`:
`libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libvulkan-dev libfontconfig-dev
libfreetype-dev libx11-xcb-dev libxcb1-dev libssl-dev pkg-config clang`.

```sh
cargo fetch --locked                      # once, to fill the dependency cache
cargo fmt --all -- --check
cargo build --all-targets --locked --offline
cargo test --locked --offline
cargo clippy --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline
cargo run --example minimal               # small consumer app
cargo run --example gallery               # every upstream example page
```

These are the source gates (fmt, build, tests, strict Clippy and documentation).
Respect `CARGO_TARGET_DIR` in any script you add.

Platform notes: both examples enable `gpui_platform`'s `wayland`/`x11` features on Linux.
Isolated X11 verification is recorded in [docs/PLATFORMS.md](docs/PLATFORMS.md); sampled actual
Wayland runtime is recorded in `docs/receipts/wayland-20261025/`, with full acceptance outstanding.
iOS has an unqualified backend/bundle scaffold; Android has no upstream
backend. Please report what you tried on Windows. Native Windows runtime results are
missing evidence the project needs.

## Project conventions

- **Exact GPUI pins.** `gpui-pre` and `gpui-pre-platform` are pinned with `=`. Changing them,
  enabling dependency features that add packages to `Cargo.lock`, or adding dependencies needs a
  maintainer decision first. Open an issue with the reason and the `Cargo.lock` delta.
- **Approved local compatibility copies.** The root manifest selects three exact locked
  packages from `vendor/`; see [vendor guidance](vendor/README.md) for approved deltas and
  provenance. Cargo patches are not inherited by downstream consumers: apply equivalent
  configuration at the application workspace root. Further dependency changes need approval.
- **Upstream is the reference.** Compare against the pinned package's CSS (`src/style/`), example
  HTML/JS (`src/example/`) and, for pickers, weui.js 1.2.17. Fetch them with
  `scripts/weui_upstream.py`. Do not copy unrelated upstream code.
- **Generated files.** `src/theme/palette.rs`, `src/assets.rs`, `docs/tokens.json` and the icon
  SVGs in `assets/` come from
  `python3 -I scripts/extract_weui.py "${CARGO_TARGET_DIR:-target}/weui-upstream/weui/package"`.
  `assets/loading.svg` is extracted from the pinned upstream SVG with animation handled by
  GPUI. Keep the separately authored CSS-ring mask `assets/loading-primary.svg`.
  Change the generator, not the output. Regenerating rewrites the eight `*-dark.png` files with
  identical pixels but different PNG bytes, depending on the Pillow version, so restore them with
  `git restore -- 'assets/*-dark.png'` only after verifying their pixels are unchanged and that
  those files contain no intended edits. Do not restore the whole assets directory.
- **Public API docs.** Every public item has rustdoc that states behaviour, defaults, units
  (lengths are logical pixels), ownership of state and callbacks, and accessibility behaviour.
  Module docs name the upstream source and known limitations.
- **Code comments.** Keep inline comments rare. Put rationale in the commit message or docs.
- **Tests.** Focused tests are approved. Cover a concrete regression or contract; keep real runs
  (screenshots, input, accessibility trees) as separate evidence. A broader test-suite design
  should still be discussed in an issue.
- **Accessibility is not optional.** Interactive components need a role, an accessible name,
  keyboard operation and visible focus. Overlays use `Presence`/`Modal`. Application roots should
  be wrapped in `accessibility::ModalScope`.
- **Honest claims.** "Matches" names the exercised behaviour. Untested is a valid result. Keep
  failed evidence instead of deleting it.

## Parity and behaviour reports

A parity report always includes:

1. **Route** (gallery page or upstream `#hash`) and **state** (`--state` / `scripts/parity` key).
2. **Theme**: light/dark, plus brand colour or care mode if used.
3. **Screenshot pair**: native and upstream at the same size, scale and scroll position.
4. **Platform**: OS and version, display scale, how the app was built (commit, profile), and
   upstream browser and version.
5. What you compared (visual, input sequence, animation timing, accessibility tree, screen reader
   or IME), the result, the relevant upstream CSS/JS, and what you did not check.

[docs/PARITY-TOOLING.md](docs/PARITY-TOOLING.md) explains how to produce pairs on macOS and Linux.
The issue templates ask for exactly these fields.

## Pull requests

- Keep each PR to one coherent change. Describe what you ran and the results, including
  failures, and name the platforms you could not test.
- Update `CHANGELOG.md` under *Unreleased*, and `docs/STATUS.md`/`docs/PARITY.md` when a claim
  changes.
- Contributions are accepted under the project licence, MIT OR Apache-2.0 (inbound = outbound).
  WeUI-derived assets stay under Tencent's MIT licence (`LICENSE-WEUI`, `NOTICE`).
- Maintainers may help rebase or finish a PR. Your authorship is preserved in the commit history.
