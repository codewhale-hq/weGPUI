# Upstream sources

- WeUI npm package: `weui@2.6.26`, SHA-1 `457eaa5a7ab9539b9944ff736bf0afffdd5d7baf`.
- Tokens: `dist/style/weui.css`, default `.wx-root,body` and explicit dark selectors. Care mode is excluded from these two palettes.
- Dimensions and behaviour: `src/style/widget/`, `src/style/base/variable/`, `src/example/`.
- GPUI: crates.io `gpui-pre =0.3.7`, `gpui-pre-platform =0.3.7`.
- Input base: `gpui-pre-0.3.7/examples/input.rs`.
- API examples read: `hello_world.rs`, `input.rs`, `popover.rs`, `animation.rs`, `focus_visible.rs`, `a11y.rs`.

Run `python3 scripts/extract_weui.py /path/to/extracted/weui/package` to reproduce token and icon extraction. The npm source package is intentionally outside the repository. Extraction does not execute upstream scripts.

`docs/tokens.json` preserves the raw CSS values, including expressions. Rust fields use Hsla for colours and f32 for unitless or pixel dimensions. Font-relative expressions retain their CSS form for reference.
