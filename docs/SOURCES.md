# Upstream sources

- WeUI npm package: `weui@2.6.26`, SHA-1 `457eaa5a7ab9539b9944ff736bf0afffdd5d7baf`.
- Tarball: `https://registry.npmjs.org/weui/-/weui-2.6.26.tgz`, SHA-256 `f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435`.
- Example runtime: `dist/example/index.html` loads external weui.js 1.2.17 and WeChat JS SDK 1.0.0; the CSS tarball hash alone does not pin those responses.
- Tokens: `dist/style/weui.css`, default `.wx-root,body` and explicit dark selectors. Four palettes are generated: light, dark, care-light and care-dark.
- Dimensions and behaviour: `src/style/widget/`, `src/style/base/variable/`, `src/example/`.
- GPUI: crates.io `gpui-pre =0.3.7`, `gpui-pre-platform =0.3.7`.
- Input base: `gpui-pre-0.3.7/examples/input.rs`.
- API examples read: `hello_world.rs`, `input.rs`, `popover.rs`, `animation.rs`, `focus_visible.rs`, `a11y.rs`.

Run `python3 scripts/extract_weui.py /path/to/extracted/weui/package` to reproduce token and icon extraction. The npm source package is intentionally outside the repository. Extraction does not execute upstream scripts.

`docs/tokens.json` preserves the raw CSS values, including expressions. Rust fields use Hsla for colours and f32 for unitless or pixel dimensions. Font-relative expressions retain their CSS form for reference.
