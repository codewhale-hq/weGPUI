# Approved Linux bridge follow-up — 2026-10-25

These runs verify the owner-approved additional state forwarding and GPUI window-bounds
hooks on the second continuation's integrated artifacts. The local packages retain
`gpui-pre` 0.3.9, `accesskit_atspi_common` 0.19.1 and `accesskit_unix` 0.22.1.
The preceding media and Wayland receipts retain their original artifacts and observations;
their earlier bridge limitations do not describe these newer artifacts.

## Provenance and method

Parent Git commit: `9c4515c788bbacc42a166534e39dc393ddbcb30b`, with the integrated
uncommitted continuation. Each run has `provenance.txt` with its timestamp, binary SHA-256,
Cargo.lock SHA-256 and toolchain. Its `source-sha256.txt` records tracked application source
at runtime. `fixture-and-collector-sha256.txt` additionally records copied probe and collector
sources; `compatibility-source-sha256.txt` records Cargo files and the affected vendored
compatibility sources. Probe source is preserved separately in every run directory.

The root's `components-batch-2-build-5-framework.log` completed the all-target build and
45 tests before these samples. No Cargo build was run by this runtime collector. Five
immutable executable copies were used from
`/workspace/.wegpui-cloud/artifacts/bridge-framework/`; the runtime binary hashes, rather
than that external directory's existence, identify these recorded artifacts.

The environment was Debian 13.5 x86_64, Rust/Cargo 1.98.1, Mesa 25.0.7 lavapipe Vulkan,
Xvfb 21.1.16 and Openbox 3.6.1. Native Wayland runs used Weston 14.0.2's X11 backend,
pixman renderer and kiosk shell, initially 900×800 logical pixels at scale 1. The GPUI
process had **no DISPLAY**. XTEST events went to the compositor's X11 output, which
translated them into Wayland input; no Xwayland client was used. Preserved
`wayland-protocol.log.gz` files include actual `wl_keyboard` and `xdg_toplevel` traffic.
Screenshots capture Weston's output with ImageMagick `import -window`.

## Recorded results

| Run | Verified result |
| --- | --- |
| `media/` | Controlled row initially exports Expandable and Collapsed. Pointer focus followed by Left requests opening, invokes the callback and exports Expandable and Expanded. Tab focuses Archive. Escape closes through the callback, restores row focus and returns to Collapsed. |
| `nested/` | Only Inner dialog is exposed while the inner layer is open. Tab and Shift+Tab remain between Cancel and OK. Escape restores Open inner dialog focus, then another Escape restores Open sheet focus. |
| `text/` | Word editing exports the exact 19-scalar string, combining accent included. Ctrl+End reports caret 19; Ctrl+A reports selection `[0,19]`. Wrapped notes exports the exact 131-scalar multiline/combining/ZWJ string with trailing newline; Ctrl+End reports 131 and Ctrl+Home followed by Ctrl+Shift+End selects `[0,131]`. |
| `text/`, `x11-input/`, `x11-text-bounds/` | Read only exports ReadOnly without Editable. Disabled lacks Enabled, Sensitive, Focusable and Editable. The invalid UTF-16-limit sample exports InvalidEntry. |
| `x11-cells/` | Three loading-cell controls export Busy. Disabled cells lack Enabled and Sensitive. Loading cells remain Enabled/Sensitive and lack Focusable, matching their source contract. |
| `x11-headings/` | Both Repeated heading nodes export heading role with `level:6`. |
| `x11-input/`, `x11-text-bounds/` | Initial and moved Frame origins agree with `xwininfo`'s native absolute client origin. Text, caret and the relative first-character bounds remain unchanged after moving the window. Absolute first-character bounds move by the exact native displacement. |

For the last coordinate sample, native client origin changes from `[1,20]` to `[201,140]`.
Word editing's first-character desktop bounds change from `[21,68,10,28]` to
`[221,188,10,28]`; window-relative bounds remain `[20,48,10,28]`. Thus desktop coordinates
equal window coordinates plus the native client origin before and after the `[200,120]`
move. `*-xwininfo.txt` is the native reference; `xdotool getwindowgeometry` can include a
different reparenting offset and is retained only as diagnostic data.

Representative Wayland open-row, nested-dialog and selected-text captures were opened and
visually reviewed. The controls, modal stacking and selection highlights render. The
installed fonts still show missing-glyph boxes for the family emoji; correct Unicode
export does not qualify that glyph's appearance. This is a targeted regression sample,
not a full visual parity sweep.

## Scope and reproduction

AT-SPI trees, interfaces and queried text are accessibility observations. They are **not
screen-reader acceptance**. These samples do not qualify every component, host, theme,
state, compositor, hardware setup, scaling scenario or interaction. The Wayland platform's
window-bounds hook remains a no-op because that backend has no global window origin;
the X11 absolute-coordinate result must not be applied to Wayland global coordinates.

Run `python3 docs/receipts/wayland-20261025/bridge-followup/verify.py` to check the saved
contracts. `verification.txt` records its passing result. Protocol archives are lossless;
the checker reads either raw or compressed traces.

To reproduce the Wayland media sequence after restoring/building the preserved fixture:

```sh
WESTON_X11_DISPLAY=:104 \
STEPS='row:@450,640|open:Left|first:Tab|close:Escape' \
scripts/linux/wayland-session.sh OUT /absolute/path/to/media_contract_probe
```

Nested sequence: `open:@450,88|inner:Tab Return|cancel:Tab|ok:Tab|wrap:Tab|back:shift+Tab|closeinner:Escape|closeouter:Escape`.
Text sequence: `word-end:ctrl+End|word-selected:ctrl+a|notes:Tab|notes-end:ctrl+End|notes-selected:ctrl+Home ctrl+shift+End`.
The preserved media receipt's `run.sh` drives X11 `snapshot` and `moved` cases using
`MEDIA_BIN`, `MEDIA_SOURCE_FILE` and `MEDIA_CASE`; each invocation starts a private D-Bus,
Xvfb and Openbox session. All temporary probe examples are evidence fixtures, not shipped
library API.

Provenance curation addendum (2026-10-25T09:16:51+00:00): the cache-fix `accesskit_unix/src/atspi/bus.rs`
SHA-256 was appended to `compatibility-source-sha256.txt` after the runtime runs. The root
confirmed that source file had not changed since the approved cache patch. This addition
does not change the per-run frozen binary or runtime-source manifests.
