# Search bar homepage variant — Linux X11 (Xvfb + openbox), 2026-10-24

Gallery `--page searchbar_homepage`, keyboard only.

- `before-01-tab-grey-fill.png`: keyboard focus on the 搜索 entry was only a `FG-3` grey fill
  on the `BG-2` field, hard to see.
- After the fix: 00 unfocused is pixel-identical to before (ImageMagick AE 0); 01 Tab puts a
  2px brand ring on 搜索 (Role::Button, Enter emits `Open`); 02 typing does nothing (the
  homepage variant has no text field, as upstream); 03 Tab rings the 拍照 camera button;
  04 Tab moves on to the page footer link.
