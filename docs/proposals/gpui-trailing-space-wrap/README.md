# Shaped-row trailing-space correction — approved trial

The Linux parity run reproduces early wrapping in Article and the bottom-fixed button page.
At 17px Noto Sans, both native shaping and Chromium measure
`Lorem ipsum dolor sit amet, consectetur` at approximately 323.884px. It fits the article's
327px content width. The following space increases the measured width to approximately
328.303px. GPUI's monotonic `LineLayout::compute_wrap_boundaries` treats that trailing space
as overflow and moves the fitting word `consectetur` to the next row. This adds an article row
and shifts its images. Selecting Noto Sans explicitly produced identical native captures;
changing component padding would hide the cause.

[trailing-space.patch](trailing-space.patch) adds one guard in that shaped-row function:
ignore overflow caused only by an ASCII space after visible content. Preserve the existing
leading-space policy and ordinary overflow handling. It changes no component API, dependency,
logical RTL path, caret or selection implementation. The unshaped editor `LineWrapper` is
outside this proposal; consistency and editor whitespace policy need separate review.

## Evidence and limits

`verify.py` extracts the actual vendored layout types and classifier using the existing
logical-wrap harness's minimal GPUI type stand-ins. It applies the replacement in memory;
it never writes into `vendor/`. Eight synthetic shaped-advance cases cover fitting words,
multiple/trailing spaces, exact fit, genuine overflow, oversized words, leading spaces and CJK.
The unchanged source fails the three trailing-space regressions and passes five controls;
the proposed source passes all eight. Both outputs are retained here.

```sh
python3 -I docs/proposals/gpui-trailing-space-wrap/verify.py --workdir target/trailing-space-proposal
```

This is an extracted-function check, not the packaged vendor test suite or full Unicode/CSS
whitespace conformance. The separately approved native trial restores the article's eight rows
and corresponding image placement in all eight appearance modes; 24 Article/bottom-fixed-button
pairs were recaptured. Source gates and the broader matrix are recorded in
[the parity receipt](../../receipts/weui-parity-20261025/README.md). CJK punctuation/ellipsis, non-ASCII spaces,
RTL wrapping/caret selection, and renderer baseline differences remain separate gaps.
The patch applies to the original prototype's `line_layout.rs`; its source identity is in
[checksums.txt](checksums.txt). Rebase and requalify it if the prototype is removed.

## Approval boundary

The owner's pasted takeover request says **"no vendor edits beyond this step"** and
**"If you change that logic for any reason, stop and ask Hunter to re-capture on his Mac"**.
The owner subsequently answered **"Approve the separate trailing-space trial"** on 2026-10-25.
That permits this guard and its fresh build/capture checks. That trial approval did not release the earlier RTL **Hold landing** decision or approve step 3.
The later [publication review](../../REVIEW-2026-10-26.md) records the owner's subsequent request
to commit all existing work; step 3 remains unapproved. Four prototype files
remain byte-identical; `line_layout.rs` differs only by this approved guard. The old macOS
captures qualify their old source and do not qualify the new trial. The later request to commit is distinct from the original permission to trial the guard.
