# Desktop release gates doable on Linux — 2026-10-24

Host: Debian 13 cloud box, rustc/cargo 1.98.1, Xvfb :98, X11 only.

- `cargo package --list --locked --offline --allow-dirty` at `f5fe0b6`: 183 files
  (`package-list.txt`). Includes LICENSE-APACHE, LICENSE-MIT, LICENSE-WEUI, NOTICE, assets,
  the new `src/scrolling.rs` and `src/components/date_field.rs`; excludes `docs/receipts`.
- `cargo package --locked --offline --allow-dirty` verify build: exit 0 (`package-verify.txt`).
- Fresh consumer: `cargo new`, README "Using the library" snippet copied verbatim, README Git
  dependency plus exact GPUI pins with `x11`/`wayland` features as the README instructs.
  Resolved `wegpui` at `f5fe0b6` from GitHub, built in 50 s on a warm registry cache, opened
  a window under Xvfb and rendered the Account cell and Continue button
  (`linux-x11-fresh-consumer.png`). Not a fresh machine: the Cargo registry cache was warm
  and system libraries were already installed.

Not done here: macOS/Windows consumers, packaged gallery binaries, signing/notarization,
screen readers, Wayland, scaling.
