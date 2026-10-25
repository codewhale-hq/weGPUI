# Native Wayland runtime samples — 2026-10-25

These are real GPUI Wayland client runs under Weston 14.0.2 nested on Xvfb, on
Debian 13.5 x86_64. They qualify the named samples only. They do not establish
complete Wayland, component, platform, theme or screen-reader acceptance.

## Environment and source

Rust/Cargo 1.98.1, exact locked `gpui-pre` / `gpui-pre-platform` 0.3.9,
Mesa 25.0.7 lavapipe Vulkan, Weston X11 backend with pixman renderer and kiosk
shell, Openbox 3.6.1 and Xvfb 21.1.16. No physical GPU or monitor was used.
The client fills the compositor's initial 900×800 logical output at scale 1.

Each run's `provenance.txt` records versions, command, parent Git commit and
binary/Cargo.lock SHA-256. `source-sha256.txt` records the source snapshot.
The parent is `52650f3b830ed7e8d1cee7921d495bff06e0628d`. Initial binaries include
the continuation's focus-reveal additions. Follow-up binaries additionally include
the Cell/dialog text and nested-modal accessibility fixes. Each source snapshot
and binary is distinguished by its manifest and artifact hash. The later
Ctrl+Home/End alias addition postdates the other `*-final` runs and is verified
separately in `component-boundaries-final/`; earlier manifests remain unchanged.

The GPUI process has **no `DISPLAY`** and uses `WAYLAND_DISPLAY=wayland-wegpui`.
`wayland-protocol.log` (losslessly archived as `.log.gz`) records `xdg_toplevel`,
`wl_surface`, `wl_keyboard` and `wl_pointer` traffic. Weston does not run Xwayland. XTEST input goes to Weston's
X11 output window, and Weston translates it to genuine Wayland input events.
Screenshots use ImageMagick `import -window` on that compositor output window.
These captures therefore include the complete Wayland output, not an X11 GPUI
window. `wayland-info.log` records compositor protocols.

## Reviewed runtime observations

| Run | Observation |
| --- | --- |
| `minimal/` | Pointer opens dialog. Tab selects Cancel, then OK, then wraps to Cancel. Shift+Tab returns to OK. Enter invokes the confirm callback: the visible counter changes 0→1. Focus returns to the Open dialog trigger with a ring. Return reopens; Escape dismisses and restores trigger focus. AT-SPI hides the page while the dialog is open. |
| `minimal/resize*.png` | Output resizes 900×800→640×600→900×800. The protocol records matching `xdg_toplevel.configure`/acknowledgement and the counter remains 1. Rendering adapts without a panic. This is output resizing, not live scale or monitor-move testing. |
| `component-states/` | Clicks Display name before keys. Types `wayland`; Ctrl+A/C and End/Ctrl+V produce `waylandwayland`; Ctrl+Z returns to `wayland`; Ctrl+Shift+Z restores the doubled text. These are same-client clipboard/undo observations. Tab+Enter saves once; another Enter during loading does not duplicate it (`saves=1` occurs once in stderr/protocol log). The next Tab skips Unavailable and reaches Send code. |
| `nested-modals/` | Opens Outer sheet, then Inner dialog. Tab/Shift+Tab stay between the inner Cancel/OK buttons. Escape closes only the inner layer, returning the ring to Open inner dialog; another Escape closes the outer layer and returns the ring to Open sheet. The visible log is `inner-close, outer-close`. |

The referenced screenshots were opened and visually reviewed alongside the
corresponding AT-SPI snapshots and protocol events. AT-SPI snapshots are tree
observations, not Orca or another screen-reader result.

## Verified accessibility fixes

- `minimal-followup/` repeats the minimal sequence on the integrated binary.
  The body now appears as the named Label `A WeUI dialog rendered by native GPUI
  components.` `initial-details.json` reports the Confirmed row description `0`;
  `confirm-details.json` reports `1`, matching the visibly changed counter.
- `nested-modals-followup/` repeats the nested sequence. While the inner dialog
  is active, its tree contains only Inner dialog, body and buttons. The outer
  layer remains painted but is absent from AT-SPI. Closing the inner dialog
  restores Outer sheet and its focused opener; closing the outer restores the
  page and Open sheet focus. Tab/Shift+Tab trapping and rings still pass.
- `component-states-interim/` verifies the first TextRun implementation's ASCII
  Text interface: the queried entry reads `wayland`, `waylandwayland`, `wayland`,
  `waylandwayland` after typing, paste, undo and redo. This directory is interim
  evidence, preceding the grapheme-aware TextRun correction.
- `component-states-final/` repeats the sequence on the final grapheme-aware
  binary. Text content matches all four edits; caret offsets are 7 or 14 as
  appropriate. Ctrl+A exports selection `[[0, 7]]`. The Save guard and disabled
  Tab skip still pass. This ASCII sample does not qualify Unicode or password
  screen-reader behavior; those have separate source/runtime receipts.
- `nested-modals-final/` repeats the sequence on the final depth-ranked modal
  binary. Only Inner dialog is exposed while it is active; focus trapping,
  visible rings and sequential close/restoration still pass. The separate
  [modal-order receipt](../nested-modal-a11y-20261025/README.md) covers the
  deeper/sibling ordering case on X11.

- `component-boundaries-final/` uses the preserved seeded probe source on the
  artifact with the later Ctrl+Home/End aliases. After Tab twice, the Multiline
  sample's Text interface contains 24 scalar characters, including a combining
  accent, ZWJ emoji and trailing newline. Ctrl+Home reports caret 0; Ctrl+End
  reports 24. Ctrl+Shift+Home selects `[0, 24]` with caret 0; Ctrl+Home followed
  by Ctrl+Shift+End selects the same range with caret 24. Initial Unicode text
  readback matches the full combining/ZWJ string; password readback contains
  only three bullets. This is queried AT-SPI data, not a screen-reader result.

`python3 docs/receipts/wayland-20261025/verify.py` checks the recorded final
contracts, including the queried Text interface and descriptions. Its passing
output is preserved in `verification.txt`; it supplements visual review.

## Gaps found by the original runs

- The initial minimal tree exposes `Confirmed` without the displayed counter
  and an unnamed dialog-body Label. The visual callback succeeds, but this tree
  alone could not qualify the text as accessible. `minimal-followup/` verifies
  the correction through the Label name and row description, as above.
- `nested-modals/inner-atspi.txt` exposes Outer sheet and its controls alongside
  Inner dialog. The keyboard trap passes; lower-modal accessibility isolation
  fails in that original tree. `nested-modals-followup/` verifies the fix; the
  before evidence is retained.
- A resize can remove the visible focus ring while the AT-SPI focused state is
  retained. No claim of focus-ring preservation across output resize is made.
- IME, external-application clipboard exchange, live scale changes, monitor
  moves, hardware acceleration and other compositors remain untested here.
- The seeded Unicode probe renders the family emoji as missing-glyph boxes in
  this font environment. Combining accents, the masked password and selection
  highlights render; complete emoji glyph appearance is unqualified despite
  correct exported Unicode text.
- Disabled-button state export remains the known AccessKit limitation.
- No screen reader runs in this Wayland receipt; no inventory row is accepted.

## Failed and diagnostic attempts

- `failed-diagnostic-window-lookup/`: Rust 1.98.1 runtime tools launched a
  **preserved Rust 1.95 binary** only to test compositor capability. The client
  produced Wayland buffer traffic, but the harness guessed the wrong Weston X
  window title and exited before captures. This is diagnostic evidence and
  does not qualify the current source or Rust 1.98.1 build. The harness now
  obtains the X window id from Weston's own startup log.
- `minimal-attempt1/`: the intended OK activation used one Tab after opening.
  Initial focus is the dialog container, so that Tab correctly selected Cancel;
  Enter canceled and the counter stayed 0. The screenshots and action labels
  are retained. `minimal/` corrects the sequence with two Tabs.
- An earlier fresh-container launch exhausted the Docker vfs disk before any
  display processes started (`mkdir: No space left on device`). Subsequent
  sessions shared one persistent runtime container. That startup failure
  supplies no GUI result.

## Reproduce

Build all targets with Rust 1.98.1 and locked dependencies first. Install Weston,
Xvfb, Openbox, xdotool, ImageMagick, D-Bus, wayland-utils, Mesa Vulkan drivers,
at-spi2-core and python3-pyatspi. The harness starts a private D-Bus session,
dedicated Xvfb and Weston; it removes `DISPLAY` only from the GPUI process.
`WESTON_X11_DISPLAY` selects a free display (default `:191`).

```sh
WESTON_X11_DISPLAY=:194 \
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json \
STEPS='open:@450,110|cancel:Tab|ok:Tab|wrap:Tab|back:shift+Tab|confirm:Return|reopen:Return|dismiss:Escape|resize:size=640x600|resizeback:size=900x800' \
  scripts/linux/wayland-session.sh /tmp/wegpui-wayland-minimal \
  "${CARGO_TARGET_DIR:-target}/debug/examples/minimal"
```

Each run also records `*-details.json` with descriptions and Text content; final
runs include Text caret/selection queries. Protocol logs are losslessly gzip
archived after review.

The `actions.log` in each other run provides its exact remaining sequence. Pass
those tokens through `STEPS` with the matching example binary.
