# Native interaction and reader followup — 2026-10-25

This follows GitHub main `49268de7bb78a69cb56b96f6cff02a2d9284e71d`. Five ordinary source
files change; no dependency source, version, feature, edge or Cargo file changes in this batch.
The two additional framework proposals remain unapplied pending owner approval.

## Implemented and verified

- [Real fcitx/Pinyin](../ime-x11-20261025/README.md) exposed selected-text loss when canceling
  inline composition. Empty preedit now restores the active original snapshot and selection
  direction silently. Real input and separate protocol checks pass 35 snapshots, including
  zero Changed on cancellation, one per commit/Undo/Redo, suffix/UTF-16 limits, both commit
  orders and state/limit changes. Default external preedit and configured inline preedit are
  distinguished; inline operation is not categorically unsupported.
- [Swiped Escape](../overlay-contracts-20261025/swiped-escape-followup/README.md) now bubbles
  from a collapsed row. Native Escape dismisses its Modal; an open row closes first, then the
  second Escape dismisses. Callbacks and focus restoration pass.
- [Responsive article images](../article-responsive-20261025/README.md) scale with their
  container and stop at the pinned asset's 690px intrinsic width. Four light widths and dark
  480px match the local exact-CSS reference's sizes and 24px following gap. Ten final captures
  were visually reviewed. The rejected 900px flow gap and 700px raster-rounding limit remain
  documented. This is a static packaged HTML/CSS comparison, not the blocked full SDK shell.
- [Agreement warning semantics](../orca-controls-20261025/README.md) adds
  `Semantics::invalid(bool)` and exposes `Agree::warn` as invalid. Actual Orca speech and
  native states show the cue appearing and clearing in six checked/unchecked states, with
  one checkbox callback and one independent terms callback. Pinned example code confirms
  this warning represents submission without consent.
- The same reader receipt records actual checkbox/Switch/radio, consent, general and
  cell-style button disabled/loading, and Vcode speech. Busy-tree observations are separate
  from spoken cues. Disabled IconButton flat-review availability remains unqualified because
  temporary X11 keyboard grabs disturb the backend's focus state. PCM was recorded; no human
  listening sign-off is claimed.

## Source gates and provenance

All five required gates pass on Rust/Cargo **1.98.1** and GPUI **0.3.9**, after the temporary
IME observer was archived and removed. [gates.log](gates.log) is the exact run:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo build --all-targets --locked --offline` | PASS |
| `cargo test --locked --offline` | PASS: 47 unit tests, one doctest ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | PASS |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline` | PASS |

`source-parent.txt`, `source.patch`, `source-sha256.json` and `binary-sha256.json` identify the
gated source and built examples. Each runtime receipt separately identifies its executable:
baseline `49268de` captures, intermediate fixed probes and the final integrated gallery must
not be described as one artifact. Earlier source checks and harness failures are retained.
[Website API verification](website-api/README.md) rebuilds from this strict rustdoc and checks
the new invalid-state method and exact current Input/Agree/Swiped/Semantics source pages;
historical native website images remain unchanged. No deployment or CI dispatch occurred.

## Confirmed remaining framework gaps

- [Saved modal AT actions](../overlay-contracts-20261025/custom-actions-followup/README.md):
  real retained Click and Focus requests at requested 0/1ms after hiding can invoke owner
  code or steal restored focus. Slower requests are rejected by the adapter. The prepared
  [scoped dispatch proposal](../../proposals/modal-a11y-action-guard/README.md) is not applied.
- [X11 grab focus](../orca-controls-20261025/x11-focus-modes-20261025/README.md): actual
  same-window Grab/Ungrab events correlate with reader activation/focus pulses and review
  resets; the backend handles these as ordinary focus changes. The exact
  [two-handler filter proposal](../../proposals/x11-grab-focus/README.md) is not applied.

These extend the scope of the earlier approved dependency patches and require additional
owner approval. No upstream issue or external post was made. No component row is fully
accepted: current-source other hosts, complete state/theme/reader coverage, audio listening,
hardware desktop, the full paired reference and distribution remain open. VoiceOver stays
off. UTC today and all publication/version/signing/website decisions remain unchanged.
