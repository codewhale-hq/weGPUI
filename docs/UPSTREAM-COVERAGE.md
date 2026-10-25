# Upstream example coverage

Inventory of WeUI 2.6.26: **61 HTML files — 58 within 29 component directories plus `home.html`, `index.html` and `layers.html`**. Every file maps to a gallery route built from the public library components; no route is a placeholder. Capture states are the `--state` values registered in `scripts/parity/*.json`, each with a replay that drives the official page into the same state.

Visual results are in [PARITY](PARITY.md). Mapping a file here records implementation, not parity.

| Upstream HTML under `src/example/` | Library components | Gallery route | Capture states |
| --- | --- | --- | --- |
| `actionsheet/actionsheet.html` | `components::{action_sheet}` | `actionsheet` | ios, android |
| `article/article.html` | `components::{article}` | `article` | — |
| `badge/badge.html` | `components::{badge, cells}` | `badge` | — |
| `button/button.html` | `components::{button, agree}` | `button` | — |
| `button/button_bottom_fixed.html` | `components::{button, agree}` | `button_bottom_fixed` | agreed |
| `button/button_default.html` | `components::{button, agree}` | `button_default` | — |
| `dialog/dialog.html` | `components::{dialog, modal}` | `dialog` | ios1, ios2, ios3, android1, android2 |
| `flex/flex.html` | `components::{flex}` | `flex` | — |
| `footer/footer.html` | `components::{footer}` | `footer` | — |
| `form/form.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form` | — |
| `form/form_access.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_access` | — |
| `form/form_bottom_fixed.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_bottom_fixed` | dialog, checked, toast |
| `form/form_checkbox.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_checkbox` | toast, checked |
| `form/form_input_status.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_input_status` | error, amount, toast |
| `form/form_page.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_page` | filled, toast |
| `form/form_page_old.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_page_old` | filled |
| `form/form_primary.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_primary` | switch, filled |
| `form/form_radio.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_radio` | toast, checked |
| `form/form_select.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_select` | toast |
| `form/form_select_primary.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_select_primary` | select1, select2, select3 |
| `form/form_switch.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_switch` | toast, checked |
| `form/form_textarea.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_textarea` | — |
| `form/form_vcode.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_vcode` | dialog, checked, toast, warn, countdown |
| `form/form_vertical.html` | `components::{form, checkbox, radio, switch, select, vcode, agree, toptips, textarea, input, toast, half_screen_dialog, picker}` | `form_vertical` | — |
| `gallery/gallery.html` | `components::{gallery}` | `gallery` | closed |
| `grid/grid.html` | `components::{grid}` | `grid` | — |
| `half-screen-dialog/half-screen-dialog.html` | `components::{half_screen_dialog, modal}` | `half-screen-dialog` | 1, 2, 3, 4, 5 |
| `home.html` | `home` route: grouped categories with expanding lists (`pages/home.rs`) | `home` | category-1 |
| `icons/icons.html` | `components::{icons}` | `icons` | — |
| `index.html` | gallery shell (`examples/gallery/main.rs`, `chrome.rs`): grouped home, page navigation, back, light/dark/system, care mode, brand and reduced-motion menus | `home` route | — |
| `information-bar/information-bar.html` | `components::{information_bar}` | `information-bar` | closed |
| `input/input.html` | `components::{input, textarea, cells, checkbox, radio, vcode}` | `input` | — |
| `layers.html` | `layers` route (`pages/layers.rs`): 3D layer stack, click/keyboard isolation | `layers` | popout, mask, navigation, content, 3d |
| `list/list.html` | `components::{cells, swiped}` | `list` | closed |
| `loading/loading.html` | `components::{loading}` | `loading` | — |
| `loadmore/loadmore.html` | `components::{loadmore}` | `loadmore` | — |
| `msg/msg.html` | `components::{msg, button, cells, form_preview}` | `msg` | — |
| `msg/msg_custom_area_cell.html` | `components::{msg, button, cells, form_preview}` | `msg_custom_area_cell` | — |
| `msg/msg_custom_area_preview.html` | `components::{msg, button, cells, form_preview}` | `msg_custom_area_preview` | — |
| `msg/msg_custom_area_tips.html` | `components::{msg, button, cells, form_preview}` | `msg_custom_area_tips` | — |
| `msg/msg_success.html` | `components::{msg, button, cells, form_preview}` | `msg_success` | — |
| `msg/msg_text.html` | `components::{msg, button, cells, form_preview}` | `msg_text` | — |
| `msg/msg_text_primary.html` | `components::{msg, button, cells, form_preview}` | `msg_text_primary` | — |
| `msg/msg_warn.html` | `components::{msg, button, cells, form_preview}` | `msg_warn` | — |
| `navbar/navbar.html` | `components::{navbar}` | `navbar` | tab2, tab3 |
| `panel/panel.html` | `components::{panel}` | `panel` | — |
| `picker/picker.html` | `components::{picker, half_screen_dialog}` | `picker` | single, date, multi, cascade |
| `preview/preview.html` | `components::{form_preview}` | `preview` | — |
| `progress/progress.html` | `components::{progress, icons}` | `progress` | uploading |
| `searchbar/searchbar.html` | `components::{search_bar}` | `searchbar` | — |
| `searchbar/searchbar_filled.html` | `components::{search_bar}` | `searchbar_filled` | focused, typed |
| `searchbar/searchbar_grey.html` | `components::{search_bar}` | `searchbar_grey` | focused, typed |
| `searchbar/searchbar_homepage.html` | `components::{search_bar}` | `searchbar_homepage` | — |
| `searchbar/searchbar_outlined.html` | `components::{search_bar}` | `searchbar_outlined` | focused, typed, filled |
| `slider/slider.html` | `components::{slider}` | `slider` | value80 |
| `steps/steps.html` | `components::{steps}` | `steps` | — |
| `steps/steps_horizonal.html` | `components::{steps}` | `steps_horizonal` | — |
| `steps/steps_vertical.html` | `components::{steps}` | `steps_vertical` | — |
| `tabbar/tabbar.html` | `components::{tabbar, badge}` | `tabbar` | tab2, tab3 |
| `toast/toast.html` | `components::{toast, modal}` | `toast` | success, fail, longtext, loading, text |
| `uploader/uploader.html` | `components::{uploader, gallery, toptips}`; add opens the native file chooser (upstream `<input type=file accept="image/*" multiple>`); non-image files become error items; transport is application-owned | `uploader` | viewer, preview |

Total: 61 files.

## CSS families without their own example

| Family | Where it is ported | Notes |
| --- | --- | --- |
| top tips (`weui-toptips`) | `components::toptips` | shown by `form_input_status` (`--state error`) |
| agree (`weui-agree`) | `components::agree` | `button_bottom_fixed`, `form_vcode`, `form_bottom_fixed` |
| list tips / information bar | `components::information_bar` | `information-bar` |
| swiped cells (`weui-cell_swiped`) | `components::swiped` | `list` (`--state closed`) |
| button cell/icon/xmini/overlay/wrap/bottom-fixed | `components::button` | `button`, `button_default`, `button_bottom_fixed` |
| mask, animations (`weui-mask`, `weui-animate`) | `components::modal` (`Presence`, `Modal`) | fade/slide transitions, reduced motion respected |
| care mode (`data-weui-mode=care`) | `Theme::set_care`, care light/dark palettes from `weui.css` | gallery Appearance menu and `--care`; not compared screen by screen, see STATUS |
| brand override (`--weui-BRAND`) | `Theme::set_brand` | gallery menu and `--brand RRGGBB` |
| safe-area insets (`env(safe-area-inset-*)`) | base padding only | desktop windows have no insets, so this equals upstream with `env()` = 0; mobile inset handling is not implemented |

The original `steps_horizonal.html` spelling is preserved intentionally.

## Independent review qualification

All 61 upstream filenames were counted again on 2026-10-24. `index` is the gallery shell; home and layers include gallery-specific implementations. The mapping establishes route coverage, not complete behaviour. See [the review](REVIEW-2026-10-24.md) for per-family evidence, sample-only/no-op actions and outstanding differences.
