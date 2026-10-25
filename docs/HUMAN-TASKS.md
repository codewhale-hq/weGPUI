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
   cloud Orca samples are partial: reopen/restoration hits malformed AccessKit cache signals,
   and synthesized audio still needs human intelligibility review. See receipts/orca-20261025.
5. **Owner decisions still pending** (from STATUS): local-time source for `picker::today()`
   (DateField defaults to UTC "today"), crates.io name/versioning, publication timing, website
   deployment target, signing/notarization requirements for packaged galleries.
6. **Release authorization** before any crates.io publish, website deployment, tag, release
   or CI dispatch.
7. **Upstream AccessKit report (optional):** `accesskit_atspi_common 0.19.1` never exports
   disabled for buttons (see `receipts/a11y-p1-20261024`). Filing an upstream issue is an
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
11. **AccessKit cache compatibility:** actual Orca reopen/restoration fails with malformed
    AddAccessible/RemoveAccessible signatures in the locked stack. See
    `receipts/orca-20261025/cache-investigation.md`. Dependency changes/forks/registry patches
    require owner approval; no such change or external report was made.
