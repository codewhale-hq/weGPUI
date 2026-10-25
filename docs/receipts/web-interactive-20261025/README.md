# Interactive browser verification — 2026-10-25

The browser gallery is **WeUI HTML/CSS/JavaScript**, with local interaction adapters. These
checks do not execute Rust/GPUI in the browser or qualify native component completion.
Recorded native captures and desktop receipts retain their own source/host scope.

## Candidate 1: complete interactive suite

`final-candidate1-rerun/summary.json` records **PASS**, exit 0, complete suite, with the built
site unchanged throughout collection. The immutable site is
`/workspace/.wegpui-cloud/web-main-candidate-1`. Its app.js SHA-256 is
`719eb46ed44fcabe1818f5bb9ca70daed198f47ef1195229393cfc7bbe73b100`.
The complete 634-file manifest is `final-candidate1-rerun/built-site-sha256.json`
(manifest SHA-256 `1228030425e2683a9b320ac06bc4beb500f26bfa721a65046bc2ea622eb9dd94`).

The archived validator is `final-candidate1-rerun/validator.py`, SHA-256
`5bb5282dc35c9a7896fb215a786cd859ca833c92f5e775346f8eba9148fbacd0`.
The collection's Git HEAD is `3840b47370f8defa925cbff7d3bf12ccf6250722`; its recorded dirty
working state is context. The executed built-file hashes identify the actual web candidate;
Git HEAD alone does not attest its uncommitted website changes.

Chromium **156.0.8078.47** ran headlessly in the existing Debian 13 container through
`/opt/parity-python/bin/python` and Playwright. Each mount starts its own loopback HTTP server
and fresh browser context. The project test serves the site only at `/weGPUI/`, without root
assets that could hide broken absolute paths. Requests to other origins or write methods
are refused and fail the check; data/blob URLs remain available for local previews.

| Check | Recorded result |
| --- | --- |
| Route loading/responsiveness | All 60 routes at 375/768/1280px under both `/` and `/weGPUI/`: 360 successful observations, no broken HTML images or document/component-page horizontal overflow. |
| Interactive cases | 20 checks per mount, 40 passing checks total. No console/page/network/outside-origin/write-request error was observed. |
| Overlays | Dialog, half-screen dialog and ActionSheet pointer opening/closing, Tab/Shift+Tab trapping, Escape and actual-opener restoration. |
| Input and choices | Real HTML readonly/disabled/maxlength, Unicode/multiline counter, checkbox/radio/switch changes, visible consent validation and guarded Vcode send. |
| Navigation and editing | Navbar/Tabbar pointer/keys/panels; search submit/clear/cancel; picker staging/cancel/disabled skip/cascade; slider pointer/keys; progress start/cancel. |
| Media | In-memory local PNG → blob preview → keyboard open/Escape/restore → delete, plus gallery close/reopen. No upload request is sent. |
| Content/feedback | Swipe keyboard and real drag/delete; per-notice dismissal/restore; passive timed toast; footer/grid/panel/preview/message callbacks; home/layers keyboard navigation; real article scrolling. |
| Themes | At all three widths: normal/care light palette values, dark background, explicit purple-brand override. Care is a contrast palette, not a font-size enlargement. |
| Embedding | Partial appearance messages preserve the same input element, value, selection and focus; a real same-origin message from a different window is ignored. |
| Main shell | Responsive deep links/reload, real iframe dialogs, toolbar draft preservation, brand reset, catalog keyboard navigation, example state, history, native-capture mode switch and preferences across routes. A 768px inner preview scrolls horizontally on a 375px page without outer-page overflow. |

Six primarily presentational families—badge, icons, flex, loading, loadmore and steps—are
recorded as content/layout/asset checks, without invented callback acceptance. The route
inventory covers 31 grouped families. Graphical `steps_horizonal` deliberately hides its text
in the pinned source; its actual painted step geometry is checked instead of requiring text.

Per-mount JSON, action/configuration records, screenshots, HTTP logs and Playwright trace ZIPs
are retained. Three labeled overview sheets (`review-controls.jpg`, `review-themes.jpg`,
`review-shell.jpg`) were opened and inspected. Raw narrow main-shell, restored modal/history,
Unicode textarea and dark-theme captures were also opened. The sheets are downscaled review
aids; original PNGs are unchanged. This is a bounded visual review, not pixel-perfect or
all-state appearance acceptance.

## Later corrections and bounded requalification

Candidate 1's full interactive pass remains unchanged. The separate historical-image checker
then exposed two main-shell issues outside that suite:

1. `legacy-palette-candidate1/result.json` reproduces a native-token palette change leaving
   `#native-themes` for `#themes`. The one-line app fix retains the current page name.
   Candidate 2 (`/workspace/.wegpui-cloud/web-main-candidate-2`) changes **only app.js** from
   candidate 1; all 633 other files match. App SHA-256 is
   `9eb728841869d86aa15152a4fa74c37ff79a5dfddd91b1674422e06c5a767879`.
   `candidate2-delta.json` and `candidate2-site-sha256.json` preserve that comparison.
2. `legacy-candidate2/` completes the historical capture phase but fails the later responsive
   loop at 390px on the web theme page. `themes-overflow-candidate2/geometry.json` records
   document width 390 versus scroll width 395: both grid sections retain a 375px automatic
   minimum at x20. The root correction adds `.two-column>section{min-width:0}`.

Candidate 3 (`/workspace/.wegpui-cloud/web-main-candidate-3`) copies candidate 2 and changes
**only style.css**. All other 633 files match; `candidate3-delta.json` and
`candidate3-site-sha256.json` retain the proof. Style SHA-256 is
`27d70324b31e50c573a5e26d821d11bcfb6946313dac973f03f0b86cc6cd9bff`.
The candidate directories remain separate and read-only.

Candidate 2's bounded interactive theme/main-shell tests pass at both mounts in
`targeted-candidate2/`. Candidate 3 is the final qualified artifact:

| Candidate 3 check | Result |
| --- | --- |
| `legacy-candidate3/summary.json` | PASS, both collectors exit 0. Each mount checks all 60 routes, 264 historical state/theme views and 528 decoded capture files; 27 responsive page/width combinations, clipboard, comparison, catalog, missing-route and four palettes on both token-page routes also pass. These are historical images, not native runtime checks. |
| `targeted-candidate3/summary.json` | PASS at both mounts, unchanged build: three affected-surface checks per mount (six total). Themes/brand/care and complete main-shell flows remain green. |
| Web/native token-page correction | Both pages retain their route after palette selection at 375/390/768/1280px. All 16 page/width/mount observations have no outer overflow; the web page's nested care-dark checkbox frame responds to actual pointer and Space input. The native token page has no substituted live frame. |

Candidate 3's 390px theme-page screenshot was also reviewed by the integrating root agent;
the previously overflowing grid and its live care-dark preview fit. The complete 360-route
suite is not relabeled as having run on candidate 3: candidate 1 supplies that proof, while
the exact two-file artifact comparison and affected-surface reruns delimit the later candidate's
qualification. All earlier failed evidence remains retained. No further source change was
made after these green results.

## Preserved attempts and interpretation

- `development-attempt1..4/` preserve exploratory failures while the bridge/main UI evolved.
  Actual Clear focus and picker-pointer issues were corrected by the bridge owner. Other
  failures were harness assumptions: zero-height wrappers around fixed dialogs/toasts, hidden
  legacy consent markup, a dynamic unchecked-radio locator, reusing a deleted route instance,
  a pointer left outside a resized viewport, CSP-incompatible bare wait expressions, care
  font enlargement, and requiring text in a graphical-only page. They are not claimed as
  remaining product failures. Runs whose build changed during collection stay developmental.
- `development-audit5/` passes the 180 project-prefix route/width checks and partial-config
  preservation checks, but is explicitly a subset rather than the final combined suite.
- `final-candidate1/` passes all standalone cases and fails its main-shell pointer step because
  the action is below the outer viewport. `shell-scroll-diagnostic/geometry.json` records the
  action at y988 in a 900px window; an ordinary outer iframe scroll moves it to y565 and the
  same pointer click succeeds. The corrected full runner includes that user scrolling step.
  Neither application code nor the immutable candidate changed for that correction.

## Limits and reproduction

The browser composition check uses Chrome's Input.imeSetComposition/insertText protocol. It
is labeled separately from a real OS IME. The color control is tested through its HTML input
event, not its native popup picker. File input uses an in-memory transparent one-pixel PNG;
the checks establish lifecycle/focus/locality, not photo quality or a system file-chooser UI.
No browser screen reader, human usability sign-off, Safari/Firefox or other operating system
is qualified. No CI dispatch, deployment or public post was performed by this collector.

Run the complete suite against a built site using the existing environment:

```sh
/opt/parity-python/bin/python website/check_interactive.py \
  --site /absolute/path/to/site --out /new/evidence/directory
```

The runner creates both hosting mounts itself. Existing output directories are refused to
preserve previous evidence. `--only` and `--hosting` are development options recorded as
incomplete. `run_legacy.py` retains the exact historical checker and runs it at both mounts;
that checker now selects Native mode explicitly and checks web/native palette routes separately.
