# Actual Orca cache compatibility followup — 2026-10-25

These runs use real Orca 48.1, AT-SPI 2.56.2, speech-dispatcher 0.12.0, eSpeak NG 1.52.0,
PulseAudio 17, Xvfb/openbox and software rendering on Debian 13.5. They are reader observations,
separate from the D-Bus wire-format proof and accessibility tree snapshots.

The owner approved the two exact local compatibility changes in
`../../proposals/accesskit-linux-compat/`. No registry source or package version changed.

| Run | Artifact SHA-256 | Result |
| --- | --- | --- |
| `orca-before-compat-old-artifact` | `140ae671543c5ecade562d93ea74a12f40d5de27dfc2916cabaa11dbcd03b815` | Old minimal executable retained as a control. Orca logs malformed Cache AddAccessible signatures; do not treat the contemporaneous dirty working-tree hash as this binary's build attestation. |
| `orca-after-compat` | `a871ba61d54fc5b4c4ddb61f91af35b9a9e8778ae1c21b5a1d42bcf0f6172199` | Rebuilt with approved cache/state copies on Rust 1.98.1. Repeated open/close/reopen and confirmation/restoration sampled successfully; exit 0. |

The after transcript announces the initial dialog title/body, Cancel and OK, wraps Tab to
Cancel, restores Open dialog after Escape, reads the same dialog after Return, reads Cancel
and OK again, restores the opener after confirmation and reads a third opening. There are
no malformed cache-signature warnings in that run. Debug traces can still mark legitimately
removed closed objects defunct; this is not a claim that every debug line or reader sequence
is free of defunct objects.

The recorded after PCM is 24.947375 seconds, mono 16-bit at 16 kHz, with nonzero signal.
The transcript, PCM statistics and screenshots were inspected. Audio was **not reviewed by
listening**, so intelligibility acceptance remains open. These samples do not accept every
component, reader, desktop or host. Current-source input/date reader and Wayland/state/origin
followups are in `../input-contracts-20261025/`, `../form-contracts-20261025/` and
`../wayland-20261025/bridge-followup/`.

Each directory retains exact actions, command/environment/toolchain/binary provenance,
tree/Text snapshots, screenshots, debug logs, audio and run status. Failures are preserved.
Cargo patches must be adopted at downstream application workspace roots; see
`../../../vendor/README.md`. No upstream issue, external post or publication was made.
