# Linux control contracts — 2026-10-25

This continuation implements reusable control fixes and drives callbacks with real XTEST
pointer/Enter/Space input. It is not gallery-route coverage, upstream-JS parity, or full
cross-platform/screen-reader acceptance.

## Source and fixes

The baseline is canonical GitHub `9c4515c788bbacc42a166534e39dc393ddbcb30b`.
`baseline-control_contracts.rs.txt` is the exact initial fixture compiled against an
immutable copy of that source; its binary SHA-256 is
`f4e2d276595965a861cb7c1d9eb98bb9490b9f806dd1fc9493141857e9941736`.

The first control candidate adds the following fixes:

- Standard button labels and form-group verification-code labels use upstream weight 500,
  including text measurement for the two-line button clamp. Previously they used 600.
- Default loading-button spinners stay `#606060` on overlay backgrounds too, as the pinned
  `.weui-btn_loading.weui-btn_default` rule requires.
- A checked standalone checkbox has a contrasting 2px outer focus ring. Its earlier tinted
  background was hidden beneath its opaque checked glyph; compare the baseline/current
  `choice-standalone-checkbox.png` captures.
- Disabled icon buttons do not show the enabled `btn_icon` pressed recolouring.
- Long consent text wraps within its available width, and the terms link registers its own
  focus-reveal bounds. Compare `baseline-agree/initial.png` with
  `candidate-agree/agree-long-link-revealed.png`.

The exact first candidate is `controls-source.diff` plus
`candidate1-control_contracts.rs.txt`, applied to the baseline. The complete source/assets
hash manifest is `candidate-source-sha256.json`; binary SHA-256 is
`90d9cb7af7a774d757098e6392469cd53135f080738062b40766663d5a7c30ec`.
It ran from `/workspace/.wegpui-cloud/controls-candidate-9c/control_contracts`.

Subsequent source fixes discovered while driving that candidate:

- Keyboard activation of a consent link could leave its same-event propagation guard set.
  If its callback did not redraw, the following checkbox click was swallowed. The guard now
  clears at the end of the event cycle. Long link labels are also constrained to their
  container, and the warning shake uses upstream's CSS `ease` for each keyframe interval.
- Rapid switch reversal jumped to the previous endpoint. The motion now samples its current
  knob/track values, applies CSS reversing-shortening factors independently to the 350ms knob
  and 100ms track transitions, and uses the track's default CSS `ease`. Reduced motion settles
  both channels without restarting them when motion is enabled again. Four unit regressions
  cover continuity, shortening, completed-channel timing and reduced-motion behaviour.

## First candidate matrix

`matrix-console.log` preserves all 64 invocations: four button sizes and the cells/icons,
choices, consent and verification-code pages in each of these modes:

| Mode | Appearance | Care | Brand override |
| --- | --- | --- | --- |
| light | light | off | none |
| dark | dark | off | none |
| care | light | on | none |
| dark-care | dark | on | none |
| brand | light | off | 7C3AED |
| dark-brand | dark | off | 7C3AED |
| care-brand | light | on | 7C3AED |
| dark-care-brand | dark | on | 7C3AED |

Each button-size invocation contains 18 controls: default/primary/warn × ordinary/overlay ×
enabled/disabled/loading. Each enabled control receives one pointer activation, one Enter
activation and one Space activation, with exactly three callback receipts. Disabled/loading
controls receive pointer attempts and must emit none; Tab must visit only enabled controls.
The 32 size invocations cover 576 rendered button-state combinations. Measured heights are
48/40/32/28 logical pixels (`button-dimensions.json`).

The cells/icons page exercises all three cell kinds with enabled/disabled/loading guards,
and ordinary/`btn_icon` icon buttons with disabled guards. Choice checks cover controlled
checkbox/row/switch updates, disabled guards, nested checkbox propagation, radio wrapping,
skipped disabled options and independent group state. Consent checks include enabled/warn
and disabled-parent terms links. Verification-code checks cover both styles, repeated-start
and activation guards, reset, expiry and restart; the extended driver also checks that an
old canceled timer cannot decrement a newly started timer prematurely.

This first candidate finished with **56 successful script exits and eight failed exits**.
Failures are retained:

- Six consent runs failed the newly added enabled-link keyboard sequence. The first plain
  pointer-link baseline passed, but the extended baseline keyboard sequence reproduced the
  bug against unmodified `9c4515c`: after link pointer/Enter/Space, the first checkbox click
  emitted zero callbacks and the second emitted one (`baseline-agree-keyboard`).
- `care-buttons-medium` completed its callback/focus assertions successfully, then the shell
  failed because the harness file was edited while Bash was reading it. The resulting
  `in/python3` error is retained. It is a harness failure, not a component verdict.
- `dark-brand-choices` did not launch: the preceding Xvfb had not fully shut down. The failed
  display/application logs are retained. `run-v2.sh` waits for child shutdown and X readiness.

`failed-harness-desktop-coordinates` is an earlier failed pointer-driver attempt: this GPUI
AT-SPI backend returned window-local bounds even for the desktop-coordinate request. Real
XTEST clicks now use the client-window origin explicitly. Later invocations preserve copies
of their exact driver/launcher in `drive-used.py` and `run-used.sh`. Per-run live-worktree
hashes are diagnostic context; the immutable source manifest and binary hash above establish
what the first candidate actually ran.

## Switch baseline

`baseline-switch-motion` records 90 cropped X11 frames at 60fps. Two real Enter activations
were sent about 96ms apart. The old switch produced a 12px adjacent-frame knob-center jump
and reached the full-on endpoint before moving back off. `motion-summary.json` preserves
all measured centers; both callbacks and final off state were confirmed. The frame sampler
only measures this sequence, not every timing condition or physical display/compositor.

## Integrated source reruns

The later integrated source was built with all targets and 44 unit tests passing by the root
agent. The immutable control binary used for these reruns has SHA-256
`d9a48b7b46874c025ffc9e8bc0dfb3054f9839456fa764ca416c4e516b22b5af`.
Its source context is `integrated-source.diff`, `integrated-source-sha256.json` and
`integrated-control_contracts.rs.txt`; the binary was copied to
`/workspace/.wegpui-cloud/controls-integrated-20261025-0839/control_contracts` before other
builds could replace it.

All **14** invocations in `final-suite-console.log` passed, plus the separate critical
`final-agree-keyboard` regression:

- Enabled/warn consent and all terms-link pointer/Enter/Space callbacks passed in all eight
  theme combinations. Each enabled checkbox emitted exactly three changes, each terms link
  exactly three callbacks; the disabled checkbox emitted none. Long consent focus reveal
  passed. The focused keyboard regression now reports checkbox click counts one then two,
  compared with zero then one on the original source.
- The extra long terms-link label wrapped within 260px and activated exactly three times
  without toggling its checkbox, in both light and dark+care+custom-brand modes.
- The two earlier harness failures were rerun successfully: care medium buttons and
  dark+brand choices. The earlier failed evidence is retained.
- The extended light verification-code run passed both styles, repeated starts, activation
  guards, Tab skipping, reset/restart, expiry and the canceled-old-timer timing check.
- Current switch motion produced 90 frames with two callbacks and final off state. For this
  rapid-reversal sequence, the largest sampled adjacent-frame movement was 5px rather than
  the baseline's 12px, and peak travel was 13px rather than the erroneous full 20px endpoint.
  The four unit contracts establish mathematical continuity and CSS timing factors; this
  one capture does not establish every OS/compositor/timing condition.

`switch-motion-comparison.png` and `.svg` plot the preserved samples with Matplotlib 3.10.8;
`switch-motion-frames.png` shows frames around each run's largest sampled step. The plot
aligns the first visible motion, not absolute input latency. Original frame PNGs, action
monotonic times, tree samples, screenshots, callback logs and exact drivers remain under
`baseline-switch-motion/` and `final/switch-motion/`.

## Static CSS reference

`static-css/` contains a separate, explicitly labelled static HTML fixture using the verified
WeUI 2.6.26 `dist/style/weui.css` (SHA-256
`8db42592f96d571568c49ae1b8484f55042e6a6dab25f8144460cd4a23530fd3`). It was rendered by
Chromium 156.0.8078.47 without the SDK or upstream example JavaScript. Its computed styles
confirm weight 500, heights 48/40/32/28, and `rgb(96,96,96)` default-overlay loading spinners.
These are static CSS references, not a new pass of the normal upstream JS shell.

Initial fixture attempts remain under `initial-button-elements` (used HTML button elements
rather than the example's anchors) and `initial-disabled-attribute` (added `[disabled]` to
overlay anchors). The pinned overlay demo uses only `.weui-btn_disabled`: additionally
setting `[disabled]` triggers a later, equally specific generic rule and changes its palette.
The corrected fixture follows the pinned demo's markup.

Custom-brand checks are a native theming extension: the pinned checked-checkbox/consent SVGs
embed fixed green, while the reusable native glyph uses `Theme.brand`. The reference-page
SDK fetch remains blocked; this fixture does not remove that recorded full-shell limitation.

## Visual review and colour samples

All 63 available first-candidate initial captures were reviewed in the sixteen files under
`review-sheets/` (the failed dark-brand choices launch has an empty slot). Button sizing,
ordinary/overlay palettes, warning colours and disabled/loading distinctions were visible.
Full-size checkbox/consent cases were also inspected. The disabled `btn_icon` crop was pixel
identical while held/released in seven modes (`disabled-icon-pixels.json`).

`compare-colors.py` compared 162 selected native button interior pixels with the browser
fixture's computed background colours composited over the fixture cards. Every sample was
within 1/255 per RGB channel (`static-color-checks.json`). This checks those background
samples; it is not a whole-image parity or accessibility-contrast result.

## Limits

All runtime receipts here are Linux x86_64, Debian 13.5, Rust 1.98.1, gpui-pre 0.3.9, Xvfb plus
openbox, XTEST and software Vulkan, at interaction scale 1. Images show a 1100×900 logical
window. Native and static-browser fixture layouts/fonts are different contexts; computed CSS
values and selected components were compared, not whole-page pixel equality. No macOS,
Windows, mobile, real screen-reader, touch, live monitor/scaling or release acceptance follows
from this control fixture. No ACCEPTANCE row is fully accepted by these receipts.
