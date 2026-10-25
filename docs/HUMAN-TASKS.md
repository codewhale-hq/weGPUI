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
   cache/reopen and X11 grab-focus failures. Actual fixed-source Orca now retains flat review
   and says `icon-2 button grayed.`; recorded synthesized audio still needs human
   intelligibility review (`receipts/x11-grab-focus-20261025/`).
   See receipts/orca-20261025, linux-bridge-20261025 and input-contracts-20261025. Full per-family
   reader acceptance remains incomplete, including Linux AT text replacement (the bridge lacks
   an EditableText interface).
5. **Owner decisions still pending** (from STATUS): local-time source for `picker::today()`
   (DateField defaults to UTC "today"), crates.io name/versioning, publication timing, website
   deployment target, signing/notarization requirements for packaged galleries.
6. **Release authorization** before any crates.io publish, website deployment, tag, release
   or CI dispatch. The 60-route / 132-preset interactive gallery is built and locally verified
   in full candidate1 at root and `/weGPUI/`: 40 browser checks, including 360 route/viewport
   observations. Final candidate 3's three targeted checks per mount and complete legacy
   checks pass at both mounts; each legacy run verifies 264 historical views and 528 decoded
   images. Theme/native-theme pages pass at 375/390/768/1280px with retained palette routes
   and live iframe pointer/Space interaction; the final narrow theme capture was reviewed.
   Live-host, additional-browser and actual screen-reader qualification for the website remain open. The
   [manual Pages flow](../website/HOSTING.md) is prepared; Pages is not enabled and no live
   deployment is established. The original no-CI-dispatch / no-deployment instruction still
   requires final owner authorization for the reviewed source and chosen destination before
   enabling Pages or running publication. Browser results leave all 40 native rows unaccepted.
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
11. **Downstream compatibility integration:** the owner approved the existing AccessKit,
    GPUI and Linux-backend copies, including the added modal action/cache and X11 focus-mode
    fixes. **Current source requires the matching GPUI copy to compile**, because
    Semantics/Modal calls its added `Window::with_a11y_action_guard` API. All consumers need
    workspace-root patches for `gpui-pre` → `vendor/gpui_pre`, `gpui-pre-linux` →
    `vendor/gpui_pre_linux`, `accesskit_unix` → `vendor/accesskit_unix` and
    `accesskit_atspi_common` → `vendor/accesskit_atspi_common` in the matching checkout;
    dependency-manifest patches are not inherited. See [vendor guidance](../vendor/README.md).
    Sampled root gates, saved-action/cache, X11, real Fcitx and actual Orca checks pass.
    Publication/hosted forks remain unauthorized. No external report was made. Additional Linux EditableText support
    needs atomic host range-edit and clipboard contracts, beyond a contained bridge patch;
    see [the feasibility note](proposals/linux-editable-text/README.md). Advertising a partial
    interface is not a safe substitute.
12. **Remaining compatibility qualification:** the owner approved and applied the exact
    [modal dispatch](proposals/modal-a11y-action-guard/README.md) and
    [X11 focus filter](proposals/x11-grab-focus/README.md), including existing
    `gpui-pre-linux 0.3.9`. Five combined source gates pass with 47 root tests/one ignored
    doctest; seven saved-action timing cases, fixed cached/live controls and real X11/IME/Orca
    samples have separate receipts. Root tests do not execute the two new vendor unit tests:
    the explicit attempt is blocked before execution by 21 upstream test-asset/optional-test
    dependency errors. Do not add dependencies to make that attempt green without approval.
    The extracted NodeBuilder harness passes two supplemental metadata regressions with
    existing locked rlibs; that is not a full vendor-suite or 49-root-test pass. Six extended
    raw/listener/epoch/passive/ancestor/two-window modes pass their sampled native contracts.
    Cache performance, full ordering/reader/host qualification remain open;
    no package publication, hosted fork or external report is authorized by these approvals.
