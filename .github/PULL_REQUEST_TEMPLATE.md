## Summary

<!-- What changed and why. Link issues with "Refs #N" (or "Fixes #N" only when it fully fixes it). -->

## Evidence

<!-- Commands you ran and their results, including failures. For UI changes: route, state, theme,
screenshot pair, platform/version, and the input or accessibility checks you performed. -->

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo build --all-targets --locked --offline`
- [ ] `cargo clippy --all-targets --locked --offline -- -D warnings`
- [ ] `RUSTDOCFLAGS='-W missing-docs' cargo doc --no-deps --locked --offline` adds no warnings

## Not tested / limits

<!-- Platforms, screen readers, IMEs or states you could not check. -->

## Checklist

- [ ] Public items have rustdoc (behaviour, defaults, units, ownership, accessibility)
- [ ] Generated files changed only through `scripts/extract_weui.py`
- [ ] No GPUI pin, dependency or lockfile change without a linked maintainer decision
- [ ] `CHANGELOG.md` and affected docs (`docs/STATUS.md`, `docs/PARITY.md`) updated
