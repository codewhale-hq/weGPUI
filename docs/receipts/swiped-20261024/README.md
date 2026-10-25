# Swiped cell — Linux X11 (Xvfb + openbox), 2026-10-24

Gallery `--page list`; the second row starts open with 删除, as upstream.

- `before-tab1-delete-ring-invisible.png`: Tab focused 删除, but its white ring sat next to the
  white list row and only made the red button look 2px narrower.
- After the fix (the warn action ring uses theme `fg_0`, default actions use brand):
  01 Tab: dark ring on 删除; 02 Tab: brand ring on the row; 03 Right closes the row;
  04 Shift-Tab skips the now-hidden 删除 and wraps to the footer link; 05 Tab back and Enter
  opens; 06 Escape closes; 07 Left opens; 08 dark theme: light ring on 删除.
