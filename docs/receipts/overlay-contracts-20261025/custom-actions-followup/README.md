# Prepared saved custom-action probe

**Not built or run. No defect or passing result is established here.** The current batch's
closing guard covers raw pointer/default-Click input and the built-in overlay callbacks tested
in the parent receipt. Explicit custom accessibility listeners use another dispatch path;
this followup will determine whether an owned SwipedCell request can still arrive during a
slow closing Modal. Arbitrary application handlers remain the caller's model responsibility.

`probe.rs.txt` is a small public-API fixture, prepared after the thirteen-case run. Copy it
to `examples/saved_overlay_action_probe.rs` only when the integrating agent allocates Cargo;
then format/build on the complete new source with the exact lockfile and record hashes.
The source has not been compiled, so API/driver corrections may be needed. The fixture uses
a controlled SwipedCell with a normal application callback; it deliberately does not add a
Presence check in that callback. It logs each request and whether Presence is logically open.

Planned actual AT-SPI procedure, in a fresh D-Bus/Xvfb/Openbox session at scale 1:

1. Start the copied executable, capture its SHA/source/toolchain/patch manifests, focus it and
   click **Open slow modal** with native pointer input. Wait until the two-second opening settles.
2. Find **Saved custom action row** in a fresh AT-SPI tree. Save that exact object proxy and its
   Action interface, including action names. Invoke its Click once and verify exactly one
   `open=true presence_open=true` request, Expanded state and visible actions; invoke again and
   verify exactly one close request. This establishes the custom-action baseline.
3. Save the row proxy/interface again. Send native Escape. Immediately call its saved Click,
   measuring dispatch delay from Escape with `time.monotonic_ns`. Repeat separate fresh sessions
   at delays 0ms, 5ms, 20ms, 100ms and 500ms so a missed frame is not reported as a race proof.
4. Preserve API return/error, the full app event log, a fresh tree and a native window capture
   before Escape, during the fade and after it. A rejected/defunct proxy alone is not a native
   callback result. Inspect whether a `request` arrives with `presence_open=false`; only that
   observation would establish the suspected closing-dispatch gap.
5. After the fade, reopen with native pointer input. Verify the row remains keyboard/AT
   operable and the opener restores after closing; do not accept a fix that merely disables
   accessibility actions permanently.

The same-session saved proxy matters: fresh-client tree lookup after hiding may no longer
find the hidden node and would not exercise a queued action. The existing approved cache
compatibility copies must remain in provenance. This plan is neither actual screen-reader
speech nor acceptance of a component, theme, host or timing matrix. Any failure must remain
alongside its later corrected result.

`drive.py.txt` preserves an unexecuted pyatspi/XTEST collector for that sequence. Its Python
syntax was checked without importing pyatspi or launching an app. After copying it to a local
`.py` path in the app's accessibility session, run one fresh app/session per delay:

```sh
python3 drive.py --window X_CLIENT_ID --delay-ms 20 --out NEW_OUTPUT_DIRECTORY
```

The collector saves proxy return/errors, measured command/action times, fresh trees and
window captures. Retain the application's stderr alongside them to determine whether a
callback arrived with Presence false. The escape timestamp records the XTEST command's
start/return; it is not a claim of the compositor's actual event delivery time. The driver
currently has no passing or failing runtime qualification.
