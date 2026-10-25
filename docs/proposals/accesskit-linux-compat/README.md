# Approved Linux AccessKit compatibility fixes

Status: the owner explicitly approved both local dependency fixes on 2026-10-25. They are
applied in `vendor/`, selected by root Cargo patches, with both package versions unchanged.
Registry files were not edited. The original proposal and proof remain below for provenance.

[compat.patch](compat.patch) is the exact proposed code delta for local vendored copies of
`accesskit_unix 0.22.1` and `accesskit_atspi_common 0.19.1`:

1. Wrap CacheItem/ObjectRef in a one-element tuple for cache Add/Remove signals. Actual Orca
   runs reject the current flattened signatures and retain defunct objects after modal reopen.
2. Export disabled independently from read-only support: disabled buttons must lose
   Enabled/Sensitive; read-only enabled fields retain them, with ReadOnly set separately.
   Disabled text fields should not advertise Editable.

The [standalone probe](signature-probe.rs.txt) was compiled with Rust 1.98.1 against the already
built locked rlibs, without a new Cargo manifest/dependency or registry edit. It checks actual
serialized D-Bus header bytes (the parsed Signature API normalizes away the distinction).
[signature-proof.txt](signature-proof.txt) shows all four assertions pass: current Add/Remove
wire signatures are malformed, while tuple-wrapped messages match the protocol XML exactly.
This proves the signal encoding. The subsequent actual Orca runs in
`../../receipts/linux-bridge-20261025/orca-after-compat/` additionally exercise repeated
dialog close/reopen/restoration without malformed cache-signature warnings. The old-artifact
control is retained separately. Removed closed objects can still legitimately appear as
defunct in debug logs; this does not establish complete reader acceptance or audio intelligibility.

The approved implementation copies the two exact packaged crates (including their licenses) into `vendor/`,
applies this patch, adds root `[patch.crates-io]` entries for those local paths, and updates the
lockfile source references while retaining both versions. Final integrated gates are recorded
in the component-contracts continuation receipt. A separately approved additional patch is
documented in `../a11y-state-and-bounds/`.
No new package version or unrelated dependency is requested. Preserve source checksums and
patch provenance. Repeat actual Orca modal close/reopen/background/nested restoration and
AT-SPI disabled/read-only/editable-state checks before claiming either blocker fixed.

This creates a maintained local dependency fork: consumer applications must inherit equivalent
root patch configuration, and publication of patched crates is not authorized. The library's
registry contents remain untouched. The local copies are maintained source dependencies.
No upstream issue, external message, tag, release or publication is part of this proposal.
