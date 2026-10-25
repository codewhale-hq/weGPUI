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
