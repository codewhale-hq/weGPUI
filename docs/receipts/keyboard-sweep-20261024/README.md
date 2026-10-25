# Keyboard/focus sweep — Linux X11 + openbox, 2026-10-24

Source `c62e278`, debug gallery, 480x650, Xvfb :97 with openbox, `import -window`. Each page:
three Tabs, 40% scale captures (sweep-*.png rows = page, columns = Tab 1–3). No panics in any
of the 15 runs (slider, list, searchbar, uploader, gallery, panel, grid, msg, actionsheet,
picker, progress, form_vcode, form_textarea, article, footer).

Observed: visible focus on swiped list cells and their actions, search-bar variant buttons,
uploader thumbnails and description link, panel media boxes and 查看更多, grid cells, msg route
buttons, action-sheet trigger, picker triggers, progress cancel buttons, textarea and form
buttons, page-footer link. Gallery viewer delete button has a white 2px ring on the dark bar
(`gallery-delete-focus.png`). Slider: Right×3 moves 50→53 with visible ring; the upstream
"不可调" slider is static and not focusable (`slider-keys.png`). Text-input carets are 1px and
not visible at 40% scale; caret drawing is verified separately at full scale.

Not checked: activation results for every target, screen readers, dark/care themes.

## Themes

`themes-dark-care-form-select.png`: form_select in dark, care, dark+care. Pairs = (two Tabs:
ring on the +86 select; four Tabs + Enter: country menu open with focused current option).
Rings and menu focus are visible in all three.
