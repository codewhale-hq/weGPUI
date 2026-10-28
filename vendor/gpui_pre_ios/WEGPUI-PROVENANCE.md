# GPUI iOS compatibility copy

This directory contains `gpui-pre-ios` 0.3.9 from crates.io, licensed under
Apache-2.0. Its published crate archive SHA-256 is
`13e735b73ae85ed188764287df4d24837045d71a122c51c2264aed8c38ffd51b`.
The package identifies its upstream as Zed revision
`b4ecc454225fc1819e37e890b62df5473b2ba95b`. The license is retained in
`LICENSE-APACHE`; registry cache metadata and its independent lockfile are omitted.

The local Cargo manifest enables the UIKit toolbar and bar-button bindings.
`src/ios/text_input.rs` maps the patched GPUI keyboard configuration to UIKit
and provides a Done action above the digit keypad when Done is requested; other
actions use the default keyboard so an action key remains available. The matching GPUI core patch
and the workspace-root patch are required. This copy does not enable native
password entry; that remains fail-closed until secure behavior is verified.
