# Vendored GPUI word breaking — 2026-10-25

The owner approved step 1 of [docs/proposals/gpui-text-i18n](../../proposals/gpui-text-i18n/README.md)
in chat on 2026-10-25: extend `LineWrapper::is_word_char` in `vendor/gpui_pre` for Arabic, Hebrew,
Devanagari and Greek, with no new dependency and **Hangul left as is**; add unit tests; re-run the
gates; re-capture the Languages page. This receipt records that. GitHub `main` is canonical; no CNB
writes, issues, comments, tags, releases or deployments were made.

## Source

| | |
| --- | --- |
| Base | `main` at `1b35ca9`, working tree clean before the change |
| Changed file | `vendor/gpui_pre/src/text_system/line_wrapper.rs`, +4 lines of code, +110 lines of tests |
| Locked archive | `gpui-pre-0.3.9.crate` `34f61c1c…f1c0`; the vendored copy of this file was byte-identical to the archive's (`58feea87…72ed`) before the change |
| Patched file | `a670eafc…c87d` (full hashes in [checksums.txt](../../proposals/gpui-text-i18n/checksums.txt)) |
| Delta | [word-break.patch](../../proposals/gpui-text-i18n/word-break.patch) |
| Other source | unchanged: tree hash of `src examples scripts Cargo.toml Cargo.lock` is still `d8ffe13e04669848` ([gates/tree-hash.txt](gates/tree-hash.txt)); `Cargo.lock` and every package version are untouched |
| Host | macOS 26.1 arm64, Rust and Cargo 1.99.0, `--locked --offline` |

`is_word_char` is called by `wrap_line`, `truncate_line` and `LineLayout::compute_wrap_boundaries`,
so ellipsis truncation of these scripts now also prefers word boundaries.

## Tests

The vendored `gpui-pre` test target cannot compile from the packaged crate: it lacks the
upstream test fonts and dev-dependencies (the earlier attempt stopped with 21 errors, linked from
[STATUS](../../STATUS.md)). So the seven affected tests were run by
[verify-word-break.py](../../proposals/gpui-text-i18n/verify-word-break.py), which compiles the
**unmodified non-test source** of `line_wrapper.rs` against a stand-in for the GPUI items it uses
(a `TextSystem` giving every character a 9.6 px advance) and runs the tests **extracted verbatim**
from the file.

| Run | Result |
| --- | --- |
| Patched code, patched tests ([log](verification/harness-patched.txt)) | 7 pass: the existing `test_is_word_char`; Arabic, Hebrew, Devanagari and Greek character tests; neighbouring scripts (Armenian, Syriac, Thai, Hangul, Han) unchanged; `wrap_line` boundaries fall only after a space for an Arabic, a Hebrew, a Devanagari and a Greek sentence |
| Locked registry code (unpatched), patched tests ([log](verification/harness-unpatched-code.txt)) | 5 of 7 fail, as they must: the four script tests, and the wrapping test with `wrapped inside a word: "نص حكيم له سر قاطع وذ"`, the same split seen in the capture; the existing test and the unchanged-scripts test pass |

Limits: the stand-in measures every character as 9.6 px, so the wrapping test checks the logic,
not font metrics; it is not upstream's own test run; Greek has unit-test coverage only because the
gallery has no Greek text.

## Gates

After the change `gpui-pre` and its dependents recompiled (`Compiling gpui-pre v0.3.9
(…/vendor/gpui_pre)` in [gates/build.log](gates/build.log)), and the five gates pass on that tree
([summary](gates/summary.txt)):

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo build --all-targets --locked --offline` | pass |
| `cargo test --locked --offline` | 59 unit tests pass; 3 documentation examples ignored. These are weGPUI's own tests, unchanged in number; the seven vendored tests above are not among them |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | pass (the vendored crate is a path dependency, so its code is not linted) |
| `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps` | pass |

## Captures

Before and after use the same gallery source; "before" is a copy of the binary built at `1b35ca9`,
"after" the rebuilt one. Pixel comparisons of the route sweeps skip the top 60 device pixels of each window capture (the
title bar), because its traffic-light buttons change with window activation between runs; the
scrolled Languages captures come from `scripts/capture.py`, which already crops the title bar.

| Comparison | Result |
| --- | --- |
| **Languages page, English locale, scrolled to the Arabic, Hebrew, Hindi and Thai rows** ([image](images/languages-before-after.png)) | Arabic: the word `وذو` is no longer split across lines. Hindi: `सर्वनाश` stays whole on the second line. Hebrew: unchanged (one line). Thai: unchanged, `สัตว์` is still split |
| Same page, **Arabic locale** (right-to-left layout) ([image](images/languages-ar-before-after.png)) | the same two improvements, mirrored |
| Same page one screen up (Chinese, Japanese, Korean, Spanish, French, German, Portuguese, Russian) | pixel-identical before and after |
| All 61 routes, English ([table](captures/sweep-en-before-after.tsv)) | 58 pixel-identical. Three differ, only in loading-spinner regions: `button_default` 0.14 %, `loading` 0.11 %, `loadmore` 0.03 %. Two captures of the *unchanged* binary differ in the same regions by 0.20 %, 0.09 % and 0.03 % ([table](captures/sweep-noise-same-binary.tsv)), so this is spinner animation, not the change |
| 16 routes, Arabic locale ([table](captures/sweep-ar-before-after.tsv)): languages, list, dialog, picker, form_vcode, msg, article, panel, preview, steps_vertical, information-bar, half-screen-dialog, uploader, form, navbar, tabbar | all 16 pixel-identical |

So Latin, Cyrillic, Chinese, Japanese and Korean wrapping is unchanged on every captured page,
and the only visible change is the two rows above.

## What this does not establish

- **Order.** A wrapped right-to-left paragraph still lays out in visual line order: the Arabic
  description's first line is still the end of its sentence. That is step 2 of the proposal and is
  not approved or applied.
- **Thai and Hangul** still break between any two characters, by design of this slice.
- Linux, Windows and the other platforms' shapers were not exercised. The change is in shared,
  platform-independent code, but every capture here is macOS.
- Real keyboard input and IME in a wrapped field, and screen-reader output, were not tested.
- Arabic Extended-B (`0870–089F`) and scripts that were never on the list (Armenian, Syriac,
  Georgian, Ethiopic, Khmer, ...) are untouched.
- Hosted CI was not run for this change.
