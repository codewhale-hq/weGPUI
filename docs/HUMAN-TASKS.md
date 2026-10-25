# Tasks that need the owner or another machine

Updated 2026-10-25 by the Linux cloud agent. Each item says what unblocks it.

1. **Signed commits.** Commits `b58b6fa` onward were made on a Linux box with no signing key and
   show as unverified. To sign future agent commits, provide a signing key on that machine
   (`git config user.signingkey …`, `commit.gpgsign true`) or accept unsigned agent commits.
   History is not rewritten.
2. **macOS rerun of focus reveal and DateField** (authorized Apple host): `cargo run --example
   gallery -- --page preview --size 480x650`, Tab ×3 / Shift+Tab ×3, then wheel up and Tab;
   `--page input`, Tab to 时间, Enter, confirm date, confirm time, Escape on reopen. Compare with
   `docs/receipts/focus-date-20261024/`. Unblocks the macOS column in ACCEPTANCE.
3. **Windows rerun** of the same sequences on the Windows VM / Windows subagent, plus the
   form_bottom_fixed sequence at 480x420.
4. **Screen-reader acceptance** (NVDA or Narrator on Windows, Orca on Linux; VoiceOver stays off
   unless the owner changes that instruction). Needed for every ACCEPTANCE row. October 25
   cloud Orca samples are partial. The owner-approved compatibility patches repair the sampled
   cache/reopen failure; recorded synthesized audio still needs human intelligibility review.
   See receipts/orca-20261025, linux-bridge-20261025 and input-contracts-20261025. Full per-family
   reader acceptance remains incomplete, including Linux AT text replacement (the bridge lacks
   an EditableText interface).
5. **Owner decisions still pending** (from STATUS): local-time source for `picker::today()`
   (DateField defaults to UTC "today"), crates.io name/versioning, publication timing, website
   deployment target, signing/notarization requirements for packaged galleries.
6. **Release authorization** before any crates.io publish, website deployment, tag, release
   or CI dispatch.
7. **Upstream AccessKit report (optional):** `accesskit_atspi_common 0.19.1` never exports
   disabled for buttons in the original locked source (see `receipts/a11y-p1-20261024`).
   The approved local compatibility copy now fixes the sampled export. Filing an upstream issue is an
   external post. Owner said not to file it (2026-10-24); kept here only as a record.
8. **Windows/macOS rerun of 2026-10-24 late fixes:** primary/warn focus ring, information-bar
   focus, paste/undo grouping, set positions (check NVDA/VoiceOver-off AX inspector reports
   "1 of 2" for the first radio), footer link focus, nested modals (`examples/nested_modals`),
   overlay-primary ring, search-bar rings and swiped-action rings.
9. **Linux host qualification still needed:** physical monitor moves, live scale changes,
   hardware compositor/GPU and desktop audio review. This cloud continuation can run nested
   Weston Wayland and Orca with a virtual audio sink; see the October 25 receipts. Those
   representative samples do not complete the per-component desktop/screen-reader matrix.

10. **Pinned reference download:** allow the pinned SDK host `res.wx.qq.com` through the
    managed cloud network policy (or provide an already verified cached artifact) to complete
    `scripts/weui_upstream.py fetch` and rerun paired dark/care/brand comparisons. October 25
    native-only captures are reviewed but do not replace this comparison.
11. **Downstream accessibility patch integration:** the owner approved local copies of the
    existing AccessKit crates and GPUI 0.3.9, with narrowly scoped cache/state/window-origin
    fixes. The sampled Orca reopen and X11 state/origin checks pass; see `vendor/README.md`
    and both proposals. Cargo patches are not inherited by applications: downstream workspace
    roots must adopt equivalent configuration to receive these fixes. Publication/hosted forks
    remain unauthorized. No external report was made. Additional Linux EditableText support
    needs atomic host range-edit and clipboard contracts, beyond a contained bridge patch;
    see [the feasibility note](proposals/linux-editable-text/README.md). Advertising a partial
    interface is not a safe substitute.
12. **Additional compatibility approval:** real saved AT Click/Focus requests at requested
    0/1ms after modal hide can invoke a callback or steal restored focus; X11 Grab/Ungrab
    events also disturb Orca window/focus and flat review. The exact
    [modal dispatch](proposals/modal-a11y-action-guard/README.md) and
    [X11 focus filter](proposals/x11-grab-focus/README.md) proposals are unapplied. They extend
    the earlier approved patch scope, including a local copy of existing `gpui-pre-linux
    0.3.9`, and require additional owner approval. No package/version change, hosted fork,
    publication or external report is proposed. New source gates and native regressions
    remain necessary after approval; preparation checks are not patched runtime results.
