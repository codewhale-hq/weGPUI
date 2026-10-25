# Native state captures — Linux X11, 2026-10-25

All 72 registered startup states were rerun in dark, light+care, and light+custom-brand
`7C3AED`: **216 native captures, 72/72 per mode, zero capture failures**. All 36 contact sheets
were visually reviewed, with 20 raw-image followups. The reports list the exact files:
[dark review](dark-review.md) and [care/brand review](care-brand-review.md).
No confirmed native rendering defect was found in this sample. Typography/wrapping comparison
targets remain in the review notes. This does not cover every public component/state/theme
combination, keyboard interaction, callbacks or accessibility.

**These are native-only captures, not a new paired WeUI parity pass.** The pinned upstream
fetch succeeded for the two npm packages but the configured proxy denied
`res.wx.qq.com/open/js/jweixin-1.0.0.js` with HTTP 403. The failure is retained in
[the continuation receipt](../linux-continuation-20261025/upstream-fetch-blocked.log).
No SDK substitution, modified reference or network-policy bypass was used. The earlier
61/72 dark run was not reused.

`run.sh` records the actual invocation, Xvfb screens (`1600x3600x24`, displays 95–97), openbox,
scale 2, mode-specific options and native-only selection. Each output directory has its own
`sweep.log`, per-state stderr, raw PNGs and sheets. Captures are 750×1624 pixels. The scripts
returned zero and PNG decoding/sheet generation succeeded; those facts alone are not a
visual acceptance verdict.

Source provenance: GitHub parent `52650f3b830ed7e8d1cee7921d495bff06e0628d` plus the exact
`source.diff` saved before capture; its SHA-256 and gallery binary SHA-256 appear in
`provenance.txt`. Built with Rust/Cargo 1.98.1 and pinned gpui-pre 0.3.9 on Debian 13,
software Vulkan rendering, Xvfb and openbox. The subsequent accessibility-only source changes
(text values/descriptions, radio-group parents and modal tree isolation) do not appear in this
sweep's source patch; their runtime evidence is recorded separately. These images must not be
reported as captures of a later binary. See the final source manifest in
`../linux-continuation-20261025/` for the integrated gate source.

The saved `Xvfb -version` probe printed its unsupported-option help; package versions are
recorded separately in `environment-packages.txt`. This diagnostic did not interrupt capture.
