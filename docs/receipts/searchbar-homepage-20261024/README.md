# Search bar homepage variant — Linux X11 (Xvfb + openbox), 2026-10-24

Gallery `--page searchbar_homepage`, keyboard only.

- `before-01-tab-grey-fill.png`: keyboard focus on the 搜索 entry was only a `FG-3` grey fill
  on the `BG-2` field, hard to see.
- After the fix: 00 unfocused is pixel-identical to before (ImageMagick AE 0); 01 Tab puts a
  2px brand ring on 搜索 (Role::Button, Enter emits `Open`); 02 typing does nothing (the
  homepage variant has no text field, as upstream); 03 Tab rings the 拍照 camera button;
  04 Tab moves on to the page footer link.

## Filled variant back/clear/cancel (`filled-before/`, `filled-after/`, `drive-filled.sh`)

The same grey-fill focus was used on the back, clear and 取消 buttons. They now use the 2px
brand ring with -2px margins. ImageMagick AE before vs after: 00 idle, 01 Tab, 02 focusing and
06 back-to-input are 0 (layout unchanged); 03 differs by 22 px (caret blink); 04 (clear
focused) and 05 (取消 focused) differ only by the ring. The back arrow uses the same change;
it was not reached in this drive.
