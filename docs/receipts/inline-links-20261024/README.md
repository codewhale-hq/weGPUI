# Inline-link bounds and activation — October 24, 2026

Source hashes are in corrected-source.json. The corrected Windows package is
`d36b7030399667cb1a9b3a93d19fec30f80cc30a7ca377e512bef8a76850282f`.

The first candidate exposed actual shaped bounds but failed a native macOS check:
clicking the short link then Enter activated the wrapped link because their bounding
rectangles overlap. The failed event log is retained as macos-runtime.stderr.txt.

The corrected implementation prevents rectangle-based mouse focus, focuses the
actual clicked text link, and routes accessibility Click actions directly to the
named link handler. Targets retain separate Tab stops and wrapped text highlights.

macOS26.1/Apple Silicon/Rust1.99: component sample builds. Native CUA sequence:
short-link click→guide1, Enter→guide2; Tab/Enter→preferences1; second wrapped-line
click→preferences2; ShiftTab/Space→guide3; named long-link activation→preferences3.
The corrected log and focused screenshots preserve these results. AX contains the
two named link nodes. No screen-reader acceptance is claimed.

Windows11 ARM/Rust1.99/MSVC14.44: corrected package build, strict all-target Clippy
and strict rustdoc passed. Native pointer/keyboard verification is still in progress
at this checkpoint. Full guest logs are retained under the local Windows artifact
folder; source remained unchanged during checks.

The owner granted a one-time exception for the small cached macOS build while
another chat held the shared build slot. The other build was not interrupted.
The exception covered this candidate and its runtime-discovered correction.
