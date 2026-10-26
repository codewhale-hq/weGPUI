# WeUI parity plan and execution

## Target

Tencent/weui `master` was checked on 2026-10-25. It is still **2.6.26**, commit
[`7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e`](https://github.com/Tencent/weui/commit/7a6f8ad7700d0d6f1361dbe5e2942b4b129e7a7e).
Its distributed CSS is byte-identical to the repository's pinned browser CSS: SHA-256
`8db42592f96d571568c49ae1b8484f55042e6a6dab25f8144460cd4a23530fd3`.
There is no version upgrade to perform before closing the native parity gaps.

Parity covers component geometry, colours, typography and truncation, theme/state behaviour,
interaction/callback contracts, keyboard access and accessible names/values. Each accepted
result needs its source, reference, scene and host. Renderer/font rasterisation differences
and deliberate accessibility/native adaptations need explicit evidence and descriptions.
Release packaging and publication remain in the separate desktop release checklist.

## Execution order

| Stage | Work | Pass condition | State |
| --- | --- | --- | --- |
| 1. Freeze and inventory | Pin upstream source, map every example and style family, enumerate default/registered states | Immutable source identity; no missing upstream route or unclassified style family | Complete: 61 HTML files, 59 widget styles, 132 scenes |
| 2. Current paired evidence | Rebuild current native source, use zh-Hans against the same-host reference; cover light/dark, care, brand, states, responsive and scrolled scenes | Every requested scene captured; all failures retained; reviewed differences tracked | Three full 1,056-pair sweeps complete across recorded sources; responsive/scrolled extensions retained; detailed review remains partial |
| 3. Correct differences | Fix measured geometry, clipping, type, state, animation and callback differences in component source | Targeted before/after evidence and meaningful regression checks; no new unrelated vendor/dependency change | Footer, typography and Layers corrected; approved trailing-space and CJK closing-mark trials pass; remaining differences in receipts |
| 4. Qualify component contracts | Exercise availability, events, keyboard, focus, reduced motion and accessibility per family | Named contract evidence on Linux, macOS and Windows; full reader qualification separately identified | Linux Footer and Layers eight-mode contracts pass; SearchBar 24 runs/120 transitions pass; Layers adds 80 normal-motion checks and bounded actual Orca speech; other hosts open |
| 5. Close the matrix | Repeat source gates, retain checksums, update every open difference and adaptation | No unexplained difference or unverified required contract; unsupported cases stay visibly open | Five source gates pass; full parity acceptance remains open |

## Reference and matrix

The baseline is 60 upstream routes plus 72 registered non-default states: 132 scenes.
Light/dark × normal/care × default/custom brand gives **1,056 paired scenes**, before extra
viewport widths, long-page scroll positions and interaction sequences. Native captures use
`--lang zh-Hans`; the library's deliberate English default is not compared with Chinese copy.
The native-only Languages page is outside that upstream route count.

`scripts/weui_reference.py` prepares the pinned repository's distributed example without
executing its build/release scripts. It retains the official CSS/markup and byte-identical
weui.js 1.2.17. Its only HTML changes remove the unused external WeChat SDK tag and localise
weui.js. `REFERENCE.json` records those changes and both HTML hashes. This reference qualifies
WeUI UI appearance/behaviour; it does not qualify WeChat SDK integrations. The existing strict
`weui_upstream.py` full-runtime fetch remains available when its SDK dependency is reachable.

Large fresh capture sweeps stay in generated evidence directories. Selected reviewed pairs,
manifests and summaries belong in receipts; route counts and pixel scores alone are not passes.

## First measured correction

The previous completion work made FooterLink layout boxes at least 44px tall. Upstream's
`weui-wa-hotarea` expands an invisible target while preserving the visible text row's line box.
The parity correction must retain the accessible/pointer target, restore the visible row and
separator geometry, and verify normal/fixed footers with and without a host-provided inset.

Other recorded differences to remeasure include form-description/label wrapping, article
typography and spacing, panel punctuation/ellipsis, motion timing, and responsive/scrolled
regions. Every new finding will be added to the execution receipt with its disposition.

The [Unicode continuation](receipts/unicode-parity-20261026/README.md) replaces the bounded
break policy with the approved locked Unicode 15.0 iterator. It captures 112 targeted pairs
and closes the reproduced 320px form-description row-count difference. Measured article
line endings match at 375px with the same resolved Noto Sans font. Baseline/raster differences,
dictionary segmentation and the remaining interaction/host/reader matrix stay explicit.

## Preserved constraints

The owner initially held the logical-wrap prototype, then requested publication of all
current work after complete review on October 26. The [publication review](REVIEW-2026-10-26.md)
records that authorization; the Linux failure remains an acceptance blocker. The owner separately approved the
[trailing-space guard trial](proposals/gpui-trailing-space-wrap/README.md): four prototype vendor files
stay byte-identical to the supplied patch; `line_layout.rs` differs only by that guard, outside
the logical RTL/caret path. The owner then approved the separate
[CJK closing-mark trial](proposals/gpui-cjk-closing/README.md), extending the existing classifier
in `line_wrapper.rs` for twenty closing marks. Its thirteen extracted cases and native Panel
checks pass. The [October 26 receipt](receipts/weui-parity-20261026/README.md) records current
source gates, typography, Layers geometry/labels/focus/actions and bounded native contracts. Step 3 was subsequently approved; the [RTL continuation](receipts/rtl-completion-20261025/README.md)
records new row-aware input mapping, glyph-order normalisation and current host samples.
The subsequent [shared continuation](receipts/code-parity-20261026/README.md) adds the
owner-approved direct locked Unicode bidi/clock edges, text-owning GPUI mapping, styled painting,
shaped clamping and locale dates. Full host/IME/reader qualification remains open. No deployment,
package publication or platform-support claim follows from this parity work. macOS/Windows/hardware and human
reader checks cannot be replaced by Linux screenshots or browser tests.
