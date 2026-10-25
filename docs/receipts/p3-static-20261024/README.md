# P3 static and simple families — Linux X11 (Xvfb + openbox), 2026-10-24

Gallery pages badge, steps_vertical, steps_horizonal, loadmore, article, flex, grid, icons,
loading, progress, footer, panel, msg_success, preview and list.

- `<page>.png`: light theme; `dark-<page>.png`: `--theme dark`; `care-<page>.png`: `--care`.
  No panics in any run. (`steps.png` and `msg.png` are the index pages for those families.)
- `atspi-<page>.txt`: AT-SPI tree from `dbus-run-session -- scripts/linux/session.sh`.
- `kbd-<page>-*.png`: Tab, Tab, Enter on grid, icons, preview and progress. Grid cells show
  a focus fill, the preview buttons and progress cancel buttons show rings, and Enter on the
  footer link goes home. Grid/preview/progress callbacks are empty, as they are upstream.

Notes: upstream marks the loadmore dot `role="option"`; here it is a list item. Progress
cancel buttons are all named 取消, as upstream. Heading levels in `article` are not forwarded by
AT-SPI 0.19.1 (known).
