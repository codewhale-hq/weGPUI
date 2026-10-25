# Approved iOS dependency path — Linux verification

Base: `29e1502d5b822e7426975323cb5e54d565d0704d`. Same checkout and isolated Debian
13/Xvfb container as the [windowing slice](../cloud-linux-20261024/README.md).
`source.json` identifies the exact candidate source. The GPUI pins move from
0.3.7 to 0.3.9; `lock-delta.json` records 24 added and 77 removed name/version
pairs (including replacements). No other direct dependency was added.

Both examples use the shared iOS platform helper. `gpui_platform` is excluded on
iOS because version 0.3.9 lacks that dispatch; target-specific `gpui_ios` supplies
it. The bundle layout follows the published upstream 0.3.9 example. No public
library API changed.

## Evidence categories

- **Linux build and static checks:** all-target build, seven library tests,
  all-target Clippy `-D warnings`, strict rustdoc passed on
  Rust 1.99.1 (`linux-checks.txt`). The initial formatting check failed on gallery chain
  wrapping (`fmt.txt`, exact raw output in `fmt-original.txt.gz`); rustfmt corrected
  it and the final check passed (`fmt-after.txt`).
- **Rendering and real input:** both rebuilt examples rendered in isolated Xvfb
  using lavapipe at scale 1. Minimal click OK visibly increments Confirmed to 1;
  reopen/Escape dismisses. Select click/Down/Return changes 微信号 to QQ号;
  reopen/Escape retains QQ号 and the trigger focus ring. Native screenshots were
  inspected; there is no new upstream visual-parity verdict.
- **Accessibility trees:** AT-SPI dumps show named modal controls, isolate the
  background while open, and restore the page on close. These are not Orca tests.
- **Screen readers:** none run. No Linux or mobile screen-reader acceptance.
- **Bundle-script contracts:** five Python tests pass using explicitly fake
  Apple tools. They exercise both examples, paths containing spaces, retained
  compiler failures, ambiguous artifact rejection, and host/device-target guards.
  The real Linux invocation exits 1 before attempting an Apple build
  (`linux-bundle-guard.txt`). Bash syntax validation passed.
- **Apple application build/runtime:** not attempted; no authorized Apple host
  is attached. Linux does not provide the Metal/Xcode build tooling. The scaffold
  does not demonstrate touch, keyboard, IME, insets, rotation, lifecycle or VoiceOver.
- **Windows/Android application runtime:** not attempted. Android still has no
  upstream platform backend. No platform support claim follows from this slice.

Library-only mobile target checks and the Rust 1.95 check are recorded separately
in the corresponding logs; their final outcomes are appended below.

Final local results: `cargo +1.95.0 build --all-targets --locked --offline -j 4`
and all seven library tests pass (`linux-msrv.txt`). Library-only checks pass for
`aarch64-apple-ios`, `aarch64-apple-ios-sim` and `aarch64-linux-android` with the
locked candidate. These commands type-check weGPUI/core dependencies, not the
iOS backend example or an Android application. No mobile executable was linked.

Runtime and initial build checks identify `source-before-formatting.json`; only
rustfmt wrapping in the example changed afterward. `source.json` records the
final formatted candidate, checked again by the final build/Clippy commands.

## Later hosted check

The subsequent [CI run](../ci-20261024/README.md) at `05cae89` passed macOS native
build/tests/Clippy, but the iOS example failed in `gpui-pre-ios 0.3.9` on six missing
core APIs. This supersedes the earlier “not attempted” Apple-build checkpoint.
No Apple application runtime acceptance was attempted or established.
