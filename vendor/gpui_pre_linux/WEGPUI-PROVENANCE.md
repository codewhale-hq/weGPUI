# gpui-pre-linux 0.3.9 compatibility provenance

The owner explicitly approved this additional local compatibility copy on 2026-10-25.
It was extracted from the existing locked crates.io archive, without editing the registry.

```text
Package: gpui-pre-linux 0.3.9
Archive: gpui-pre-linux-0.3.9.crate
Archive SHA-256: e28213c3fde6f13e91ba0b74b90cbaadec9373e36a031eda98561d31059870f3
Original src/linux/x11/client.rs SHA-256:
9dc83361d502a8d0ed1729276f0d5f4eff97b584eb73acf233968016aa7b1057
Modified src/linux/x11/client.rs SHA-256:
5f7ae89fac0192dbbe8d177d344e706e32f0ddeb5d63473a5d070eef7d262aa3
```

The archive hash matches the prior Cargo.lock registry checksum. The exact proposal
and original file hashes are in `docs/proposals/x11-grab-focus/`. Only
`src/linux/x11/client.rs` differs from the original upstream files: twelve lines
ignore NotifyGrab/NotifyUngrab in FocusIn/FocusOut before activation, accessibility
focus and keyboard/IME state changes. Normal/WhileGrabbed and detail handling are
unchanged. This provenance file is additional local documentation.

The original Cargo manifests, dependency requirements, feature definitions and
Apache-2.0 license are retained byte-for-byte. The root path patch selects the same
package/version locally; no package or dependency version is upgraded and no crate
is published. Application workspace roots must apply the documented patch table;
patches in a dependency are not inherited.
