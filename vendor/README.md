# Approved accessibility compatibility copies

The owner approved the original compatibility copies on 2026-10-25 and authorized
additional dependencies and copies for the continuation. Package versions remain `accesskit_unix 0.22.1`, `accesskit_atspi_common 0.19.1`, `gpui-pre 0.3.9`,
`gpui-pre-linux 0.3.9` and `gpui-pre-ios 0.3.9`. Copies came from the exact locked crate archives; licenses and
package provenance are retained. Registry files were not edited. Root Cargo patches
select these copies; the approved Unicode bidi continuation adds one direct edge to the already
locked `unicode-bidi 0.3.18`. The owner subsequently approved ICU4X 2.4.0 dictionary
segmentation. It replaces the interim direct `unicode-linebreak 0.1.5` edge and adds four
package versions, without changing existing versions. The local core also bridges the
published iOS backend's missing focus, selection-fragment and lifecycle APIs.

## Compilation and consumer setup

**The matching patched GPUI copy is required to compile current weGPUI source.**
Semantics/Modal integration calls the added `Window::with_a11y_action_guard` API.
Unpatched registry GPUI 0.3.9 does not provide that API. All five patches below belong
in the consuming application's workspace-root Cargo.toml, using a matching checkout:

```toml
[patch.crates-io]
gpui-pre = { path = "/path/to/weGPUI/vendor/gpui_pre" }
gpui-pre-linux = { path = "/path/to/weGPUI/vendor/gpui_pre_linux" }
gpui-pre-ios = { path = "/path/to/weGPUI/vendor/gpui_pre_ios" }
accesskit_unix = { path = "/path/to/weGPUI/vendor/accesskit_unix" }
accesskit_atspi_common = { path = "/path/to/weGPUI/vendor/accesskit_atspi_common" }
```

Dependency-manifest patch sections are not inherited. A Git or path dependency on
weGPUI alone does not apply these workspace patches. Versions stay unchanged, but
this preview therefore depends on its documented local compatibility sources.

## Scope and provenance

The original deltas and checksums are under `docs/proposals/accesskit-linux-compat/`,
`a11y-state-and-bounds/`, `x11-grab-focus/` and `modal-a11y-action-guard/`.
The approved changes repair AT-SPI cache signal encoding; forward disabled/read-only,
invalid, busy, expanded/collapsed and heading-level metadata; update X11 window bounds;
guard accessibility action dispatch for hidden/closing/stale modal targets, including
cached views; and ignore temporary X11 Grab/Ungrab focus notifications before backend
activation/IME changes. Normal and WhileGrabbed handling is retained.

Modified upstream source files are:

- `accesskit_unix/src/atspi/bus.rs`
- `accesskit_atspi_common/src/node.rs`
- `gpui_pre/src/window.rs`, `src/window/a11y.rs`, `src/view.rs`
- `gpui_pre_linux/src/linux/x11/client.rs`
- `gpui_pre/src/text_system/line_wrapper.rs` (text breaking; see below)
- `gpui_pre/src/text_system.rs`, `src/text_system/line_layout.rs`,
  `src/text_system/line.rs`, `src/elements/text.rs`, and new
  `src/text_system/logical_wrap.rs` (step 2 prototype; known Linux rendering limits, see below)

Earlier Linux runtime receipts are in `docs/receipts/linux-bridge-20261025/`,
`input-contracts-20261025/`, `data-contracts-20261025/` and
`wayland-20261025/bridge-followup/`. Their exact artifact hashes distinguish those
runs from the later guard and focus-mode changes. Cache/reopen and X11 bounds results
remain sampled checks, not full screen-reader acceptance. The new focus-mode controls,
real Fcitx and Orca samples are in `docs/receipts/x11-grab-focus-20261025/`; exact new
artifact hashes and remaining listening/reader/IME limits are recorded there. Modal
action-guard checks remain separate. EditableText is unavailable in the bridge.

No forked package publication, GPUI version change or release distribution is
authorized by these compatibility approvals. Full accessibility/platform
qualification remains separate.

## Text-breaking change (owner-approved 2026-10-25)

The owner approved, in chat on 2026-10-25, step 1 of
[`docs/proposals/gpui-text-i18n`](../docs/proposals/gpui-text-i18n/README.md) with no new
dependency and Hangul left as is. `LineWrapper::is_word_char` in
`gpui_pre/src/text_system/line_wrapper.rs` now also treats Arabic (`0600-06FF`, `0750-077F`,
`08A0-08FF`, `FB50-FDFF`, `FE70-FEFF`), Hebrew (`0590-05FF`), Devanagari (`0900-097F`) and Greek
(`0370-03FF`, `1F00-1FFF`) characters as word characters, so line wrapping and ellipsis truncation
no longer split words of those scripts. The delta (`word-break.patch`), the locked-archive and
patched checksums, and the extracted test harness are in that proposal directory; the evidence is
[`docs/receipts/gpui-word-break-20261025`](../docs/receipts/gpui-word-break-20261025/README.md).
The vendored test target still cannot compile from the packaged crate, so the seven added or
affected tests run through `verify-word-break.py` against the unmodified source. The wrapped
order of right-to-left paragraphs is addressed by the separate candidate below; Thai breaking
and Hangul are unchanged.

## Logical-order wrapping (approved prototype; known rendering limits)

The owner approved step 2 of the same proposal, with no new dependency and landing conditional
on the Languages page, unit tests and five gates. The candidate reshapes logical lines of
reordered right-to-left/mixed paragraphs and exports their logical row starts. All five vendor
files matched the supplied macOS prototype byte-for-byte at the held receipt. Delta and full checksums are in
[`docs/proposals/gpui-text-i18n`](../docs/proposals/gpui-text-i18n/README.md).

| File under `gpui_pre/src/` | Locked archive SHA-256 | Candidate SHA-256 |
| --- | --- | --- |
| `text_system.rs` | `b39e05a8a8f7b36e13ffe1211d2c2993f68f2ce4cada87ad3414492a42e927b4` | `97a9bd817a51810d29bc92eff033e6310150e597d4d7e31cb8b79aa2db1413ea` |
| `text_system/line_layout.rs` | `a78d230d050bbf17bd968612da1d84f4c8aaefc39f82100e11b2b3a6990d45e4` | `7c46681f528ba2299f45f6390b1613c7c46d47c5b2de442e00f47ee4c62db1fc` |
| `text_system/line.rs` | `25bbf6a409ce8e70d1265a858f2d2abdbbc2c110d515abd31bb0a379cdce54e3` | `f8d61079493afe7815c5db027e3ce3d66f1c80bdd1c634f5a39f8814ca6a287b` |
| `elements/text.rs` | `3ffcc422356924ae4d6eef864421f56588be65cd931c8fe35d7e5e6438c23346` | `c0b02ee20e6500fb4d75a603dd9e5f0f0a7c5008d01da8c28e04e20b6178d912` |
| `text_system/logical_wrap.rs` | new | `eda32d8770882c2208f520b3d64640a2129b87eabaa6dfe6f8c39ea842df23b2` |

At the held receipt, the extracted tests and five Linux gates passed, but Linux Languages
captures clipped/misaligned Arabic/Hebrew paragraphs, including in an A/B baseline build. The owner initially held landing on 2026-10-25, then requested publication of all existing work
after review on October 26 ([review](../docs/REVIEW-2026-10-26.md)). That review included no platform
guard or logic revision; the Linux visual failure remained open at that source. Evidence and remaining
caret/selection, styling and platform limits are in the
[receipt](../docs/receipts/gpui-logical-wrap-20261025/README.md). Step 3 was subsequently approved. The [RTL continuation](../docs/receipts/rtl-completion-20261025/README.md)
adds visual-order normalisation before caching platform layouts and row-aware mapping in
weGPUI input. Fresh macOS/Linux ARM64 samples supersede the old sampled rendering failure;
the subsequent [shared text continuation](../docs/receipts/code-parity-20261026/README.md)
replaces the input-only shim with cached Unicode bidi geometry in text-owning layouts.

`WrappedLineLayout` now carries private logical-row metadata. Downstream GPUI code should
obtain layouts through the text-system factories; code that previously used an external struct
literal must instead initialize `WrappedLineLayout::default()` and assign its public fields.
These same-version local copies are a preview compatibility graph, not unchanged registry APIs.

## Shaped-row trailing spaces (owner-approved trial 2026-10-25)

The owner separately approved a trial of
[`gpui-trailing-space-wrap`](../docs/proposals/gpui-trailing-space-wrap/README.md).
One guard in `text_system/line_layout.rs::compute_wrap_boundaries` prevents an ASCII trailing
space from moving an otherwise fitting word onto the next row. Its SHA-256 at the publication review was
`cc576691d1835e5702c0b141e66a57d679eb98eddb0890b4671e78e963c98ef4`.
That trial left the four other prototype files and logical RTL/caret/selection implementation unchanged.
The table above identifies the original held prototype, not this later trial's whole-file hash.
Fresh Linux source/native evidence is separate from its supplied macOS captures. The later repository publication authorization is recorded in the review above. The subsequent
RTL continuation has step-3 approval and its own source hashes and current host evidence. No dependency or version changes were made.

## Historical CJK closing-mark trial (owner-approved 2026-10-26)

The owner separately approved [this bounded trial](../docs/proposals/gpui-cjk-closing/README.md).
That source’s `LineWrapper::is_word_char` closing-mark rule included twenty CJK closing
characters in `text_system/line_wrapper.rs`. The shared wrapper/truncator keeps those marks with
the preceding character; Panel now matches the sampled browser break and two-row ellipsis.
Seven existing and six new extracted cases pass, with three expected failures in the unchanged
control. Historical file SHA-256: `ba8c00801d7200854768a74e8234838be0bed7310acfabdb7585dca54a711e55`.
This is separate from both the step-1 script rule and the held logical-wrap prototype. It adds
no dependency and does not implement full Unicode line breaking, opening-mark constraints or
dictionary segmentation. Linux source/native evidence is in the
[October 26 receipt](../docs/receipts/weui-parity-20261026/README.md). The later
[publication review](../docs/REVIEW-2026-10-26.md) records repository publication authorization;
The later Unicode and ICU continuations replace this classifier; the historical hash above
does not describe current source. Full platform acceptance and package releases remain separate.

## Shared Unicode geometry and shaped clamping

The owner approved the direct locked `unicode-bidi 0.3.18` dependency. `text_system/bidi.rs`
resolves paragraph/row levels, caches visual clusters and logical ends, and maps caret, glyph
hits and discontiguous selections. `ShapedLine` and factory-built `WrappedLineLayout` expose
text-aware geometry; element `TextLayout` applies actual painted alignment. Its selection
rectangles also serve weGPUI inline links. Raw text-free `LineLayout` methods retain their
legacy low-level behavior and platform struct layout.

Descending logical glyph indices now select the correct color, background, underline and
strikethrough run. Decorations continue at the next row's aligned origin. Opening punctuation
shares one break predicate with wrapper, truncator and shaped rows; oversized RTL words break
at shaped clusters. Element line clamping checks real shaped rows, including ellipsis, so the
Panel description cannot silently clip its last characters after an estimated-width decision.

Five existing bidi tests moved into the extracted real-source harness alongside the four
layout cases. The packaged upstream test target remains unavailable; this is not a full
GPUI test-suite claim. The receipt records exact source, binaries, native probes and limits.

## Unicode break opportunities

The [Unicode continuation](../docs/receipts/unicode-parity-20261026/README.md) replaced the
script/punctuation allow-list across estimated wrapping, truncation, monotonic shaped rows
and logical RTL rows. The subsequent approved provider is ICU4X 2.4.0 with bundled Thai,
Lao, Khmer and Myanmar dictionaries and no LSTM feature or runtime network requirement.
Inline element placeholders map back to their original UTF-8 lengths. Normal opportunities
protect nonbreaking spaces, combining marks, emoji sequences and paired punctuation;
existing emergency wrapping remains when a whole word exceeds the available width.
Repeated glyph indices stay in one shaping cluster. Reordered combining glyphs use logical
row shaping. Browser dictionaries and font fallbacks can differ. Hyphenation and
font-provided ligature caret positions remain outside this implementation.

## iOS compatibility APIs

The core now supplies the lifecycle variant and focused input methods required by the
published `gpui-pre-ios 0.3.9` backend. Focus eligibility checks the registered field identity
against the window's current focus and fails closed. Selection fragments are optional for
other consumers; weGPUI input supplies document-ordered cluster fragments in window
coordinates. An empty vector means unavailable geometry. A disconnected scene can reconnect;
it is not a process-termination notification. These APIs preserve the single patched GPUI
package identity. The separately patched `gpui-pre-ios 0.3.9` copy maps the digit keyboard
request to a UIKit number pad with a Done accessory when the action is Done; other actions
use the default keyboard to retain their action key. See its
[`WEGPUI-PROVENANCE.md`](gpui_pre_ios/WEGPUI-PROVENANCE.md). Mobile runtime and accessibility
acceptance remain separate from successful linking, and iOS password entry remains disabled
pending secure native input verification.
