# Care and custom-brand native visual review — 2026-10-25

Reviewed all 72 care-light and all 72 custom-brand-light captures (144 total). Each
mode's twelve contact sheets were opened with `view_image`; twelve selected raw
750×1624 captures were also opened for full-size inspection. This is a review of
native rendering, **not upstream parity or complete component acceptance**.

The captured artifact/source is the exact one in `provenance.txt` and
`source.diff`: parent `52650f3`, gallery SHA-256
`d44112566e854f8d7d60f952fd4a963609153029bca750b24a3571751b98800d`.
The later accessibility text changes require their own source/runtime evidence;
these screenshots must retain their recorded provenance.

## Findings

- No blank/error window, missing expected modal/toast/picker state, displaced
  footer operation, or obvious text spilling outside its container was found in
  this review. The 72/72 capture-tool successes per mode are separately recorded
  in each `sweep.log`; they do not themselves establish visual correctness.
- Brand `7C3AED` visibly reaches primary buttons, checked agreement/checkbox/radio
  indicators, switches, progress/slider tracks, selected tab labels and search-bar
  keywords. Neutral surfaces and warning red remain distinct. Link blue and
  historical raster illustrations retain their own colours.
- Care-light shows darker primary green and warning red than the ordinary light
  custom-brand run, with visible enabled/disabled distinctions. This review did
  not measure contrast ratios or qualify arbitrary brand colours.
- Full-size inspection resolved an apparent contact-sheet switch-label clipping
  concern: `form_switch:toast` intentionally places the success toast over that
  label. It is not evidence of a switch-layout defect.
- `form_page:filled` wraps the fixed-width English label `verification address`
  across three lines. `form_vcode:countdown` wraps the wider resend button onto
  another line. Both remain inside their rows; the source explicitly uses a
  fixed label width and wrapping vcode layout. Their fidelity still needs the
  pinned browser comparison, so this review does not assert those line breaks
  match upstream.
- The long half-screen-dialog body fades behind its fixed operation region;
  buttons remain visible. The long toast message fits its panel on two lines.
  Long form pages continue below this viewport; a cropped bottom action in the
  first screenshot is not proof of broken scrolling.

## Limits

The reference setup failed downloading the pinned WeChat SDK from
`res.wx.qq.com` (proxy HTTP 403; preserved by the root continuation receipt).
There are no upstream screenshots in this run. Care-dark, custom-brand-dark,
other viewport sizes, animation timing, keyboard/focus interactions, callbacks,
accessibility and screen-reader behaviour were not assessed by this visual
review. Registered gallery states do not cover every reusable component state.
No ACCEPTANCE row is accepted by this review.

## Reviewed sheets

All files `care/sheets/care-01.png` through `care/sheets/care-12.png` and
`brand/sheets/brand-01.png` through `brand/sheets/brand-12.png` were opened.
The following table lists every exact sheet filename:

| Care-light | Custom-brand-light |
| --- | --- |
| `care/sheets/care-01.png` | `brand/sheets/brand-01.png` |
| `care/sheets/care-02.png` | `brand/sheets/brand-02.png` |
| `care/sheets/care-03.png` | `brand/sheets/brand-03.png` |
| `care/sheets/care-04.png` | `brand/sheets/brand-04.png` |
| `care/sheets/care-05.png` | `brand/sheets/brand-05.png` |
| `care/sheets/care-06.png` | `brand/sheets/brand-06.png` |
| `care/sheets/care-07.png` | `brand/sheets/brand-07.png` |
| `care/sheets/care-08.png` | `brand/sheets/brand-08.png` |
| `care/sheets/care-09.png` | `brand/sheets/brand-09.png` |
| `care/sheets/care-10.png` | `brand/sheets/brand-10.png` |
| `care/sheets/care-11.png` | `brand/sheets/brand-11.png` |
| `care/sheets/care-12.png` | `brand/sheets/brand-12.png` |

## Raw files opened at full size

- `care/raw/form_switch-toast-light-gpui.png`
- `brand/raw/form_switch-toast-light-gpui.png`
- `care/raw/form_vcode-countdown-light-gpui.png`
- `care/raw/form_page-filled-light-gpui.png`
- `brand/raw/searchbar_filled-focused-light-gpui.png`
- `brand/raw/searchbar_grey-focused-light-gpui.png`
- `brand/raw/button_bottom_fixed-agreed-light-gpui.png`
- `care/raw/form_input_status-error-light-gpui.png`
- `care/raw/half-screen-dialog-2-light-gpui.png`
- `brand/raw/half-screen-dialog-2-light-gpui.png`
- `care/raw/toast-longtext-light-gpui.png`
- `brand/raw/toast-longtext-light-gpui.png`

## Complete reviewed state manifest

Both `care/raw/` and `brand/raw/` contain the same following 72 basenames;
each was reviewed in its corresponding contact sheet (six entries per sheet,
ordered as below). The full-size subset is listed above.

```text
actionsheet-android-light-gpui.png
actionsheet-ios-light-gpui.png
button_bottom_fixed-agreed-light-gpui.png
dialog-android1-light-gpui.png
dialog-android2-light-gpui.png
dialog-ios1-light-gpui.png
dialog-ios2-light-gpui.png
dialog-ios3-light-gpui.png
form_bottom_fixed-checked-light-gpui.png
form_bottom_fixed-dialog-light-gpui.png
form_bottom_fixed-toast-light-gpui.png
form_checkbox-checked-light-gpui.png
form_checkbox-toast-light-gpui.png
form_input_status-amount-light-gpui.png
form_input_status-error-light-gpui.png
form_input_status-toast-light-gpui.png
form_page-filled-light-gpui.png
form_page-toast-light-gpui.png
form_page_old-filled-light-gpui.png
form_primary-filled-light-gpui.png
form_primary-switch-light-gpui.png
form_radio-checked-light-gpui.png
form_radio-toast-light-gpui.png
form_select-toast-light-gpui.png
form_select_primary-select1-light-gpui.png
form_select_primary-select2-light-gpui.png
form_select_primary-select3-light-gpui.png
form_switch-checked-light-gpui.png
form_switch-toast-light-gpui.png
form_vcode-checked-light-gpui.png
form_vcode-countdown-light-gpui.png
form_vcode-dialog-light-gpui.png
form_vcode-toast-light-gpui.png
form_vcode-warn-light-gpui.png
gallery-closed-light-gpui.png
half-screen-dialog-1-light-gpui.png
half-screen-dialog-2-light-gpui.png
half-screen-dialog-3-light-gpui.png
half-screen-dialog-4-light-gpui.png
half-screen-dialog-5-light-gpui.png
home-category-1-light-gpui.png
information-bar-closed-light-gpui.png
layers-3d-light-gpui.png
layers-content-light-gpui.png
layers-mask-light-gpui.png
layers-navigation-light-gpui.png
layers-popout-light-gpui.png
list-closed-light-gpui.png
navbar-tab2-light-gpui.png
navbar-tab3-light-gpui.png
picker-cascade-light-gpui.png
picker-date-light-gpui.png
picker-multi-light-gpui.png
picker-single-light-gpui.png
progress-uploading-light-gpui.png
searchbar_filled-focused-light-gpui.png
searchbar_filled-typed-light-gpui.png
searchbar_grey-focused-light-gpui.png
searchbar_grey-typed-light-gpui.png
searchbar_outlined-filled-light-gpui.png
searchbar_outlined-focused-light-gpui.png
searchbar_outlined-typed-light-gpui.png
slider-value80-light-gpui.png
tabbar-tab2-light-gpui.png
tabbar-tab3-light-gpui.png
toast-fail-light-gpui.png
toast-loading-light-gpui.png
toast-longtext-light-gpui.png
toast-success-light-gpui.png
toast-text-light-gpui.png
uploader-preview-light-gpui.png
uploader-viewer-light-gpui.png
```
