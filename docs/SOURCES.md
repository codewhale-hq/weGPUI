# Upstream sources

- WeUI npm package: `weui@2.6.26`, SHA-1 `457eaa5a7ab9539b9944ff736bf0afffdd5d7baf`.
- Tarball: `https://registry.npmjs.org/weui/-/weui-2.6.26.tgz`, SHA-256 `f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435`.
- Example runtime: `dist/example/index.html` loads external weui.js 1.2.17 and WeChat JS SDK 1.0.0 from `res.wx.qq.com`; the CSS tarball hash alone does not pin those responses. `scripts/weui_upstream.py` pins them:
  - weui.js: npm `weui.js@1.2.17`, tarball SHA-256 `62aa1d4538bb81ac5d77aeee2b3cdbe03bfca1d28e619ed0641caa74e791182a` (SHA-1 `2cb664240a957cc5074ee9501e71592c2501cee9`). Its `dist/weui.min.js` (SHA-256 `87707d2d625c95f247e1957d5eee8c43bbe0c056983336990868f325d9b0c6fe`) was byte-identical to the CDN copy on 2026-10-25.
  - WeChat JS SDK: `https://res.wx.qq.com/open/js/jweixin-1.0.0.js`, SHA-256 `9e1a67f12789a5fde0ade6157154dc9bdfd17a5399b654f73740e0edaa95d94e` as fetched on 2026-10-25 (no versioned package; the hash pins that response).
- Tokens: `dist/style/weui.css`, default `.wx-root,body` and explicit dark selectors. Four palettes are generated: light, dark, care-light and care-dark.
- Dimensions and behaviour: `src/style/widget/`, `src/style/base/variable/`, `src/example/`.
- GPUI: crates.io `gpui-pre =0.3.7` (snapshot of `zed@1a28cff`), `gpui-pre-platform =0.3.7`. Platform feasibility also read upstream Zed `main` at `347f171a3fd00a3b672382d78348b37bd969d17c` (2026-10-24) and crates.io `gpui-pre-ios 0.3.9` and `gpui-pre-mobile 0.1.0`; see [PLATFORMS](PLATFORMS.md).
- Contributor Covenant 2.1: fetched from contributor-covenant.org on 2026-10-25 for `CODE_OF_CONDUCT.md`.
- Input base: `gpui-pre-0.3.7/examples/input.rs`.
- API examples read: `hello_world.rs`, `input.rs`, `popover.rs`, `animation.rs`, `focus_visible.rs`, `a11y.rs`.

Run `python3 -I scripts/weui_upstream.py fetch`, then `python3 -I scripts/extract_weui.py "${CARGO_TARGET_DIR:-target}/weui-upstream/weui/package"`, to reproduce token, icon, palette and asset-table generation (including their rustdoc). The npm source package is intentionally outside the repository. Extraction does not execute upstream scripts. `assets/loading.svg` is extracted from the pinned loading SVG, with rotation handled by GPUI. Keep the separately authored `assets/loading-primary.svg`, which represents the upstream CSS-ring geometry and is included in the generated asset table. On 2026-10-25 (Linux, Pillow 12.3.0) regeneration reproduced every generated file except the eight `*-dark.png` images, which differ in PNG encoding bytes but have identical pixels.

`docs/tokens.json` preserves the raw CSS values, including expressions. Rust fields use Hsla for colours and f32 for unitless or pixel dimensions. Font-relative expressions retain their CSS form for reference.
