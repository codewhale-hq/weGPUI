# CJK closing-punctuation correction — reviewed developer preview

The real native text probe reproduces Panel's problem: its 38 characters shape to 532px at
14px, fitting two 267px rows in GPUI. GPUI breaks before the twentieth character, `。`, so it
believes no ellipsis is necessary. Chromium keeps that closing punctuation with the preceding
character, moves that character to the next row and consequently needs an ellipsis at two rows.
The missing ellipsis follows from the different break opportunity, not a missing ellipse glyph.

[cjk-closing.patch](cjk-closing.patch) adds twenty CJK closing punctuation characters to the
existing `LineWrapper::is_word_char` closing-mark rule, beside its existing ASCII equivalent.
The shared monotonic wrapper and wrapped truncator therefore keep them attached to the preceding
character in contiguous CJK text. This edits one additional vendored file, `line_wrapper.rs`,
and adds no dependency or version change. It does not change logical RTL placement or caret code.

The proposal is deliberately bounded: it does not implement opening-punctuation constraints,
full Unicode line breaking, dictionary segmentation or language-specific tailoring. A full UAX 14
implementation would need its own dependency/API decision; the existing transitive lock entry
for `unicode-linebreak` does not by itself authorize a new direct dependency.

## Reviewable checks

```sh
python3 -I docs/proposals/gpui-cjk-closing/verify.py --workdir target/cjk-closing-proposal
```

The verifier reconstructs the baseline and candidate in memory and compiles the actual vendored wrapping and
truncation code with the established word-break harness's minimal types and equal glyph advances.
Seven existing script/word regressions plus six new punctuation/clamp/control cases run.
Unchanged source fails three new cases and passes ten; the candidate passes all thirteen.
Outputs and checksums are retained here. These extracted checks do not qualify native fonts,
all Unicode text, the full packaged vendor test target or a platform's rendering.

## Approval and native checks

The owner answered **“Approve the CJK closing-punctuation trial”** on 2026-10-26, separately
extending the earlier vendor-edit restriction for this patch. The duplicate reply was applied
once. The patch is present in the developer preview. Panel's eight native/reference appearance pairs capture
successfully and show the corrected line break and ellipsis. The combined source gates and
thirteen extracted cases pass; baseline and native evidence are in the
[continuation receipt](../../receipts/weui-parity-20261026/README.md).

The later [publication review](../../REVIEW-2026-10-26.md) records the owner's request to
commit all current work. Known RTL rendering limits, unapproved step 3, the macOS recapture
requirement for logical changes, and package/site release restrictions remain. Native Linux evidence does not qualify other renderers or complete Unicode line breaking.
