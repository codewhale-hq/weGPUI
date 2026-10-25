# Approved guard application status

The owner approved the exact modal guard proposal and mandatory patched-GPUI build requirement
on 2026-10-25. The two recorded patch SHA-256 values matched before applying on baseline
`3840b47`:

- `compat.patch`: `b6c0e38be71096e125aac1a0b5aac260707c9cdbb04dd0412759908f969a1aaf`
- `library-integration.patch`: `85c40d4d5b575862b49643d12f125869cec7d0449b9e8fcb045e19b501c22463`

Only the five approved GPUI/first-party files were changed by this guard subtask. Combined
root gates pass on Rust 1.98.1: fmt/all-targets/47 unit tests/one ignored doctest/strict
Clippy/strict docs. The old-source failure receipts remain unchanged.

[Applied-source runtime evidence](../../receipts/overlay-contracts-20261025/action-guard-fixed-20261025/README.md)
records seven saved Click/Focus timings, six caller contracts and the fixed cached/live
controls. The full upstream vendor Cargo test build is blocked by missing fixtures/optional
test-support dependencies; two exact metadata regressions passed a separately documented
source-extraction harness using already-built dependencies. No new packages or lock edits
were introduced to bypass that test-infrastructure failure.

This is bounded Linux qualification, not full component/platform/reader acceptance. The
mandatory patched-GPUI consumer build requirement remains in README/vendor guidance.
