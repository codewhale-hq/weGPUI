# Uploader, gallery and swiped-cell contracts — Linux, 2026-10-25

This continuation starts from GitHub main `9c4515c`. The exact source and binary of each runtime
are recorded separately. Native callback/state observations, accessibility trees and actual
screen-reader acceptance are distinct; these receipts do not accept an inventory row.

## Pinned upstream contracts

`upstream-source-sha256.txt` identifies the inspected WeUI 2.6.26 example HTML/CSS and
weui.js 1.2.17 gallery/uploader sources in the verified upstream cache.

- The uploader example previews done thumbnails, excludes error/progress thumbnails from that
  activation, supports deletion inside the first viewer and directly on the second grid, and
  appends files chosen through its file input. The transport-oriented JS helper adds queue,
  progress/success/error and configurable before-queue hooks. Its documented example enforces
  capacity in application callbacks. Native choosing, transport, retry and status updates remain
  application-owned; this slice does not implement HTTP transport or image compression.
- The gallery has one contain-fit image, dismissal and deletion. Its delete link is a block
  filling the 60px operation bar. Neither pinned gallery source implements pinch, zoom or paging;
  those are not required parity gaps. The uploader's image is programmatically focusable; the
  JS helper also supplies a close control.
- The list example shows a statically translated row with an inert delete link. Native drag,
  snap timing, keyboard commands and controlled requests are explicit desktop adaptations.

## Implemented reusable behavior

- `Gallery` exposes the dismissible image as a keyboard stop: Enter/Space calls `on_dismiss`.
  Escape still dismisses. The full 60px delete bar now activates `on_delete`, matching its
  upstream hit area. Caller-owned `Presence` and removal policy remain explicit.
- `Uploader` supports a configurable thumbnail hint, an optional maximum file count and an
  externally tracked add-focus handle. Capacity hides the add box without discarding supplied
  files; error/progress files count too. Hiding a focused add box advances focus or blurs if
  there is no surviving stop. A visible add box without an add callback is not a Tab stop.
  `UploaderFile::status` sets presentation state without starting transport. Gallery example
  deletion restores focus to the corresponding add box, including deletion of the last image.
- `SwipedCell::open` and `on_open_change` support controlled requests; existing `default_open`
  remains an uncontrolled initializer. Repeated requests matching current state are ignored.
  Escape/Right also work from revealed actions and restore row focus. Tab from an open row
  reaches the actions in logical order; exiting bubbles to the surrounding app/modal handler.
  Controlled closure while an action is focused returns focus to the row. A row without
  actions has no extra Tab stop or expanded state.

## Probe and evidence

`media_contract_probe.rs.txt` preserves the complete probe. It owns actual upload files and
status changes, removes files in delete callbacks, opens viewers through thumbnail callbacks,
and either accepts or rejects controlled swipe requests. It does not treat inert gallery demo
callbacks or preselected startup states as interaction evidence.

`run.sh` uses a private D-Bus session with Xvfb **and Openbox**, scale 1, real XTEST pointer/key
input, window captures and AT-SPI snapshots. Its target coordinates come from accessible bounds;
activation is then performed by actual pointer events. No AT-SPI action invocation substitutes
for the thumbnail clicks. `atspi_control.py` records roles, names, descriptions, states and bounds.

The first isolated candidate build failed; `candidate-build.txt` and `candidate-source.json`
are retained. It found a real probe API typo (`Button::small` instead of `mini`, corrected).
It also reported missing new methods despite their presence in the snapshot, so that build
provides no runtime result. Later integrated source/build receipts are the qualification basis.


## Runtime observations

`runtime/` uses the integrated Rust 1.98.1 binary with the two owner-approved local AccessKit
compatibility patches. The registry was not modified. The probe and gallery executable copies
are retained outside Git under `.wegpui-cloud/artifacts/media-integrated/`; their hashes and
Cargo.lock hash are in each run. `vendor-source-sha256.txt` records the local bridge sources.

- Add appends Image 2 as 20% progress. Capacity 2 hides Add and moves focus to Fail upload.
  Fail/Complete update that file to error and done. Error/progress thumbnails are not preview
  targets; completing makes Image 2 previewable. These status changes are application callbacks,
  not a network upload result.
- Actual pointer clicks on Image 1 open the reusable viewer. Enter on its image, pointer activation
  of the image, and Escape each call the dismiss callback once and restore thumbnail focus.
- Clicking the far left of the operation bar deletes Image 1. Keyboard Space on the delete bar
  deletes Image 2. Both return focus to Add. A subsequent standalone thumbnail delete removes
  Image 3 and leaves the empty uploader with Add available. Logs and state snapshots distinguish
  all three actual removal callbacks.
- Left requests opening; a repeated Left after rendering emits no duplicate callback. Tab reaches
  Archive, and Escape from that action closes and returns focus to the row. Archive and Delete row
  each invoke their callback once, and their controlled closure restores row focus. Rejected
  opening remains closed. A real pointer drag opens it; Right closes it. The no-action row is
  not focusable and does not expose actions.
- `focus-runtime/` captures and verifies the gallery image's white focus rectangle and the full
  operation bar's white rectangle. Both were visually reviewed.
- `gallery-dark-care/` and `gallery-brand/` drive the actual `uploader` gallery route, not startup
  viewer states. They click a thumbnail, use image Enter dismissal and keyboard deletion, and
  observe five accessible done images become four. Add receives focus after deletion. The
  dark+care and light/custom-brand `8a3ffc` captures and focus rings were visually reviewed.

`verify.py` checks these recorded data/callback contracts; its result is in `verification.txt`.
It does not compare pixel parity or provide a screen-reader verdict.

## Accessibility default action correction

`a11y-before/` preserves a real AT-SPI failure on the otherwise working pointer/keyboard binary.
The row exposed `['click']`; two successful `doAction(click)` calls produced no state change or
callback. GPUI's default Click fallback synthesizes pointer input, while the row previously
only toggled a keyboard-generated click. The library now explicitly handles accessibility
Click through the same controlled request path and focuses the row. Explicit AccessKit
Expand/Collapse remain available for adapters that export them.

The recorded artifact's pinned AT-SPI bridge exports only Click and does not forward expanded/expandable state.
The row's description now also states whether operations are expanded or collapsed, while the
source still supplies the native AccessKit expanded property. This is an application-level
fallback, not a third bridge patch or full screen-reader acceptance.

`a11y-after/` verifies that correction on the fresh integrated binary: two actual AT-SPI Click
requests open and close the controlled row, emit one true/false callback each, focus the row,
and update its description to 已展开/已收起. It also clicks the progress and error statuses
and verifies neither opens a preview. The static no-action row retains its original `bg_2`
surface. These captures were visually reviewed. Earlier full pointer/keyboard matrices use
the preceding binary; this final rerun targets only the added accessibility/default-action and
static-row changes. Each run retains its own artifact/source hashes.

A subsequent owner-approved bridge extension forwards Expandable and Expanded/Collapsed
states. Its separate [Wayland bridge follow-up](../wayland-20261025/bridge-followup/README.md)
verifies actual state transitions on a newer artifact. The original observations and
artifact manifests above remain unchanged.

## Failed attempts and remaining scope

- `runtime-attempt1/` used desktop coordinates from AT-SPI as absolute X coordinates. With the
  Openbox decoration offset, small Fail/Complete buttons were missed. Large thumbnails and
  the delete bar still activated; the run stopped when Image 2 remained nonpreviewable progress.
  The driver now normalizes bounds to the frame and uses `mousemove --window` for actual native
  pointer events. This was a driver failure, preserved separately from the passing callback run.
- The neutral swipe action inherits white text on `BG-0`, exactly as pinned upstream CSS does;
  the light sample is low contrast. No palette change was silently introduced. Complete neutral
  action contrast acceptance remains open.
- Actual screen-reader announcements/order, all themes/states on every desktop, live image error
  handling, transport/compression/retry policy and platform file choosers remain outside these
  sampled callback receipts. Earlier chooser lifecycle receipts remain separate.
- The original gallery demo's static `3/10` header is upstream sample text. File removal is proved
  from actual image counts; that unchanged static string is not used as a state assertion.
