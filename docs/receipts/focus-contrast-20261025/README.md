# Native focus contrast — 2026-10-25

This receipt verifies a focused Linux X11 accessibility improvement. It does not accept any complete component row, establish upstream appearance parity or qualify all WCAG requirements. Screen-reader, Wayland and other-host evidence remain separate.

The default brand `#07c160` has only 2.385:1 contrast on white and 2.226:1 on `#f7f7f7`. Custom `#7c3aed` passes dark BG2 `#191919` at 3.085:1, but fails dark BG3 `#202020` at 2.859:1 and dark FilledGrey `#383838` at 2.058:1. A light/dark switch or one fixed theme focus token cannot address those different surfaces.

## Implementation and source

`Theme::focus_color` / `focus_color_with` calculate WCAG sRGB luminance contrast on the painted surface, preserve the preferred colour when its rendered contrast reaches 3:1 and otherwise choose opaque black or white. `surface_color` explicitly composites translucent fills over their backdrop. Two unit tests cover all four palettes, purple and translucent brands, layered surfaces, a custom palette and known contrast values. The generated palette is unchanged.

The existing `form::focus_ring(color)` API remains a 2px inset ring. New two-colour helpers provide two 2px bands for controls with inherited or unknown backgrounds. Focus styling does not change unfocused layout or colours. The final text caret is a 1px black core with 1px white side bands; its 3px paint footprint does not change text layout.

| Paint surface / situation | Focus styling paths |
| --- | --- |
| BG2 or an explicitly painted modal/list surface | ActionSheet, Dialog, FormPreview, SelectMenu |
| Selected Navbar background composited over BG2; Tabbar BG1 | Navbar, Tabbar |
| Selected picker BG3 composited over BG2 | Picker's manual selected-row outline |
| Actual Homepage field, input form, clear mask and outer bar surfaces | SearchBar opener/camera/back/clear/placeholder/cancel; outlined submit uses its brand fill |
| White thumb; uploader's explicit `#ededed` / `#2e2e2e` add box | Slider, Uploader add |
| Actual information-bar variant background with its preferred foreground | InformationBar link/close |
| BG2 row; BG0/default or red action fill | Swiped row/manual outline and actions |
| BG5 cell fill; opaque ordinary/overlay fills; unknown translucent fills | Button: computed preferred colour for opaque fills, black/white inset bands for translucent fills |
| Inherited host surface | Agree/link, Cell, CheckboxCell, RadioGroup, form links/clear, DateField, FooterLink, Grid, IconButton, MediaBox, Progress cancel, HalfScreenDialog header actions/links, VcodeButton and gallery example footer use two-colour bands |
| Small standalone controls | Checkbox draws inset bands on an empty enlarged overlay after its glyph; Switch uses outer bands |
| Unknown image pixels | Gallery draws both bands in an overlay **after** the image, inside the clipped window; Uploader preview/delete targets draw both bands above the thumbnail |
| Gallery delete's explicit `#0d0d0d` surface | Preferred theme white passes through the shared contrast helper; root applied this narrow custom-palette correction after build 7 |
| Msg paragraph BG2 | LinkText uses a contrasting 2px focus underline |

`source-parent.txt`, `source.diff`, `source-sha256.json`, `cargo-source.diff` and `unchanged-build-inputs-sha256.json` record the parent and relevant working source/toolchain inputs. `source.diff` includes the current integrated batch's changes relative to the parent, including changes made by other agents. It is not a patch attributed only to this focus task. Subsequent Checkbox/caret corrections have separate source snapshots and hashes. Root owns the complete integrated gates and final source manifest. Per-run Git-state hashes describe the launch workspace; when an earlier immutable artifact runs while later source is being corrected, the corresponding archived source snapshot and artifact digest identify that binary.

## Artifacts and runtime

The Rust 1.98.1 binaries were copied before further compilation. Every run records its exact launcher/driver, binary hash, source/environment provenance, application log, screenshots and AT-SPI focused state. Runtime packages are in `runtime-packages.txt`; pixel/contact-sheet analysis versions are in `analysis-environment.txt`. Xvfb and Openbox run together in a fresh D-Bus/AT-SPI session, with `GPUI_X11_SCALE_FACTOR=1` and XTEST keyboard/pointer input. AT-SPI locates targets and reads state; an AT action is not screen-reader output.

| Artifact | Exact source / hash | Purpose |
| --- | --- | --- |
| `/workspace/.wegpui-cloud/focus-integrated-20261025-build7/focus_contrast_probe` | `probe.rs.txt`, `binary.sha256`, original source manifest | 128 focused targets across eight theme combinations; initial caret |
| `/workspace/.wegpui-cloud/focus-integrated-20261025-build8/focus_contrast_probe` | `probe-checkbox-corrected.rs.txt`, `checkbox-corrected.rs.txt`, `checkbox-corrected-binary.sha256`, `checkbox-corrected-source-sha256.json` | Corrected Checkbox interior, pointer/keyboard/AT callbacks |
| `/workspace/.wegpui-cloud/focus-integrated-20261025-build9/focus_contrast_probe` | `probe-caret-corrected.rs.txt`, `input-caret-corrected.rs.txt`, `caret-corrected-binary.sha256`, `caret-corrected-source-sha256.json` | Final caret on Outlined and actual FilledGrey input surfaces |

The modes are light, dark, care-light, care-dark, purple-light, purple-dark, care-purple-light and care-purple-dark. Purple is the existing native `Theme::set_brand(#7c3aed)` extension; no claim is made that the pinned upstream script supports an equivalent custom-brand policy.

The 16 original focused targets per mode are Default/Primary/Warn buttons, Homepage opener/camera, outlined Back/Clear/Submit/Cancel, unchecked/checked standalone Checkbox, checked Switch, CheckboxCell, RadioGroup, Picker and Gallery image. `check-pixels.py` independently calculates contrast and checks actual native edge pixels against expected colours, including the two bands. `pixel-checks.json` contains 128 passing **edge-colour** measurements. Those measurements alone did not establish that a component's interior was correct; the visual rejection and correction below are material.

## Visual review and preserved failures

All 128 original focus targets were visually reviewed through the eight `review-focus-{mode}.png` crop sheets; `visual-review-manifest.json` lists every original filename. Full raw captures additionally inspected include light Default/Checkbox/Search submit/Picker/image, dark Primary, dark-brand Homepage/Picker/Search submit and brand Search submit enlarged in `brand/search-submit-crop.png`. The purple submit's white inset focus band was reviewed as correct; no separate defect was claimed from its rounded corners.

Visual review rejected the original standalone unchecked Checkbox focus paint: the outer white drop shadow filled its transparent centre, visible in all four dark-mode `checkbox-off.png` captures. Those rejected captures remain in the original mode directories and `review-focus-*` sheets. The correction tracks keyboard focus and paints inset bands on a separate empty enlarged overlay after the glyph. The centre retains its host surface; disabling the component also suppresses the overlay immediately.

The build 8 `checkbox-fixed-{mode}/` runs all pass. In every mode, pointer, Space, Enter and an AT-SPI `click` action each fire exactly once, yielding `true,false,true,false` and a final unchecked state: 32 observed callbacks across eight modes. Focused unchecked centres exactly equal the unfocused centre: white in the four light modes and `#191919` in the four dark modes. The two outer 2px bands are independently checked. `checkbox-fixed-results.json` and `review-checkbox-corrected.png` record and visually compare all eight corrected results. This does not verify a focused-to-disabled transition through a stale AT proxy.

Build 7 caret snapshots in `caret-{mode}/` and `caret-pixels.json` verify the original brand core/solid backing on the eight Outlined BG2 inputs only. A source-backed counterexample showed that purple plus black backing could both fall below 3:1 on dark FilledGrey. Root changed the caret to black and white side-by-side paint; those original caret captures remain historical evidence, with their limited scope. The final build 9 results are recorded below.

Other preserved harness failures:

- `failed-host-no-xvfb/`: the launcher was mistakenly invoked on the host, which lacks Xvfb. All actual runtime qualification uses the prepared container.
- `failed-picker-role-expectation/`: the driver expected ComboBox, but the actual picker column correctly exposed ListBox. The expectation was corrected before rerunning.
- `failed-caret-geometry-tuple/`: PyAT-SPI returned character extents as a tuple; the first driver incorrectly accessed `.x`. Corrected runs index the tuple.
- The initial caret pixel lookup used the rounded accessible character end as the exact raster column. Actual fractional advance rounded the caret one pixel later. The checked analysis now searches within two pixels of the accessible end and requires the unique three-pixel caret pattern; the native screenshots are unchanged.

## Final caret and FilledGrey results

All eight build 9 `caret-final-{mode}/` runs pass. Each captures the actual Outlined BG2 input and the same reusable SearchBar switched to FilledGrey (`#ededed` light, `#383838` dark), with its input focused. `check-final-caret-pixels.py` finds the unique white/black/white three-pixel caret next to the accessible character end and verifies the actual painted surface five pixels to its right. All 16 caret surfaces pass: at least one of the black/white paints contrasts above 3:1 against the measured surface. `caret-final-pixels.json` records the coordinates and ratios. All 16 crops were visually reviewed in `review-caret-final.png`.

The same eight runs keyboard-focus FilledGrey Back, Clear and Cancel. All 24 actual focus-edge pixels match the independently calculated preferred/fallback colour and reach 3:1 against their respective painted surfaces; `grey-action-pixels.json` records them. All 24 crops were reviewed in `review-filled-grey-actions.png`. In particular, dark-purple Clear correctly falls back to white on `#383838`, while dark-purple Back/Cancel preserve purple on BG2. This adds focused-state coverage for FilledGrey; it does not complete its callback/disabled/read-only/theme-state acceptance matrix.

The transient Rust probe was removed after each artifact was copied; its exact source remains in the receipt. Launchers can rerun the immutable artifacts directly; rebuilding a probe requires restoring the corresponding `*.rs.txt` to a temporary example and using the recorded integrated source/toolchain.

## Limits

The helper's unit tests cover custom/translucent colour inputs; the native theme matrix uses the built-in light/dark/care palettes and opaque purple brand. It does not qualify every arbitrary host palette, image, inherited opacity, transition, clipping context, component state or focus appearance rule. The root's one-line custom-white Gallery delete correction uses the tested helper, but no dedicated native custom-white Gallery-delete fixture was run. Standalone host backgrounds and component-specific accessibility/keyboard contracts still need their complete acceptance coverage. No source or screenshot count establishes full WeUI completion.
