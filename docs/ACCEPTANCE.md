# Component acceptance inventory — WeUI 2.6.26

Generated from source on 2026-10-24 (columns API…a11y are mechanical source facts; *Verified*
and *Open* are evidence notes; see STATUS "Behaviour by family" and receipts). A row is
**accepted** only when every applicable column has runtime evidence on macOS, Windows and Linux,
in light, dark and care mode, plus screen-reader acceptance. **No row is accepted yet.**

Legend: *Focus* — keyboard focusable. *Keys* — `custom` = component key handling,
`click` = GPUI click activation on Enter/Space. *Reveal* — registers with
`scrolling::FocusScroll` (`overlay` = lives in a Modal, which manages its own focus).
*Priority* — P1 blocks desktop developer preview, P2 needed for "entire WeUI", P3 static or
presentational.

| Module | Public types | Callbacks / events | State setters | Focus | Keys | Reveal | A11y | Verified so far | Open | Priority |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `action_sheet` | ActionSheetItem, ActionSheet | on_click, on_dismiss | warn | yes | click (Enter/Space) | overlay | role+name | Overlay, item callbacks, Escape/mask dismiss via Modal | Dismissal matrix, item disabled state (upstream has none), screen-reader order | P2 |
| `agree` | Agree | on_change | disabled, warn, checked | yes | click (Enter/Space) | yes | role+name | Checked/disabled/warn + shake, link callback | Disabled/warn matrix not driven at runtime | P2 |
| `article` | Article, ListStyle, ListItem, ArticleList | — | — | n/a | n/a | n/a | role+name | Static content, heading/list roles | Heading level not forwarded by AT-SPI 0.19.1 | P3 |
| `badge` | Badge | — | — | n/a | n/a | n/a | none | Static | None interactive; visual parity unsampled | P3 |
| `button` | Size, ButtonKind, Button, ButtonArea, BottomFixedOpr, BottomFixedOprPage | on_click | disabled, loading, warn | yes | click (Enter/Space) | yes | role+name | Kinds/sizes, disabled, loading guard, cell variant, BottomFixedOpr | Runtime matrix of disabled/loading per size; — | P1 |
| `cells` | CellsStyle, Cells, Cell | on_click | disabled, warn, selected | yes | click (Enter/Space) | yes | role+name | Clickable/link/disabled rows, track_focus | Access/arrow variants visual parity | P2 |
| `checkbox` | Checkbox, CheckboxCell | on_change | disabled, checked | yes | click (Enter/Space) | yes | role+name | Controlled checked, disabled | Runtime checkbox matrix untested | P2 |
| `date_field` | DateFieldMode, DateValue, DateFieldEvent, DateField | DateFieldEvent | disabled | yes | click (Enter/Space) | yes | role+name | New 2026-10-24: Date/DateTime, Change/Cancel events, value/range/disabled API | No typed entry, locale display, seconds; screen reader untested | P1 |
| `dialog` | DialogButtonKind, DialogButton, Dialog | on_click, on_dismiss | disabled, warn | yes | click (Enter/Space) | overlay | role+name | Buttons, dismiss, focus restore (minimal example) | Announce order untested | P1 |
| `flex` |  | — | — | n/a | n/a | n/a | none | Layout helper | Resize behaviour unsampled | P3 |
| `footer` | FooterLink, Footer | on_click | — | yes | click (Enter/Space) | yes | role+name | Links clickable/focusable | Gallery callbacks empty; home footer link focus from Mac candidate not ported | P2 |
| `form` | CellKind, FormCell, FormGroup, Form, TipsGroup, ClearableInput | on_tap | disabled, warn | yes | click (Enter/Space) | yes | role+name | FormCell/Group/Form/TipsGroup/ClearableInput, label_for, BottomFixedPage FocusScroll | Validation states vs upstream JS; vertical layout parity | P1 |
| `form_preview` | FormPreviewButtonKind, FormPreviewButton, FormPreview, FormPreviewList | on_click | — | yes | click (Enter/Space) | yes | role+name | Buttons focusable, reveal_focus | Callbacks empty in gallery (static upstream) | P3 |
| `gallery` | Gallery | on_delete, on_dismiss | — | yes | click (Enter/Space) | overlay | role+name | Viewer open/delete/dismiss | Pinch/zoom, thumbnail-click proof | P2 |
| `grid` | Grid, Grids | on_click | — | yes | click (Enter/Space) | yes | role+name | Clickable cells | Gallery callbacks empty | P3 |
| `half_screen_dialog` | HdIconButton, HalfScreenDialog | on_click, on_dismiss | — | yes | click (Enter/Space) | overlay | role+name | Open/Tab/Escape/drag samples | Body FocusScroll wrapped 2026-10-24; nested-modal focus runtime | P1 |
| `icons` | IconVariant, Icon, IconButton | on_click | disabled | yes | click (Enter/Space) | yes | role+name | 32 SVGs, IconButton disabled | No interaction contract beyond IconButton | P3 |
| `information_bar` | InformationBarKind, InformationBar, ListTips | on_close | — | yes | click (Enter/Space) | yes | role+name | Close callback | Link/floating variants untested | P2 |
| `loading` | LoadingKind, Loading | — | — | n/a | n/a | n/a | role+name | Kinds, busy role | Animation timing, live reduce-motion change | P3 |
| `loadmore` | LoadMoreKind, LoadMore | — | loading | n/a | n/a | n/a | role+name | Presentation, loading state | None | P3 |
| `modal` | Presence, MaskKind, Placement, Motion, Modal | on_dismiss | — | yes | custom | overlay | role+name | Presence, ModalScope, focus trap, Escape | Nested modal stacking runtime | P1 |
| `msg` | LinkText, MsgIcon, Msg | — | — | yes | click (Enter/Space) | yes | role+name | LinkText shaped bounds, keyboard activation | Screen-reader acceptance | P2 |
| `navbar` | Navbar | on_select | selected | yes | custom | no | role+name | Arrow-key selection, roving focus | Linux rerun; overflow scroller not FocusScroll-wrapped | P2 |
| `panel` | Panel, MediaBoxKind, MediaMeta, MediaBox | on_click | — | yes | click (Enter/Space) | yes | role+name | MediaBox click, focus styling | CJK punctuation/clamping differs | P2 |
| `picker` | PickerItem, PickerEvent, Picker | PickerEvent | disabled, selected | yes | custom | overlay | role+name | Single/multi/cascade/date, keys, disabled rows; mid-animation key panic fixed 2026-10-24 | today() is UTC (owner decision); all-disabled diverges from weui.js by design | P1 |
| `progress` | Progress | on_cancel | — | yes | click (Enter/Space) | yes | role+name | Cancel button | NaN fix source-only | P3 |
| `radio` | RadioGroup | on_change | selected | yes | custom | yes | role+name | Roving arrows, scoped IDs, disabled | — | P2 |
| `search_bar` | SearchBarVariant, SearchBarEvent, SearchBar, SearchResults | on_select, SearchBarEvent | — | yes | click (Enter/Space) | yes | role+name | Variants, events, results, IME commit/cancel | Homepage variant runtime | P2 |
| `select` | SelectLayout, SelectCell, SelectMenu | on_click, on_dismiss, on_select | selected | yes | custom | yes | role+name | Overlay menu adaptation, keys, Escape restore | macOS key sequence inconclusive | P1 |
| `slider` | Slider | on_change | disabled | yes | custom | yes | role+name | Drag/keys, step snapping, disabled | Zero-step runtime receipt | P2 |
| `steps` | StepsLayout, Step, Steps | — | — | n/a | n/a | n/a | role+name | Static | None | P3 |
| `swiped` | SwipedActionKind, SwipedAction, SwipedCell | on_click | warn | yes | custom | yes | role+name | Drag reveal, hidden actions, keyboard | Delete callback empty upstream | P2 |
| `switch` | Switch | on_change | disabled, checked | yes | click (Enter/Space) | yes | role+name | Controlled checked, disabled, focus ring | — | P2 |
| `tabbar` | TabbarItem, Tabbar | on_select | selected | yes | custom | no | role+name | Arrow selection | Linux rerun | P2 |
| `textarea` | TextArea | — | disabled, warn | n/a | n/a | yes | role+name | Max length, counter, IME | UTF-16 counting vs upstream JS length | P2 |
| `toast` | ToastKind, Toast | — | loading, warn | n/a | n/a | n/a | role+name | Kinds, passive focus, busy | Timing, alert subrole | P2 |
| `toptips` | ToptipsState, Toptips | — | warn | n/a | n/a | n/a | role+name | Warn, overlay | Timing | P3 |
| `uploader` | UploaderStatus, UploaderFile, Uploader | on_add, on_delete, on_preview | error | yes | click (Enter/Space) | yes | role+name | Add/delete/preview, error status, portal lifecycle | Transport app-owned | P2 |
| `vcode` | VcodeEvent, VcodeButton, VcodeImage | VcodeEvent | — | yes | click (Enter/Space) | yes | role+name | Countdown guard, events | — | P2 |
| `input` | InputKind, InputEvent, TextInput | InputEvent | disabled | yes | custom | yes | role+name | Kinds, IME, undo/redo, clipboard, selection | Numeric kinds filter but do not validate; XIM preedit not inline | P1 |

## Cross-cutting acceptance items

| Item | State |
| --- | --- |
| Offscreen focus reveal | Implemented (`scrolling`), Linux X11 runtime only. Dialog body, half-screen body, `BottomFixedOprPage` and `BottomFixedPage` wrapped. `navbar::tab_panel` returns `Stateful<Div>`, so callers wrap it in `FocusScroll` themselves. `FocusScroll::inset_bottom` keeps revealed fields above the 80px fade in both bottom-fixed pages (fixed 2026-10-24, `linux-x11-form-bottom-fixed-inset-fixed.png`). |
| Light / dark / care layouts | Website checks cover 264 state/theme selections of captures; native per-component care-mode runtime unsampled. |
| Screen readers | None accepted (VoiceOver stays off per owner; NVDA/Narrator/Orca not run). |
| IME / selection / clipboard / undo | `TextInput`/`TextArea` sampled on Linux XIM and macOS Pinyin; Windows IME not run. |
| Scaling / resize | Not run. |
| Set positions | Fixed 2026-10-24 (were off by one); Linux AT-SPI verified. |
| Disabled buttons on Linux | AccessKit AT-SPI 0.19.1 does not export disabled for buttons; documented upstream limitation (`receipts/a11y-p1-20261024`). |
| Upstream static demos vs broken callbacks | Static upstream demos (empty handlers): footer, grid, form_preview, msg, list delete. Gallery keeps them inert intentionally; library callbacks exist. |

## Regenerating

The mechanical columns come from a scan of `src/components/*.rs` and `src/input.rs` (`pub
struct/enum`, `pub fn on_*`, `*Event` enums, state setters, `focusable()`/`track_focus`,
`on_key_down`/`on_action`, `reveal_focus`/`FocusTarget`, `.role(`/`aria_label`). Update the
notes by hand when evidence changes.
