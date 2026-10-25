# Showcase verification — 2026-10-24 Pacific

The website is a developer preview; no public deployment was performed. This is
cloud website evidence, not native GPUI rendering/input, an accessibility-tree
qualification, or screen-reader acceptance.

- `build.txt`: all 60 Rust gallery routes match the coverage inventory; 132 recorded
  states, 528 capture assets, generated API documentation included.
- `browser-checks.json` / `.txt`: Chromium 154.0.8037.57 in the isolated Linux cloud
  environment. All 60 routes and 39 component API links pass; search, empty state,
  comparison, clipboard, state/theme deep links, keyboard skip/focus, mobile menu,
  four palettes and invalid-route handling pass. Six pages at widths 390/768/1440
  have no horizontal overflow. No browser JavaScript exceptions.
- `overview-desktop.png`, `overview-mobile.png`, `component-desktop.png`: actual
  website screenshots. Native component images within them are historical captures
  from `2c8ed10`, explicitly labeled in the site.
- JavaScript syntax, actionlint and whitespace validation pass. CI now builds the
  full showcase after strict rustdoc and retains the static artifact.

Failed attempts are retained: `attempt1-build.txt` records root-owned target output;
`attempt2-build.txt` records the unreadable rustdoc lock. The generator now writes
into ignored `website/dist` and excludes the lock. `attempt3-browser.txt` records
an exact accessible-label locator timeout (the select's name included option text);
the test now targets the state selector directly. No product failure was hidden.

The site has not been audited with an actual screen reader. Native application
acceptance remains in PLATFORMS and RELEASE-READINESS. The existing hosted iOS
upstream build failure remains visible; this website slice does not change that gate.

Before push, remote main advanced to `7c9bb9e` with a host palette API. The site
slice was rebased onto it. Strict rustdoc then caught missing documentation on
`Theme::with_palette`; the method is now documented without changing behavior.
`attempt4-rustdoc-path.txt` preserves an initial missing Cargo PATH;
`attempt5-rustdoc-missing-docs.txt` preserves the strict documentation failure.
`rustdoc.txt` is the subsequent passing strict build. The complete site and browser
suite were rerun successfully after integration, along with fmt, JavaScript syntax,
actionlint and whitespace checks. Hosted CI for this new slice remains separate.
