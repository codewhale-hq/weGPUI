# Tasks that need the owner or another machine

Updated 2026-10-24 (late) by the Linux cloud agent. Each item says what unblocks it.

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
   unless the owner changes that instruction). Needed for every ACCEPTANCE row.
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
9. **Linux items the cloud box cannot do:** Wayland session, Orca, live scale changes and
   monitor moves (no compositor or Orca on the box).
