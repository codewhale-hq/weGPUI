# weGPUI component gallery

The homepage is a visual catalog of all **71 native Rust/GPUI gallery routes**, including
nine Mac desktop examples alongside the Languages and Desktop form pages. Search and category filters lead directly
to recorded native previews, light/dark themes, component states and the actual Rust gallery
source. Getting started and theming default to Rust. Desktop/Phone controls select actual
native captures of the two compositions. The Rust gallery opens in desktop mode by default
and adapts when resized.

Each component page identifies its capture host and source revision. The 159 native scenes
and 862 bundled capture images include historical macOS comparisons, recent Linux replacement
pairs, two hash-verified macOS Languages captures, 276 desktop-gallery captures, and 46 current desktop component captures (45 macOS and one Linux VM). Native captures show recorded Rust
rendering; the browser does not execute GPUI. Mobile backends remain experimental.

The **WeUI browser reference** tab retains all 60 pinned upstream HTML/CSS/JavaScript routes
and 132 state presets, with WeUI 2.6.26, WeUI.js 1.2.17 and local interaction adapters. It is
secondary to the Rust gallery and keeps its own provenance. Upload examples preview files
locally with no server upload transport. See [playground documentation](playground/README.md).

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
`native-evidence.json` selects recent samples from a checked-in receipt. The builder verifies
its matrix, capture manifests and sample hashes, and rejects duplicate sample identifiers.
The browser inventory excludes the eleven native-only routes. The website includes them
with their native Rust source and hash-verified captures, then checks the full native
registry. Unexpected missing or extra routes fail validation. Run
`python3 -m unittest discover -s website/tests -v` for the five inventory/build regressions.
Historical local Chromium verification passed 42 checks across both mounts (360 route/viewport
observations), including retained live-frame/draft/focus during palette changes; see
[the Unicode/website receipt](../docs/receipts/unicode-parity-20261026/README.md).
The [Rust gallery website receipt](../docs/receipts/rust-gallery-20261029/README.md) records the initial Rust-first refresh.
The [desktop gallery receipt](../docs/receipts/desktop-gallery-20261029/README.md) records the
native desktop matrix and the checks for Desktop/Phone switching.
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

The capture-view suite requires the preview server above to be running:

```sh
python3 website/check_browser.py --browser /usr/bin/chromium
```

It checks 542 recorded layout/state/theme views, decodes all 806 referenced capture images, and checks
source/API links and surrounding website behavior. `--url URL` selects the server base
and `--out PATH` selects evidence output; its default is UTC-stamped under
`target/website-browser/`. Both suites preserve source/build provenance and exercise
the website. Native GPUI and actual screen-reader acceptance require separate checks.

## Publication

The gallery is published at <https://codewhale-hq.github.io/weGPUI/>. The latest reviewed
update publishes `b368c94` through [run 40281443167](https://github.com/codewhale-hq/weGPUI/actions/runs/40281443167).
That deployment’s live-origin verification confirmed the clean source revision, all 264 recorded views and 528
referenced images, plus selected interactive/deep-link/palette checks. See the
[deployment receipt](../docs/receipts/pages-update-20261026/README.md). The original owner-run
publication and earlier cloud-proxy restriction retain their own historical receipt.

Deploy the complete `website/dist/`, including `.nojekyll`, only after owner approval.
[HOSTING.md](HOSTING.md) describes the manual-only GitHub Pages workflow, required
settings, project-prefix behavior and Cloudflare alternative. Subsequent source pushes
do not trigger this manual-only workflow. CI dispatch and deployment
remain separately authorized actions. Keep the developer-preview label until the
[desktop release gates](../docs/RELEASE-READINESS.md) are complete.
