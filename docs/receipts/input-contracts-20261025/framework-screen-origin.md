# X11 screen-coordinate origin — baseline gap and approved correction

Observed with immutable input probe `826147ddd6af7c7e98d1fc1f23a69d0689d0dd804f13c1aa01d6930b79f4738c`,
Rust 1.98.1, Xvfb/openbox, scale 2. See `attempt3-scale2/runtime.json` and provenance.

- XTEST/X11 reports client position `(321,207)`, size `960×1440` physical pixels.
- AT-SPI Text character extents requested in desktop coordinates are `(40,234,23,56)`.
- The same extents requested in window coordinates are also `(40,234,23,56)`.
- Text getOffsetAtPoint round-trips inside that coordinate system, but desktop coordinates
  lack the window's screen origin. This does not qualify magnifier/highlight screen placement.

The form agent independently observed the same mismatch for Component extents on a moved
native window. Reproduction drivers use window-relative XTEST coordinates rather than
assuming these desktop extents can drive screen coordinates.

## Pinned-source boundary

`gpui-pre-linux 0.3.9/src/linux/x11/window.rs:2010` implements
`PlatformWindow::a11y_update_window_bounds`, calculates scaled outer/inner window rectangles,
and calls `accesskit_unix::Adapter::set_root_window_bounds` at line 2036.

`gpui-pre 0.3.9/src/platform.rs:1393` declares that trait hook, but a complete source search
finds no call to it anywhere in the core. The adapter consequently retains its default
window origin. `Window::platform_window` is `pub(crate)` at `window.rs:1220`; the component
library has no supported public route to invoke this hook. The component's own text-run
geometry follows GPUI's `Element` builder convention: physical **window** coordinates.

The AccessKit adapter's setter documentation specifically says it is useful on X11; Wayland
applications generally cannot determine their global window origin. Wayland global screen
coordinates therefore need their own platform policy and evidence.

## Approved framework correction and follow-up evidence

A GPUI fix should call the existing platform hook after adapter initialization and on window
geometry changes, outside any platform-state borrow. Test a moved X11 window at scales 1/2,
including client/decorated insets, and compare both Text and Component desktop/window extents.
Adding a screen offset in each component would violate GPUI's coordinate convention and
could double-transform correct backends, so this continuation does not do that.

No GPUI registry source was edited and no upstream post was made for this gap. It remains
explicitly separate from the owner-approved AccessKit cache/disabled-state compatibility work.

The owner subsequently approved invoking this existing GPUI hook from `bounds_changed` and
before accessibility tree delivery. Build 5 includes that reviewed compatibility change.
`core-scale1/runtime.json` now reports Invalid state forwarding and correct X11 desktop
translation before and after a real window move: desktop minus window text coordinates
matches raw `xwininfo -id` absolute client coordinates. It also verifies the inserted glyph
remains inside its control after cross-chunk replacement.

`xdotool getwindowgeometry` differs from raw X server coordinates by the window's relative
SSD offset in this environment. That discrepancy must not be mistaken for an AT-SPI error.
The scale-2 follow-up also passes direct `XTranslateCoordinates(client, root, 0,0)` through
libX11: initial client `(320,187)` and moved client `(101,100)` match raw xwininfo and desktop
minus window text extents exactly. Both scales pass the origin checks. Results remain scoped
to the recorded source/artifact and this X11 host; other desktop platforms still need reruns.
