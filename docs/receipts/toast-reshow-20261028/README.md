# Toast re-show timeout probe

Source identity: clean product code at Git HEAD `e75d635641c349993486b6884703399429dac7b9`; source manifests before build, after build, and after runtime match byte-for-byte. Baseline uses `/src`; candidate uses an artifact-only source copy with the exact patch applied, and the vendor dependencies are the same. `presence.patch.gz` decompresses to the patch with SHA-256: `86573ca7838c2b59e6b7084680573921b98cde56cfbc2ecd7d118fcf65142055`. Binary hashes are in `binary-sha256.txt` and were checked again after runtime.

The fixture renders `Toast::presence_for` and `Toast`, invokes actual show and hide click callbacks using focused keyboard activation, and reads state and notification nodes over Linux AT-SPI. Each show schedules a three-second `Presence::hide_after`. The overlap snapshot is bounded by the first and second activation timestamps, and checks callback counts and the public `generation`.

| Variant | Motion | First activation to snapshot | Second activation to snapshot | Result |
| --- | --- | ---: | ---: | --- |
| Baseline | normal | 3.439 s | 2.177 s | closed; two show callbacks; generation 1 |
| Baseline | reduced | 3.424 s | 2.170 s | closed; two show callbacks; generation 1 |
| Candidate | normal | 3.462 s | 2.204 s | open; two show callbacks; generation 1; AT-SPI notification `Saved toast` |
| Candidate | reduced | 3.444 s | 2.183 s | open; two show callbacks; generation 1; AT-SPI notification `Saved toast` |

The candidate closes after the second deadline. Each run also checks a single show and natural expiry, explicit hide followed by reopen across the original deadline, and explicit hide with no reopen. Expected state, callback counts, and open-generation values passed in all four runs. Raw snapshots and monotonic timestamps are in the four `*-normal.json` and `*-reduced.json` files.

Build was performed using the shared Codewhale build lock, offline and locked, in `wegpui-rtl-qualification` with `CARGO_HOME=/work/cargo`, `CARGO_TARGET_DIR=/target`, `TMPDIR=/work/tmp`, followed by both source snapshot comparison and `sha256sum` of the binaries. Runtime: `docker exec wegpui-rtl-qualification bash /work/full-parity/toast-reshow-probe/run.sh`, which uses isolated Xvfb, software Vulkan, Openbox, an isolated D-Bus session, and AT-SPI. Run output is `runtime.log`.

The first harness attempted AT-SPI click actions while the toast's passive overlay was visible; the second action did not reach the underlying button and its callback count remained one. That failed attempt, its original fixture, and build/source receipts are preserved in `failed-atspi-click/`. It is not evidence of the timeout behavior. The successful driver instead uses keyboard activation of the existing actual click callbacks.

Scope: one Linux software-rendered fixture and accessibility tree; it does not establish macOS, Windows, mobile, screenshot-pixel, or upstream browser parity. Multiple independent delayed hides without an intervening show retain the prior first-deadline-wins behavior. The candidate changes the documented repeat-show timer behavior without changing `Presence::generation()` or the lifecycle generation used by overlay interaction.

The final product source is identified in `product-source.json`. The macOS ARM64 library suite passed 54 tests and strict all-target Clippy. The Windows x64 GNU all-target cross-check passed with the [recorded wrapper](../windows-cross-20261028/README.md); it did not execute Windows tests. Five website inventory tests and selected root/project-prefix browser checks passed on macOS Chromium. The analogous top-tips issue was reproduced, but its candidate runtime validation remains incomplete.
