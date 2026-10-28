# Dictionary text and experimental iOS continuation — 26 October 2026

Implementation and verification receipt. Native qualification is in progress; the named
checks below keep their own source identity. This is not full platform acceptance.

The owner approved ICU4X dictionaries and subsequently authorized dependencies needed to
finish parity. Windows remains build/tests only and VoiceOver remains off.

## Implemented changes

- ICU4X 2.4.0 supplies shared break opportunities and bundled Thai, Lao, Khmer and Myanmar
  dictionaries. Four packages are added; no existing versions change. The interim direct
  unicode-linebreak edge is removed. Unicode-3.0 licensing is included in previews and the site.
- Shaped wrapping now treats repeated glyph indices as one cluster. Reordered combining
  glyphs use logical row shaping, including left-to-right dictionary scripts.
- The existing vendored GPUI core supplies the four API shapes missing from the published
  iOS backend: lifecycle disconnect, focus identity, focused eligibility and selection fragments.
  The library exports document-ordered selection fragments from its actual shaped clusters,
  supplies UTF-16 document length, validates native selection updates and reports edit eligibility.
  Application-side content/selection changes invalidate the focused native session, with no
  repeated notification for unchanged caret-blink frames.
- The iOS examples use fully visible bounds and the gallery supplies touch Back navigation.
  Focus scrollers reveal their target when the visible viewport size changes.

## Runtime comparison and limits

The controlled Linux ARM64 / X11 / software-renderer fixture compares four scripts at 160,
225, 327 and 650 logical pixels with the pinned WeUI 2.6.26 Chromium reference. At the
cluster-fixed dictionary source, 14 of 16 normalized row sequences match. Lao uses Noto Sans
Lao explicitly in this controlled comparison: Chromium's default DejaVu Sans fallback has a
different advance width. The default-font difference is retained separately.

The two residual differences are identified. ICU and Chromium group one Khmer country-name
phrase differently at 225 px. At 160 px Chromium lets an unbreakable Myanmar compound overflow
to about 193 px; GPUI's existing emergency wrapping keeps it inside the available width.
Neither difference is a blanket parity pass. A local ICU auto/LSTM experiment reduced matches
to 10/16 and was rejected; the candidate uses dictionary-only compiled data.

The iPhone 17 / iOS 26.2 simulator links, installs and launches the gallery, and touch
category navigation is observed. The first rendered source exposes status-area overlap;
the later safe-area source corrects the sampled portrait/landscape layout and touch Back
navigation. The native text session, software-keyboard tap, lower-field reveal and actual Pinyin
preedit/commit (`mi` → `米`) are observed. The Done-key check found an inserted space instead
of submission; its subsequent correction requires a fresh runtime check. Slow/black initial
startup observations are retained. There is no mobile accessibility adapter,
physical-device, full IME, rotation or lifecycle acceptance. Android still lacks a backend.

Both website mount suites pass all 42 checks, including 360 route/viewport observations.
The website is a separate HTML implementation; those checks do not qualify native GPUI.

Review found and corrected two native-input gaps: password sessions must fail closed until
secure entry exists, and application-side edits must invalidate UIKit's focused text session.
The final source also distinguishes a single-line native Done key from newline paste. Static
review of the resulting fixes found no further blocking issue; runtime checks remain separate.

Final source hashes, build results and runtime evidence are retained alongside this receipt
as verification completes. Earlier Unicode/website receipts remain evidence for their named
commits. Complete desktop reader/IME/hardware and distribution qualification remains open.

On iOS, password fields deliberately reject native text sessions and expose no native
document text or length. The pinned UIKit backend has no secure-entry trait; visually
masking a plain native responder would expose password text to system UI/accessibility.
Native secure entry is required before iOS password editing can be supported. Desktop
password behavior is unchanged.

## Completed checks and publication

The reviewed source committed as `c343621` completed the macOS and Linux ARM64
format, all-target build, library and gallery tests, Clippy and documentation gates.
The extracted text contracts and a fresh standalone preview consumer build passed.
The [Linux capture packet](linux-captures/README.md) records 112 paired captures
and verification of all 224 image hashes; captures alone are not visual acceptance.
The macOS gallery also accepted mixed Hebrew, combining and emoji input and restored
the empty field with undo.

[Pages run 40651595598](https://github.com/codewhale-hq/weGPUI/actions/runs/40651595598)
published `c343621`; the live site's data identifies that commit and `sourceDirty: false`.
This publication does not qualify native parity. Hosted CI for this slice was skipped
and remains deferred to the final candidate under the workspace release policy.

A subsequent source review identified that tapping an already focused iOS field after
Done may not reopen the keyboard. A subsequent iOS-only gesture change requests the keyboard again for an editable,
non-password field. Its simulator bundle builds; Done, same-field reopening and
password transitions still need a runtime recheck. Simulator coordinate interaction
returned `windowNotFoundAtPosition` during this attempt, so no runtime pass is claimed.
This follow-up is separate from the evidence for `c343621`. Number, digit and telephone fields also currently receive
the default iOS keyboard. Password editing remains intentionally unavailable until secure
traits and privacy of the native responder can be verified.
