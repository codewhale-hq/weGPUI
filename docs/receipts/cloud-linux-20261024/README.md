# Cloud Linux verification — October 24, 2026 (America/Los_Angeles)

Base fetched main: `08bf4b2c07eeda869ac3c48d065ba0c5a4b10b46`.
One checkout: `/workspace/weGPUI`, branch `main`. No separate handoff attachment was
visible; continuation uses the committed STATUS, PLATFORMS, CI-PROPOSAL and patch.

The approved Linux patch applies cleanly and adds exactly 34 packages, removing
none. See `lock-delta.json`. The results below supersede the initial setup checkpoint.

Environment setup: Debian 13 managed cloud environment, no personal Mac access.
Rust 1.99.1 installed locally. The initial `sudo -n apt-get update` failed because
sudo is absent (exit 127); direct `apt-get update` failed with filesystem permission
errors (exit 100). These attempts made no dependency installation. Logs retained.
The existing managed Docker daemon is healthy (28.4.0); verification runs in an
isolated `debian:trixie-slim` container mounting this same checkout. Image digest:
`sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad`.

Build, rendering, XTEST input, AT-SPI trees and screen-reader outcomes are
recorded separately. No Apple, Windows, Android or platform-support acceptance
follows from these Linux checks.

## Results for the windowing patch

- **Build:** `cargo build --all-targets --locked -j 4` passed on Rust 1.99.1.
  `cargo fmt --all -- --check` and strict rustdoc (`-D warnings -W missing-docs`,
  `--no-deps --locked --offline`) passed. `cargo clippy --all-targets --locked --offline -j 4 -- -D warnings` also passed.
  Logs accompany this receipt.
- **Launch/rendering:** both native examples launched under Xvfb `:99`, 1280×1024,
  scale 1, Mesa lavapipe (`lvp_icd.json`). Screenshots were visually inspected.
  These are native-only diagnostic images, not upstream parity comparisons.
- **XTEST input:** minimal: click Open, click OK => visible count changes 0→1;
  reopen and Escape => dialog closes. Gallery `form_select`: click selector,
  Down, Return => visible selection changes 微信号→QQ号; reopen and Escape =>
  QQ号 retained, green focus ring on trigger. See screenshots and `run-x11.sh`.
- **AT-SPI:** both sessions expose trees through distro `pyatspi`. Open dialogs
  replace background controls in the tree; closing restores them. Select option
  names are present. Tree `selected` state during keyboard navigation still
  describes the committed value, while the final combo-box name reflects QQ号.
- **Screen reader:** not run. These tree reads do not constitute Orca acceptance.
- **Other platforms:** no Wayland compositor, Apple host, Windows host or Android
  runtime was used. IME, resizing and packaging were not requalified here.

## Preserved unsuccessful attempts

`attempt1-minimal-*`: the positioning command omitted an explicit search pattern;
xdotool reported `Unknown command: 0`. Clicks missed the centered app. The script
returned 0 and a valid tree, but did **not** prove dialog interaction.

`attempt2-minimal-*`: corrected positioning opened the dialog. `Tab Return` closed
it with Confirmed still 0; this was cancellation, not a successful OK assertion.
The final probe uses the visibly identified OK button at `(267,396)`.

All sessions log a D-Bus fd-limit warning (`Operation not permitted`). The session
and accessibility bus nevertheless ran and the displayed trees were captured.
Neither the warning nor process exit alone is an acceptance verdict.

The reproducer expects this container's installed dependencies and running Xvfb.
Invoke from the checkout: `bash docs/receipts/cloud-linux-20261024/run-x11.sh minimal`
(or `select`). Run serially on this display. Existing output files should be moved
to a new attempt prefix before rerunning so failed evidence is not overwritten.
