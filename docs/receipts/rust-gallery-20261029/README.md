# Native Rust gallery website — 29 October 2026

The founder requested a focused showcase of the Rust gallery, with the extra marketing and
project-status content removed. The homepage now displays all 61 native gallery routes,
including Languages. Search, category filters and light/dark captures lead directly to
component states, native captures and the actual gallery Rust source. Native GPUI is the
default; the 60 upstream browser examples remain a separately labeled reference.

The static builder exports the exact gallery source files, validates the complete native
registry and retains capture provenance. The Languages page adds two Linux ARM64 captures
from the RTL receipt after checking their stored hashes. The site contains 133 native scenes,
540 capture assets, and 61 exported source files. Recorded portrait layouts are native desktop
captures; they do not establish mobile backend support or current platform qualification.

## Verification

- Static build: 61 routes and 133 scenes, with no network, npm or Cargo compilation.
- Five existing inventory/build regressions pass after updating their native route and image
  counts and checking the exported Languages source against its repository source.
- `node --check website/app.js`, `cargo fmt --all -- --check` and `git diff --check` pass.
- Existing shell-integration and theme-pages-layout browser checks pass on both root and
  `/weGPUI/` mounts: four targeted check results. These cover the 61-entry native homepage at
  375/768/1280 widths; native source equality; explicit browser-reference deep links; keyboard
  catalog navigation; retained live-frame drafts and appearance settings; and native/reference
  theme layouts at 375/390/768/1280 widths. This is a focused subset, not the full browser suite.
- In-app review confirms the native catalog on desktop and phone widths, direct empty-search
  recovery, category filtering and decoded native dark-theme overlay captures.
- The Impeccable detector reported only the inherited Inter font choice. The stylesheet now
  uses the platform font stack that matches the native component presentation.

`source-sha256.json` identifies the final UI, builder and adapted existing checks. Browser
reports, traces and screenshots are in the local packet
`/Volumes/VIXinSSD/CW/artifacts/wegpui-gallery-20261029/` (`browser-final/` for all four targeted
results and `shell-final/` for the final shell recheck after the empty-search recovery and
native-tab theme fixes). Neither final hosted CI nor deployment was run. The published Pages
site retains its earlier revision until an independently authorized deployment.


## Founder screenshot correction

The founder caught missing emoji glyphs in the historical Linux Languages capture used by
`87861a4`. That image was an unsuitable showcase. The website now uses the hash-verified
macOS English-paragraph capture, where the emoji render correctly, and a separately named
macOS Arabic-selection state for dark appearance. The image provenance, state selector and
native run command follow the actual capture. The catalog remains 61 routes and now exposes
134 native scenes. This replaces the website sample; it does not qualify Linux emoji rendering.

Native preview images now use the page's normal scrolling, removing the clipped nested image
scroll area. The English paragraph presentation trims the partial line above its first full
row; the recorded source PNG remains unchanged. The original `source-sha256.json` describes
`87861a4`; `followup-source-sha256.json` describes this correction.

All five existing inventory/build regressions pass again. JavaScript syntax and diff checks
pass. In-app observations confirm decoded light/dark macOS images, correct state/theme
transitions in both directions, and a 292-pixel preview without page overflow in a 375-pixel
viewport. No native runtime, hosted CI or deployment was performed for this presentation fix.
