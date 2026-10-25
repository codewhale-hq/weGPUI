# Approved modal action guards — Linux, 2026-10-25

The approved guard fixes the sampled saved-action failures: all seven Click/Focus timing
cases preserve the opener and emit no closed-model callback. Six additional caller contracts
verify immediate close/reopen, passive content, ancestor scopes and two-window isolation.
The [fixed cached-view run](cached/README.md) keeps targets/actions available after parent-only
redraws. These are scoped Linux observations, not complete component or reader acceptance.

## Source and executable provenance

The owner approved the exact [guard/cache proposal](../../../proposals/modal-a11y-action-guard/README.md)
and the separate X11 Grab/Ungrab focus filter. They were applied to baseline `3840b47`, retaining
Rust 1.98.1 and the existing package versions. Every consumer adopts all four explicit
workspace-root patches. The library now requires the new patched-GPUI method and does not
compile against unpatched registry GPUI 0.3.9; dependency manifests do not propagate patch tables.
No published fork, registry modification, release or distribution is qualified here.

[consumer-artifacts.txt](consumer-artifacts.txt) records original Cargo executable hashes,
the copied runtime hashes and GNU strip 2.44 commands. Only new copies had debug sections
removed, in the same shell operation as their copy; Cargo outputs and old tested artifacts
were not stripped. The executed hashes are:

| Fixture | Original build SHA-256 | Stripped runtime SHA-256 |
| --- | --- | --- |
| Saved Click/Focus | `ef78a1aacef363aad339fee3a6ebacac44e204a443535779a947277b25f47a81` | `d4aac72ad1f93e76bb5083d9a387cd4d5a14acfc811e33213429a25ae854776e` |
| Cached/live | `7e5690d4d2a28eb04f5bf82041576417c6fc1dac206441e69ddd2f77416b05de` | `c6b2ba7e82c0241424d0ddc01383baba4c993dc54374b06022555c30e180333d` |
| Extended caller contracts | `43aac54838d1ef4c00eeecb82bad79b12a62135e5983ceb3c1ac39fd5d3f2cff` | `63650f5c0d66213ed27c9477930171963ea1b472f2914382be39f2bb4757d4ce` |

The immutable executables are under `/workspace/.wegpui-cloud/artifacts/native-guard-approved/`.
They are not committed. Per-run source diffs, consumer Cargo files/source, full vendor hash
manifests, toolchain/OS versions, collector copies, native captures, callback logs and fresh
AT-SPI trees identify each run. The extended fixture's exact source is
[contracts-probe.rs.txt](contracts-probe.rs.txt); its successful locked/offline build and hashes
are in [contracts-build.log](contracts-build.log) and [contracts-artifact.txt](contracts-artifact.txt).
New unexecuted Cargo probe binaries and their same-inode deps links were removed after the
stripped copies/hashes were secured to avoid exhausting disk; [cleanup record](build-cache-cleanup.json)
identifies those rebuildable outputs. Old runtime artifacts and original failure receipts remain.

The runtime uses Debian 13.5 x86_64, Xvfb plus Openbox, private D-Bus/AT-SPI sessions, scale 1
and real XTEST input. Timing cases use display 103; caller contracts use 116. Actual AT requests
are separate from screen-reader sessions. No Orca/audio session is part of this collector.

## Source gates and the vendor test bound

[gates.log](gates.log) records the combined-source five-gate PASS with Rust/Cargo 1.98.1,
locked/offline where applicable: fmt, all-target build, 47 root unit tests, one ignored doctest,
strict all-target Clippy and strict rustdoc. No production Rust changed after that gate run.

The pre-commit [final rerun](final-gates-rerun.log) also passes all five gates with
47 tests and one ignored doctest. It sets `RUSTUP_TOOLCHAIN=1.98.1`,
`CARGO_HOME=/workspace/.wegpui-cloud/cargo` and
`CARGO_TARGET_DIR=/workspace/.wegpui-cloud/target-198` inside `wegpui-198`, then runs
`cargo fmt --check`, `cargo build --all-targets --locked --offline`,
`cargo test --locked --offline`,
`cargo clippy --all-targets --locked --offline -- -D warnings` and
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline` in that order.
An earlier [launch failure](final-gates-preflight-failure.log) used a login shell
that hid the Rust tools; it stopped before compilation. The successful rerun uses
the container's configured path. All seven recorded applied-source hashes still match.

The explicit locked/offline GPUI Cargo unit attempt is retained in
[vendor-unit-attempt.log](vendor-unit-attempt.log). It cannot build the published upstream test
configuration: IBM Plex/Lilex fixture files are absent, and optional test-support dependencies
such as proptest/postage/futures-concurrency/util_macros are unavailable in the existing graph.
No package, feature or lock change was made to bypass that failure. **The full vendor unit
suite did not pass and the new tests were not executed through that Cargo invocation.**

As a separate supplemental check, the production `A11yNodeBuilder` text and its two new tests
were extracted verbatim and compiled with rustc --test against already-built locked
AccessKit/smallvec/log artifacts. Both regressions pass: real/synthetic nodes track changed
predicates immediately; nested guard references compose and frame reset clears metadata.
[metadata-extracted-tests.log](metadata-extracted-tests.log),
[metadata-extraction.json](metadata-extraction.json) and the
[extracted source](metadata-extracted.rs.txt) record the exact source hashes, existing rlibs
and limits. This harness substitutes std map/String aliases and disables debug-only metadata;
it is not the complete GPUI crate unit suite and does not turn 47 root tests into 49.

## Native results

[verify.py](verify.py) validates the recorded counters, controls and focus trees;
[verification.txt](verification.txt) and [summary.json](summary.json) record its result.
AT true return values alone are never treated as a successful state change.

| Case | Observations |
| --- | --- |
| Saved custom Click 0/1/20/100/500ms | Normal open true/false callbacks and reopen callback work. Saved calls during logical close emit no callback, including near 0ms calls that return true. Opener is focused after full close. |
| Saved built-in Focus 0/1ms | Open Component.grabFocus moves Dialog→row. Saved Focus after hide returns true or encounters a removed target, but cannot steal restored focus. Opener remains focused; reopen works. |
| Ordinary caller callback | A raw custom AT handler without any Presence check runs while open; its saved request after close returns true but cannot mutate or focus the closed child. |
| Two matching callbacks | First callback closes its scope; the second is suppressed within the same action request, before any new tree can remove the target. First count 1, second 0; opener restored. |
| Close→reopen in first callback | First closes/reopens to generation 2. The second old-session callback stays suppressed even though Presence is open again. A new-frame request works and closes normally: first 1→2, second 0 throughout. |
| Passive own targets/background | Open own and background callbacks work. After passive hide, saved own returns true without firing; background continues1→2→3 while closed. Reopening permits the fresh own callback 1→2. No page focus/trap/modal restriction is added. |
| Closed ancestor/live child | The fixture intentionally leaves inner Presence true while closing its outer owner. Saved inner Click and Focus return true but do not fire/steal focus. Parent opener restores after the outer fade. |
| Two windows | Window A closes and rejects its saved caller callback; independent window B background callbacks continue 1→2. Window A opener restores. |
| AT-active cached view | Both parent-only redraws retain cached/live nodes. Cached/live render counts advance 7→8 and 13→14; retained and fresh Click/Focus controls all work. No stale-object errors occur in the successful fixed run. |

The original [failing 0/1ms Click/Focus runs](../custom-actions-followup/README.md) and
[cached omission](../custom-actions-followup/cached-baseline/README.md) remain unchanged.
The fixed cache runner's first attempt failed to wait for Openbox readiness; that infrastructure
failure is preserved under `cached/run-1`, while `cached/run-2` waits for both X and WM readiness
and completes. No component pass is taken from the failed launch.

Representative multiple-reopen, passive-closing, ancestor-closing and two-window closed
captures were opened and reviewed. The native panels/page render; callbacks and focus are
qualified by the logs/trees, not their appearance alone. Normal nested keyboard/tree, X11
focus-grab/Fcitx and actual Orca followups are separate root/Wayland receipts on the same graph.

## Scope

The guard enforces closed/reopened-session liveness. It does not authorize only the topmost
modal while several remain logically open; arbitrary external deferred priorities remain
outside that claim. The ancestor test proves input gating while a child flag remains true,
not automatic child close, animation inheritance or announcement orchestration. Applications
still own child lifecycles. Cached AT-active correctness is sampled; cache performance and
activation/deactivation transitions are not measured. This is not full focus/theme/state,
actual screen-reader, hardware or cross-platform acceptance. No component row is fully accepted.
