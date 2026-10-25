# weGPUI browser gallery

An interactive-first static gallery with 60 live browser routes and 132 state presets,
using pinned WeUI 2.6.26, WeUI.js 1.2.17 and local interaction adapters. Try controls,
forms, overlays, light/dark themes, care mode and a custom brand color. Upload examples
preview selected images locally in the browser; they have no server upload transport.

These demos are a separate HTML/CSS/JavaScript implementation. Native Rust/GPUI source,
setup and acceptance evidence remain available through `#native-guide` and the
**Native Rust captures** view, for example `#components/button_default?view=native`.
Its 528 comparison images are historical captures, not live native controls or current
platform acceptance. See [playground provenance and adapters](playground/README.md).

## Build and preview

From the repository root, with Python 3.11 or newer:

```sh
python3 website/build.py
python3 -m http.server 4173 --bind 127.0.0.1 --directory website/dist
```

Open http://127.0.0.1:4173/. The normal build uses checked-in assets and needs no
Cargo, Rust toolchain, npm install or network access. Scripts, styles, images and
licenses are bundled locally. Output defaults to the ignored `website/dist/`;
`--out PATH` selects another output directory. Relative assets and hash routes support
both a domain root and a project prefix such as `/weGPUI/`, without rewrite rules.

The builder checks route/state inventory, template hashes and local asset provenance.
Source/install links identify the checkout's exact HEAD; a dirty preview is marked.
Rebuild from committed source before sharing revision-pinned setup instructions.

Rust API documentation is optional. The default build links to Rust source. To include
fresh rustdoc from the same checkout:

```sh
cargo doc --no-deps --locked
python3 website/build.py --include-api
```

With `--include-api`, Cargo metadata resolves the documentation directory, including
`CARGO_TARGET_DIR` and Cargo configuration. `--api-dir PATH` selects a prebuilt rustdoc
tree explicitly. Copying an existing tree does not establish its source freshness.

## Browser verification

Install Playwright's Python package and Chromium, then run the interactive suite:

```sh
python3 website/check_interactive.py --browser /usr/bin/chromium
```

It starts its own temporary servers and checks both `/` and `/weGPUI/` by default.
Checks cover all 60 routes at 375/768/1280px, representative interactions, overlay
keyboard/focus behavior, local media, themes, shell navigation and native-view access.
Unexpected external requests are blocked and reported. `--site PATH` chooses the built
directory; `--out PATH` chooses a new evidence directory. `--hosting root|project` or
`--only CASE,CASE` selects a development subset and is recorded as incomplete.
Default evidence goes to a fresh UTC-stamped directory under `target/web-interactive/`.

The legacy capture-view suite requires the preview server above to be running:

```sh
python3 website/check_browser.py --browser /usr/bin/chromium
```

It checks 264 historical state/theme views, decodes all 528 capture assets, and checks
source/API links and surrounding website behavior. `--url URL` selects the server base
and `--out PATH` selects evidence output; its default is UTC-stamped under
`target/website-browser/`. Both suites preserve source/build provenance and exercise
the website. Native GPUI and actual screen-reader acceptance require separate checks.

## Publication

The gallery is published at <https://codewhale-hq.github.io/weGPUI/>. GitHub records a
successful build and deployment of `e5b6996` in
[run 40211664701](https://github.com/codewhale-hq/weGPUI/actions/runs/40211664701).
The owner selected GitHub Actions in Pages settings and dispatched this publication.
The cloud proxy blocks access to the published origin, so local browser results remain
separate from deployed-origin rendering and interaction acceptance.

Deploy the complete `website/dist/`, including `.nojekyll`, only after owner approval.
[HOSTING.md](HOSTING.md) describes the manual-only GitHub Pages workflow, required
settings, project-prefix behavior and Cloudflare alternative. Subsequent source pushes
do not trigger this manual-only workflow. CI dispatch and deployment
remain separately authorized actions. Keep the developer-preview label until the
[desktop release gates](../docs/RELEASE-READINESS.md) are complete.
