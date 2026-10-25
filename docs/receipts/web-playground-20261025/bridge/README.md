# Local browser adapter implementation and review

This receipt concerns the interactive **browser** gallery. It does not execute
native GPUI, establish native component acceptance, qualify another host, or
establish complete WeUI/WCAG parity. The browser uses the locally vendored WeUI
2.6.26 templates/CSS, with browser adapters in `website/playground/bridge.js` and
`bridge.css`. Their tested bodies are archived here as `.txt`; `provenance.json`
records their hashes and the canonical parent at the final local check.

The adapter executes no upstream inline example scripts, WeChat SDK, upload
transport or analytics. Upstream sources/licenses and package digests are recorded
in `website/playground/vendor/PROVENANCE.json`. The full website/browser validator
has its separate [receipt](../../web-interactive-20261025/).

## Implemented interaction boundaries

- Dialogs, Android/iOS action sheets, half-screen variants and image viewers:
  iframe-local background inertness, Tab/Shift+Tab containment, Escape dismissal,
  visible close controls and actual opener restoration. The fifth half-screen
  variant also supports its downward handle gesture and keyboard dismissal.
- Pickers: pinned WeUI wheel structure/CSS, pointer/touch/wheel and keyboard input,
  disabled-item skipping, staged confirm/cancel, independent and cascading
  columns, and calendar day clamping. Browser date examples use the browser's
  current local date; the native DateField UTC policy is untouched.
- Forms: native editable/readonly/disabled controls, textarea counter, card/amount
  validation, visible-consent validation, terms feedback without toggling consent,
  Clear buttons that survive pointer/focus transitions, and guarded local Vcode
  countdown. Hidden legacy footer controls do not determine visible validation.
- Search activation, submit, clear/cancel, keyboard suggestions, tabs with roving
  focus, sliders with pointer/keyboard input, swiped cell disclosure/deletion,
  information-bar dismissal/restoration, and local callback feedback for example
  links/buttons/panel actions.
- Files: actual browser file inputs create object-URL previews. Local preview and
  deletion work by pointer/keyboard, restore useful focus, and revoke object URLs
  on deletion/unmount. The progress route is a bounded local simulation with
  cancellation. Files are not uploaded.

The route shell validates route/state/theme/care/brand inputs and same-origin,
actual-parent messages. `mount`, `applyState` and `cleanup` are the bounded bridge
API; the adapter does not evaluate historical state-replay JavaScript strings.

## Actual checks and visual review

`smoke.py` mounts all 60 pinned routes and their 72 named states in actual
Chromium: **132/132 initialization cases passed**, without page exceptions. This
checks initialization/state application, not every possible interaction.

`flows.py` records six actual pointer/keyboard checks: dialog focus containment and
button dismissal/restoration; picker disabled skipping/date cancellation; button
disabled/loading guards and Space callback; real file/object-URL keyboard preview
and deletion/focus; real pointer swipe/reveal/delete; and per-bar dismissal/restore.
All six passed. These direct bridge checks supplement the independent integrated
website validator. They are not screen-reader or cross-browser acceptance.

The actual Footer `WeUI首页` link also passed navigation to Home after an exact
legacy `javascript:home();` adapter was added. An initial ad-hoc check searched
for that link on Article rather than Footer and timed out before any activation;
`footer-home-result.json` retains that distinction.

`capture_review.py` serves the built website under a GitHub project-style
`/weGPUI/` prefix. Each capture batch contains 32 screenshots: eight selected
route/states in light, dark, care-light and custom-purple-light. The batches
reported zero page exceptions and zero attempted outside-origin requests.
Per-batch `metadata.json` records the exact served playground file hashes,
Chromium version, UTC and focused elements. The viewport is 375×812 at DPR1.

All four `sheet-{light,dark,care,brand}.jpg` contact sheets in **each** batch were
visually inspected. Each sheet contains, in order:

1. `button_default-default`
2. `dialog-ios1`
3. `half-screen-dialog-4`
4. `picker-single`
5. `picker-date`
6. `picker-cascade`
7. `searchbar_filled-typed`
8. `uploader-viewer`

The first batch exposed normal dialogs being painted below their mask, misplaced
picker cancellation, and fades covering the selected picker text. Those captures
remain in `captures-first/`. The corrected batch shows dialogs above the mask,
reachable cancellation, readable selected rows and the current date rather than
day1. The complete raw `captures-corrected/light-picker-date.png` was also reviewed
for the header, selected day, all three columns and bottom confirmation. No
additional clipping/stacking defect was established in the reviewed corrected
captures. This is selected visual regression review, not full theme/state parity.

The adapters retain WeUI's 240px wheel and indicator at112px; the lower gradient
is shortened to72px so its full56px selected row remains uncovered. Picker header
side/main wrappers preserve the pinned half-screen positioning rules. These are
explicit browser compatibility adaptations, not unmodified upstream rendering.
The corrected captures precede only the subsequent Footer Home routing fix;
their tested bridge bodies are retained inside `captures-corrected/`. The final
132-case smoke and actual Footer navigation check include that routing change.

## Preserved failures and limits

`flow-results-before-route-class.json` and its exact harness preserve a failed
direct information-bar test. That harness omitted the route CSS class, so all
bars used their base fixed positioning and overlapped. The real router already
sets the route class. The corrected direct harness passed the scoped close test;
this failure does not establish a remaining product defect.

Independent development failures remain in the integrated validator receipt,
including genuine Clear/picker/consent findings and subsequently corrected
harness assumptions. Care mode changes pinned palette variables; expecting the
17px button font to enlarge automatically was a test assumption, not source-backed
care behavior.

The first capture batch has exact served-source digests. Only its CSS and HTML
bodies were recovered from retained browser traces (`first-tested-source/`);
`first-tested-source-recovery.json` explicitly lists unrecovered old JavaScript
bodies. The final bridge bodies and final capture-source hashes are retained.

No browser screen reader, Safari/Firefox, real touch hardware, deployed GitHub
Pages host, real upload endpoint, camera service or every possible component
combination is qualified here. The production web page describes its controls as
browser-rendered WeUI and keeps native Rust evidence distinct.
