# Website update and live-origin verification

The owner requested continued weGPUI parity work and website updates. The reviewed source
and receipts were pushed to `main` at `b368c941ecb500925234a6981803fa91fc1e1a3b`.
The existing manual-only Pages workflow was dispatched through the owner's authenticated
GitHub CLI, without changing DNS, the domain, Pages settings, secrets or permissions.

[Run 40281443167](https://github.com/codewhale-hq/weGPUI/actions/runs/40281443167) passes
both build and deploy. The website is <https://codewhale-hq.github.io/weGPUI/>.
[Workflow metadata](workflow.json) records the checked-out SHA and completed jobs.
The live `data.js` reports that same commit and `sourceDirty: false`; [identity](identity.json)
retains the downloaded data hash. The earlier cloud proxy restriction does not apply to
this local host's successful HTTP/browser observations. Local Codex browser control was
disabled; the existing isolated Chromium website verifier supplied the browser evidence.

On the published origin, [the recorded-view verifier](recorded-browser.json) passes all
264 state/theme views and decodes all 528 referenced images. It checks the five recent
pairs' source/platform provenance, responsive overflow, state/theme deep links, comparison,
clipboard copying, skip/focus behavior, navigation, missing-route handling and all palettes.
No page errors occur. [The interactive check](interactive-browser.json) reuses the existing
shell and theme-layout cases on the public origin: direct deep links and reloads at
375/768/1280px, a live dialog, iframe/draft retention, theme/care/brand controls, keyboard
catalog selection, state/history, and palette changes at 375/390/768/1280px pass.
This is a selected live-origin check, not a second full 42-case sweep. The complete local
42-case root/project run remains in the [Unicode receipt](../unicode-parity-20261026/README.md).

The initial live-interactive driver used a nonexistent method name and stopped before
executing its cases. Correcting the fixture to the existing `Checks.shell` method produced
the retained pass; no product code or website deployment changed for that driver correction.

The site now distinguishes historical macOS captures from recent Linux ARM64 samples,
validates their receipt hashes during builds, links the latest Unicode continuation, updates
platform claims, and preserves live preview controls while changing palettes. Capture views
remain recordings; the live WeUI browser demos remain a separate HTML implementation.
No native GPUI runtime, real IME, reader, package-release or platform-support pass follows
from this website publication. Full native parity remains open.
