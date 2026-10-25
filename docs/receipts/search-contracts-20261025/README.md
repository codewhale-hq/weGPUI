# SearchBar native contracts — 2026-10-25

Linux X11 callback/focus/read-only proof on the integrated build 5 artifact. Exact source snapshots are `build5-search_bar.rs.txt` and `build5-probe.rs.txt`; artifact digest is `build5-binary.sha256`. The immutable executable is `/workspace/.wegpui-cloud/search-integrated-20261025-build5/search_contracts_probe`. Root's integrated build passed all targets and 45 unit tests. This receipt records runtime observations separately from those source checks.

`build5-runtime/result.log` passed the real XTEST pointer and keyboard assertions. The driver queries AT-SPI to identify targets and focused state; those queries are not screen-reader acceptance. The launcher starts Xvfb, Openbox and a fresh D-Bus/AT-SPI session with `GPUI_X11_SCALE_FACTOR=1`. Each run saves its exact launcher/driver, environment/source/binary provenance, application events, screenshots and accessibility snapshots.

Verified behavior:

- Homepage `Focusable` moves focus from the camera to the actually rendered opener, confirmed by `home_focused=true` and the focused accessibility target. Pointer, Enter and Space emit `Open` three times and camera activation emits `Camera` three times after the bootstrap click.
- Outlined `words("Keyword")` is visibly rendered and exposed as a Label. Explicit cancellation while initially internally unfocused emits one `Cancel` per call and preserves text with `clear_on_cancel(false)`.
- Read-only text ignores typing, Backspace and Delete and exposes no UI clear action. Making a nonempty editable field read-only hides its clear target; typing/deletion and clicking the former clear position preserve `alpha` with no `Changed` or `Clear` events.
- Editable clear emits one `Clear` and one `Changed`, empties text and retains input focus. Outlined pointer/Enter/Space search emits three `Submit` events; input Enter submits once. Back and Cancel buttons respond through all three input methods.
- Filled text editing, Enter submit, clear/refocus and default cancellation work. Default cancellation clears text and returns the placeholder.

The final observed event counts were outlined Changed 8, Cancel 6, Submit 4, Focus 1, Clear 1, Back 3, Blur 1; filled Changed 11, Submit 1, Clear 1, Cancel 1, Focus 1, Blur 1; home Open 3, Camera 4. Camera includes the initial real-click bootstrap. Outlined Cancel includes two explicit API calls, three button activations and one automatic empty blur cancellation. Counts are fixture observations, not API promises about duplicate focus notification ordering across hosts.

`failed-no-initial-focus/` is preserved. The first driver attempted function keys while only the frame was active and no child had focus, so the root key handler did not receive them. The successful driver begins with a real camera click, then verifies programmatic focus. No component fix was attributed to that harness failure.

This fixture covers Homepage, Outlined and Filled in the default light theme; it does not qualify FilledGrey, disabled transitions, all themes/states, Wayland, screen readers or other operating systems. Later build 7 focus-contrast styling supersedes these focus screenshots; see `../focus-contrast-20261025/`. No component acceptance row is established by this receipt.
