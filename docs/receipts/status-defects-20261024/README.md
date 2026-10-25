# STATUS known-defect families rechecked on Linux X11 (openbox), 2026-10-24

- `countdown-*` (`examples/component_states`): Enter on "Start countdown twice" starts one
  countdown. The vcode button shows 重新发送(3s) at 0.5s and 重新发送(1s) at 2s, Sent is 1 not 2,
  and the disabled vcode is skipped by Shift-Tab.
- `sendcode-loading.png`: Enter on the Save profile cell button shows its loading spinner and
  Saved becomes 1.
- `panel-tab*`: clickable small-appmsg panel rows and 查看更多 take visible focus rings.
- `radio-*` (`form_radio`): Tab lands on the checked radio, Down moves and selects with a
  ring, and Tab continues to 添加更多.
