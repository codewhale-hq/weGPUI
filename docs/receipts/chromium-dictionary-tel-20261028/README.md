# Chromium dictionaries and telephone input — 28 October 2026

This continuation is identified by [source hashes](source-sha256.json), recorded from
parent `265fbaf03143d43bdc003b07eacb55afa2064410` plus the continuation. The
provider uses Chromium 156's pinned abridged Khmer and Lao dictionaries with ICU4X
2.4.0's line rules and matcher; Thai and Myanmar retain ICU4X's baked data. The source,
licenses, hashes and regeneration method are in the [vendor data directory](../../../vendor/gpui_pre/src/text_system/browser_dictionary/README.md).
ICU4X and Chromium's patched ICU4C use different matching algorithms, so sharing data
is not a claim that their outputs always agree.

The [Linux ARM64 X11 comparison](comparison.json) matches all 16 normalized row
sequences for Thai, Lao, Khmer and Myanmar at widths 160, 225, 327 and 650 logical pixels,
using a controlled Lao font as in the [earlier receipt](../dictionary-mobile-20261026/linux-captures/README.md).
The `breaks` field in that raw comparison is produced by a separate **stock** ICU4X
segmenter for diagnostic comparison; the `lines` and row geometry are from the patched
GPUI binary. The [probe](dictionary-probe.rs), [driver](dictionary-driver.py) and
[binary hash](dictionary-binary-sha256.txt) identify the fixture. This is one corpus,
not a complete language or visual parity pass.

Telephone fields accept Unicode text, including vanity numbers and extensions, rather than
validating telephone syntax. Single-line programmatic values remove CR/LF; edits and paste
replace newlines with spaces. Tabs are retained; other controls remain filtered. IME
preedit removes NUL before accessibility exposure, and unmark sanitizes committed text
and snaps selection to grapheme boundaries. This changes behavior for consumers that
previously relied on Tel's restrictive character filter; applications should validate
telephone values as appropriate.

The [X11 input probe](tel/result.json) passes 26 steps covering actual keyboard and
clipboard edits, undo/redo, UTF-16 limits, disabled/read-only behavior and kind transitions.
Its marked-text and grapheme cases invoke the input handler synthetically; they do not
prove a real OS IME, AT-SPI tab geometry, or iOS telephone keyboard behavior. The
[probe source](tel/probe.rs) and [driver](tel/driver.py) are retained.

The candidate passes 54 macOS library tests, a macOS gallery build, all-target Clippy,
strict documentation, an iOS simulator bundle build and four extracted text contracts.
The existing Python suites pass 33 script and five website tests. The same source builds both Linux probes and a verified preview archive as a separate
Linux consumer. The
prior [site receipt](../overflow-ios-20261028/README.md) records the published site and
its own source. Windows UI remains outside the owner's scope; hosted final CI, real reader,
IME, hardware and mobile acceptance remain open. The macOS gallery is 64,491,432 bytes,
372,560 bytes above the earlier `c343621` gallery; the delta includes other intervening
changes and is not solely attributable to the dictionaries.

The wider [192-case comparison](extended/summary.json) covers 32 attributed inputs
at six widths, including mixed punctuation and spacing. It matches 189 normalized
row sequences: all 48 sampled Lao, Myanmar and Thai cases each match; Khmer matches
45 of 48. Two widths of `km-icu-rights` and one of `km-icu-prefix` differ. The
[full results](extended/comparison.json), [corpus](extended/corpus.json), screenshots
and probe/driver files are retained. The corpus is not linguistically reviewed or
comprehensive. The subsequent website update passes selected [local shell and theme
checks](website/extended/summary.json) on both mount paths.

The [live website](https://codewhale-hq.github.io/weGPUI/) was published from
`1c633f9c1d24cf0180e2dd9336f5d05d2f4a6007` by [Pages run 40656436658](https://github.com/codewhale-hq/weGPUI/actions/runs/40656436658).
Its clean source identity and selected [live shell and theme checks](website/live-1c633f9.json)
pass; they do not qualify native GPUI.

An independent [holdout boundary corpus](holdout/holdout-oracle.json) uses 32 new
CLDR person-name test fields and correlated punctuation, spacing and cross-script
variants. Against the private exact Chromium-pinned ICU4C 78.2 line iterator with
its verified data and `zh_CN` locale, the shipped ICU4X provider matches 252 of 480
boundary sequences; an unshipped experimental Khmer matcher matches 322 of 480.
The [results](holdout/holdout-candidate.json), [oracle generator](holdout/holdout-oracle.py)
and experimental [source](holdout/src/main.rs) are retained. The oracle generator
requires the separately retained private ICU build artifact; it is not standalone. The per-case `language`
field is the harness's `km` grouping; IDs and `holdout_language` in the oracle identify
the actual scripts; [attributed results](holdout/holdout-attributed-results.json)
provide corrected labels. These are correlated short-name and synthetic contexts, not 480
independent natural-language samples or rendered browser comparisons. They demonstrate
that broader segmentation parity remains open; the experimental matcher was not landed.
