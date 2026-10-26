# Unicode wrapping and website continuation

Qualified implementation: [`0cf3f19`](https://github.com/codewhale-hq/weGPUI/commit/0cf3f1933a17d8f3d55fe22b8e990f797265a7ac).
The owner requested continued parity work, review, main pushes and website updates, and approved
the existing locked `unicode-linebreak 0.1.5` direct edge. Windows stays build/tests only;
VoiceOver remains off. This receipt closes reproduced defects, without declaring full parity.

## Changes and review

The GPUI script/punctuation allow-list is replaced by Unicode 15.0 break opportunities in
estimated fragment wrapping, estimated truncation, monotonic shaped rows and logical RTL rows.
Inline object replacement characters map back to their original UTF-8 lengths. Normal break
opportunities protect nonbreaking spaces/hyphens, combining marks, emoji ZWJ/flag sequences,
script words and paired punctuation. Existing emergency wrapping still splits oversized words.
Dictionary segmentation, hyphenation, newer Unicode revisions and exact ligature caret tables
are not implemented. The Cargo lockfile adds one direct edge; no package/version/checksum set
changes. Existing bidi/clock versions and the four required consumer root patches remain.

The review checked logical byte offsets across fragment/object boundaries, glyph cluster
indices, shared break decisions, empty input, protected punctuation and negative controls.
Existing test functions were updated; no new test functions were introduced. Extracted vendor
contracts run real source with the two locked Unicode crates. The packaged GPUI test target
still lacks its upstream assets/development graph; these checks are supplemental.

The website now validates the selected receipt manifests and image hashes before exposing
five recent comparison pairs. Each native image has its own source, host and viewport, while
other images keep their historical macOS identity. Platform claims reflect the recorded
macOS/Linux/Windows checks. The complete recorded 1,056-pair batch keeps its original source;
it is not relabelled as this executable. The latest continuation is linked separately.

Review also reproduced a palette-switch defect: rebuilding the theme page destroyed the live
iframe and reset its controls. Palette changes now update swatches, code and iframe settings
in place. Existing browser checks verify the same iframe, checkbox draft and keyboard focus
at 375/390/768/1280px under both root and project mounts. No browser asset/library version changed.

## Captures and typography

The two [capture manifests](captures/) record the pre-commit base and exact Rust/Cargo hashes.
[Source identity](source-identity.json) verifies those bytes against the qualified commit; both
widths use the same frozen Linux binary. All **112 pairs / 224 PNG hashes** pass validation:
seven default scenes × eight appearances × two widths (320 and 375). These are a targeted
extension, not another full 1,056-pair capture. Raw corpora remain in the local full-parity
artifact folder. This receipt retains manifests, result rows, selected original PNGs and
[review sheets](review/). Earlier full matrices retain their own source identity.

At 320px, the form description now has four rows, matching the reference; the earlier source
used three. At 375px, it has the reference's three rows. See [the form comparison](review/forms.png).
The 17px article's eight normalized line endings at a 327px content width match exactly;
[the measurement](fonts/article-line-comparison.json) retains native and browser strings.
The browser uses Noto Sans/CJK SC. Although GPUI requests IBM Plex Sans, its actual resolved
primary font here is also Noto Sans, confirmed by the retained native probe. No font substitution
was made. Renderer baseline/raster differences remain; this is not pixel-identity acceptance.

macOS builds the same implementation and renders the [narrow form](macos/macos-form320.png)
and [shared Arabic/Hebrew/English paragraphs](macos/macos-generic-rtl.png). Its platform font
has different metrics from the Linux comparison. Unicode clipboard paste readback includes
Arabic, Hebrew, combining accent and ZWJ emoji; undo restores the exact preceding draft.
The CUA key-typing attempt dropped non-ASCII characters; clipboard paste then succeeded.
That attempt is retained as a tool/input-path limitation and does not qualify a real OS IME.
AX text is observation evidence, not a reader result. The owned sample was closed.

## Verification

Final source gates and consumer results are retained under [checks](checks/). macOS all-target
build, 54 library/three gallery tests, strict Clippy and strict rustdoc pass on frozen source.
Linux all-target build, the same 54/three tests, strict Clippy and strict rustdoc also pass.
The initial Linux attempt stopped before formatting because rustfmt was absent; installing
the official toolchain components resolved that tooling failure. [Final hosted CI](https://github.com/codewhale-hq/weGPUI/actions/runs/40281279626)
passes at `b368c94`, including Linux stable/MSRV, macOS, Windows MSVC all-target
build/library/gallery tests/Clippy, and mobile library-only type checks. Windows visible UI
was not unlocked or driven. The separate experimental iOS bundle still fails; its diagnostics
remain distinct from the passing library type checks. Its six `gpui-pre-ios` errors are
missing lifecycle/input-handler APIs in the published backend/core pair; [stderr](checks/ios-bundle.txt)
retains them. [CodeQL](https://github.com/codewhale-hq/weGPUI/actions/runs/40281278870) also passes.

The focused harnesses pass seven word cases, 16 logical and nine shared layout/bidi cases,
13 punctuation cases and eight trailing-space plus five shared bidi cases. The deliberate
negative controls fail as expected. Five website build/inventory regressions and 264 recorded
state/theme views (528 referenced PNGs) pass, including new per-image provenance assertions.
The isolated site contains 538 PNG assets: historical 528 plus ten recent replacement assets.

The initial interactive sweep passed root and failed project-prefix on an aborted iframe
request during palette replacement. The first focus assertion in the corrected check lacked
an explicit focus action; that attempt is retained too. The corrected palette/layout checks
pass at both mounts. The final complete sweep passes all 42 checks, with 360 route/viewport observations,
no changed build bytes during execution and both mounts passing. Results are retained under
[website/final](website/final/). Historical
failed attempts are preserved under [website](website/); they are not promoted to passes.

The fresh preview consumer initially failed because this container mixed a newer FreeType
pkg-config file with an older bzip2 development package lacking `bzip2.pc`. Updating the
official bzip2 development package resolves that system prerequisite; the failed attempt
is retained. The corrected fresh consumer now builds locked/offline successfully. The
verified source bundle hash is `c76e32cbd496f0a66078d5b8a8d5852db76580a6f1801f728cbb1f741e129f21`.
This is separate from the cached native-source build results.

## Remaining acceptance

Full parity is still open: complete real IME, native-speaker/reader and complex-script/font
coverage; Windows visible UI outside this run's scope; Linux hardware/windowing diversity;
and distributable installation. Mobile remains experimental with backend gaps. Source,
capture, browser, hosted build and actual-reader evidence are distinct. See
[acceptance](../../ACCEPTANCE.md) and [release readiness](../../RELEASE-READINESS.md).

The [website update](../pages-update-20261026/README.md) is published at `b368c94`;
live-origin recorded views and interactive checks pass. Its deployment evidence is separate.
The dictionary dependency decision is prepared in the [next proposal](../../proposals/gpui-dictionary-break/README.md);
the owner subsequently approved the dictionaries and dependencies needed to continue. That
next implementation is separately qualified and does not relabel this receipt.
No crate publication, signing, billing, paid provider operation or support claim is made.
