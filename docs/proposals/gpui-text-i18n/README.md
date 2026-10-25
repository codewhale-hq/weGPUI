# Proposal: script-aware line breaking in the vendored GPUI

**Status.** Step 1 was approved by the owner on 2026-10-25 (in chat: no new dependency, Hangul
left as is) and is applied to `vendor/gpui_pre`; the delta is [word-break.patch](word-break.patch)
and the evidence is the [receipt](../../receipts/gpui-word-break-20261025/README.md). Steps 2
and 3 are still proposals and need approval. Vendor guidance: [vendor/README](../../../vendor/README.md).
The localisation work ([I18N](../../I18N.md)) worked around everything that could be worked
around in weGPUI itself; this is what remained.

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

## Steps 2 and 3 — still proposals

2. **Wrap in logical order.** Compute candidates and boundaries from the text in byte order,
   measuring with the glyph advances, then resolve each resulting line's visual order on its own.
   Touch points: `layout_wrapped_line`, `WrappedLineLayout`, and the consumers of
   `WrapBoundary` (`position_for_index`, `index_for_position`, `closest_index_for_x`, and
   `WrappedLine::paint`). I have not prototyped this one; it needs the per-platform shapers'
   run output reviewed, and it is the larger and riskier half. It is what makes the first line of
   a wrapped Arabic or Hebrew paragraph hold the start of the sentence.
3. **Make `x_for_index`/`index_for_x` bidi-aware**, so `src/bidi.rs` can be deleted. It would
   also extend caret and selection to wrapped lines, which `TextArea` cannot do today.

## Not proposed

New dependencies, a new text shaper, ICU, hyphenation, vertical text, or changes to the Linux
and Windows vendor copies beyond the shared `text_system` code.
