# WeUI parity continuation — 2026-10-25 to 2026-10-26 UTC

Later publication review: [October 26 review](../../REVIEW-2026-10-26.md) records the final
Footer/tooling corrections and authorization to commit the existing work. This receipt retains
its earlier source and approval state.

Follow-up: the [October 26 receipt](../weui-parity-20261026/README.md) supersedes the current-source
claims below. It corrects the half-screen spacing interpretation and records the later weight,
Layers and separately approved CJK trial changes. This receipt's source hashes remain historical.

**Full parity is not complete.** Upstream coverage and the named Linux capture worklist are
complete; detailed comparison, remaining rendering corrections and native-host contracts are
open. The [execution plan](../../WEUI-PARITY-PLAN.md) and the ledger below distinguish them.

## Source and authority

The checkout starts at `b1ddcae2b37b1901c17c78c6f06d9f4224ce03c2`, with the prior completion
continuation and held logical-wrap prototype present. Nothing was committed, pushed, released
or deployed. The owner separately answered **“Approve the separate trailing-space trial”**.
Only its one monotonic-wrapper guard was added to vendored code. The original RTL landing hold
and unapproved step 3 remain; the old macOS captures do not qualify this later trial.

Current native executable SHA-256:
`2901f2af72197896390fe4aee0a8b6b8734e94408534ce005992536d46bb01cb`.
[The current manifest](metadata/current/manifest.json) hashes 305 Rust/manifests files;
[verification](metadata/current-verification.json) rechecked every file, all 2,112 current PNGs
and their recorded hashes: zero mismatches and zero reference page errors. The restored regular
gallery rebuild has the same executable hash after temporary diagnostic instrumentation was removed.

`parity-code.patch` isolates existing-file changes from this continuation. Its Footer base is
the prior independent completion source; its GPUI base is the supplied held prototype. The new
scripts/tests and plan live at their ordinary repository paths. This patch is a review aid,
not permission to land the combined working tree.

## Target and execution

Tencent/weui is still 2.6.26 at `7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e`.
The distributed CSS matches the pinned browser CSS byte-for-byte. The inventory maps all
61 HTML files (60 routes plus shell) and 59 widget LESS files, with no unclassified family.
The 72 extra registered states produce 132 scenes and eight appearances produce 1,056 pairs.
These counts show coverage, not accepted component behaviour.

The prepared reference retains official CSS/markup and byte-identical WeUI.js 1.2.17. It removes
only the unused external WeChat SDK tag and localises the WeUI.js script. Preparation refuses
to remove a used SDK. [Reference metadata](metadata/REFERENCE.json) records hashes and edits.
This checks UI examples and companion UI behaviour, not WeChat SDK integrations.

Host: Debian Linux x86_64, Rust/Cargo 1.95.0, Xvfb/Openbox, Mesa software rendering, 2× native
scale, zh-Hans, reduced native motion. Browser: Chromium 156.0.8078.47, Playwright 1.64.1,
Pillow 11.1.0; exact versions are also in the current manifest. Chromium uses Noto Sans and
Noto Sans CJK SC in the inspected text ([font evidence](metadata/reference-fonts.json)). An
explicit native Noto Sans control produced identical captures and the same resolved font id.

| Run | Successful pairs | Qualification |
| --- | ---: | --- |
| Complete pre-trial matrix | 1,056 / 1,056 | After Footer/title corrections, before trailing-space guard |
| Complete current matrix | 1,056 / 1,056 | Approved guard source; 60 defaults + 72 states in every appearance |
| Targeted Article/bottom-fixed-button trial | 24 / 24 | Eight appearances, default/agreed where registered |
| Responsive 480/700/900×812 | 36 / 36 | Article, Footer, Form primary/page, List, Panel; light/dark |
| Scrolled 375×812 | 12 / 12 | Same six routes; requested 800px with browser clamp recorded |

Scrolling controls include Footer at 0 and Form page at 5px; Article reaches 800px, Form primary
276px, List 191px and Panel 431px. These are not twelve full 800px movements. Raw scores and
per-scene source/image hashes are retained in `metadata/`. Full raw corpora and offline
`review.html` viewers are generated outside Git. Reproduce them with
[PARITY-TOOLING](../../PARITY-TOOLING.md); the frozen current runner is retained with its manifest.
The viewer provides scene/appearance selection and an overlay reveal; it starts every scene
unreviewed. Actual Chromium checks pass on the pre-trial, targeted-trial and current viewers,
including image decoding, selection, navigation and the 75% reveal, without page errors.

## Changes verified

- **Footer:** invisible minimum 44px targets preserve the upstream visible row and separator
  geometry. Original tall-row and corrected pairs are in `samples/`. Fixed backgrounds, wrapping
  and host-supplied insets from the prior completion work remain. Native checks in all eight
  appearances measure Help at 45×44, preserve 16+24=40px bottom padding and activate at both
  horizontal and vertical target edges. Enter and Space each add exactly one callback after
  focus, for four activations total. The keyboard driver, source and per-mode results are retained.
- **Article/long Latin text:** the approved guard stops a trailing ASCII space from moving a
  fitting word. Native and Chromium measure the fitting example at about 323.884px within
  327px; the following space increases it to 328.303px. The extracted actual layout code fails
  three regressions before the guard and passes all eight cases after it. The article returns
  to eight visible rows. A consistent image-background strip is 26 logical pixels late before
  the guard and one pixel early afterward in all eight appearances
  ([probe](metadata/article-geometry.json)); this is a sampled paint coordinate, not an element-box
  measurement or an exact-raster acceptance. The light pair's raw difference drops from 9.5823%
  to 4.5117%. Long button-page wrapping also improves; glyph/baseline differences remain.
- **Form title:** 500/medium weight now follows `.weui-form__title`, replacing semibold.
- **Footer demo:** the registration link now has a focusable link helper and the upstream URL
  callback. Source/build and its static capture pass; external URL dispatch was not invoked.
- **Capture reliability:** fresh browser contexts prevent state leakage. Reference asset drift,
  duplicate scene ids, unsafe paths and writable shared binaries are rejected before output.
  The old `.page:last-child` scroller selector failed on forms with trailing script siblings;
  selecting the final container page fixes all twelve replayed scrolled captures.

The Footer callback probes precede the approved wrapper guard; their exact fixture and executable
provenance are separate from the current full matrix. They do not qualify unrelated contracts.

## Checks and retained failures

All five Rust source gates pass on the current Rust source: fmt, locked/offline all-target build,
59 unit tests with three ignored doc examples, Clippy with warnings denied, and strict rustdoc.
The first run's doctest execution failed because the temporary directory disallowed execution.
That also prevented four existing fake-Apple-tool Python cases from running correctly. Moving
executable fixtures to a workspace temporary directory makes the full test command pass;
`logs/gates.log` and `logs/failed-gates-rerun.log` retain both outcomes. No test was weakened.

The seven step-1 and 16+4 logical-wrap extracted tests still pass. The new trailing-space harness
passes eight cases with the guard, while its negative control retains three failures. These
are extracted-function checks, not the unbuildable full packaged GPUI test target. The final
Python suite passes 30 script tests and five website tests. New capture-input checks exercise
reference drift, writable reuse, parent paths and duplicate scene evidence.

The first large run exhausted disk when Docker copied concurrent tool filesystems: 366 successful
pairs and 28 recorded failed scenes survive; writing its final summary also failed. Duplicate
scratch copies were removed, inactive executables were preserved compressed and the cached tools
were reused without another full filesystem copy. Both later full matrices complete. The first
scrolled run retains four selector failures; its twelve-pair rerun passes. A viewer test initially
assumed the next scene after Article was Button on the full inventory; correcting the test's
expected Badge navigation passes. No failed attempt is reported as a pass.

## Review scope and remaining work

All 60 default routes were inspected at contact-sheet scale in light, dark and both care variants
on the pre-trial source: 240 observations in sixteen retained sheets. Full-size inspection covered
Footer and Article before/after, Panel and Form primary, and current care-mode long button and
half-screen-dialog 2 text. Additional selected pairs are retained for follow-up. Image decoding,
hash verification and scores do not replace detailed review of all 1,056 current scenes.

| Priority | Remaining gap | Evidence / completion condition |
| --- | --- | --- |
| P1 | CJK punctuation and ellipsis | Panel can begin a line with punctuation and omit the browser's ellipsis; qualify real line/clamp behaviour before another GPUI proposal. The trailing-space approval does not authorize a broader vendor change. |
| Corrected finding | Long half-screen body text | The initial visual reading of a 16px early gap was wrong. Pixel remeasurement on October 26 places the native secondary text 3 device pixels later (1.5 logical pixels) in the inspected light-care pair; the intended 16px child gap exists. No padding fix was warranted. The later receipt retains the measurement. |
| P1 | RTL wrapping and selection | The existing Linux paragraph failure and landing hold remain. Step 3 still needs separate approval; no wrapped-caret change was made. |
| P2 | Form descriptions/labels and typography | Browser/native hyphenation, mixed-script wrapping and baseline/raster differences still need explicit review and appropriate fixes or accepted adaptations. |
| P2 | Layers 3D example | Native labels stay upright and projection differs from CSS transforms. Strict parity needs rendering work; an adaptation must be explicitly documented/accepted. |
| P1 | Complete interaction/motion/reader qualification | Static seeded states do not prove enabled/disabled transitions, events, focus, animation timing or reader speech. Extend the per-family matrix in ACCEPTANCE on current source. |
| P1 | macOS, Windows and hardware | Fresh current-source rendering/input/windowing and native-reader evidence are unavailable on this Linux host; retain the existing VoiceOver restriction. No platform/component is promoted to accepted. |
| P2 | Finish comparison review | Classify the remaining current states/brand combinations at useful scale and extend responsive/zoom/long-page cases where layout changes; do not infer correctness from scores or route counts. |

Application-owned transports, WeChat SDK integrations, native distribution/signing and publication
are separate from WeUI component fidelity. Native localisation additions still have their own
review/IME/locale limits in I18N; the Chinese upstream reference does not accept every language.
