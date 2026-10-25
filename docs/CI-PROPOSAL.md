# Contributor CI

The approved final configuration is implemented in
[`.github/workflows/ci.yml`](../.github/workflows/ci.yml). Workflow configuration
and local checks are separate from a hosted CI result; consult the run for the
exact pushed SHA. No hosted pass is implied by committing this file. The first run passes Linux, MSRV,
mobile-library and macOS native checks but fails the iOS bundle inside the upstream
0.3.9 backend; the failing gate is retained. See the CI receipt below.

| Job | Scope |
| --- | --- |
| Linux | Stable all-target build, seven library contracts, formatting, Clippy with warnings denied, strict rustdoc, and five bundle-script fixture tests |
| Linux MSRV | Rust 1.95.0 all-target build and library contracts |
| Mobile library type-checks only | iOS device, iOS simulator and Android `cargo check --lib`; no app linking or runtime acceptance |
| Apple builds only | macOS all-target build/tests/Clippy plus an iOS simulator bundle build; no installation, simulator boot, launch or screen reader |

Linux and mobile-library jobs run on pushes to `main`, pull requests and manual
dispatch. Apple builds run on `main` pushes and manual dispatch, keeping them off
pull requests. `macos-15` records its selected Xcode/SDK versions; this is a hosted
build job under the approved CI setup, not permission to use a personal Mac or to
claim Apple runtime acceptance.

Actions are pinned to verified upstream tag SHAs (`checkout` v4.2.2,
`upload-artifact` v4.6.2). Repository permissions are read-only and checkout does
not persist credentials. No third-party cache action is added. Jobs have bounded
timeouts and superseded runs are cancelled. Simulator Cargo/signing diagnostics
are uploaded even on failure, with seven-day retention; a missing diagnostic
artifact does not count as a successful build.

Local verification for the final candidate is recorded in
[the 0.3.9 receipt](receipts/ios-path-20261024/README.md). Workflow lint results and
hosted observations are recorded in [the CI receipt](receipts/ci-20261024/README.md).
Apple jobs may expose failures that cannot be reproduced in the Linux cloud
container. Preserve those logs and fix evidence-backed build issues without
converting a build result into a runtime claim.

The existing `.cnb.yml` continues to run Linux formatting/build/Clippy. No hosted
CNB pass is recorded. GitHub's strict documentation and focused tests extend
those baseline checks.

Native Windows execution, Wayland, Orca, mobile touch/keyboard/IME/insets/lifecycle,
VoiceOver/TalkBack and packaging acceptance still require separately identified
receipts. A future Android application job needs an upstream backend first.
