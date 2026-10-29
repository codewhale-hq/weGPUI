# Toptips re-show timeout: macOS native probe

Both baseline and artifact-only candidate built offline under the shared build lock and executed as macOS native GPUI applications. The baseline uses clean canonical source at `37ed98edce211a23d9ad3d6ade0b9939fb3da395`; the candidate uses an artifact-only copy with exactly `toptips.patch` applied. Source snapshots before build, after build and after runtime match; their SHA-256 is `e866ccbccd0b148ae4204b30a416fa2402c3da2ab0f318034b32de00a21a21ba`. Candidate patch SHA-256 is `5b34149e8f01a4aee74fadb7a4c8487494dcc9b549950fe49c9828485c56920b`. Both compiled binary hashes in `binary-sha256.txt` were rechecked after runtime.

The fixture renders real `Toptips` in a native GPUI window and triggers its actual show/hide methods through scheduled GPUI callbacks. Every show schedules a two-second `hide_after`; the log records `Instant`-based times, visible state, callback counts and closed-to-open transitions. It does not read the private generation field. No physical input or VoiceOver is used.

| Variant | Motion | First show to overlap | Second show to overlap | Observed state |
| --- | --- | ---: | ---: | --- |
| Baseline | normal | 2.252 s | 1.414 s | closed, 2 show callbacks, 1 open transition |
| Baseline | reduced | 2.252 s | 1.416 s | closed, 2 show callbacks, 1 open transition |
| Candidate | normal | 2.255 s | 1.417 s | open, 2 show callbacks, 1 open transition |
| Candidate | reduced | 2.255 s | 1.416 s | open, 2 show callbacks, 1 open transition |

Both candidate runs closed after the newer deadline. All four runs also passed single-show expiry, explicit hide followed by reopen across the old deadline, and explicit cancellation. For the reopen comparison, the overlap occurred 2.305–2.308 s after the old show and 1.049–1.050 s after the new show. Detailed structured events, counts and timestamps are in `baseline-normal.json`, `baseline-reduced.json`, `candidate-normal.json` and `candidate-reduced.json`; the driver checks the time windows and expected states.

Build: `python3 /Volumes/VIXinSSD/CW/codewhale/scripts/build-lock.py /Volumes/VIXinSSD/cw-build-cache/build.lock -- sh -c '... cargo build --manifest-path baseline/Cargo.toml --locked --offline; cargo build --manifest-path candidate/Cargo.toml --locked --offline ...'`, with `CARGO_TARGET_DIR=/Volumes/VIXinSSD/CW/weGPUI/target`. Full compiler output (`build.log`) and the runtime log remain in the local recovery packet; the structured runtime events and source hashes are checked in here. Runtime: `python3 driver.py > runtime.log 2>&1`; no UI automation, service restart, Windows execution or product source edits were used.

This verifies the bounded macOS state/timer behavior. It does not establish visual pixels, accessibility speech, physical input, Linux candidate behavior, Windows, or mobile behavior. The separate Linux artifact `../toptips-reshow-probe/RECEIPT-PARTIAL.md` records its valid baseline reproduction, failed timing guard and Docker outage; the candidate Linux runtime remains unverified.
