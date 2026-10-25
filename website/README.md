# weGPUI showcase

A static, dependency-free website in the main repository. It presents every gallery
route, recorded light/dark states, native/WeUI comparisons, source links, setup,
themes, API docs and honest platform readiness. Captures are historical macOS
images, not live web-rendered GPUI controls or current platform acceptance.

## Build and preview

Python 3.11+ is required. From the repository root:

```sh
python3 website/build.py
python3 -m http.server 4173 --bind 127.0.0.1 --directory website/dist
```

Open http://127.0.0.1:4173. Hash routes work on static hosts without rewrites.
For the full API reference, first generate documentation from the same checkout:

```sh
cargo doc --no-deps --locked
python3 website/build.py --include-api
```

The builder verifies the coverage inventory against the Rust gallery and requires
every recorded screenshot. It copies 528 capture assets for 60 routes / 132 states.
Source and installation links use the checkout's exact HEAD. Without `--include-api`,
API links fall back to Rust source. Output is ignored at `website/dist/` and excluded
from the crate. CI builds the full site after strict rustdoc and uploads it as the
`wegpui-website` artifact; it does not deploy it.

## Browser verification

Install Playwright's Python package and a cloud/local Chromium browser, then:

```sh
python3 website/check_browser.py --browser /usr/bin/chromium
```

The preview server must already be running. `--browser ''` uses Playwright's
installed default Chromium. `--out` chooses an evidence directory. The checks cover
all routes, API links, search/empty state, deep links, themes, comparison, copying,
keyboard focus, mobile navigation, responsive overflow and missing routes.
These tests exercise the website, not native GPUI or a screen reader.

## Publication

No public host or domain is configured yet. The full `website/dist/` directory is
the deployable artifact, including `.nojekyll`. Use a static host supporting HTTPS;
retain its source SHA and deployment URL in the release evidence. Rebuild API docs
and the site together before deployment. Keep the developer-preview label until
the desktop release gates in `docs/RELEASE-READINESS.md` are complete.
