# Scaling and resize — Linux X11 (Xvfb + openbox), 2026-10-24

- `scale-1.5-*` / `scale-2-*`: `GPUI_X11_SCALE_FACTOR=1.5` and `2`, `--page form_page`, one Tab:
  the window opens at 563×973 and 750×973 device px, text and icons are crisp, and the caret is
  drawn in the focused 微信号 field.
- `resize-*`: `xdotool windowsize` at scale 1 to 320×600, 600×900 and 900×500. Layout reflows
  with no panics. At 320px long placeholders clip at the field edge, and the "verification
  address" label breaks inside the word at the fixed label width, as upstream `word-wrap:
  break-word` does.

Not covered: Wayland, fractional scaling changed while running, and monitor moves.
