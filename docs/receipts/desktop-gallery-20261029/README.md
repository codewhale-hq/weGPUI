# Native desktop gallery — 29 October 2026

The founder identified that the website showed phone compositions only. The native Rust
example now opens at 1120 × 760 by default. At 960 logical pixels and wider it adds persistent
component navigation, theme controls and a bounded content area. Narrow windows retain the
phone composition. Both use the same components and gallery state; no browser renderer or
second component runtime was added.

The registry has 62 routes. Existing WeUI controls use their layout and size APIs in desktop
compositions; the button examples wrap compact inline variants, and the overview presents
category groups in columns. The new Desktop form route consumes the actual
`wegpui::desktop::FormRow` from `1d5d9ae`, together with `TextInput`, `Switch` and compact
`Button` controls. It exposes aligned labels on desktop and stacked labels in narrow windows.
It demonstrates local view state only, with no account, transport or durable storage.

The website defaults to Desktop and offers Desktop/Phone controls on the catalog and every
native component. Desktop images come from the real native application at 1120 × 760,
recorded at 2× resolution. The builder validates native route/state coverage, image hashes,
PNG dimensions, viewport identity, source identity and unique capture entries. Rust gallery
and component sources are exported as plain-text assets. The 60 upstream HTML examples keep
their separate browser-reference tab.

The native source is committed as `2c56bab8e79584f1de13a779945b7dc592e4d6f2`.
`metadata/committed-source.json` records that all 477 frozen Rust, manifest and lock inputs
match that commit. The Codewhale application still uses its own dependency pin; this gallery
run does not establish compatibility for that application's older pin.

## Local verification

- Linux ARM64, GPUI 0.3.9 with the repository's existing compatibility sources, Rust 1.99.1.
- Locked, offline native gallery build and strict gallery Clippy pass under the machine's
  two-slot build lock. Three existing gallery tests pass; no test cases were added.
  `cargo fmt --all -- --check` passes.
- Eight native running-product observations pass: keyboard text entry, draft retention
  through a theme change, save status, switch toggling, reset, offscreen navigation focus
  reveal, sidebar navigation and tab selection. These used AT-SPI actions/focus and XTEST
  keyboard events in an isolated Xvfb session; they do not establish screen-reader speech.
  See `metadata/native-observations.json`.
- Frozen native source is recorded in `metadata/native-source.json`. The capture manifest
  records the executable SHA-256 and the actual arguments for each image.
- Completed matrix: 272 desktop captures across 136 scenes and both appearances, plus four
  narrow-window Desktop form captures. All 276 captures retain their image hashes, source
  manifest and executable identity; the final source check passed after capture completion.
- Static build passes with 62 routes, 136 scenes and 816 bundled PNGs. All five existing
  website inventory/build cases pass. JavaScript syntax, Python compilation and diff whitespace
  checks pass.
- The existing capture-view browser check passes across 542 layout/state/theme views,
  806 referenced images and 41 source/API links. It also checks clipboard commands, comparison,
  keyboard access, mobile catalog navigation, all four palettes and 21 responsive page views.
- The focused shell and theme-page checks pass at both the root and `/weGPUI/` prefix: four
  check results, with widths from 375 to 1280 pixels. The complete browser-interaction suite
  was not rerun. `metadata/website-checks.json` contains the bounded results.

The first expanded overview crashed because several simultaneously expanded Cells groups
shared an accessibility identifier. Each group now has a unique identity; the failure and
initial partial pass remain in the local artifact packet. A 0.7-second Layers capture was
uniform and was rejected. The valid retry waits for its heavier canvas to render. Captures
also reject early application exit and bound screenshot subprocesses, preserving failures.

The new sidebar uses the existing `FocusScroll` wrapper so keyboard focus remains visible
through its complete component list. The narrow form's actions wrap when needed.

Representative native views were inspected in light/dark appearance and narrow/wide layouts.
This evidence does not establish full visual parity, every control interaction, screen-reader
speech, IME, hardware rendering, Windows/macOS runtime, mobile devices or release packaging.
No provider spend, hosted CI, deployment or package publication occurred.

Local logs, initial failures and native observations are retained under
`/Volumes/VIXinSSD/CW/artifacts/wegpui-desktop-gallery-20261029/`.
