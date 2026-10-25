# Fresh API website follow-up — 2026-10-25

The site was rebuilt with the root-produced strict Rustdoc tree for build-7 source,
including the new public `Theme::focus_color` and `Theme::focus_color_with` helpers
and the latest DateField/picker continuation. The website collector invoked no Cargo
commands and changed no website, library or vendored source during this follow-up.

```sh
python3 website/build.py --include-api --api-dir /workspace/.wegpui-cloud/target-198/doc
python3 website/check_browser.py --url http://127.0.0.1:4178/ \
  --browser /usr/bin/chromium \
  --out docs/receipts/component-contracts-20261025/website/final-api/browser
```

A temporary loopback-only Python server served `website/dist`; it was stopped after
verification. No public deployment, CI dispatch or publication occurred.

## Exact snapshot

`source-sha256.txt` identifies Cargo files, current Rust source and website source at
the time of this build. `api-sha256.txt` identifies every copied API file: 769 files,
16,317,333 bytes. `provenance.json` records the timestamp, tool versions and observed
HEAD `a3ce3afca6417b7dd1b9d701456ae2d21d2150f4`, with `source_dirty: true`.
That HEAD is contextual and is not a final commit attestation. These hashes identify
the checked source/API snapshot; later gates or commits remain separate.

Python was 3.12.14, Node was 24.19.0 and Chromium was 154.0.8037.57. The API was generated
by the root's Rust 1.98.1 strict documentation check. The collector only copied and
inspected that output; it did not claim a second strict Rustdoc execution.

## Results and limits

The existing complete browser checker passed 60 routes, 264 state/theme routes,
528 decoded captures, 39 component API links, 18 viewport/route combinations and all
four palettes, with no JavaScript page errors. Search, deep links, comparison, copying,
keyboard focus, mobile navigation, overflow and missing-route checks passed.
`browser-checks.txt` and `browser/browser-checks.json` retain the results.

The supplemental `api-target-checks.json` verifies the get-started API root, theme API
link, both new Theme method anchors and their source links, and DateField reference/source
links. All requested local pages returned HTTP 200. Its first attempt assumed an exact
DateField heading name that differed from Rustdoc's accessible name; the diagnostic is
preserved in `api-target-attempt1.txt`. The corrected locator checks the actual h1 text.
There was no source, API or link change between those attempts.

The website's native images still come from historical macOS capture revision `2c8ed10`.
This rebuild refreshes API documentation, not those native images, and provides no new
native runtime, visual parity or screen-reader acceptance. Final source/commit and release
qualification remain governed by the repository's acceptance and release-readiness docs.
