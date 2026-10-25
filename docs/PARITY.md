# Visual parity

Historical evidence for the candidate at commit `2c8ed10` (gallery binary built from that source). Every gallery route and every registered capture state was captured natively and beside the official WeUI 2.6.26 example, in light and dark: **264 pairs, 528 PNG files** in `docs/parity/`, plus brand and care-mode samples. Each pair was inspected (light at pair scale, dark at contact-sheet scale, the 40 largest pixel-difference scores individually).

## Independent review qualification

The [2026-10-24 review](REVIEW-2026-10-24.md) rechecked 28 pairs across 13 routes. The table below preserves earlier verdicts and must not be read as independent proof of every state. Additional form-description wrapping and native-window clipping differences were found. Action-sheet corners were corrected after the baseline captures. Complete behaviour, animation and accessibility parity remain unverified.

## Linux and tooling update (2026-10-25)

The upstream page is now fetched, verified and served by `scripts/weui_upstream.py`, and `scripts/capture.py` can capture natively on X11 Linux ([PARITY-TOOLING](PARITY-TOOLING.md)). One Linux pair (`dialog:ios1`, light) was produced to validate the pipeline (`receipts/takeover-20261025/linux-x11-capture-pair-dialog-ios1-light.png`). A Linux sweep then captured all 60 routes in their default state, light theme only (native X11 at 2×, Chromium 1243 against the verified local page; 60 of 60 captures succeeded in 3 min 7 s). Ten contact sheets are in `receipts/takeover-20261025/linux-sweep/`. At contact-sheet scale every route's layout, colours and controls correspond. Differences seen: `layers` is still flat natively at the 1 s settle while upstream is mid 3D transition (the known start-up timing difference); `article` paragraphs wrap one line longer natively, shifting the images about 13 pt; and the `form_page`/`form_page_old` label hyphenation differs as on macOS. States, dark theme, care mode and scrolled regions were not swept on Linux. Linux pairs use Noto CJK fonts on both sides and are not comparisons against the macOS images below. The table below was not recaptured. Behaviour fixes made after the baseline (Presence transitions, LinkText focus, uploader chooser) do not change these static first-frame captures, except where noted in [STATUS](STATUS.md).

## Method

- Native: `scripts/capture.py` launches `gallery --page ROUTE [--state S] --theme T --size 375x812 --reduce-motion`, captures the window with `screencapture`, crops the 375×812 pt content at @2x and converts the display profile to sRGB.
- Official: the pinned package served at `http://127.0.0.1:7193/dist/example/index.html#ROUTE`, Chromium (Playwright) 375×812 viewport, device scale 2, `prefers-color-scheme` = theme; states replay the JavaScript registered in `scripts/parity/*.json`.
- Viewport only: long pages are compared over their first 812 pt. Brand: WeUI default unless named. Care mode off unless named.
- File names: `<route>[-<state>]-<light|dark>-<gpui|weui>.png`.

## Differences common to every pair

- Glyph rasterisation: CoreText via GPUI vs Chromium/Skia; system UI fonts on both sides, slightly different hinting and line-box rounding (≤1 pt).
- Native screenshots include macOS bottom-corner clipping; browser viewports are rectangular.
- Spinners and caret blink are frozen by reduced motion natively; Chromium frames are taken at an arbitrary animation phase.

## Results by route

Verdict “Matches” means no layout, spacing, size, colour or content difference beyond the common items above.

| Route | Verdict and residual differences | Captures |
| --- | --- | --- |
| `home` | Matches. | default ([light](parity/home-light-gpui.png) / [weui](parity/home-light-weui.png); [dark](parity/home-dark-gpui.png) / [weui](parity/home-dark-weui.png)), `category-1` |
| `layers` | Differs (adaptation). Default route: official frame is captured mid-transition (pictures still on the tilted layers at 1.2 s); native frame is the settled state with labels. `--state 3d` compares settled states: geometry, colours and order match; labels are drawn upright on the projected planes because GPUI has no 3D text transform. In one of four captures the native default page was still flat after 3 s (start-up timer); not reproduced since. | default ([light](parity/layers-light-gpui.png) / [weui](parity/layers-light-weui.png); [dark](parity/layers-dark-gpui.png) / [weui](parity/layers-dark-weui.png)), `popout`, `mask`, `navigation`, `content`, `3d` |
| `button` | Matches. | default ([light](parity/button-light-gpui.png) / [weui](parity/button-light-weui.png); [dark](parity/button-dark-gpui.png) / [weui](parity/button-dark-weui.png)) |
| `button_default` | Matches. | default ([light](parity/button_default-light-gpui.png) / [weui](parity/button_default-light-weui.png); [dark](parity/button_default-dark-gpui.png) / [weui](parity/button_default-dark-weui.png)) |
| `button_bottom_fixed` | Matches layout and controls. Lorem paragraph breaks one line differently: GPUI breaks before a lone “?” where Chromium keeps it on the line. | default ([light](parity/button_bottom_fixed-light-gpui.png) / [weui](parity/button_bottom_fixed-light-weui.png); [dark](parity/button_bottom_fixed-dark-gpui.png) / [weui](parity/button_bottom_fixed-dark-weui.png)), `agreed` |
| `list` | Matches. | default ([light](parity/list-light-gpui.png) / [weui](parity/list-light-weui.png); [dark](parity/list-dark-gpui.png) / [weui](parity/list-dark-weui.png)), `closed` |
| `input` | Matches. The date/time rows mimic Chromium’s native `<input type=date>` text; the calendar glyph is a WeUI-style icon, not Chromium’s control. | default ([light](parity/input-light-gpui.png) / [weui](parity/input-light-weui.png); [dark](parity/input-dark-gpui.png) / [weui](parity/input-dark-weui.png)) |
| `form` | Matches. | default ([light](parity/form-light-gpui.png) / [weui](parity/form-light-weui.png); [dark](parity/form-dark-gpui.png) / [weui](parity/form-dark-weui.png)) |
| `form_primary` | Differs: description wrapping and CJK/slash spacing differ in the independent sample. | default ([light](parity/form_primary-light-gpui.png) / [weui](parity/form_primary-light-weui.png); [dark](parity/form_primary-dark-gpui.png) / [weui](parity/form_primary-dark-weui.png)), `switch`, `filled` |
| `form_page` | Matches. The “verification address” label hyphenates as “verificati-on” natively vs “verifica-tion” (CSS `hyphens`). | default ([light](parity/form_page-light-gpui.png) / [weui](parity/form_page-light-weui.png); [dark](parity/form_page-dark-gpui.png) / [weui](parity/form_page-dark-weui.png)), `filled`, `toast` |
| `form_page_old` | Matches; same hyphenation difference as form_page. | default ([light](parity/form_page_old-light-gpui.png) / [weui](parity/form_page_old-light-weui.png); [dark](parity/form_page_old-dark-gpui.png) / [weui](parity/form_page_old-dark-weui.png)), `filled` |
| `form_input_status` | Matches layout, colours and states. macOS text shaping inserts visible spacing at CJK/Latin boundaries (“置顶 tipsbar”, “16 位”); source strings are identical to upstream. | default ([light](parity/form_input_status-light-gpui.png) / [weui](parity/form_input_status-light-weui.png); [dark](parity/form_input_status-dark-gpui.png) / [weui](parity/form_input_status-dark-weui.png)), `error`, `amount`, `toast` |
| `form_vcode` | Matches. Upstream bug: two buttons share `id=showTooltips`, so the visible 确定 never shows the toast on the official page; the port wires it as the script intends, and the `toast` replay shows the example’s own `#js_toast` markup. | default ([light](parity/form_vcode-light-gpui.png) / [weui](parity/form_vcode-light-weui.png); [dark](parity/form_vcode-dark-gpui.png) / [weui](parity/form_vcode-dark-weui.png)), `dialog`, `checked`, `toast`, `warn`, `countdown` |
| `form_bottom_fixed` | Matches. | default ([light](parity/form_bottom_fixed-light-gpui.png) / [weui](parity/form_bottom_fixed-light-weui.png); [dark](parity/form_bottom_fixed-dark-gpui.png) / [weui](parity/form_bottom_fixed-dark-weui.png)), `dialog`, `checked`, `toast` |
| `form_access` | Matches. | default ([light](parity/form_access-light-gpui.png) / [weui](parity/form_access-light-weui.png); [dark](parity/form_access-dark-gpui.png) / [weui](parity/form_access-dark-weui.png)) |
| `form_checkbox` | Matches. | default ([light](parity/form_checkbox-light-gpui.png) / [weui](parity/form_checkbox-light-weui.png); [dark](parity/form_checkbox-dark-gpui.png) / [weui](parity/form_checkbox-dark-weui.png)), `toast`, `checked` |
| `form_radio` | Matches. | default ([light](parity/form_radio-light-gpui.png) / [weui](parity/form_radio-light-weui.png); [dark](parity/form_radio-dark-gpui.png) / [weui](parity/form_radio-dark-weui.png)), `toast`, `checked` |
| `form_switch` | Matches. | default ([light](parity/form_switch-light-gpui.png) / [weui](parity/form_switch-light-weui.png); [dark](parity/form_switch-dark-gpui.png) / [weui](parity/form_switch-dark-weui.png)), `toast`, `checked` |
| `form_select` | Matches. | default ([light](parity/form_select-light-gpui.png) / [weui](parity/form_select-light-weui.png); [dark](parity/form_select-dark-gpui.png) / [weui](parity/form_select-dark-weui.png)), `toast` |
| `form_select_primary` | Matches. | default ([light](parity/form_select_primary-light-gpui.png) / [weui](parity/form_select_primary-light-weui.png); [dark](parity/form_select_primary-dark-gpui.png) / [weui](parity/form_select_primary-dark-weui.png)), `select1`, `select2`, `select3` |
| `form_textarea` | Matches. | default ([light](parity/form_textarea-light-gpui.png) / [weui](parity/form_textarea-light-weui.png); [dark](parity/form_textarea-dark-gpui.png) / [weui](parity/form_textarea-dark-weui.png)) |
| `form_vertical` | Matches. | default ([light](parity/form_vertical-light-gpui.png) / [weui](parity/form_vertical-light-weui.png); [dark](parity/form_vertical-dark-gpui.png) / [weui](parity/form_vertical-dark-weui.png)) |
| `toast` | Matches. | default ([light](parity/toast-light-gpui.png) / [weui](parity/toast-light-weui.png); [dark](parity/toast-dark-gpui.png) / [weui](parity/toast-dark-weui.png)), `success`, `fail`, `longtext`, `loading`, `text` |
| `dialog` | Matches. | default ([light](parity/dialog-light-gpui.png) / [weui](parity/dialog-light-weui.png); [dark](parity/dialog-dark-gpui.png) / [weui](parity/dialog-dark-weui.png)), `ios1`, `ios2`, `ios3`, `android1`, `android2` |
| `half-screen-dialog` | Matches. | default ([light](parity/half-screen-dialog-light-gpui.png) / [weui](parity/half-screen-dialog-light-weui.png); [dark](parity/half-screen-dialog-dark-gpui.png) / [weui](parity/half-screen-dialog-dark-weui.png)), `1`, `2`, `3`, `4`, `5` |
| `progress` | Matches. | default ([light](parity/progress-light-gpui.png) / [weui](parity/progress-light-weui.png); [dark](parity/progress-dark-gpui.png) / [weui](parity/progress-dark-weui.png)), `uploading` |
| `steps` | Matches. | default ([light](parity/steps-light-gpui.png) / [weui](parity/steps-light-weui.png); [dark](parity/steps-dark-gpui.png) / [weui](parity/steps-dark-weui.png)) |
| `steps_vertical` | Matches. | default ([light](parity/steps_vertical-light-gpui.png) / [weui](parity/steps_vertical-light-weui.png); [dark](parity/steps_vertical-dark-gpui.png) / [weui](parity/steps_vertical-dark-weui.png)) |
| `steps_horizonal` | Matches. | default ([light](parity/steps_horizonal-light-gpui.png) / [weui](parity/steps_horizonal-light-weui.png); [dark](parity/steps_horizonal-dark-gpui.png) / [weui](parity/steps_horizonal-dark-weui.png)) |
| `msg` | Matches. | default ([light](parity/msg-light-gpui.png) / [weui](parity/msg-light-weui.png); [dark](parity/msg-dark-gpui.png) / [weui](parity/msg-dark-weui.png)) |
| `msg_success` | Matches. | default ([light](parity/msg_success-light-gpui.png) / [weui](parity/msg_success-light-weui.png); [dark](parity/msg_success-dark-gpui.png) / [weui](parity/msg_success-dark-weui.png)) |
| `msg_warn` | Matches. | default ([light](parity/msg_warn-light-gpui.png) / [weui](parity/msg_warn-light-weui.png); [dark](parity/msg_warn-dark-gpui.png) / [weui](parity/msg_warn-dark-weui.png)) |
| `msg_text` | Matches. | default ([light](parity/msg_text-light-gpui.png) / [weui](parity/msg_text-light-weui.png); [dark](parity/msg_text-dark-gpui.png) / [weui](parity/msg_text-dark-weui.png)) |
| `msg_text_primary` | Matches. | default ([light](parity/msg_text_primary-light-gpui.png) / [weui](parity/msg_text_primary-light-weui.png); [dark](parity/msg_text_primary-dark-gpui.png) / [weui](parity/msg_text_primary-dark-weui.png)) |
| `msg_custom_area_preview` | Matches. | default ([light](parity/msg_custom_area_preview-light-gpui.png) / [weui](parity/msg_custom_area_preview-light-weui.png); [dark](parity/msg_custom_area_preview-dark-gpui.png) / [weui](parity/msg_custom_area_preview-dark-weui.png)) |
| `msg_custom_area_tips` | Matches. | default ([light](parity/msg_custom_area_tips-light-gpui.png) / [weui](parity/msg_custom_area_tips-light-weui.png); [dark](parity/msg_custom_area_tips-dark-gpui.png) / [weui](parity/msg_custom_area_tips-dark-weui.png)) |
| `msg_custom_area_cell` | Matches. | default ([light](parity/msg_custom_area_cell-light-gpui.png) / [weui](parity/msg_custom_area_cell-light-weui.png); [dark](parity/msg_custom_area_cell-dark-gpui.png) / [weui](parity/msg_custom_area_cell-dark-weui.png)) |
| `article` | Matches. | default ([light](parity/article-light-gpui.png) / [weui](parity/article-light-weui.png); [dark](parity/article-dark-gpui.png) / [weui](parity/article-dark-weui.png)) |
| `navbar` | Matches. | default ([light](parity/navbar-light-gpui.png) / [weui](parity/navbar-light-weui.png); [dark](parity/navbar-dark-gpui.png) / [weui](parity/navbar-dark-weui.png)), `tab2`, `tab3` |
| `tabbar` | Matches. | default ([light](parity/tabbar-light-gpui.png) / [weui](parity/tabbar-light-weui.png); [dark](parity/tabbar-dark-gpui.png) / [weui](parity/tabbar-dark-weui.png)), `tab2`, `tab3` |
| `panel` | Matches. Description clamp: Chromium applies CJK kinsoku and an ellipsis on line 2; GPUI may start line 2 with “。” and the text happens to fit without ellipsis. | default ([light](parity/panel-light-gpui.png) / [weui](parity/panel-light-weui.png); [dark](parity/panel-dark-gpui.png) / [weui](parity/panel-dark-weui.png)) |
| `actionsheet` | Baseline iOS top corners were square; corrected and recaptured in the review. | default ([light](parity/actionsheet-light-gpui.png) / [weui](parity/actionsheet-light-weui.png); [dark](parity/actionsheet-dark-gpui.png) / [weui](parity/actionsheet-dark-weui.png)), `ios`, `android` |
| `icons` | Matches. | default ([light](parity/icons-light-gpui.png) / [weui](parity/icons-light-weui.png); [dark](parity/icons-dark-gpui.png) / [weui](parity/icons-dark-weui.png)) |
| `searchbar` | Matches. | default ([light](parity/searchbar-light-gpui.png) / [weui](parity/searchbar-light-weui.png); [dark](parity/searchbar-dark-gpui.png) / [weui](parity/searchbar-dark-weui.png)) |
| `searchbar_filled` | Matches. | default ([light](parity/searchbar_filled-light-gpui.png) / [weui](parity/searchbar_filled-light-weui.png); [dark](parity/searchbar_filled-dark-gpui.png) / [weui](parity/searchbar_filled-dark-weui.png)), `focused`, `typed` |
| `searchbar_grey` | Matches. | default ([light](parity/searchbar_grey-light-gpui.png) / [weui](parity/searchbar_grey-light-weui.png); [dark](parity/searchbar_grey-dark-gpui.png) / [weui](parity/searchbar_grey-dark-weui.png)), `focused`, `typed` |
| `searchbar_outlined` | Matches. | default ([light](parity/searchbar_outlined-light-gpui.png) / [weui](parity/searchbar_outlined-light-weui.png); [dark](parity/searchbar_outlined-dark-gpui.png) / [weui](parity/searchbar_outlined-dark-weui.png)), `focused`, `typed`, `filled` |
| `searchbar_homepage` | Matches. | default ([light](parity/searchbar_homepage-light-gpui.png) / [weui](parity/searchbar_homepage-light-weui.png); [dark](parity/searchbar_homepage-dark-gpui.png) / [weui](parity/searchbar_homepage-dark-weui.png)) |
| `picker` | Matches. | default ([light](parity/picker-light-gpui.png) / [weui](parity/picker-light-weui.png); [dark](parity/picker-dark-gpui.png) / [weui](parity/picker-dark-weui.png)), `single`, `date`, `multi`, `cascade` |
| `footer` | Matches. | default ([light](parity/footer-light-gpui.png) / [weui](parity/footer-light-weui.png); [dark](parity/footer-dark-gpui.png) / [weui](parity/footer-dark-weui.png)) |
| `gallery` | Matches. | default ([light](parity/gallery-light-gpui.png) / [weui](parity/gallery-light-weui.png); [dark](parity/gallery-dark-gpui.png) / [weui](parity/gallery-dark-weui.png)), `closed` |
| `flex` | Matches. | default ([light](parity/flex-light-gpui.png) / [weui](parity/flex-light-weui.png); [dark](parity/flex-dark-gpui.png) / [weui](parity/flex-dark-weui.png)) |
| `loading` | Matches. | default ([light](parity/loading-light-gpui.png) / [weui](parity/loading-light-weui.png); [dark](parity/loading-dark-gpui.png) / [weui](parity/loading-dark-weui.png)) |
| `loadmore` | Matches. | default ([light](parity/loadmore-light-gpui.png) / [weui](parity/loadmore-light-weui.png); [dark](parity/loadmore-dark-gpui.png) / [weui](parity/loadmore-dark-weui.png)) |
| `uploader` | Matches. | default ([light](parity/uploader-light-gpui.png) / [weui](parity/uploader-light-weui.png); [dark](parity/uploader-dark-gpui.png) / [weui](parity/uploader-dark-weui.png)), `viewer`, `preview` |
| `preview` | Matches. | default ([light](parity/preview-light-gpui.png) / [weui](parity/preview-light-weui.png); [dark](parity/preview-dark-gpui.png) / [weui](parity/preview-dark-weui.png)) |
| `grid` | Matches. | default ([light](parity/grid-light-gpui.png) / [weui](parity/grid-light-weui.png); [dark](parity/grid-dark-gpui.png) / [weui](parity/grid-dark-weui.png)) |
| `badge` | Matches. | default ([light](parity/badge-light-gpui.png) / [weui](parity/badge-light-weui.png); [dark](parity/badge-dark-gpui.png) / [weui](parity/badge-dark-weui.png)) |
| `slider` | Matches. | default ([light](parity/slider-light-gpui.png) / [weui](parity/slider-light-weui.png); [dark](parity/slider-dark-gpui.png) / [weui](parity/slider-dark-weui.png)), `value80` |
| `information-bar` | Matches. | default ([light](parity/information-bar-light-gpui.png) / [weui](parity/information-bar-light-weui.png); [dark](parity/information-bar-dark-gpui.png) / [weui](parity/information-bar-dark-weui.png)), `closed` |

## Brand and care mode

| Sample | Result | Captures |
| --- | --- | --- |
| `button_default --brand 1485EE` | Matches; primary fill measured (22,133,237) native vs (20,133,238) official | [gpui](parity/button_default-brand1485EE-light-gpui.png) / [weui](parity/button_default-brand1485EE-light-weui.png) |
| `form_primary --care` | Matches in light and dark (larger care-mode type scale) | [light](parity/form_primary-care-light-gpui.png) / [weui](parity/form_primary-care-light-weui.png); [dark](parity/form_primary-care-dark-gpui.png) / [weui](parity/form_primary-care-dark-weui.png) |

Care mode was sampled on one route only; other routes in care mode have not been compared.
