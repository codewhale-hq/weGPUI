# Chromium Khmer line-break probe — 28 October 2026

This is exploratory evidence for one Khmer fixture, not proof of full WeUI or
cross-platform text parity. No repository or system dependency was changed, no
native build was run, and no custom data was installed.

## Reproduction

On this host, with Homebrew ICU4C 78.3 already present and the existing Docker
container running, run these as **separate processes** (ICU common data is
process-global and must be set before ICU opens any data):

```sh
python3 icu_probe.py --data stock
python3 icu_probe.py --data chromium
```

The stock run uses `/opt/homebrew/opt/icu4c@78/lib`'s existing dynamic libraries
and data. The Chromium run reads `/usr/lib/chromium/icudtl.dat` from
`wegpui-rtl-qualification` directly into memory, checks its SHA-256, and calls
`udata_setCommonData_78` before opening any break iterator. Both use `UBRK_LINE`
with the same UTF-16 text and convert ICU's UTF-16 offsets to UTF-8 offsets;
zero is omitted. `icu_probe.py --help` describes the path/container switches.
Negative ICU status codes in the results are fallback/default warnings, not
failures; positive codes are errors. Results are in `stock-78.3.json` and
`chromium-data-78.3.json`.

The installed browser identifies itself as Debian Chromium `156.0.8078.47`,
package `156.0.8078.47-1~deb12u1`. The exact browser data is 10,824,432 bytes,
SHA-256 `02a01b3188b6a41a756f7322fdb5e6b5ca6573b327b01dfc8ed662d571a6f769`.
It contains `icudt78l/brkitr/khmerdict.dict` and ICU line rule files. Its
hash matches the `common/icudtl.dat` at Chromium's pinned ICU revision, read
from the official source using a stream rather than installing it locally.

## Observation

For `zh_CN`, `km` and `root`, stock ICU4C 78.3 produces UTF-8 boundaries:

```
27 33 69 81 99 121 148 154 190 202 220 241
```

The same ICU4C 78.3 engine with the browser's exact data produces:

```
27 33 45 60 69 81 121 148 154 166 181 190 202 241
```

The current ICU4X dictionary result and the container's ICU4C 72.1 yield the
stock sequence. Chromium's native reference capture at width 225 breaks at
81 and 181, consistent with the second sequence; the GPUI capture breaks at
99 and 190. The saved `comparison.json` also records the captured text and
font evidence. Chrome's word segmenter results are additional evidence, but
are not a substitute for measuring line breaks.

This experiment uses **ICU4C 78.3 code**, not the exact browser's 78.2 code.
The Chromium source itself is patched, so one fixture and its compatible data
do not prove engine or language-wide equivalence. The browser's pinned README
says the Khmer dictionary is abridged, Lao uses an older abridged dictionary,
and `khmer-dictbe.patch` adjusts the Khmer engine to the smaller dictionary.
The observed data-dependent difference is therefore reproducible, while its
specific data-item cause and exact-engine behavior remain to be established.

## Practical implementation choices

1. Retain the existing ICU4X pure-Rust compiled dictionaries and document the
   Chromium-specific divergence. This preserves the current small cross-platform
   dependency graph, but cannot claim browser-equivalent Khmer segmentation.
2. Evaluate generating an ICU4X provider from Chromium's licensed abridged
   dictionaries and porting its general Khmer engine differences if necessary.
   This avoids a C++ runtime but has no demonstrated compatibility yet: data
   formats and segmentation algorithms differ, and its effect on all four
   dictionary scripts must be measured without phrase-specific rules.
3. For the closest reproducible reference, vendor a pinned, license-preserving
   build of Chromium's ICU 78.2 source including its Khmer patch and exact or
   intentionally filtered data, with a narrow safe C-ABI line iterator wrapper.
   Build separately for macOS, Linux, Windows and iOS, as well as every supported
   CPU; verify packaging, cross-compilation, licenses, size, and offline consumer
   setup. Avoid leaking ICU's process-global data state into host applications:
   symbol isolation and a separate data namespace must be designed and tested,
   especially where the host already loads ICU. Preserve Unicode byte indices,
   locale and whitespace behavior, object offsets, thread safety, error handling,
   and existing shaped-cluster guarantees. A version or data change requires a
   new comparison corpus rather than silently treating this snapshot as timeless.

A concrete next experiment is to build the exact pinned Chromium ICU code in
an artifacts directory under the shared build lock, feed the verified data in
an isolated process, and compare its line boundaries and rendered rows with
Chromium and ICU4X on Khmer, Lao, Thai, Myanmar, mixed scripts and punctuation.
Before adopting it, also compare binary/data growth and all supported targets;
the fixture alone cannot choose between these options.

## Sources

- Chromium tag and dependency: https://raw.githubusercontent.com/chromium/chromium/156.0.8078.47/DEPS
- Pinned ICU README and modifications: https://chromium.googlesource.com/chromium/deps/icu/+/19dfb44a62ae60ce6b0060de8c0cef5d5c527177/README.chromium
- Exact bundled data: https://chromium.googlesource.com/chromium/deps/icu/+/19dfb44a62ae60ce6b0060de8c0cef5d5c527177/common/icudtl.dat
- ICU BreakIterator and dictionary design: https://unicode-org.github.io/icu/userguide/boundaryanalysis/
- Existing comparison: [`comparison.json`](../../dictionary-mobile-20261026/linux-captures/dictionary/comparison.json)
- ICU4X 2.4.0 segmenter source: `icu_segmenter` 2.4.0, `src/line.rs` in the crates.io distribution
