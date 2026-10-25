# Focus scrolling, date/datetime fields and picker key crash — Linux X11, 2026-10-24

Source: working tree on top of `ec40ab8` (the commit that adds this directory). Debian 13 box,
rustc/cargo 1.98.1, Xvfb `:98` 1600x1800x24, `GPUI_X11_SCALE_FACTOR=1`, debug gallery build,
xdotool key/wheel input, ImageMagick `import` captures. Linux X11 only: no macOS, Windows,
Wayland or screen-reader run was made for this change.

## Gates (`*.txt`)

fmt check, `cargo build --all-targets`, `cargo test` (13 library tests pass incl. 4 new
`scrolling` tests; 1 doc test ignored), strict all-target Clippy, strict rustdoc: all exit 0.

## Runtime sequences

1. `linux-x11-preview-tab-sequence.png` — `--page preview --size 480x650`. Frames: initial,
   Tab (upper 操作, no scroll), Tab (lower 辅助操作 offscreen: page scrolls minimally, ring
   visible), Tab (操作 beside it, no scroll), Shift+Tab (no scroll), Shift+Tab (upper 操作
   visible, no scroll), Shift+Tab (wraps). A first attempt that called `window.refresh()`
   from prepaint scrolled but did not repaint until the next input; fixed by refreshing on the
   next frame (failed attempt not captured as a separate file; described here).
2. `linux-x11-preview-manual-scroll-kept.png` — after focus on the lower action, six wheel-up
   steps: offset stays where the user put it across later frames; a further Tab scrolls only to
   reveal the newly focused target.
3. `linux-x11-input-tab-sequence.png` — `--page input`, ten Tabs through radio, links,
   checkboxes, qq/phone text inputs, vcode button, 日期 and 时间 fields; each newly focused
   control below the fold is revealed.
4. `linux-x11-input-date-datetime-sequence.png` — Enter on 时间 opens a date sheet; Down /
   Tab / Down keys; Enter confirms the date and opens the hour/minute sheet; Enter confirms:
   field shows `2027-12-25 03:01` and focus returns to it. Enter reopens at the remembered
   rows; Escape cancels and leaves the value. Shift+Tab to 日期, Enter, Enter: `2026-10-25`.

## Defect found and fixed

Pressing arrow keys in a picker column while its snap animation was running panicked debug
builds: `Column::visual` called `window.request_animation_frame()` from a key handler
("this method can only be called during request_layout, prepaint, or paint", backtrace through
`Picker::step` → `snap` → `visual`). Event handlers now read a side-effect-free
`Column::current`; only render requests animation frames.

## Not checked

Screen-reader announcement of the fields and sheets, IME, nested `FocusScroll` at runtime,
pointer-driven sheet use on this build, performance of the 1900–2100 cascade on low-end
hardware, other OSes.
