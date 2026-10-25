# Proposal: script-aware line breaking in the vendored GPUI

**Status: proposal. Nothing here has been applied.** The change is to `vendor/gpui_pre`, which
the vendor guidance reserves for owner approval ([vendor/README](../../../vendor/README.md)).
The localisation work ([I18N](../../I18N.md)) worked around everything that could be worked
around in weGPUI itself; this is what remains.

## What is wrong

Both problems show on the *Languages* page, in English, with no right-to-left layout involved
([capture](../../receipts/i18n-20261025/images/languages-page-en-scrolled.png)):

1. **Words are split in scripts GPUI does not know as word characters.** The Hindi description
   breaks inside `सर्वनाश` (`…सर्वना` / `श करने…`), the Arabic one inside `وذو`, the Thai one in the
   middle of `สัตว์`. English, Latin-script, Cyrillic and Bengali text breaks only at spaces.
2. **A wrapped right-to-left paragraph breaks in visual order.** The two-line Arabic description
   starts with the end of its sentence (`…مكتوب على ثوب أخضر ومغلف بجلد أزرق.`) and carries the
   beginning (`نص حكيم له سر قاطع وذ`) on the second line, so it reads backwards.

A third limit is already handled in weGPUI: `LineLayout::x_for_index`/`index_for_x` assume glyph
indices rise with x, so a caret in Arabic or Hebrew lands on the wrong edge. `src/bidi.rs` maps
caret, hit testing and selection through the glyph runs instead, for lines that are not wrapped.

## Where it comes from

- `vendor/gpui_pre/src/text_system/line_wrapper.rs`, `LineWrapper::is_word_char` (line 501):
  an allow-list of ASCII, Latin-1, Latin Extended A/B, Cyrillic, Vietnamese, combining marks and
  Bengali. Every other character counts as a break opportunity, as CJK does. Arabic, Hebrew,
  Devanagari, Thai, Hangul and Greek are not on it.
- `vendor/gpui_pre/src/text_system/line_layout.rs`, `LineLayout::compute_wrap_boundaries`
  (line 193): walks `runs[..].glyphs[..]` in the order the shaper returns them. For a
  right-to-left run that is visual order, so "the previous character", the candidate break and
  the width check all run on the reversed text, and `WrapBoundary { run_ix, glyph_ix }` selects
  the leftmost glyphs for the first line.
- `LineLayout::x_for_index` (line 108) returns the first glyph whose `index >= index`, which
  assumes left-to-right.

## Proposed change

1. **Extend `is_word_char`** with the ranges of Arabic (`0600–06FF`, `0750–077F`, `08A0–08FF`,
   `FB50–FDFF`, `FE70–FEFF`), Hebrew (`0590–05FF`), Devanagari (`0900–097F`), Greek
   (`0370–03FF`, `1F00–1FFF`) and Hangul syllables and jamo (`1100–11FF`, `AC00–D7AF`). It is a
   dozen lines, adds no dependency, and fixes the Hindi and Arabic word splits. Thai has no
   spaces, so breaking it correctly needs a word dictionary (ICU or `unicode-linebreak` plus
   data); that would be a dependency and is **not** part of this proposal.
2. **Wrap in logical order.** Compute candidates and boundaries from the text in byte order,
   measuring with the glyph advances, then resolve each resulting line's visual order on its own.
   Touch points: `layout_wrapped_line`, `WrappedLineLayout`, and the consumers of
   `WrapBoundary` (`position_for_index`, `index_for_position`, `closest_index_for_x`, and
   `WrappedLine::paint`). I have not prototyped this one; it needs the per-platform shapers'
   run output reviewed, and it is the larger and riskier half.
3. **Make `x_for_index`/`index_for_x` bidi-aware**, so `src/bidi.rs` can be deleted. It would
   also extend caret and selection to wrapped lines, which `TextArea` cannot do today.

## How it would be verified

- Unit tests in `line_wrapper.rs` for each added range (the file already tests `is_word_char`).
- Re-capture the *Languages* page: Arabic must start `نص حكيم`, Hindi must keep `सर्वनाश`
  together, Latin and CJK wrapping must be pixel-identical to today. Compare the 8 routes whose
  stored `docs/parity` image matches with no differing pixel now ([pixel table](../../receipts/i18n-20261025/pixdiff-zh-hans.tsv)).
- The five gates, and the Linux and Windows shapers, which have not been exercised by this work.

## Not proposed

New dependencies, a new text shaper, ICU, hyphenation, vertical text, or changes to the Linux
and Windows vendor copies beyond the shared `text_system` code.
