# Form validation states and toast/toptips timing — Linux X11 (Xvfb + openbox), 2026-10-24

Gallery `--page form_input_status`, keyboard only (xdotool), timings measured from the key press.

- 01: card number typed, submit button becomes enabled (it is disabled while the card is empty).
- 03-tab*: Tab order card → clear → holder → amount → read-only email → submit; the
  disabled 微信号 field is skipped.
- 04–06: Enter on 确定 with a 4-digit card: the 错误提示 top tips appear and the card row turns
  red (`weui-cell_warn`); the tips are still up at 1.5s and gone at 2.5s (2s, as upstream
  `setTimeout(…, 2000)`); the red row stays until the card text changes. Focus stays on 确定.
- 07: amount longer than 8 digits shows the inline red 最多支持8位数 tips (`Role::Alert`).
- 09–12: 16-digit card, Enter on 确定: the 已完成 success toast is up at 0.5s and 1.5s and gone
  at 2.5s (`TOAST_DURATION` 2s plus a 100ms fade); focus stays on 确定 (passive toast).
