# Component contract continuation — 2026-10-25

This is the second Linux cloud batch after `9c4515c`, rebased by ordinary fast-forward onto
the independently verified vertical-label fix `a3ce3af`. No unavailable dirty Mac work was
used or qualified. GitHub main is canonical; no CNB writes occurred.

## Evidence by family

| Receipt | Scope |
| --- | --- |
| [Controls](../control-contracts-20261025/README.md) | Button/IconButton, Vcode, Agree/choices, interrupted Switch motion, eight theme/care/brand combinations, static verified-CSS samples. |
| [Input](../input-contracts-20261025/README.md) | Native/AT selection, Unicode word/line geometry and long chunks, IME/undo, masking, scale/move and actual Orca readback. |
| [Forms](../form-contracts-20261025/README.md) | Typed DateField, readonly/disabled/range/error, event counts, owner updates, pending sheets and focus transitions. |
| [Overlays](../overlay-contracts-20261025/README.md) | Closing-frame guards, modal callbacks, long Select, full-i32 lazy date range, focus locations and short-sheet drag/viewport behavior. |
| [Data/content](../data-contracts-20261025/README.md) | Progress, notice/footer positioning and occlusion, modal hiding, navigation/slider callbacks and article semantics/layout. |
| [Media](../media-contracts-20261025/README.md) | Upload capacity/statuses, viewer keyboard/delete target and controlled swipe behavior. |
| [Search](../search-contracts-20261025/README.md) | Homepage focus, keyword semantics, readonly clear guards and clear/cancel/submit/back callback counts. |
| [Focus contrast](../focus-contrast-20261025/README.md) | Shared background-aware colours and dual bands, with exact captured artifact and theme/pixel results. |
| [Orca cache](../linux-bridge-20261025/README.md) | Actual reader close/reopen/restoration after approved signal encoding repair, with old-artifact control. |
| [Bridge/Wayland](../wayland-20261025/bridge-followup/README.md) | Actual Wayland state/nested/text regressions; X11 invalid/busy/read-only/disabled/heading and absolute-coordinate proof. |

Each receipt names its own exact artifact and source. Earlier runtime captures do not
automatically validate later edits. Accessibility trees, actual reader transcripts, PCM and
visual reviews are distinct evidence. The original failures remain in their directories.

## Dependency scope and consumer check

The owner explicitly approved two sets of compatibility changes, now applied only to local
copies of the three existing locked packages: AccessKit Unix 0.22.1, AT-SPI Common 0.19.1 and
GPUI 0.3.9. Exact source checksums, deltas, licenses and proofs are linked from
[vendor guidance](../../../vendor/README.md). No registry file, package version or unrelated
dependency edge changed. Further dependency changes and hosted/published forks are not authorized.

A fresh consumer outside the repository compiled the complete minimal app with explicit
workspace-root patches. The initial attempt omitted the example's support module and failed;
`components-batch-2-consumer.log` preserves that harness failure. After copying the existing
support module, the locked offline build passed. `components-batch-2-consumer-retry.log`
also shows all three packages resolve to the intended local copies, including the platform
backend's transitive dependencies. `consumer/` retains the exact manifest, lock and source.
This is a local path consumer build; it is not a published-package, Git-installation,
fresh-machine runtime, signing or distribution pass. Dependency-manifest patches are not inherited.
The separate `readme-consumer/` also compiles the exact first Rust block from the current
README, with source/binary hashes and lock retained; this is a build-only check.
The [actual consumer runtime](consumer/README.md) additionally passes dialog keyboard trapping,
callback counts 0→1→2, repeated opening and focus restoration on its identified pre-contrast
X11 executable. It does not qualify every later source change.

`dependency-verification.json` compares all 654 lock packages against `a3ce3af`: only the
source/checksum entries of the three approved packages change; versions and dependency edges
are identical. `vendor-delta-verification.json` confirms only the three approved upstream
source files differ, with no unexpected missing files. Package listing passes
(`package-list.log`): root licenses and NOTICE are included, but Cargo automatically excludes
the nested vendored packages. Only `vendor/README.md` is in that list. A published crate would
therefore still need a resolved downstream patch distribution policy; this check is not a
package verification build or permission to publish.

## Website verification

`website/` retains local build/browser results for the updated setup and platform facts,
workspace-root patch guidance and distributed license/NOTICE files. Chromium checked 60
routes, 264 state/theme selections, 528 images, 39 API links, 18 viewport routes and four
palettes without page errors. The three patch paths and four served license files were
also checked. Ten existing Python fixtures and the recorded syntax checks passed.
Site images remain explicitly historical captures; these checks do not refresh native
parity. The initial browser run used pre-final API docs. `website/final-api/` records a
full browser rerun with the later API, and `website/post-focus-api/` refreshes from the final
strict rustdoc after the checkbox/caret corrections. Served source pages exactly match the
current Checkbox and TextInput files. No public site was deployed.

## Gate qualification

All five gates passed on the final Rust source with **Rust/Cargo 1.98.1**, GPUI 0.3.9 and
locked offline dependencies. [gates.log](gates.log) is the complete run after all temporary
examples were archived and removed:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo build --all-targets --locked --offline` | PASS |
| `cargo test --locked --offline` | PASS: 47 unit tests, one doctest ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | PASS |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline` | PASS |

Intermediate build logs are retained here. The strict preflight found two formatting lints
in a temporary forms probe; that failed run is preserved and is not a passing gate. Build 7
included the later contrast and date-announcement fixes; its temporary probes are retained
as receipt source, not shipped examples. Root's last Gallery change resolves the delete ring
against its actual fill even when a host changes `Palette.white`; default-theme pixels are
unchanged from the focus fixture's earlier identified artifact.

Full-image review then found that an outer shadow filled the transparent centre of an
unchecked checkbox in dark themes. Edge-pixel assertions alone had missed that defect.
The corrected tracked overlay preserves the centre and passes eight-mode keyboard/pointer/AT
callbacks and focused/unfocused interior checks. The caret was also changed to black/white
bands for inherited surfaces: brand plus black could both fail on dark grey. The earlier
source/artifact manifests and gate run remain under `before-checkbox-focus-correction-*`
and `components-batch-2-gates-final.log`; **the final gate is the later
`components-batch-2-gates-after-focus-review.log` copied to `gates.log`**.
Current `source-sha256.json`, `source.patch` and `binary-sha256.json` identify that later source.
The final native contrast receipt passes sixteen Outlined/FilledGrey caret surfaces and
twenty-four FilledGrey action indicators, all visually reviewed. It retains the earlier
128 passing edge measurements alongside the visual failure those measurements missed.

## Remaining acceptance

No component row is fully accepted. Current-source macOS/Windows, human audio listening,
the complete screen-reader/state/theme matrix, physical display/live scale/hardware GPU and
fresh installation remain open. VoiceOver stays off. The Linux bridge lacks EditableText;
native replacement of an AT-selected range is not AT text replacement. Paired full-shell
reference parity remains blocked by the configured SDK-host network policy; static verified
CSS samples and native screenshots do not replace it. UTC today, package/version, website,
signing and release decisions remain with the owner. No issue/post, CI dispatch, tag,
release, deployment or publication occurred.

The [EditableText feasibility note](../../proposals/linux-editable-text/README.md) explains
why a partial wire interface is unsafe. Prepared saved custom-action/closing and selected-text
replacement followups are explicitly unbuilt/unapplied; they establish neither a defect
nor a runtime pass for those additional paths.
