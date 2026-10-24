# Status — 2026-10-24

weGPUI is an independent Rust library and gallery port of WeUI 2.6.26. GitHub is the canonical contributor home; CNB is a mirror. All 61 upstream example files have gallery mappings, but complete visual, behavioural and accessibility parity is **not verified**. See the [independent review and per-family table](REVIEW-2026-10-24.md).

The requested platform target is macOS, Windows, Linux, iOS and Android. Only macOS arm64 has local build and interaction evidence. Mobile support is not implemented or verified.

## Done

- Frozen `51be7e3`: macOS `cargo build --all-targets --locked --offline`, `cargo clippy --all-targets --locked --offline -- -D warnings`, and `cargo fmt --all -- --check` passed. Only the upstream `block v0.1.6` future-incompatibility notice remained.
- Coverage inventory: all 61 HTML filenames match [UPSTREAM-COVERAGE](UPSTREAM-COVERAGE.md). Home/layers are gallery-specific; index is the shell. Public components cover the component families.
- A scratch extraction reproduced 65 generated files, including 32 SVG icons and four 180-field palettes.
- Independent recapture: 28 pairs across 13 routes, light and dark, including overlays and single/date pickers. 52 of 56 images reproduced committed files exactly. Residuals and limitations are in [PARITY](PARITY.md).
- Fixed navbar/tabbar arrow navigation to follow the focused tab, and action-sheet child corners. Fresh native input and light/dark screenshots cover these changes.
- Fixed the AX dumper to walk the whole first window. Nine routes were dumped.
- Real Pinyin marked-text composition and candidate commit succeeded in TextArea. This is limited evidence, not general IME qualification.

## Partly done

- **Behaviour:** see the per-family review table. Static examples and empty sample callbacks do not prove working product flows.
- **Accessibility:** roles, values and selected states are sampled; modal background controls remain exposed and picker/select option labels are missing from AX dumps. No VoiceOver session has been run.
- **Input:** ordinary typing/selection and textarea Return/count have evidence. Single-line Pinyin composition was attempted but not confirmed; cancellation, selection replacement and composition at length limits remain untested.
- **Rustdoc:** `cargo doc --no-deps --locked --offline` succeeds, but the missing-docs audit reports 1,019 warnings. API rustdoc and generator preservation are approved; implementation remains outstanding.
- **Reduced motion/animations:** startup preference support exists. Live setting changes and frame-by-frame timing are unverified. Modal slide distance uses the viewport height rather than the panel height, and reverse easing differs from CSS.
- **Visual parity:** the archive contains 264 regular pairs plus three brand/care pairs. The independent review only sampled 28 pairs. Long-page content and care mode across every family remain unverified.

## Known unresolved defects and gaps

- `SelectMenu`: option arrows use global focus traversal; real-input probes showed focus reaching a background trigger. Focus initialization, containment and disabled-option traversal need a verified fix.
- `Slider::range`: non-finite caller values can reach `f32::clamp` or invalid layout arithmetic. Inputs/range/step need validation.
- `Picker::initial_index`: remembered/default values can select disabled items; all-disabled/empty columns need an explicit confirm contract. `today()` uses UTC, unlike the browser example's local date.
- `Presence`: hide without an opener only blurs the root focus handle, not a focused child; interrupted transitions and exact easing/travel need review.
- `LinkText`: multiple mouse links share a single keyboard target/first handler. Article heading levels/list-item semantics and hidden swipe actions need accessibility work.
- Text input length counts graphemes rather than HTML UTF-16 units; numeric character filtering is not complete input validation; undo/redo is absent.
- Uploader gallery add uses a bundled image. A native chooser and honest file/error lifecycle demonstration are outstanding; transport remains application-owned.
- Public naming mixes `Navbar`/`Tabbar` with `SearchBar`, and terse `hd`/`bd`/`ft` helpers. Document the existing API before deciding on breaking changes.
- **Linux:** no desktop backend feature is enabled (`font-kit` only). The pinned backend requires `wayland` or `x11`. Compilation and a displayed, interactive gallery must both be qualified.
- **iOS/Android:** no native platform branch exists in the pinned `gpui-pre-platform` constructor. Resolve feasibility before promising support or changing dependencies.

## Not done / missing receipts

- Native Windows/Linux/iOS/Android builds and runtime evidence; mobile safe areas, touch, software keyboards, rotation and lifecycle.
- Hosted CNB CI result; GitHub Actions approval and configuration. No new workflow has been enabled.
- VoiceOver and other screen-reader sessions; comprehensive single-line and multiline IME coverage.
- Reproducible upstream download/serve tooling, contributor parity guide, README usage example, contribution/security/conduct/changelog documents and templates.
- Measured MSRV and package-content review. Rust 1.99.0 is verified; the resolved graph declares a 1.89 lower bound, which is not yet a measured project MSRV.
- Complete public API docs; a test suite requires owner approval.
- Release policy, crates.io name/publication, screenshot storage policy, and localisation/RTL scope. No release or publication occurred.
- GitCode mirror update requires owner credentials; do not infer its state from other mirrors.

[Sources](SOURCES.md) · [Coverage](UPSTREAM-COVERAGE.md) · [Visual evidence](PARITY.md) · [Independent review](REVIEW-2026-10-24.md)
