# Native dark state review — 2026-10-25

Reviewed all **72 native dark captures** through all 12 contact sheets below, then inspected
8 selected raw captures at their full 750×1624 size. No blank capture, missing requested
open overlay or confirmed dark-theme regression was found in this review. Capture success
and visual review are separate results: the sweep reported `72 ok, 0 failed`.

This is **native visual evidence, not upstream parity**. The pinned reference fetch stopped
at the blocked WeChat SDK request, so there are no fresh browser pairs. No component row
is accepted by these screenshots.

## Source and environment

Use [provenance.txt](provenance.txt), [source.diff](source.diff), [run.sh](run.sh),
[dark-console.log](dark-console.log) and [dark/sweep.log](dark/sweep.log) for the exact run.
The recorded source is `52650f3b830ed7e8d1cee7921d495bff06e0628d` plus the preserved diff
(SHA-256 `98d908b364bb881999d6177d2b072a366b8ad1ac39c7bdd412be4b847c02f79a`).
The gallery binary SHA-256 is
`d44112566e854f8d7d60f952fd4a963609153029bca750b24a3571751b98800d`.
Rust is 1.98.1; GPUI remains pinned to 0.3.9. The runtime is Debian 13 x86_64,
Xvfb `:95` at 1600×3600×24 with Openbox 3.6.1 and Vulkan lavapipe. Captures use
375×812 logical pixels at X11 scale 2, `--reduce-motion`, dark theme, default brand and
no care mode. The command was `DISPLAY=:95 scripts/linux/sweep.sh OUT --theme dark
--states --only gpui`, with the exact output directory and Python in `run.sh`.
This review covers that captured binary, preceding subsequent accessibility-text changes.

## Reviewed contact sheets

Each row contains six captures in left-to-right, top-to-bottom order. Images in these sheets
are native screenshots only; their layout does not imply native/reference pairing.

| Reviewed sheet | Captures |
| --- | --- |
| [dark-01.png](dark/sheets/dark-01.png) | `actionsheet-android`, `actionsheet-ios`, `button_bottom_fixed-agreed`, `dialog-android1`, `dialog-android2`, `dialog-ios1` |
| [dark-02.png](dark/sheets/dark-02.png) | `dialog-ios2`, `dialog-ios3`, `form_bottom_fixed-checked`, `form_bottom_fixed-dialog`, `form_bottom_fixed-toast`, `form_checkbox-checked` |
| [dark-03.png](dark/sheets/dark-03.png) | `form_checkbox-toast`, `form_input_status-amount`, `form_input_status-error`, `form_input_status-toast`, `form_page-filled`, `form_page-toast` |
| [dark-04.png](dark/sheets/dark-04.png) | `form_page_old-filled`, `form_primary-filled`, `form_primary-switch`, `form_radio-checked`, `form_radio-toast`, `form_select-toast` |
| [dark-05.png](dark/sheets/dark-05.png) | `form_select_primary-select1`, `form_select_primary-select2`, `form_select_primary-select3`, `form_switch-checked`, `form_switch-toast`, `form_vcode-checked` |
| [dark-06.png](dark/sheets/dark-06.png) | `form_vcode-countdown`, `form_vcode-dialog`, `form_vcode-toast`, `form_vcode-warn`, `gallery-closed`, `half-screen-dialog-1` |
| [dark-07.png](dark/sheets/dark-07.png) | `half-screen-dialog-2`, `half-screen-dialog-3`, `half-screen-dialog-4`, `half-screen-dialog-5`, `home-category-1`, `information-bar-closed` |
| [dark-08.png](dark/sheets/dark-08.png) | `layers-3d`, `layers-content`, `layers-mask`, `layers-navigation`, `layers-popout`, `list-closed` |
| [dark-09.png](dark/sheets/dark-09.png) | `navbar-tab2`, `navbar-tab3`, `picker-cascade`, `picker-date`, `picker-multi`, `picker-single` |
| [dark-10.png](dark/sheets/dark-10.png) | `progress-uploading`, `searchbar_filled-focused`, `searchbar_filled-typed`, `searchbar_grey-focused`, `searchbar_grey-typed`, `searchbar_outlined-filled` |
| [dark-11.png](dark/sheets/dark-11.png) | `searchbar_outlined-focused`, `searchbar_outlined-typed`, `slider-value80`, `tabbar-tab2`, `tabbar-tab3`, `toast-fail` |
| [dark-12.png](dark/sheets/dark-12.png) | `toast-loading`, `toast-longtext`, `toast-success`, `toast-text`, `uploader-preview`, `uploader-viewer` |

## Full-size follow-up and observations

- [form_switch-toast](dark/raw/form_switch-toast-dark-gpui.png): success toast and
  `已完成` are visible over the switch rows. An initial reduced-sheet scan missed the toast;
  both the raw image and a second sheet inspection resolve that suspicion. It is not a
  missing-state failure.
- [form_vcode-countdown](dark/raw/form_vcode-countdown-dark-gpui.png): the disabled
  `重新发送(60s)` button wraps below the input. Both controls remain legible. The source uses
  a wrapping Vcode row; exact reference wrapping and countdown timing were not established.
- [form_page-filled](dark/raw/form_page-filled-dark-gpui.png): `verification address`
  wraps within “verification” into three lines at the narrow label width. The field and
  footer remain visible without overlap. Upstream has `lang="en"` and the `.hyphens()`
  label mixin; the exact hyphenation/font comparison remains open without a browser pair.
- [form_input_status-error](dark/raw/form_input_status-error-dark-gpui.png): the red
  error banner and red card value are visible on dark surfaces. The confirm button extends
  below the initial viewport. This is a long-page crop, so offscreen content and focus reveal
  cannot be judged from this image.
- [half-screen-dialog-2](dark/raw/half-screen-dialog-2-dark-gpui.png): long body text
  fades behind the bottom action area; both actions remain clear. Scrolling and focus reveal
  were not driven by the sweep.
- [picker-single](dark/raw/picker-single-dark-gpui.png): selected `的士票` is bright on
  a darker selection band; nearby and disabled options fade. The confirm button is visible.
  This does not prove keyboard selection or disabled-option accessibility semantics.
- [layers-content](dark/raw/layers-content-dark-gpui.png): the white inset article is
  the embedded `layers_content.png` illustration, not an unthemed native article surface.
  The navigation/popout illustrations likewise retain their embedded light content.
- [tabbar-tab2](dark/raw/tabbar-tab2-dark-gpui.png): the second tab label is green,
  tab badges remain visible and the fixed bar stays within the viewport. The placeholder
  icon artwork is visible; this image does not prove arrow-key interaction.

Across the sheets, requested action sheets, five dialog variants, half-screen sheets, select
pickers and toast variants are visible with dark surfaces and dimmed backgrounds where
applicable. Checked states, red warning states, filled search results, selected tabs and the
slider value 80 are distinguishable. No systematic light background appeared outside embedded
illustrations/image-viewer assets. Low-emphasis/disabled text is deliberately subdued in several
controls; this review did not measure contrast ratios or establish accessibility acceptance.

## Limits

Only registered startup states and the initial viewport were captured. No pointer, keyboard,
callback, focus, live-theme change, timing, animation, accessibility-tree or screen-reader
acceptance follows from this review. Additional public variants and custom states are outside
the 72-state registry. Light/care/custom-brand captures, dark+care/brand combinations, macOS
and Windows need their own evidence. The absence of a new confirmed visual defect here does
not establish complete WeUI appearance or behavior.
