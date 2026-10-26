# Vendored GPUI logical-order wrapping — 2026-10-25

**Not landed.** The owner approved the step 2 prototype of
[gpui-text-i18n](../../proposals/gpui-text-i18n/README.md), with no new dependency and landing
conditional on the Languages page, unit tests and five gates. The supplied macOS captures
and the Linux source gates pass. The extra Linux visual check still shows clipped/misaligned
Arabic and Hebrew paragraphs. The handoff explicitly requires an owner decision before landing
when Linux mis-orders or mis-aligns lines. **The owner chose “Hold landing” on 2026-10-25.**
No commit or push was made for this candidate.

## Source and scope

| Item | Evidence |
| --- | --- |
| Base | GitHub `origin/main` and clean checkout both at `b1ddcae2b37b1901c17c78c6f06d9f4224ce03c2` after fetch; the base had not moved |
| Supplied bundle | `wegpui-step2-prototype-20261025.tar.gz`, SHA-256 `8aaec7b94c13310944224d9edbb8d2c1a54826f55ebce450576a76239331ffa6`; every entry in its `SHA256SUMS.txt` verified |
| Supplied patch | `step2-prototype.patch`, SHA-256 `888c94d3ce634d79eaaa3db8e9d3e8b5aecd6f47bbea71b8a5b19c15b616e486`; `git apply --check` passed and applying produced exactly the expected ten paths |
| Vendor delta | [logical-wrap.patch](../../proposals/gpui-text-i18n/logical-wrap.patch); five files, including new `logical_wrap.rs` |
| Locked archive | `gpui-pre-0.3.9.crate`, SHA-256 `34f61c1cfd36fc9c436af09308c963822ae27a45a84feb21a77ba5c9e051f1c0`; the four pre-existing files were hashed directly from this archive |
| Source hashes | [source-checksums.txt](source-checksums.txt), [proposal checksums](../../proposals/gpui-text-i18n/checksums.txt), [bundle hashes](bundle-checksums.txt) |
| macOS host | Supplied evidence: macOS 26.1 arm64, Rust/Cargo 1.99.0; [context](macos/gates4/context.txt) |
| Linux host | Debian 13 container, x86_64, Rust/Cargo 1.95.0, Xvfb and Mesa software Vulkan; [context](linux/captures/context.txt) |

All five changed vendor files, including the complete `layout_logical_lines` and
`wrapped_line_start_index` implementation, are byte-identical to the supplied prototype.
The input and message-link row consumers and extracted-test harness also match. The only
differences from the supplied ten-file source are removal of the optional TextArea experiment
and the existing Switch test assertion adjustment described below. `Cargo.toml`, `Cargo.lock`,
package versions and dependency requirements are unchanged. No other vendor code was edited.

The implementation measures glyph advances in logical byte order, chooses word-start breaks,
shapes each resulting line with an LRM/RLM preserving paragraph direction, strips that mark,
and keeps explicit logical row starts in the synthetic layout. Unsupported cases and unaffected
left-to-right text take the existing wrapper. Row-start consumers use the new API for byte
ranges, without claiming to repair caret x or partial selections.

## Tests and gates

The packaged GPUI test target lacks upstream test fonts and dev-dependencies. These extracted
tests are supplemental tests of the real source, with stand-ins for GPUI types and metrics;
they are not the upstream test suite or platform-shaper acceptance.

| Check | Linux result |
| --- | --- |
| `python3 -I docs/proposals/gpui-text-i18n/verify-logical-wrap.py` | 16 standalone-module tests and 4 tests extracted from the real layout types pass; [log](linux/verification/logical-wrap.txt) |
| Same command with `--glyph-derived-rows` | Exit 101, expected: three logical-row tests fail and the unchanged-row fallback test passes; [log](linux/verification/glyph-derived-rows.txt) |
| `python3 -I docs/proposals/gpui-text-i18n/verify-word-break.py` | 7 pass; [log](linux/verification/word-break.txt) |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo build --all-targets --locked --offline` | Exit 0; GPUI and its dependents recompiled |
| `cargo test --locked --offline` | Exit 0; 59 unit tests pass, 3 documentation examples ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | Exit 0; the vendor path dependency is not itself Clippy-linted |
| `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline` | Exit 0 |

Final Linux gate logs and summary are in [linux/verification/final](linux/verification/final/summary.txt).
The supplied macOS five-gate logs are in [macos/gates4](macos/gates4/summary.txt); the supplied
harness run is [macos/harness-run.txt](macos/harness-run.txt).

The first Linux test run failed an unchanged Switch test at `src/components/switch.rs:330`:

```text
assertion `left == right` failed
  left: 100.000001ms
 right: 100ms
test result: FAILED. 58 passed; 1 failed
```

[The full failure](linux/verification/initial/test.log) is retained. Rust 1.95's
`Duration::mul_f32(1.)` rounds 100 ms upward by one nanosecond. The existing assertion now
allows an absolute difference of one nanosecond. No new Switch test or production animation
change was added; the wrapping logic is unchanged. All five gates then passed, and were run
again after the diagnostic baseline build to restore and verify the candidate executable.
That first restoration attempt used a stale GPUI artifact from the shared Cargo target and
failed to find `wrapped_line_start_index` ([log](linux/verification/after-baseline-cache-failure/build.log)).
Cleaning only the generated `gpui-pre` artifacts forced the correct source to rebuild; the
final logs record the subsequent complete gate run. No source fix was needed for this cache
collision.

## Supplied macOS captures

These images were supplied, not recaptured in the cloud. Their meaning is retained because
the vendor wrapping implementation remains byte-identical to the captured prototype.

| Comparison | Observation |
| --- | --- |
| Arabic description, English locale | First line now holds the sentence's beginning: [step 1 vs step 2](macos/images/en-step1-vs-step2.png) |
| Arabic description, Arabic locale | Same logical-order improvement with mirrored layout: [comparison](macos/images/ar-step1-vs-step2.png) |
| Arabic, Hebrew and English mixed-direction paragraphs | Reading-order rows, left-aligned in English and right-aligned in Arabic: [comparison](macos/images/wrapped-paragraphs-en-vs-ar.png) |
| 61 English routes | Supplied table: 58 identical below title-bar chrome; the other three are spinner regions in `button_default`, `loading` and `loadmore`: [table](macos/tables/sweep-en-step1-step2.tsv) |
| 16 Arabic-locale routes | Supplied table: all 16 identical: [table](macos/tables/sweep-ar-step1-step2.tsv) |

The previous session attributed the three noisy routes to spinner animation using unchanged
binary controls; those controls are not included in this bundle and were not repeated here.
[The TextArea capture](macos/images/textarea-arabic-plain-and-selected.png) is retained as
historical experiment evidence only. Its gallery field was removed, and it is not acceptance
evidence for the candidate. The source had no TextArea experiment during the supplied gates.

## Linux visual check and A/B control

Built with the repository's existing X11/Wayland example features. Captures use the same
font set, Xvfb screen `1600x3600x24`, `GPUI_X11_SCALE_FACTOR=2`, a D-Bus session, Mesa lavapipe,
and `scripts/capture.py languages --only gpui --lang en|ar --scroll 1050|2150`. Each file is
750×1624 pixels. The capture session has no window manager; caret/focus is not evaluated.
There is no upstream browser capture in this native-only run.

The control was built from a clean archive of `b1ddcae` plus only the identical Languages page
paragraph samples and translation-table entry ([gallery delta](linux/baseline-gallery.patch)).
It contains no step 2 vendor or consumer changes. [Build log](linux/captures/baseline-build.log)
and executable hashes in the context distinguish it from the candidate. Candidate captures
were repeated after the final gates, with the executable hash checked before and after capture.

| Locale and scroll | Before step 2 | Candidate |
| --- | --- | --- |
| English, 1050 | [baseline](linux/captures/baseline/languages-en-scroll1050-light-gpui.png) | [candidate](linux/captures/final/languages-en-scroll1050-light-gpui.png) |
| Arabic, 1050 | [baseline](linux/captures/baseline/languages-ar-scroll1050-light-gpui.png) | [candidate](linux/captures/final/languages-ar-scroll1050-light-gpui.png) |
| English, 2150 | [baseline](linux/captures/baseline/languages-en-scroll2150-light-gpui.png) | [candidate](linux/captures/final/languages-en-scroll2150-light-gpui.png) |
| Arabic, 2150 | [baseline](linux/captures/baseline/languages-ar-scroll2150-light-gpui.png) | [candidate](linux/captures/final/languages-ar-scroll2150-light-gpui.png) |

**Failure:** Arabic and Hebrew paragraphs clip horizontally and show only fragments across too
few rows. In English, the candidate's first visible Arabic fragment contains later text such as
`weui.io`, rather than the complete beginning shown by macOS. In Arabic, the paragraph begins
on the visible row but extends beyond the left edge. The English paragraph containing embedded
Arabic and Hebrew wraps visibly into four rows in both builds. The short Arabic description
also clips.

The A/B control establishes that clipping predates step 2. It does not establish that the
candidate is a complete fix or an acceptable Linux result. This is a compatibility failure
of the candidate's intended behaviour, not evidence that step 2 introduced all the clipping.
No platform guard or wrapping-logic adjustment has been made. The owner chose to hold.
Resuming landing requires a new decision to accept the unchanged prototype with Linux flagged,
or authorization for a revised implementation and fresh macOS capture.

## Remaining limits and disposition

- Step 3 remains unapproved: caret x, hit-test precision and partial-selection geometry inside
  wrapped right-to-left rows are not repaired by these logical row-start changes.
- A TextArea does not right-align in right-to-left locales. The optional gallery experiment
  was dropped before the Linux gates and captures.
- Reshaped line width can exceed its requested width slightly because break metrics come from
  whole-paragraph advances. Multi-style right-to-left runs, underline and strikethrough remain
  untested. Thai/Hangul breaking and the step 1 script ranges are unchanged.
- Windows, screen-reader output, real RTL keyboard/IME input and native-speaker review remain
  unverified. Reading captures is not native-speaker acceptance.
- No hosted CI or CodeQL result exists for this uncommitted candidate. No deployment, release,
  tag, branch creation or CNB write occurred. The requested two commits, fast-forward push,
  post-landing Linear update and pushed-head CodeQL check are deferred under the owner hold.

Logs preserve failures and use placeholders for absolute host, build and temporary paths.
Image bytes and the supplied comparison tables are unchanged. [evidence-checksums.txt](evidence-checksums.txt)
hashes the retained evidence; source and original bundle hashes are recorded separately.
