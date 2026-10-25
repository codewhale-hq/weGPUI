# Approved accessibility state and window-origin compatibility

Status: the owner explicitly approved these additional compatibility fixes on 2026-10-25.
They are applied to the existing local AccessKit copy and the exact local `gpui-pre 0.3.9`
package. All three patched packages retain their locked versions; no new package was added.
The earlier two AccessKit fixes and their original proof remain separately documented.

Actual component verification found that the pinned bridge omits InvalidEntry, Busy,
Expandable/Expanded/Collapsed and heading level, despite the native components supplying the
corresponding AccessKit fields. New date errors, loading buttons, swipe/search state and article
headings cannot complete Linux accessibility qualification while those fields are discarded.

Actual scale1/2 Text tests also show AT-SPI DESKTOP extents equal WINDOW extents after moving
the X11 window, omitting its screen origin. `gpui-pre-linux 0.3.9` already implements
`PlatformWindow::a11y_update_window_bounds` and forwards physical outer/inner bounds to AccessKit.
The `gpui-pre 0.3.9` core declares that hook but never invokes it; its platform window is private,
so a component cannot safely repair this through public APIs. See the runtime/source trace in
`docs/receipts/input-contracts-20261025/framework-screen-origin.md`.

[compat.patch](compat.patch) proposes:

- Extend the existing `accesskit_atspi_common 0.19.1` local copy with direct state/heading-level
  forwarding (no new interface or crate).
- Vendor the exact `gpui-pre 0.3.9` package and add only two calls to its existing platform hook:
  when bounds change and immediately before an active accessibility tree update. Platforms
  using the default no-op remain unchanged. The Linux implementation then supplies real origin.

This requires an **additional local framework fork**. GPUI version stays exactly0.3.9; no new
package or public API is proposed. Source/license/provenance would be retained and only the
patch applied. Registry files, hosted forks and published packages would remain untouched.
Cargo patches must be adopted at downstream application workspace roots.

Verification includes the complete five Rust gates, real Invalid/Busy/expanded/level exports,
Orca announcements, X11 initial/moved/scaled/resized text extents and point hit tests, plus
Wayland regressions (Wayland itself does not disclose global screen coordinates). No screen
reader, magnifier, platform or full component acceptance follows from these samples.

Runtime followups are in `../../receipts/input-contracts-20261025/`,
`../../receipts/data-contracts-20261025/` and
`../../receipts/wayland-20261025/bridge-followup/`. Direct X11 coordinate translation and
raw `xwininfo` agree with AT-SPI SCREEN minus WINDOW text extents at scales 1 and 2 and
after moving. `xdotool getwindowgeometry` double-counts decorations in this setup and is
not the authoritative origin measurement. Wayland checks use genuine clients with DISPLAY
absent; the protocol does not expose a global desktop origin. Hardware qualification remains open.
