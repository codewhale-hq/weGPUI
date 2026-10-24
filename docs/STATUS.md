# Status — 2026-10-24

weGPUI ports every WeUI 2.6.26 example family to native GPUI components, with a gallery that reproduces all 61 upstream example files. Visual evidence covers every route and registered state in light and dark. The remaining gaps are listed below; they are real and are not hidden by the verdicts in [PARITY](PARITY.md).

## Done

- **Coverage:** all 61 upstream HTML files (58 in 29 component directories plus `home`, `index`, `layers`) map to gallery routes built from public library components; no placeholder routes remain. See [UPSTREAM-COVERAGE](UPSTREAM-COVERAGE.md).
- **Library:** one module per family in `src/components/` (38 modules), a shared `Modal`/`Presence` overlay layer, `TextInput` (single- and multi-line), `TextArea`, `Theme` with exact light, dark, care-light and care-dark palettes (180 variables each), system appearance, explicit mode and brand override.
- **Generated sources:** `scripts/extract_weui.py` regenerates `assets/` (32 upstream SVG icons plus example images), `src/assets.rs`, `src/theme/palette.rs` and `docs/tokens.json`. A scratch regeneration from the pinned package reproduces all of them byte for byte.
- **Visual parity:** 264 native/official pairs (528 PNGs) at 375×812 pt @2x, light and dark, every route and every registered state, plus brand and care-mode samples. All routes match apart from the residuals listed in PARITY.
- **Build gates** (macOS, source `2c8ed10` and the later label-only commit): `cargo build --all-targets --locked --offline` and `cargo clippy --all-targets --locked --offline` exit 0 with no crate diagnostics; `cargo fmt --all -- --check` passes. The only remaining output is the future-incompatibility notice for `block v0.1.6`, which comes through `cocoa` from the pinned `gpui-pre-macos 0.3.7`. It is upstream, cannot change without unpinning GPUI, and is left visible. Receipts: `docs/receipts/takeover-build.txt`, `docs/receipts/takeover-clippy.txt`.
- **Interactions driven with real macOS input events** (`scripts/drive.py`):
  - search: focus, typing, results, clear, cancel;
  - dialog: open, Tab trapped inside, Escape closes;
  - action sheet: open, mask dismiss;
  - picker: drag with inertia/snap; Down/Up/End skipping the disabled item; Tab across date-picker columns; Escape restores focus to the opener;
  - switch: click and Tab+Space;
  - textarea: typing, Return, live count;
  - layers: Tab+Enter isolation;
  - gallery: Tab/Shift-Tab traversal with visible focus rings.
- **Accessibility tree** (macOS AX dump): the dialog is exposed as a focused modal `dialog` with its actions; the radio group exposes checked values; switches expose on/off and disabled; sliders expose values; the picker column is a focused list with the selected option marked; form headings carry their text.

## Partly done

- **Accessibility:** roles, names, values and focus are present and sampled on five routes. VoiceOver itself has not been run. Picker option labels are set through AccessKit but did not appear in the AX dump's name attribute. Page content behind an open modal is still present in the AX tree; the modal node is flagged modal and focused.
- **Reduced motion:** the system preference is read at start-up and on activation (`motion::sync_reduce_motion`), and every animation honours `App::reduce_motion`. Toggling the macOS setting while the app runs was not tested.
- **Care mode:** palettes and type scale are implemented. Only `form_primary` was compared against the official care mode (light and dark, matches).
- **Layers:** a native projection of the 3D layer stack. Labels are drawn upright because GPUI has no 3D text transform. In one of four captures the default page was still flat after 3 s, so the start-up timer appears timing-sensitive; not reproduced since.
- **Hosted CI:** `.cnb.yml` defines Linux fmt/build/clippy (`-D warnings`) on push to CNB. `2c8ed10` was pushed to CNB, but the run result was not read; it needs CNB access.

## Not done

- **Real Chinese IME composition:** not exercised. Unicode paste and programmatic Unicode key events were used; neither proves marked-text composition.
- **Windows and Linux:** never run natively; only the CNB Linux build config exists.
- **Localisation:** no locale system and no right-to-left layout. Component default strings (搜索, 取消, 确定) are overridable by callers, and sample text reproduces upstream verbatim.
- **Mobile safe-area insets:** not applicable to desktop windows and not implemented.
- **GitCode mirror:** the push needs the owner's credentials; `gitcode/main` is still `e8216d7`. CNB (`origin`) and GitHub hold the current `main`.
- **Release:** no release packaging, crates.io publication or macOS runner qualification.

## Known residual differences

Full list per route: [PARITY](PARITY.md). In summary:
- macOS shaping inserts spacing at CJK/Latin boundaries;
- line-breaking differs for “?” and CJK kinsoku/ellipsis in the panel clamp;
- “verification” hyphenates at a different point;
- the date/time rows draw a WeUI icon in place of Chromium's native control;
- the default `layers` frame differs because the official screenshot is captured mid-transition.

One upstream bug is reproduced as intended behaviour: in `form_vcode`, two buttons share an id, so the visible 确定 never shows the toast in the official page.

## Commits (since the initial licence commit `e8216d7`)

`8064418` checkpoint · `d4c04b1` foundation · `ef939fe` CNB metadata · `4d357df` panel, preview, progress, slider, steps, information bar, uploader, gallery, msg · `ca1abbb` interaction driver, AX dump, sRGB captures, CI config · `7d4b44c` buttons, loading, loadmore, badge, icons, flex, footer, article, grid, swiped list · `17b598a` semibold weights, gallery Tab traversal · `483d57d` TextInput, TextArea, search · `c1b4369` form system · `dcceb37` overlays, picker, navbar, tabbar, layers · `2c8ed10` clippy-clean generator, complete SVG extraction · then the evidence/docs commit, which adds the form heading label and the `form_vcode:toast` replay fix.

Work from `483d57d` to `dcceb37` was written by parallel workers in an earlier session that stopped at an API usage limit. The takeover lead reviewed it, fixed the defects listed in those commit messages, and verified it on the final tree. Intermediate commits were not built individually.
