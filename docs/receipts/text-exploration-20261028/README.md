# Unlanded four-script and cluster investigations — 28 October 2026

The shipped source remains `dad6cf3b245b0256ca31e7be9cd5582f98a87e75`. An experimental
four-script dictionary matcher was built and tested, but is not included in that source.
Its exact dirty candidate is recoverable from the local backup inventory at
`CW/backups/wegpui-four-script-render-regression-20261028`.

The controlled [leading-mark A/B](leading/result.json) compared 14 problematic inputs
and their 14 minimized substrings at six widths against pinned Chromium/WeUI. The
[baseline](leading/baseline.json) matched 102 of 168 row sequences; the
[candidate](leading/candidate.json) matched 75. Twenty-seven became worse, none improved,
and 43 changed while both remained mismatched. Because this is a real sampled regression,
the candidate was not landed. The [corpus](leading/corpus.json) and comparison script
are retained; results do not represent complete language coverage.

A separate Linux ARM64 X11/Mesa [cluster fixture](cluster/native.json) showed native text
`ม😀ះ` splitting the emoji and Khmer mark onto separate rows at narrow widths, whereas
[Chromium](cluster/browser.json) kept them together and allowed overflow. The
[SPACE-plus-combining control](cluster/browser.png) can break in both; a blanket extended-
grapheme filter would change permitted Unicode line-breaking behavior. Font fallback also
differs for emoji, and this is not a platform-wide result.

A case-specific, unshipped [virama reshaping proof](virama/reshape.patch) reshaped one
minimized Myanmar example when it overflowed. Its [48-case capture](virama/summary.json)
matched 29 row sequences; it does not implement a general shaping solution or qualify
editing. GPUI currently snaps hit testing against row substrings, so permitting a line
start inside a document grapheme can place a caret inside that grapheme. Full-document
selection, IME and accessibility behavior must be solved with any general reshape path.

The updated local website passes selected [shell and theme checks](website/summary.json)
at both mount paths.
