# Four-script extended dictionary rendering probe

This artifact was built and run against source `1c633f9`; the retained summary reports
189 of 192 matching row sequences. It expands the previous four-sentence comparison to 32 cases (eight each for
Khmer, Lao, Myanmar and Thai) at 112, 160, 225, 320, 375 and 650 logical pixels:
192 native/reference comparisons. The corpus includes the previous samples,
ICU4X 2.4.0 dictionary/line fixtures, partial dictionary words, and synthetic
variants mixing script with Latin text, punctuation, nonbreaking space, zero
width space, and a word joiner. These are small samples, not a comprehensive
language or Unicode conformance corpus.

## Attribution

`ICU4X-LineBreakDictionary.txt` is a verbatim copy from `icu_segmenter 2.4.0`'s
`tests/testdata/LineBreakDictionary.txt`. Its `|` break annotations are removed
when constructing the selected texts, so its expected results are not mistaken
for Chromium expectations. The prior four original samples come from
`/work/full-parity/dictionary-final/comparison.json`. `corpus.json` records each
case's origin. `synthetic-icu-derived` cases combine those test words with
punctuation and spacing; they are not asserted to be linguistically reviewed.
The pinned ICU4X license is retained in `LICENSE-UNICODE-3`. The original ICU4X
source notes the Myanmar repeated-script sample is from Web Platform Tests:
https://github.com/web-platform-tests/wpt/blob/master/css/css-text/word-break/word-break-normal-my-000.html
The first rendering source is adapted from the existing
`/work/full-parity/dictionary-probe/src/main.rs`; the browser driver follows
`/work/full-parity/dictionary-final.py`.

## Run on the existing Linux fixture

Both scripts must run from the macOS host while Docker container
`wegpui-rtl-qualification` is running with the existing `/src`, `/work` and
`/target` binds. The first invocation acquires the shared two-slot machine build
lock and does a locked offline Cargo build. The second runs the probe in its
own Xvfb display and compares to the locally pinned WeUI 2.6.26 Chromium
reference using Playwright.

```sh
cd /Volumes/VIXinSSD/CW/artifacts/wegpui-completion-20261025/linux/full-parity/dictionary-extended
./build.sh
./compare.sh
```

The locked offline build succeeded and the retained summary records the source and binary hashes. The build records the hash of every source, vendor, example and
asset input plus the corpus and binary before/after compilation. The capture
refuses to run if source/corpus/binary diverged from the build and rechecks at
the end. New output is a timestamped `comparison-*` directory with `summary.json`,
`native.json`, per-case `comparison.json`, one native fixture window screenshot,
and a browser screenshot for each differing row sequence. Keep build output
separately. The lock does not prevent another agent editing source, so the hash
checks are necessary. The fixture requires a free X display `:215`, the existing
Mesa software renderer, fonts, Chromium, Playwright, Xvfb, Openbox, xdotool and
ImageMagick; it aborts instead of claiming an already-used display.

Native output records stock ICU4X opportunities and candidate opportunities
from the **same source file used by GPUI** separately. Rendered native rows come
from the patched GPUI shaped layout with `OverflowWrap::Normal`. The browser
loads the pinned WeUI `form_page` with `zh-CN` browser locale, consistent with
the earlier fixture. It uses the default reference system font except for Lao,
which explicitly requests Noto Sans Lao to match the existing native fallback;
CDP font-family data is recorded for every case. Browser rows are collected
using DOM Range geometry. Comparisons strip trailing ordinary spaces only;
NBSP and ZWSP are retained. A mismatch can reflect font, layout, or segmenter
behavior and must be diagnosed, not automatically attributed to the dictionary.
The native screenshot is a rendering sanity sample, not 192 reviewed images;
passing row comparisons does not establish visual, native input, accessibility,
physical device or broad language acceptance. Browser-only mismatch screenshots
are not native/reference pairs.
