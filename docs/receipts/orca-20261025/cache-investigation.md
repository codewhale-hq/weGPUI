# Orca modal reopen: AT-SPI cache investigation

The captured run establishes a **cache-signal protocol defect in the pinned dependency stack**.
It also records a real screen-reader failure on modal reopen. The visual window and a fresh
AT-SPI client show the reopened modal, while Orca retains objects marked defunct and skips
announcing them. This is not a screen-reader acceptance pass.

## Observed sequence

Source/toolchain/binary provenance is in [minimal-fixed/provenance.txt](minimal-fixed/provenance.txt):
`52650f3` plus the recorded dirty source hash, Rust 1.98.1, GPUI 0.3.9, Debian 13,
Orca 48.1 and AT-SPI 2.56.2. The action timestamps are UTC.

| Time / step | Orca debug evidence |
| --- | --- |
| 07:33:28 first open | `SPEECH OUTPUT: 'weGPUI dialog A WeUI dialog rendered by native GPUI components.'` |
| 07:33:33 / 35 / 37 Tab | `Cancel button.`, `OK button.`, `Cancel button.` |
| 07:33:40 Escape | `Open dialog button.`; trigger focus is restored |
| 07:33:42 reopen | Focus event arrives for dialog `weGPUI`, but Orca logs `Ignoring defunct object` |
| 07:33:44 / 46 Tab | Orca ignores defunct `Cancel` and `OK` objects; speech contains key echo only |

See [actions.txt](minimal-fixed/actions.txt),
[orca-debug.log](minimal-fixed/orca-debug.log) (speech at lines 901, 1915, 2144, 2372 and 2621;
reopen rejection at 2744; button rejections at 2926 and 3023),
[reopen-atspi.txt](minimal-fixed/reopen-atspi.txt),
[to-ok-atspi.txt](minimal-fixed/to-ok-atspi.txt), and
[reopen.png](minimal-fixed/reopen.png).
The new AT-SPI process sees a focused modal with its body and buttons and does not report those
objects defunct. That fresh-client result does not replace Orca's observed failure.

## Exact signature incompatibility

[orca.log](minimal-fixed/orca.log) records both failures, including at 07:33:42.552 on reopen:

| Cache signal | Required signature | Observed signature |
| --- | --- | --- |
| `AddAccessible` | `((so)(so)(so)iiassusau)` | `(so)(so)(so)iiassusau` |
| `RemoveAccessible` | `(so)` | `so` |

Each signal requires **one struct argument**. The observed messages flatten that outer struct
into separate body arguments. This is unrelated to the old pre-2015 cache representation,
which uses `((so)(so)(so)a(so)assusau)` and is also accepted by libatspi.

Static trace of the exact Cargo.lock versions, read from the local registry without editing it:

1. `accesskit_unix 0.22.1`, `src/atspi/bus.rs:395–425`: `emit_cache_add` passes `&item`
   directly to `emit_cache_signal` (line 399); `emit_cache_remove` passes `&reference`
   (410). The helper forwards that body directly into `zbus::Connection::emit_signal`.
   [Versioned source](https://docs.rs/crate/accesskit_unix/0.22.1/source/src/atspi/bus.rs).
2. `atspi-common 0.13.0`, `src/cache.rs:13` defines the struct fields, and its bundled
   `xml/Cache.xml:90–102` specifies the required single-struct signal signatures above.
   [Versioned protocol XML](https://docs.rs/crate/atspi-common/0.13.0/source/xml/Cache.xml).
3. `zbus 5.19.0`, `src/connection/mod.rs:344–373` builds the signal from the supplied body.
   `src/message/fields.rs:73–92` explicitly serializes a body signature using
   `to_string_no_parens()`. Passing the cache struct as the whole body therefore removes
   exactly the struct wrapper required as the signal's sole argument. The locked
   `zvariant` version is 5.15.0.
   [Versioned serializer](https://docs.rs/crate/zbus/5.19.0/source/src/message/fields.rs).
4. The receiver source matching the installed AT-SPI version independently confirms rejection:
   GNOME `at-spi2-core 2.56.2`, `atspi/atspi-misc.c:881–899` accepts only the current or
   legacy **wrapped** Add signature and returns immediately after the exact warning seen
   here. Lines 505–516 reject Remove unless it is `(so)`.
   [Receiver source](https://github.com/GNOME/at-spi2-core/blob/2.56.2/atspi/atspi-misc.c#L881).

## Why the same object can stay defunct

`accesskit_atspi_common 0.19.1`, `src/adapter.rs:91–108`, emits a normal
`StateChanged(Defunct, true)` when a node is removed from the platform tree, then the cache
Remove signal and interface removal. Its `add_node` path (49–61) registers interfaces and emits
the cache Add signal; it does not separately emit `Defunct(false)`.
Hiding the background/modal subtree in weGPUI exercises this normal remove/re-add path.

With the malformed Remove ignored, the libatspi client can retain its existing object; with
malformed Add ignored, that object's cached state is not refreshed. The receiver's valid Add
handler would call `_atspi_dbus_set_state` at `atspi-misc.c:686`; valid Remove would dispose
and remove the old object at lines 523–528. This explains how Orca can continue seeing defunct
objects while a fresh client reads the live tree successfully.

The signature defect is proven by the log and source trace. Its connection to the stale
Orca objects is strongly supported by the matching lifecycle, but no corrected dependency
was run, so this investigation is **not a verified remediation** and does not rule out an
additional lifecycle issue after the protocol defect is corrected. The evidence does not
establish a separate weGPUI node-ID or focus-restoration bug in this sequence.

## Remaining action

Keep modal reopen/restoration screen-reader acceptance open. A corrected dependency build
requires the owner's approval under the no-fork/no-registry-patch constraint. Any approved
future candidate should verify both signal signatures and repeat open → Tab cycle → Escape →
reopen → Tab → confirm with the same Orca process, preserving speech and audio. Also retest
background controls and nested modal re-exposure because they share the remove/re-add path.
No dependency source, Cargo pin, registry file or external issue was changed by this investigation.
