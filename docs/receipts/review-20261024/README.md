# Review receipts

Baseline: `51be7e3873404941ea9e51f3a76383c1d2443d7d`, macOS arm64, Rust 1.99.0. Build, clippy with `-D warnings`, and fmt each returned zero on the frozen clean tree. Paths in build logs are redacted as `<checkout>`; commands used an external Cargo target directory.

`image-comparison.json` compares 56 fresh images to the committed baseline, not native to browser. The sample is listed in the [review](../../REVIEW-2026-10-24.md).

`ax-*.txt` walks the whole first window. `ime-*.txt` records a successful textarea Pinyin commit and an unsuccessful single-line attempt. VoiceOver was not run.

`fixes-build.txt` is the passing gallery build for the navbar/tabbar/action-sheet fixes plus an experimental SelectMenu change. The SelectMenu change was later reverted to the separately verified baseline because its focus checks were inconclusive. The other fixes were driven and visually inspected using that binary. `fix-input-capture.json` records those commands/exits, but an exit code alone is not an interaction verdict. Final fmt and `git diff --check` passed. No final all-target build/clippy rerun is claimed for this handoff.

Fresh action-sheet native screenshots replace the four affected archive images. Other fresh screenshots and the complete raw local artifact bundle are not in this repository; independent reviewers should rerun the recorded captures.
