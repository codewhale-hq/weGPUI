# GitHub Pages publication — 2026-10-25

The owner configured GitHub Pages to use GitHub Actions and dispatched the existing
manual workflow. GitHub records successful build and deploy jobs for source
`e5b6996ca29bba628386293220fe604c6e6acf7a`:

- [Workflow run 40211664701](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701)
- [Successful deploy job](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701/job/121224070447)
- Returned environment URL: <https://codewhale-hq.github.io/weGPUI/>

The files under `collector/` are preserved unchanged. `run.json` and `jobs.json`
retain normalized non-secret run/job fields obtained
through read-only GitHub connector requests. `deploy-selected.log` contains the
exact timestamped success and evaluated-URL lines. `summary.json` records the
source URLs, complete decoded-log digest, normalized-object digests and their
scope. Raw HTTP bytes and temporary signed download URLs were not retained.
`files-sha256.json` hashes the five collector files; all five hashes were verified
after copying. This integration note and `gates.log` are separate from that manifest.

A prior public HTTPS probe was rejected by the cloud proxy during CONNECT with
403 Forbidden, before an origin HTTP response. It did not obtain the site's HTML
or title. No alternate-route attempt or retry was made. GitHub deployment success
is established; public-origin rendering, deep links, interactions and screen-reader
behavior remain unqualified. Local Chromium results are recorded separately in
[the browser receipt](../web-interactive-20261025/README.md).

The agent only read deployment evidence and updated source documentation. It did
not enable Pages, dispatch the owner's workflow, change a custom domain or DNS,
or publish a package/release. Subsequent source pushes do not run the manual-only
Pages workflow. The deployed revision remains `e5b6996` until another authorized
publication. Native GPUI component acceptance is unchanged.

`gates.log` records the five required checks for this documentation update using
Rust/Cargo 1.98.1 in container `wegpui-198`. The environment is
`CARGO_HOME=/workspace/.wegpui-cloud/cargo`,
`CARGO_TARGET_DIR=/workspace/.wegpui-cloud/target-198` and
`RUSTUP_TOOLCHAIN=1.98.1`. The ordered commands are `cargo fmt --check`,
`cargo build --all-targets --locked --offline`, `cargo test --locked --offline`,
`cargo clippy --all-targets --locked --offline -- -D warnings` and
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline`.
