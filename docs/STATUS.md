# Status — takeover checkpoint, 2026-10-24

**This is an early, incomplete port. Phase 1 is partly implemented. Phases 2–4 are not complete. The full-port acceptance criteria have not been met.** Implementation stopped at the owner's request to prepare a handoff for another model.

## Done

- Initialized the standalone repository on `main` with MIT/Apache-2.0 licensing and Tencent/GPUI attribution.
- Obtained the official WeUI 2.6.26 package; SHA-1 `457eaa5a7ab9539b9944ff736bf0afffdd5d7baf`.
- Extracted 180 variables per palette, preserving raw expressions in `docs/tokens.json`; extracted 28 upstream SVGs and added one loading SVG based on upstream geometry. All 29 SVG files parsed as XML at handoff.
- Added initial theme/input components and a gallery with 10 demo entries; built and launched it on macOS. Existence does not imply complete family coverage.
- Observed Unicode paste, select-all replacement, emoji deletion, caret movement, switch changes, dialog Tab focus, Escape dismissal and reopening via the restored opener. These are manual observations, not complete acceptance results.
- Established all three repositories. At the pre-handoff remote check CNB, GitHub and GitCode `main` contained `e8216d76cb9674b50a5540d707867fb232540268`.
- Inventoried all 61 upstream HTML files: 58 within 29 component directories and 3 root pages. See [UPSTREAM-COVERAGE](UPSTREAM-COVERAGE.md).

## Partly done

- Button, cells/list, badge, input/check/radio/switch, search, loading/loadmore, toast, dialog and action-sheet implementations are preliminary. Demo content is simplified and omits variants.
- Theme switching and brand override exist; tokens and derived component colors need audit. System appearance is observed. GPUI animations respect `App::reduce_motion`, but automatic system reduced-motion synchronization is absent.
- Some accessibility roles, labels and focus behavior exist; the whole tree, modal semantics and keyboard interaction need systematic verification.
- The input retains the marked-text protocol with a composition selection-offset correction. A real Chinese IME has not been tried.
- Build passed with no ordinary compiler diagnostics for this crate, but an upstream future-incompatibility warning remains. This does **not** satisfy the warning-free gate.
- CNB is the selected primary; GitHub/GitCode are intended mirrors. Automated mirroring and CI/CD are absent. User-assisted GitCode push succeeded; the agent's later noninteractive push could not retrieve a password from Keychain. Remote read access succeeds; future writes are not verified.

## Not done

- Phase 2: message results, previews, top tips, steps, half-screen dialog, panels, footer, tabbar/navbar, progress and slider.
- Phase 3: grid, uploader UI, viewer, picker, agree, article, information bar, flex, icons demo, textarea, complete forms and remaining examples/CSS variants.
- Complete official example content and grouped gallery index.
- `cargo clippy`, complete behavior/accessibility/reduced-motion checks, Windows or Linux execution.
- **Parity screenshots: 0 files, 0 pairs.** No family is certified as matching. See [PARITY](PARITY.md).
- Hosted CI, macOS runner qualification, automatic synchronization, release packaging and crates.io publication.

## Exact local check evidence

Target directory: `/Volumes/VIXinSSD/cw-build-cache/target/wegpui`.

The first `cargo build --example gallery --offline` failed with 2 library API errors. After repair, `cargo build --example gallery --locked --offline` failed with 12 gallery errors. Those were fixed. The latest invocation of the latter command exited 0:

```text
   Compiling wegpui v0.1.0 (/Volumes/VIXinSSD/CW/weGPUI)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

Exact output: `docs/receipts/phase1-build-final.txt`; failed outputs are preserved alongside it. No Rust source or Cargo manifest was newer than the successful build receipt at handoff. `cargo fmt --all -- --check` passed earlier. No clippy command has run. No native build was started for this handoff because another build was active.

`phase1-source.json` predates repairs and is not a hash receipt for the successful source. `handoff-source.json` records 69 current source/asset hashes; it is an inventory, not a new build result.

## Known concrete gaps

- Medium button uses 17px text; upstream uses 14px. Loading gap/shape, overlay, xmini, icon/cell/wrapped and bottom-fixed variants need work.
- Switch cells use generic padding; switch animation is absent. Checkbox/radio geometry needs comparison. Demo switch and checkbox share one boolean.
- Toast automatic dismissal and failure/long-text states are missing. Overlay transitions and modal accessibility flags are incomplete; the backdrop leaves gallery navigation outside it.
- Several demo backgrounds, spacing, descriptions and content differ from upstream.
- Search clear/cancel and search variants need verification; textarea and the complete form API are absent. Clipboard line-ending normalization needs review.
- Brand override changes `BRAND`; actual CSS dependencies need audit.
- Investigate the upstream dependency warning within pinned-version constraints. Do not suppress it to report a clean build.

## Git and handoff

Canonical checkout: `/Volumes/VIXinSSD/CW/weGPUI`, branch `main`. A dedicated takeover checkpoint preserves the source. Its commit and remote state are reported in the handoff completion message; current `git status`, `git log` and remote heads are authoritative.

Existing published commit: `e8216d7` — `chore: establish standalone weGPUI licensing and upstream provenance`.

Source, assets, extractor and build receipts were untracked before this checkpoint. No branches or alternate checkouts were created. See [TAKEOVER](TAKEOVER.md) for the next-model prompt.
