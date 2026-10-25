# Approved accessibility compatibility copies

The owner approved two successive compatibility patches on 2026-10-25. Versions are unchanged:
`accesskit_unix 0.22.1`, `accesskit_atspi_common 0.19.1` and `gpui-pre 0.3.9`. Copies came from the exact
locked crate packages; licenses and package provenance are retained in each directory.
Registry files were not edited. The root Cargo patch selects only these local copies.

The exact delta, original file checksums and wire-format proof are in
`docs/proposals/accesskit-linux-compat/` and `docs/proposals/a11y-state-and-bounds/`.
The approved changes wrap AT-SPI cache signal struct arguments; distinguish disabled from
read-only; forward invalid, busy, expanded/collapsed and heading-level metadata; and call
GPUI's existing window-bounds hook when bounds change and before accessibility tree updates.
The only modified upstream source files are `accesskit_unix/src/atspi/bus.rs`,
`accesskit_atspi_common/src/node.rs` and `gpui_pre/src/window.rs`.

Actual Linux runtime receipts are in `docs/receipts/linux-bridge-20261025/`,
`input-contracts-20261025/`, `data-contracts-20261025/` and
`wayland-20261025/bridge-followup/`. Their exact artifact hashes distinguish each run.
The cache fix restores the sampled Orca close/reopen sequence; X11 text screen origins now
match the actual client position at scales 1 and 2 and after moving. These are sampled results,
not full screen-reader acceptance. The bridge still has no EditableText interface.

Cargo patches apply at an application's workspace root. Downstream applications depending
on weGPUI must apply equivalent patches there to use these fixes; dependency-manifest patch
sections are not inherited. Publication of forked crates or a change to GPUI 0.3.9 is not
part of this approval. Full accessibility/platform qualification remains separate.
