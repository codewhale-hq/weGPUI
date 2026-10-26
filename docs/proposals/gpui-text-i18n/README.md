# Proposal: script-aware line breaking in the vendored GPUI

**Status.** Step 1 was approved by the owner on 2026-10-25 (in chat: no new dependency, Hangul
left as is) and is applied to `vendor/gpui_pre`; the delta is [word-break.patch](word-break.patch)
and the evidence is the [receipt](../../receipts/gpui-word-break-20261025/README.md). The owner
also approved the step 2 prototype, with landing conditional on the Languages page, unit tests
and five gates. The owner initially held landing after reviewing
the Linux failure ([receipt](../../receipts/gpui-logical-wrap-20261025/README.md)). On October 26
the owner requested complete review and publication of all current work to GitHub; the
[publication review](../../REVIEW-2026-10-26.md) records that later authorization. At that publication source the Linux
rendering failure remained open. Step 3 was subsequently approved; see the current continuation below. Vendor guidance: [vendor/README](../../../vendor/README.md).
The localisation work ([I18N](../../I18N.md)) worked around everything that could be worked
around in weGPUI itself; this is what remained.

## Current continuation — 26 October 2026

Step 3 and the two direct locked crate references are owner-approved. The
[code/parity continuation](../../receipts/code-parity-20261026/README.md) replaces the
input-only shim with GPUI shared row geometry using `unicode-bidi 0.3.18`. Text-owning shaped,
wrapped and element layouts now share caret/hit/selection geometry, including alignment and
inline-link bounds. Styled RTL lookup/decoration painting and shaped ellipsis clamping are
repaired. Opening punctuation and oversized RTL words are handled in the shared break path.
Package versions stay unchanged; `chrono 0.4.45` supplies host-local today for weGPUI dates.

The historical prototype and held failures below retain their original source identities.
Full IME/readers, Windows visible UI, Linux hardware and dictionary breaking remain open.

## What was wrong

Both problems showed on the *Languages* page, in English, with no right-to-left layout involved
([capture before step 1](../../receipts/i18n-20261025/images/languages-page-en-scrolled.png)):

1. **Words were split in scripts GPUI did not know as word characters.** The Hindi description
   broke inside `सर्वनाश` (`…सर्वना` / `श करने…`), the Arabic one inside `وذو`, the Thai one in the
   middle of `สัตว์`. English, Latin-script, Cyrillic and Bengali text broke only at spaces.
2. **A wrapped right-to-left paragraph breaks in visual order.** The two-line Arabic description
   starts with the end of its sentence (`…مكتوب على ثوب أخضر ومغلف بجلد أزرق.`) and carries the
   beginning (`نص حكيم له سر قاطع وذ`) on the second line, so it reads backwards.

A third limit is already handled in weGPUI: `LineLayout::x_for_index`/`index_for_x` assume glyph
indices rise with x, so a caret in Arabic or Hebrew lands on the wrong edge. `src/bidi.rs` maps
caret, hit testing and selection through the glyph runs instead, for lines that are not wrapped.

## Where it comes from

- `vendor/gpui_pre/src/text_system/line_wrapper.rs`, `LineWrapper::is_word_char`: an allow-list
  of ASCII, Latin-1, Latin Extended A/B, Cyrillic, Vietnamese, combining marks and Bengali. Every
  other character counts as a break opportunity, as CJK does. It is used by `wrap_line`,
  `truncate_line` and `LineLayout::compute_wrap_boundaries`.
- `vendor/gpui_pre/src/text_system/line_layout.rs`, `LineLayout::compute_wrap_boundaries`
  (line 193): walks `runs[..].glyphs[..]` in the order the shaper returns them. For a
  right-to-left run that is visual order, so "the previous character", the candidate break and
  the width check all run on the reversed text, and `WrapBoundary { run_ix, glyph_ix }` selects
  the leftmost glyphs for the first line.
- `LineLayout::x_for_index` (line 108) returns the first glyph whose `index >= index`, which
  assumes left-to-right.

## Step 1 — applied

`is_word_char` now also accepts Arabic (`0600–06FF`, `0750–077F`, `08A0–08FF`, `FB50–FDFF`,
`FE70–FEFF`), Hebrew (`0590–05FF`), Devanagari (`0900–097F`) and Greek (`0370–03FF`,
`1F00–1FFF`). Four `matches!` clauses, no dependency, no change to any other function. Wrapping
and ellipsis truncation of those scripts therefore break only at spaces (and before the
characters that were already break opportunities), as they do for Latin text.

Left alone on purpose: **Hangul** (the owner's decision), **Thai** (no spaces between words;
correct breaking needs a word dictionary, which would be a dependency), Arabic Extended-B
(`0870–089F`, not in the approved list) and the other scripts that were never on the list.

Seven unit tests cover it ([word-break.patch](word-break.patch)): the existing
`test_is_word_char`, one test per script for the new ranges, one that the neighbouring scripts
(Armenian, Syriac, Thai, Hangul, Han) did not change, and a `wrap_line` test asserting that every
boundary of an Arabic, Hebrew, Devanagari and Greek sentence falls just after a space. The
vendored test target cannot compile from the packaged crate, so
[verify-word-break.py](verify-word-break.py) compiles the unmodified non-test source of
`line_wrapper.rs` against a small stand-in for GPUI's `Pixels`, `FontId`, `SharedString`,
`TextRun` and `TextSystem` (every character 9.6 px wide) and runs the tests extracted verbatim
from the file. Against the unpatched registry source, five of the seven fail, including the
wrapping test, with the mid-word split seen in the capture (`نص حكيم له سر قاطع وذ`); against the
patched source all seven pass ([logs](../../receipts/gpui-word-break-20261025/verification/)).

## Step 2 — approved prototype; Linux rendering remains limited

[logical-wrap.patch](logical-wrap.patch) contains the five-file vendor delta. Reordered,
over-width paragraphs containing right-to-left characters are measured in logical byte order
using the whole paragraph's glyph advances. Breaks use the existing word-candidate rules;
an unsupported break falls back to the old wrapper. Each line is shaped separately with an
LRM/RLM preserving the paragraph's base direction; the mark is stripped and non-final trailing
spaces are not shaped. The resulting synthetic layout keeps explicit logical row starts.

`WrappedLineLayout::wrapped_line_start_index` supplies those starts to text extraction,
position/index mapping, weGPUI input rows and message-link bounds. The Languages page adds
Arabic, Hebrew and English paragraphs with mixed-direction spans. The optional TextArea
experiment was dropped. No dependencies, versions or lockfile changed.

[verify-logical-wrap.py](verify-logical-wrap.py) runs 16 tests from the standalone module and
four tests extracted verbatim from the real layout types. Its `--glyph-derived-rows` negative
control fails three of the four layout tests. The seven step 1 tests also pass. These harnesses
do not exercise a platform shaper or establish correct caret x/partial-selection geometry.

All five Linux gates pass with 59 unit tests and three ignored doc examples. A pre-existing
Switch assertion needed a one-nanosecond tolerance for Rust 1.95's `Duration::mul_f32` rounding;
production animation and wrapping code were unchanged. Every patched vendor file was
byte-identical to the supplied macOS prototype at that receipt, preserving the scope of its macOS captures.

**Linux visual failure:** under Xvfb, the Arabic and Hebrew paragraphs remain clipped and
misaligned in both locales. An A/B build of the pre-step-2 source with the same gallery samples
shows clipping already existed; the prototype changes the visible fragments without producing
the complete, readable macOS layout. The receipt keeps both sets of captures. The handoff
requires an owner decision before landing when Linux mis-orders or mis-aligns lines; the owner
chose to hold on 2026-10-25, before the later repository publication request recorded above. No
platform guard or wrapping-logic fix has been made. Any logic change needs new macOS captures.

A later, separately approved [trailing-space trial](../gpui-trailing-space-wrap/README.md)
adds one guard to the monotonic shaped-row wrapper in `line_layout.rs`. It does not change
`logical_wrap.rs`, `layout_logical_lines` or row-start/caret/selection code. The original patch
and checksums here remain historical artifacts; the later trial's whole-file hash and fresh
Linux evidence are separate. That trial approval did not itself release the RTL landing hold or approve
step 3. The later publication request is recorded above; the supplied macOS captures do not
qualify the later guard.

## Step 3 — shared implementation complete; qualification remains scoped

The shared implementation and its current host evidence are in the continuation above.
Text-free raw `LineLayout` methods retain their legacy contract; text-aware consumers use
`ShapedLine`, `WrappedLineLayout` and element `TextLayout`. Exact font-provided ligature caret
positions, dictionary segmentation, real RTL IME and full host/reader matrices remain open.

## Outside this continuation

A new shaper, ICU, hyphenation, vertical text, alternate calendars and platform-specific
backend changes beyond the shared GPUI text layer.
