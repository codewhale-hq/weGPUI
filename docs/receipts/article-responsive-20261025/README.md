# Article image responsiveness — 2026-10-25

This is a focused comparison of the pinned article HTML/CSS and native image geometry/paint.
It does not qualify the complete upstream example, SDK interactions, screen readers or a whole
component acceptance row. Earlier article captures proved that image/text overlap was removed;
they did not establish responsive sizing.

## Reference proof

The verified WeUI 2.6.26 npm package's `src/example/article/article.html` contains two plain
`<img src="./images/pic_article.png" alt="配图">` elements without size attributes.
`src/style/widget/weui-page/weui-article.less` applies `max-width: 100%` to article descendants;
its image rule only sets `vertical-align: bottom`. The reset supplies no image size override.
The image's intrinsic dimensions are 690×389. Its SHA-256 is
`230d763f4eb8ad74559ca53601e140a057c02712484aa1e7379ed9709a0b0020`, matching the native asset.
Images consequently shrink proportionally with the article's content and retain their natural
690px width when the content is wider.

[verify.py](verify.py) renders that exact packaged HTML with exact `dist/style/weui.css` and
`dist/example/example.css`. It adds the router's article/visible classes, embeds the identical
image bytes and sets the theme explicitly. All HTTP/HTTPS requests are blocked; no external
SDK is loaded. Each run records the input hashes, browser bounding boxes, raw native/browser
screenshots, AT-SPI image bounds and image-background paint extents extracted from the captures.

## Preserved baseline failure

[baseline-20261025T094943Z-ZMMnFh](baseline-20261025T094943Z-ZMMnFh/) captures canonical
`49268de7bb78a69cb56b96f6cff02a2d9284e71d`; gallery binary SHA-256:
`0966e1f0bc0a12d4e1ab6b028ce8bd4559e90a9baf6aa050d865182fba19c83f`.
The capture sequence completed, then the geometry/paint validator failed as expected on the
responsive mismatch. Its failure output is retained as `validation.txt`.

| Viewport | Pinned browser image | Native painted image |
| --- | --- | --- |
| 375px | 326.98×184.34px | 327×184px, left 24px |
| 480px | 431.98×243.55px | 326×184px, centered at x77 |
| 700px | 651.97×367.56px | 326×184px, centered at x187 |
| 900px | 690×389px, left 24px | 326×184px, centered at x287 |

The PNG background detector records both adjacent images as one rectangle; the table divides
their combined 368px native height into the two identical 184px slots. At 900px the native
semantic image wrappers also incorrectly span the full 852px article content, rather than 690px.
Independently reviewed the baseline 480px native and browser captures; the reduced, centered
native images are plainly visible beside the full-content-width reference.

## Correction and verification

The gallery-only correction retains each named semantic wrapper but gives it width 100%, a
690px maximum and the 690:389 aspect ratio. Its image is absolute and fills the wrapper. The
image's intrinsic minimum size therefore cannot size the surrounding flow.

This follows the actual pinned layout path: GPUI `Style::to_taffy` forwards `aspect_ratio`;
Taffy 0.13 flexbox layout resolves that ratio through preferred/minimum/maximum sizes, filters
absolute children out of normal flex items, then lays them out after the container is sized.
It also avoids GPUI `Img::request_layout` replacing an automatic height with the intrinsic
389px when its width is relative.

The first corrected binary is SHA-256
`93921e856355d44c0a385000af847f773ab5ce3d30fc6e676ebb2549a49dbe6a`, reported by the integrating
agent as baseline `49268de` plus article and Swiped Escape changes. Its expected source snapshot
preserves baseline input.rs; runtime working-tree hashes are explicitly not build attestations,
because a later IME change was being developed concurrently.

| Run | Result |
| --- | --- |
| `corrected-light-20261025T100144Z-Y3Kfby` | All four captures completed, but editing the active Bash harness caused a trailing command-offset failure (exit 127). The run and `harness-status.txt` are retained. A separate strict paint-mask check also reported the 700px rounding difference below. |
| `corrected-light-clean-20261025T100503Z-3Wb9bC` | Capture/size validator exited 0. Independently reviewed all eight native/reference captures. Image sizes, left alignment and 690px cap were correct, but visual review found excess space after the images at 900px. The augmented `validation-with-gap.txt` correctly fails on that defect. |
| `corrected-dark-20261025T100625Z-oK0sO4` | Targeted 480px dark capture and size/paint validator exited 0. Both captures reviewed; full-width images have the reference proportions and do not overlap text. This does not qualify the failed 900px case. |

At 700px, integer layout-height rounding plus native `Contain` fitting produces a one-pixel
inset on either side of the second image. The exact-background-color mask is 650px wide inside
its 652px semantic box; the first image's mask is 652px wide. The validator retains one-pixel
semantic geometry tolerance and permits two pixels of total native paint-width difference.
The original stricter failure remains preserved; exact raster parity is not claimed.

At 900px, both native images are correctly 690×389, but the following heading was 207px below
the second image instead of the reference 24px. This was missed by image-size-only checks.
Taffy's flex-item size first derives the height from the parent's 852px width; its later width
clamp produces the correct child size while the earlier flex basis still contributes excess
parent height when shrinking is disabled. The approved follow-up also caps the images-only
parent paragraph at 690px, so each child starts with the used width before ratio resolution.
The validator now checks the following-heading gap against the reference at every width.

## Final result

[final-light-20261025T102304Z-FCiTHr](final-light-20261025T102304Z-FCiTHr/) and
[final-dark-20261025T102350Z-aGsmnk](final-dark-20261025T102350Z-aGsmnk/) both completed with
exit 0. Light validates all four widths; dark validates 480px. Both pass reference/native sizes,
aspect ratio, intrinsic cap, left alignment, following-heading gap and recorded paint tolerances.

| Viewport | Native slot width | First/second slot height | Following-heading gap |
| --- | --- | --- | --- |
| 375px | 327px | 184/185px | 24px |
| 480px | 432px | 244/243px | 24px |
| 700px | 652px | 368/367px | 24px |
| 900px | 690px | 389/389px | 24px |

Independently reviewed all ten final raw captures: native and static-CSS screenshots at each
light width, plus both 480px dark screenshots. Images grow with the content, retain the natural
width cap, stay left aligned and have no text overlap. The excess 900px blank space is gone;
its last image ends at physical y1248 and the heading begins at y1272. The disclosed 700px
native paint-edge rounding remains. Font/line-wrap differences elsewhere in the route are
outside this targeted result; full screenshot parity is not established.

Both final runs use the identical immutable gallery binary SHA-256
`74a0db8e27f5df841326f1dc4dfb896f2a40c43bf68785991de0991e1f831bf2`. The integrating agent
reported all five gates green with 47 passing tests/one ignored doctest on frozen production
source: baseline `49268de` plus current article, Swiped Escape, IME cancellation and
Semantics/Agree invalid-state changes. All current `src`/`examples`, Cargo and vendor inputs were
copied before capture; no newer proposed framework patch is part of this binary. The expected
build-source diff SHA-256 is `f4cafd5ff5299ffa0acec5b298d7b8cfcf7a88cd29ebd234acfd3d9d857c9dfc`;
the source hash-list SHA-256 is `1dff78a80b54a695292c5508a20a64ea06f3e38a69fa4ed5150ae136e4a45647`;
the vendor hash-list SHA-256 is `fccf0736b3fcb8cf12b928000592fae684fa58da8ac91f47e5862079a5c0ce42`.
Each run retains those lists, Cargo files, the expected compiled input source, the production
delta and separate runtime working-tree metadata. Runtime tree hashes are not substituted for
the integrating build's provenance.

Reproduce with `bash run.sh NAME` inside the configured `wegpui-198` container. The harness
copies the gallery binary immutably outside the repository before launching it, uses Xvfb :106
with Openbox, and captures at X11 scale 1. `ARTICLE_THEME=dark` selects dark; `ARTICLE_WIDTHS`
can select a narrower targeted set. Baseline tools: Rust 1.98.1, Chromium 156.0.8078.47 and
Playwright 1.64.1. Exact versions and hashes are retained per run. There is no Cargo invocation
in this runtime harness; the integrating agent owns the five required source gates.
