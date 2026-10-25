# DateField accessibility tree — Linux AT-SPI (Xvfb + openbox), 2026-10-24 ~23:30 PDT

`STEPS=… dbus-run-session -- scripts/linux/session.sh target/debug/examples/gallery --page input`.
The tree after each step is in `atspi.txt` and screenshots are `shot-*.png`.

- initial: `button '日期，未选择'` and `button '时间，未选择'` inside their list items.
- focus-date (Tab ×9): the date button is `focused`.
- open (Enter): `dialog '日期'` (`modal`) with 关闭, three list boxes named `日期 2026年` (focused),
  `日期 10月` and `日期 25日`, `selected` on the current items, and 确定. The page is outside the
  dialog in the tree.
- escape: the dialog is gone, focus is back on `日期，未选择`, and the value is unchanged.
- open-again + confirm (Enter, Enter): the button is renamed `日期，2026-10-25` and focus returns
  to it.

Finding: the run was on 2026-10-24 local (PDT), but the picker starts on the 25th because
`picker::today()` is UTC (a known STATUS item; a local-time source needs an owner decision on a
dependency). No screen reader (Orca) was run; this is a tree check only.
