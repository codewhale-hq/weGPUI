# Shared text, dates and parity continuation

Completed 25 October 2026 (America/Los_Angeles). The owner requested completion, review and
pushes to canonical GitHub `main`, approved wrapped RTL step 3 and direct references to the
already locked Unicode bidi and clock crates, and restricted Windows to build/tests.

Shared-library source: [`4b01775`](https://github.com/codewhale-hq/weGPUI/commit/4b01775143fedca00512cd7ea6b3188d6626e47a).
Reviewed gallery/capture follow-up: [`fa60f66`](https://github.com/codewhale-hq/weGPUI/commit/fa60f66e42e7604174f585d7be3c5725b26b186a).
The latter changes one example's warning styling, Panel module documentation and reference
replay timing; shared text/date implementation bytes are unchanged.

## Implemented and reviewed

- Replace the input-only bidi shim with GPUI shared row geometry. Unicode bidi levels resolve
  neutrals, digits and isolates; logical cluster ends are cached. `ShapedLine`, factory-built
  `WrappedLineLayout` and element `TextLayout` provide text-aware caret, glyph hit and separate
  selection spans. The element uses the actual painted alignment width. Inputs and inline-link
  bounds consume this shared geometry. Soft-wrap caret affinity chooses the next row, and
  trimmed wrap whitespace has zero-width geometry. Input endpoints remain grapheme-safe.
- Select each painted style by its logical byte interval, even when visual glyph traversal
  decreases the source index. Color, background, underline and strikethrough continue correctly
  across right-aligned wrapped rows; decoration endpoints include the row's actual alignment.
- Use one opening/closing punctuation predicate in the wrapper, estimated truncator and shaped
  row wrapper. Oversized RTL words break at shaped clusters instead of falling back to the old
  visual-order path. This remains a bounded line-break policy, not complete UAX 14 or dictionary
  segmentation.
- Clamp element text against actual shaped row counts and widths, including the ellipsis. The
  review found that Panel's estimated-width decision could silently clip its final characters;
  the current Panel sample has the reference ellipsis and preserves fitting complete text.
- Localize numeric Gregorian date presentation, typed parsing and date-picker column order.
  Public `DateValue::parse`/`format`, picker storage and emitted values retain canonical ISO /
  year-month-day contracts. Neutral `en` preserves ISO; explicit `en-US` is month/day/year,
  `en-GB` is day/month/year, Chinese/Japanese/Korean are year/month/day, and the other built-in
  locales use day/month/year. German/Russian use dots; localized alternatives use slashes.
  `today()` uses the host local clock. An unfinished draft retains its entry locale when the
  application changes locale, avoiding reinterpretation of an ambiguous numeric draft.
- Add only direct edges to existing locked `unicode-bidi 0.3.18` and `chrono 0.4.45`. The
  lockfile adds those two dependency references; no package version, checksum or package set
  changes. All four consuming workspace patches remain required; see [vendor guidance](../../../vendor/README.md).
- Reject uniformly blank native frames with bounded readiness retries. Pin the reference
  browser locale and wait for the upstream page transition before replay. Upstream moves
  focus to the page at animation end: an earlier replay could lose SearchBar focus and falsely
  compare its inactive state. SearchBar focus/value and Slider value 80 are now checked before
  capture. The amount example retains invalid semantics and its error description while its
  visual row stays neutral, matching upstream's inline-error styling.

Raw text-free `LineLayout` methods retain their low-level legacy behavior and platform struct
layout. Applications needing bidi geometry use the text-owning wrappers above. Exact
font-provided ligature caret positions are not implemented; cluster interpolation remains.
Uncertain bidi character extents remain absent from accessibility.

## Verification

All thirteen [frozen-source local checks](checks/results.json) pass at `4b01775`: macOS and
Linux ARM64 all-target builds, 54 library plus three gallery tests on both hosts, strict macOS
all-target Clippy and rustdoc, four extracted text harnesses, 33 script tests and five website
tests. Five existing bidi tests moved from root into the shared vendor harness; they were not
removed. The logical harness runs 16 wrapping and nine layout/bidi cases. The other harnesses
include seven word-break, eight trailing-space and thirteen punctuation cases (the trailing
harness also compiles the five shared bidi cases). Negative controls retain their expected
failures. Packaged GPUI lacks its upstream test assets/development graph; extracted contracts
are supplemental and do not represent its complete test suite.

The standalone preview source bundle verifies and its separate workspace consumer builds
locked/offline. The bundle contains the four patches; no package was published. The follow-up
passes affected macOS/Linux gallery tests, strict gallery Clippy, strict docs and all 33 script
checks. Unchanged library gates were not repeated for the one-line gallery fix.

[Final CI at `4b01775`](https://github.com/codewhale-hq/weGPUI/actions/runs/40274887155) and
[the reviewed `fa60f66` candidate](https://github.com/codewhale-hq/weGPUI/actions/runs/40276223271)
pass Linux stable/MSRV, macOS, Windows MSVC build/tests/Clippy and mobile library type checks.
[Job metadata](metadata/ci-jobs-final.json) records the latest results.
[CodeQL](https://github.com/codewhale-hq/weGPUI/actions/runs/40276223301) also passes at `fa60f66`. Mobile checks are not
backend or runtime acceptance. Windows was not unlocked or driven; no Windows visible UI,
IME or reader result is claimed. The earlier Windows ARM64 receipt keeps its original source.

Native macOS checks confirm Arabic three-character replacement, undo restoration and wrapped
highlighting on the frozen library binary. US date entry `03/04/2027` opens month March, day 4,
year 2027; a Down/Enter on the first displayed column commits `04/04/2027`. Earlier same-date-path
British/German demonstrations show day-first focus and live locale reformatting; their
pre-freeze binary identity is kept separately. Screenshots and AX text are in [samples](samples/).
AX output is observation evidence, not an actual-reader pass. VoiceOver stayed off.

Linux ARM64 native XTEST and independent AT-SPI readback pass select-all, `تدع`, cross-row
selection, ASCII insertion, undo restoration and pointer drag on the current shared library.
The [styled probe](samples/styled-probe.rs.txt) and its
[measured result](metadata/styled-qualified-result.json) verify three distinct Hebrew glyph
colors and matching underline/strikethrough colors under left/right alignment, plus six
right-aligned wrapped backgrounds and full-width decoration continuations. This uses Xvfb,
Openbox and Mesa lavapipe, not hardware rendering. CJK/emoji fallback fonts are installed for
this fresh corpus; the earlier missing-font captures remain historical failures.

## Reference and capture identity

The target remains WeUI 2.6.26 / revision `7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e`, with
weui.js 1.2.17. The official cached npm archive was hash-verified; its CSS, original HTML,
localized HTML, JavaScript and all 59 style sources match the retained pinned reference.
[Transport proof](metadata/transport-proof.json) records content equivalence, rather than
pretending the npm tree is a fresh Git checkout. The unused WeChat SDK remains excluded.

The [qualified matrix](metadata/qualified-matrix.jsonl) accounts for all **1,056 pairs**:
132 scenes × eight appearances. A full candidate capture at `4b01775` is followed by **584**
corrected state/control captures at `fa60f66` (all 72 registered states × eight, plus the affected
default route × eight). The other **472** unchanged default captures retain their source and
binary identities. No receipt is relabelled as a later executable. The
[verification result](metadata/verification.json) checks all 3,280 PNG hashes in those two input
corpora before selecting the final pair for each identifier. Thirty additional pairs cover
320/768-wide layouts and 700px-scrolled long pages, with separate manifests and image hashes.

Full raw images remain in the local code-parity artifact corpus. This receipt retains source
and capture manifests, all result rows/hashes, representative unchanged pixels and review
contact sheets. It does not duplicate another complete raw corpus in Git. The earlier
pre-clamp sweep, a diagnostic source/binary mismatch, and the failed first readiness selector
remain local diagnostic evidence and are excluded from the qualified union.

## Difference review and practical limits

Contact-sheet review covers all 60 default light scenes and 72 registered light states,
with dark/care/brand spot review and ranked pixel differences. Responsive/scrolled samples
are reviewed separately. The following findings explain the material comparison differences:

| Surface | Result |
| --- | --- |
| Panel two-line app-message description | Missing ellipsis fixed using shaped clamping; complete text rows remain complete. |
| Focused/typed SearchBar | Reference replay now waits for page focus transfer. Retained focus/value assertions and corrected captures pass. |
| Amount inline feedback | Neutral label/value plus red inline hint now matches reference; semantic invalid/error state remains. |
| Slider value 80 | Retained reference value is asserted; corrected pair has the intended 80 state. |
| RTL generic text and styles | Shared geometry contracts and native color/background/decoration measurements pass; full complex-script/font coverage remains open. |
| Native date controls | Locale policy is deliberate. Chromium's OS-provided date placeholders remain host-dependent even with Chinese document/browser locale. Numeric values, parsing and picker storage are independently verified. |
| Custom-brand agreement mark | Native controls follow the supplied brand; the upstream example retains hard-coded green. This is a theme adaptation. |
| Loading glyphs / caret / transient motion | A static capture can sample different animation phases. Historical normal-motion contracts keep their original identity; screenshots are not motion acceptance. |
| Typography, mixed CJK/Latin paragraphs and narrow form descriptions | Font fallback, glyph advance, baseline and native/browser line-break differences remain visible. At 320px, form description row counts can differ. Pixel identity and full Unicode segmentation are not claimed. |
| Wide Layers example footer | The pinned browser example includes external site/legal footer content at wide width; native gallery keeps its own attribution. Layer geometry remains independently covered. |

The median raster score is about 2.53% of pixels differing by more than eight channel levels;
its maximum is about 11.07%. These are inspection aids, **not parity acceptance thresholds**.
No component or desktop platform is promoted to fully supported by capture completeness.

The authorized implementation and review batch is complete. Remaining acceptance requires
real RTL IME, full reader/native-speaker coverage, Windows visible checks outside this run's
scope, Linux hardware/windowing diversity, distribution/installers and mobile backend work.
CLDR/alternate calendars, dictionary line segmentation and exact font ligature caret tables
are separate capabilities. No signing, deployment, release, website publication, provider call
or paid service operation occurred.
