# P1 accessibility fixes — Linux X11 + AT-SPI, 2026-10-24

Debian 13, rustc 1.98.1, Xvfb :98, `scripts/linux/session.sh` (at-spi2-core, python3-pyatspi).

## Set positions were off by one (fixed)

AccessKit 0.24 `position_in_set` is zero-based: `accesskit_atspi_common 0.19.1` and
`accesskit_windows 0.34.0` both add 1 when exporting. weGPUI passed `i + 1` in radio, select
menu options, article lists, form-preview items and list tips, so the first radio was announced
"2 of …". Before (`ec40ab8` baseline build and `c744f1d`): `radio button 'Email' … posinset:2`.
After: `posinset:1` / `posinset:2` (`component-states-atspi.txt`).

## Disabled buttons on Linux (upstream limitation, not fixed)

`button 'Unavailable'` (a disabled cell button) is exported without DISABLED. The node does
carry AccessKit `disabled`, but `accesskit_atspi_common 0.19.1` maps disabled to "not
sensitive" only for roles that support read-only; buttons always get Enabled|Sensitive.
Disabled switches are reported correctly (`form_switch`: `toggle button '标题文字' DISABLED`,
identical on the baseline build). The button stays unfocusable and carries the "不可用"
description, which Orca reads. macOS AX reports it disabled (review-20261024). Fixing this
needs an upstream AccessKit change; no dependency patch was made.

## Window-manager harness finding

A bare Xvfb never activates the window, and GPUI only reports focus paths for an active window
(`Window::draw` builds `WindowFocusEvent` with empty paths when inactive), so `TextInput` never
received `on_focus` and drew no caret, although typing worked. That is a harness artifact, not
a defect: native apps hide the caret in inactive windows too. With `openbox` running on the
display, the window activates and the caret is drawn (`linux-x11-openbox-input-caret.png`).
Capture with `import -window <id>` under openbox (root captures came out black).

## Primary/warn focus ring (fixed)

Filled primary and warn buttons drew focus as a 2px white inner border only, which nearly
vanishes on a light page. They now add a 2px brand outer ring outside the white gap.
`linux-x11-openbox-button-default-tab-sequence.png` (button_default, 480x650, four Tabs):
primary ring visible, loading and disabled buttons skipped, default and warn rings visible,
focus revealed while scrolling.

## Paste was merged into the preceding typing undo step (fixed)

`input` page, qq field: type `abc12` (Number kind keeps `12`), Shift+Home, Ctrl+C, End,
Ctrl+V gives `1212`. Before: one Ctrl+Z emptied the field, because the paste landed where the
last insert ended and was coalesced with it (observed on screen during the run; the before capture was overwritten by the
after run and is not retained). Fix: paste and cut always start and end their own undo step, and caret moves
break typing coalescing. After (`…-fixed.png`): undo → `12`, undo → empty, redo → `12`, redo →
`1212`.

## Select and dialog keyboard (verified, no change)

- `linux-x11-form-select-keyboard.png` (bare Xvfb, form_select): Tab to 国家, Enter opens the
  menu with the current option checked, Down moves, Enter selects 英国 and closes, focus returns
  to the row. (At 50% scale 英国 resembles 美国.)
- `linux-x11-openbox-dialog-keyboard.png` (openbox, dialog): Tab to 样式一, Enter opens; focus
  goes to the dialog container (no ring), Tab/Tab cycle 辅助操作 ↔ 主操作 inside the dialog,
  Escape closes and focus returns to 样式一 with its ring; 样式二 opens the second dialog.
