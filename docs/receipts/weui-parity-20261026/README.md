# WeUI parity continuation — 2026-10-26 UTC

Later publication review: [October 26 review](../../REVIEW-2026-10-26.md) records the final
Footer/tooling corrections and authorization to commit the existing work. This receipt retains
its earlier source and approval state.

Full parity remains open. This continuation corrects measured Linux typography, Layers and
Panel differences, and extends native interaction evidence. The
[execution plan](../../WEUI-PARITY-PLAN.md) retains the remaining work and acceptance conditions.
Nothing was committed, pushed, published or deployed. The RTL logical-wrap landing hold and
unapproved step 3 remain in place.

## Source and authority

Base: `b1ddcae2b37b1901c17c78c6f06d9f4224ce03c2`, including the earlier local continuations.
Upstream remains WeUI 2.6.26 at `7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e`. Reference CSS,
markup and local WeUI.js identities and the unused WeChat SDK exclusion are unchanged from the
[prior receipt](../weui-parity-20261025/README.md).

The owner explicitly answered **“Approve the CJK closing-punctuation trial”**. The duplicate
reply was applied once: twenty closing marks were added to the existing classifier in one
additional vendored file, `text_system/line_wrapper.rs`. This separate approval extends the
previous vendor-edit boundary only for that trial. No dependency or lockfile change was made.
The four unchanged logical-wrap prototype files still match the supplied artifact; the earlier
approved trailing-space guard remains the only additional change in `line_layout.rs`.
Neither trial supplies new macOS evidence or resolves the RTL landing hold.

Current gallery executable SHA-256:
`4615ee3a813d142147e77ab77e03bddf207d26539828daa9720282bce47635e3`.
The [current manifest](metadata/current-matrix/manifest.json) identifies all 305 Rust/manifests
files, the reference, viewport, tools and frozen binary. [Verification](metadata/current-verification.json)
rechecks those files, all 2,112 PNG hashes and both the regular and frozen executable: no
mismatches, no reference page errors. The temporary diagnostic examples were removed.

`continuation-code.patch` is a review aid against HEAD for six affected files; it also includes
the prior form-title correction in `form.rs`. It is not an independently landable patch for
the combined dirty checkout. Each earlier capture set retains its own source manifest.

## Corrections

- **Typography:** six remaining 600-weight sites now use upstream 500: three Dialog title/body/
  action cases, HalfScreenDialog title, bold Form links, and the long half-screen gallery
  description. Public documentation now says medium. The earlier Form title correction remains.
- **Layers projection and sizes:** browser border quads showed that the intermediate parent
  flattens the CSS perspective. The native projector now follows that flattened transform.
  Plane sizes follow upstream width/height media queries (150×266, 180×320, 200×355), with
  compact depth and header spacing at 320px. Three gallery tests compare actual browser quads,
  breakpoint edges and the label transformation at both scales.
- **Layers labels/focus:** labels now transform with each plane through existing SVG painting;
  no backend edit or dependency was needed. The keyboard outline follows the plane. Title and
  description have heading/paragraph semantics. Native pointer regions follow projected quads.
- **Layers actions:** a real AT action initially selected the wrong layer because rectangular
  action centers overlap. Each named control now has a direct accessibility action using the
  same activation guard. Selection moves focus to that layer; inactive layer controls leave
  the accessibility/tab order, and stale hidden activation is rejected.
- **CJK closing marks:** the native probe established that Panel's 38-character description
  shaped to 532px, but an invalid break before `。` made the text appear to fit two 267px rows.
  Keeping the closing mark with its preceding character produces the browser's wrap and
  two-row ellipsis. The [approved trial](../../proposals/gpui-cjk-closing/README.md) adds twenty
  closing marks to the existing shared wrapper/truncator classifier. It does not implement
  opening-mark constraints, full Unicode line breaking, dictionary segmentation or tailoring.

## Corrected finding

The prior receipt's apparent **16px early half-screen text gap was a mistaken visual reading**.
[Pixel remeasurement](metadata/half-screen-measurement.json) of the same prior light-care state 2
finds secondary-text ink at native device row 622 versus reference row 619: native is 1.5 logical
pixels later at scale 2. The original prepaint bounds already showed the intended 16px child
gap. No padding change was warranted or made. Baseline/raster differences remain for review.
The earlier failed interpretation is explicitly corrected in that receipt.

## Native/reference evidence

Host: Linux x86_64, Rust/Cargo 1.95.0, Xvfb/Openbox and Mesa software rendering. Paired scenes
use zh-Hans, scale 2, reduced native motion and the same pinned local reference. Chromium,
Playwright, Pillow and font identities are recorded in the manifests.

| Run | Pairs | Source qualification |
| --- | ---: | --- |
| Typography routes and all registered states, eight appearances | 176 / 176 | After weight fixes, before later Layers/CJK changes |
| Layers six states, eight appearances | 48 / 48 | Corrected projection/labels, before final action/CJK changes |
| Layers six states at 320×812, 480×812, 375×480, light/dark | 36 / 36 | Same geometry source as the preceding Layers run |
| Panel, eight appearances | 8 / 8 | Final combined source, approved CJK trial |
| Full 132-scene × eight-appearance matrix | 1,056 / 1,056 | Final combined source and frozen executable |

Selected before/current pairs, four current contact sheets (24 scene/appearance observations),
full manifests, summaries and result hashes are retained. The current sheets were inspected;
full-size checks included Panel, Layers and bottom-fixed Form, with earlier targeted inspection
recorded separately. Detailed review of all 1,056 current pairs is not complete. Static scenes
and difference scores do not establish interaction, motion, reader or host acceptance.

The generated full corpus has an offline `review.html` with scene/appearance selection and
an overlay reveal. Its Chromium check verifies eight appearances, 132 scenes, image decoding,
75% reveal and navigation without page errors ([result](metadata/viewer-check.json)).
Reproduce with [PARITY-TOOLING](../../PARITY-TOOLING.md). Large raw corpora and frozen executables
remain generated evidence outside Git; selected images and hashes are retained here.

## Native interaction checks

**Layers:** eight appearances, six steps each: Enter selects Popout, Space returns to stack,
projected pointer selects Content, flat pointer returns, AT default action selects Mask, and
AT action returns. Each selection asserts that only the selected layer control is present and
focused. All **48 steps pass** on the final gallery binary. The driver, setup, per-mode results,
focus captures and summary are in `layers-interaction/`. These are native input and AT action
checks. The separate normal-motion and actual Orca extensions below qualify their named
contracts, rather than complete timing or reader acceptance.

**Layers normal motion:** eight appearances, ten checks each (**80 passing checks**). Rapid
Popout, Mask and Content selections interrupt their 500ms transforms; a cached hidden Mask
action leaves Popout selected; return restores all four controls. After final settling, the
stack is pixel-identical to its initial screenshot in every appearance. Source, setup, endpoint
images, failures and per-mode results are in `layers-motion/`. The first attempts missed focus
before window geometry settled; the final setup waits for the activated window before pointer
coordinates are used. This does not qualify every timing/host/hardware or reader case.

**SearchBar:** FilledGrey, Filled and Outlined in eight appearances produce **24 passing runs,
120 transition steps**. Each checks typing five characters/five Changed events and one Submit;
disabling the inner input prevents editing and hides Clear; re-enabling restores enabled state,
Clear emits exactly one Changed and Clear and keeps input focus; read-only prevents typing/
deleting and hides Clear; returning to editable then Cancel clears once and emits one Cancel.
Accessible enabled/read-only state and actual text are checked at transitions. No SearchBar
production change was needed. This qualifies the inner-input availability API, not a new
whole-bar disabled API. Exact callback/state snapshots, fixture source, binary identity and
setup are in `search-contracts/`; the probe binary is separately frozen compressed outside Git.

**Layers actual Orca:** an isolated runtime supplies Orca 48.1, AT-SPI2 2.56.2, Speech
Dispatcher 0.12.0 and eSpeak NG 1.52.0. English/light/scale-1 collection passes twelve
navigation/selection/return steps. Eight time-bounded speech checks confirm each named control
and button role both focused and selected, using Orca's own current flat-review object command.
Real ALSA synthesizer PCM is retained as a 22050Hz mono WAV (38.6264 seconds, nonzero samples),
with actual speech/debug logs, setup and verification in [layers-orca](layers-orca/README.md).
Tool asset paths were relocated in an isolated extracted Debian runtime; no product source or
Rust dependency changed. The initial title-command collection and setup failures are retained.
This is a bounded reader sample; human listening, audio timing and the complete reader matrix
remain open.

## Source checks and retained failures

The five Rust gates pass: fmt, locked/offline all-target build, tests (**59 root tests**, three
ignored doc examples), Clippy with warnings denied and strict rustdoc. The separate gallery
target passes three meaningful geometry/transform tests. Seven step-1, 16+4 logical-wrap and
eight trailing-space extracted cases pass. The CJK harness passes thirteen cases; its unchanged
negative control retains three expected failures. All 30 script and five website tests pass.
`logs/gates.log` records all twelve commands with exit zero. Source hashes still match after
temporary fixture removal; no production Rust change followed these gates.

Retained failures include the first gallery-test wildcard import (macro resolution), initial
AT role/focus/window-manager setup mistakes, and the real Layers wrong-action failure repaired
above. Search fixture setup first lacked reliable focus; a later fixture build failed a Rust
borrow check and an older binary was inadvertently retried. The final driver uses a successfully
rebuilt, hash-identified fixture with a tracked root focus handle for commands while input is
disabled. Its final 24 runs pass. These failed attempts are not counted as passing checks.

## Remaining work

| Priority | Work | Completion condition |
| --- | --- | --- |
| P1 | RTL wrapping/selection | Resolve Linux clipping/misalignment under the existing hold; obtain required Mac recaptures if logical wrapping changes; step 3 still requires approval |
| P1 | Complete component contracts | Extend enabled/disabled/error/loading, focus, live events and motion sequences beyond the named sampled contracts |
| P1 | Reader and native hosts | Current-source macOS/Windows input, rendering, windowing and reader evidence; human speech review; preserve the VoiceOver restriction |
| P2 | Text fidelity | Remaining form mixed-script wrapping/hyphenation, baseline/raster differences and broader Unicode line-break cases, including opening punctuation |
| P2 | Comparison review | Classify all current state/brand pairs at useful scale; extend responsive/scroll/zoom cases where layouts change |
| P2 | Layers completion | Complete reader navigation, full timing/animation coverage and other renderers/hardware; bounded English/light Orca and eight-mode normal-motion interruptions now pass |

No component or platform is promoted to fully accepted. WeChat SDK integration, application
transports, locale review and distribution/signing/publication retain their separate scopes in
[COMPLETION](../../COMPLETION.md) and [ACCEPTANCE](../../ACCEPTANCE.md).
