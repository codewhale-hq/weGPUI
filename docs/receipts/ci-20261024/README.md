# Final CI setup — October 24, 2026

Implementation candidate: `db2b686` (the exact full SHA is recorded with the push).
Local Linux stable/MSRV builds, seven Rust tests, strict docs/Clippy, three mobile
library type-checks, and five bundle-script fixture tests passed; see the
[implementation receipt](../ios-path-20261024/README.md).

Workflow lint: actionlint v1.7.7, SHA-256
`023070a287cd8cccd71515fedc843f1985bf96c436b7effaecce67290e7e0757`, verified
against the upstream release checksums. `actionlint -shellcheck= -pyflakes=
.github/workflows/ci.yml` exits 0. Shellcheck and pyflakes were not run.
`actionlint.txt` is empty on success. Actions' pinned SHAs were checked with
`git ls-remote` against upstream v4.2.2 and v4.6.2 tags.

GitHub API inspection before enabling the build workflow found existing default
CodeQL runs, including success for `08ca36829ab3ee3ce5917cc01eed2f8d81e91360`
(run 40155882579). This is code-scanning evidence, not a build or runtime receipt.

The new workflow has not yet established a hosted build result at this checkpoint.
It does not run Apple applications, boot simulators or enable screen readers.
Hosted observations will be recorded after the workflow is pushed.

## First hosted result: build failure retained

[Run 40156480128](https://github.com/codewhale-hq/weGPUI/actions/runs/40156480128),
exact source `05cae89f1373f42af30913aa65dc5e51f7c54baf`:

| Gate | Result |
| --- | --- |
| Linux stable build/tests/Clippy/strict docs/bundle-script fixtures | Pass |
| Linux Rust 1.95 all-target build and seven tests | Pass |
| Three mobile library-only type-checks | Pass |
| macOS native all-target build, seven tests and Clippy | Pass |
| iOS simulator bundle compilation | **Fail**, inside published `gpui-pre-ios 0.3.9` |
| Simulator launch, input, accessibility tree or VoiceOver | Not attempted |

Apple runner: arm64 macOS 15.7.9, Xcode 16.4 (16F6), Rust 1.99.1. This is a GitHub
hosted runner under the approved build-only CI setup, not the user's personal Mac.
`apple-job-121058856368.txt` retains the job log with ANSI styling removed.
`run-40156480128.json` preserves job/step conclusions. The exact downloaded failure
artifact is `ios-attempt1.zip` (SHA-256
`9f021051c79f1fe6c8276f3d703536b0f60544b37722dbd59cb390a7d7e6f488`);
`ios-attempt1.stderr.txt` is its readable stderr with ANSI styling removed.
The ZIP also contains complete Cargo JSON diagnostics.

The backend requires core `=0.3.9` but references APIs missing from that core:
`AppLifecyclePhase::Disconnected` (two errors),
`PlatformInputHandler::query_accepts_focused_text_input` (two errors),
`selection_bounds_for_range` and `focus_id` (one error each). The APIs are absent,
not gated behind an unenabled Cargo feature. Registry inspection found only iOS
0.3.9, with no newer core version than 0.3.9 (`registry-versions.json`).

No dependency fork, registry-source mutation, pin substitution or permissive CI
exception was applied. The iOS job remains failing so this blocker stays visible.
A corrected upstream release or a separately authorized dependency patch is needed.
See the [prepared upstream report](../../proposals/gpui-ios-039-incompatibility.md).
The failure is a compiler result, not an authorization denial or a runtime verdict.
