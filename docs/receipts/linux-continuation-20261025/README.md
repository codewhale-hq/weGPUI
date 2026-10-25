# Linux cloud continuation — 2026-10-25

GitHub main was fetched before work and remained at
`52650f3b830ed7e8d1cee7921d495bff06e0628d`. The initial cloud checkout was clean on
`work`; that branch was preserved and `main` was created to track GitHub. No applicable
AGENTS.md was found. The owner's later uncommitted Mac changes were unavailable here and
were not used or counted as cloud validation. CNB was not accessed for writes.

## Implemented behavior

- Register missing focus-reveal targets on Navbar/Tabbar, ClearableInput and raw gallery
  controls; add the article link ring. Public builder return types stay compatible.
- Export TextInput/TextArea TextRun contents and caret/selection, using native grapheme units,
  hard-line runs and masked password text. Unicode, reversed selection, empty/trailing-newline
  and oversized-grapheme cases have regression tests. Bind Ctrl+Home/End and Shift variants
  to existing document-boundary actions after a real runtime failure.
- Supply values for static Label nodes, expose Cell value/description without changing its
  name, and give the example's independent radio groups separate named parents.
- Hide covered modal layers from assistive technology. Rank ordinary crate modal layers by
  nesting depth then equal-depth layout order, matching GPUI deferred draw rounds. A real
  nested-plus-sibling failure was reproduced and fixed. Application-defined deferred wrappers
  or overlay priorities remain outside this documented ordering contract.
- Add reusable Orca/Wayland evidence collection and care/brand/native-only sweep options.
  No Cargo dependencies, versions, forks, registry patches or date policy changed.

## Final gates

All five required gates passed with **Rust/Cargo 1.98.1**, locked dependencies and gpui-pre
0.3.9. [gates.log](gates.log) is the final run after temporary probes were removed:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo build --all-targets --locked --offline` | PASS |
| `cargo test --locked --offline` | PASS: 24 unit tests; one doctest ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | PASS |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline` | PASS |

[tool-tests.log](tool-tests.log): ten Python fixture tests pass (five existing bundle tests,
five new sweep orchestration tests). Shell syntax, Python compilation and `git diff --check`
pass for source/prose. Raw tool output and exact patch snapshots retain their original
whitespace; they are excluded from the whitespace-only diff check. The temporary probes remain only as source/evidence in receipts and as historical local
build artifacts; they are not shipped examples. The extra earlier gate run including them is
retained in `gates-with-probes.log`.

`source-sha256.json` hashes final source, assets, Cargo files and relevant tools;
`source.patch` records the exact tracked Rust/Cargo delta from the GitHub parent.
`binary-sha256.json` identifies built examples and retained probes. `environment.txt` and
`Dockerfile.txt` record the toolchain and runtime setup. The host has no Cargo: tools ran in
one shared local Docker container with the configured proxy and CA trust. The Rust 1.95
base image was preserved; Rust 1.98.1 was installed explicitly for this continuation.

## Runtime evidence

| Evidence | Result and limits |
| --- | --- |
| [Focus reveal](../focus-reveal-20261025/README.md) | Reusable controls and five short-viewport routes, forward/backward reveal and preserved manual scroll. 68 gallery focus-bound samples; screenshot review. |
| [Accessible text](../a11y-text-20261025/README.md) | Source explanation, regression contracts and seeded probe source; actual speech/tree followups linked separately. No AT editing actions/character geometry/soft-wrap segmentation. |
| [Nested modal isolation](../nested-modal-a11y-20261025/README.md) | X11 before/after two-level and three-layer ordering, keyboard restoration and passive toast checks. |
| [Orca](../orca-20261025/README.md) | Real reader speech reaches a virtual audio sink. Input readback, grapheme/password/multiline/selection, radio positions, initial modal text/buttons and Ctrl+Home/End sampled. Transcript/screenshots/PCM inspected; no listening/intelligibility acceptance. Reopen/restoration remains blocked by stale defunct objects. |
| [Wayland](../wayland-20261025/README.md) | Real GPUI Wayland clients with DISPLAY absent, nested Weston protocol evidence, input/clipboard/undo, modal callbacks/trapping/restoration, Text/caret/selection and resizing. Software nested compositor, no hardware/monitor/live-scale acceptance. |
| [Native state sweeps](../native-states-20261025/README.md) | All 72 registered states in dark, light+care and light+brand 7C3AED: 216 native captures, no capture failures. All 36 sheets reviewed, with raw-image followups; no confirmed native visual defect. No new paired parity pass. |

Each runtime receipt records its own binary and source snapshot: several useful before/after
runs predate the final integrated binary. Those artifacts must not be described as later-binary
captures. Final Ctrl+Home/End checks use the preserved probe artifact `9626474f…` on X11/Orca
and Wayland. Earlier dark/care/brand images retain their exact pre-accessibility-fix patch.

## Preserved failures and open work

- `build-initial-nospace.log`: concurrent Docker vfs containers exhausted the disk. The retry
  used one shared container and passed; no failed result was committed as passing.
- Temporary probe compile/input mistakes are kept in the focus/modal receipts.
  `gates-failed-test-macro.log` and `gates-failed-clippy-fixture.log` preserve intermediate
  test macro/lint failures, corrected before the final green gates.
- `upstream-fetch-blocked.log`: WeUI and weui.js npm downloads succeeded, but the configured
  proxy denied the pinned SDK at res.wx.qq.com. No policy bypass or reference substitution.
  Paired theme/state parity must be rerun when that artifact is available.
- [Cache investigation](../orca-20261025/cache-investigation.md): the pinned AccessKit Unix
  stack emits malformed Add/Remove cache signatures. Actual Orca modal reopen/restoration
  fails despite correct fresh tree/native focus. No dependency change or external report.
- AccessKit disabled-button export, complete screen-reader/state/theme/platform acceptance,
  Windows/macOS reruns, human audio review, emoji glyph coverage on this host, hardware Linux
  sessions and release/distribution work remain open. VoiceOver stays off. UTC today stays.

No component row is fully accepted, and this continuation does not qualify a release. Signing,
package/versioning, website destination, local-date policy and release authorization remain
owner decisions. No tags, releases, deployments, publication or CI dispatch were performed.
No signing key is configured in this cloud checkout; the integration commit is unsigned and
explicitly includes `[skip ci]`.
