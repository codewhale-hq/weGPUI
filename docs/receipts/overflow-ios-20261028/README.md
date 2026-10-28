# Form wrapping and iOS keypad continuation — 28 October 2026

The changed source is identified by [source hashes](source-sha256.json), recorded from
parent `31c0954099f301739bf036b670322be69607bcf8` plus this continuation. The browser
reference is pinned WeUI 2.6.26; the Linux ARM64 comparison uses X11, software rendering
and Chromium with the same controlled Lao font described in the earlier
[dictionary receipt](../dictionary-mobile-20261026/README.md).

GPUI retains emergency wrapping by default. The WeUI form description opts into a normal
overflow policy, which keeps an unbreakable span together until a legal line break. Policy
and line clamp participate in the shaped-layout cache key. The same batch corrects cached
unconstrained measurements and selection alignment against the element's available width.

The [controlled comparison](comparison.json) matches 15 of 16 row sequences across Thai,
Lao, Khmer and Myanmar at four widths. The previously different Myanmar 160 px case now
matches by overflowing its unbreakable span; Khmer at 225 px remains different because
ICU4X and Chromium choose different dictionary breaks. The [probe](probe.rs),
[driver](driver.py), and [binary hash](binary-sha256.txt) identify the observation. This
is not a complete visual or language parity verdict. A separate [Linux geometry
probe](geometry/result.json) passes 12 LTR/RTL, alignment and wrap-policy cases for
selection, caret and hit-test consistency against shaped rows. This is not independent
browser or screen-reader validation.

The candidate builds the macOS gallery, passes 54 library tests, all-target Clippy
and strict documentation; the iOS simulator
bundle builds after correcting an Objective-C return signature. All four existing extracted
text contracts pass. The existing Python suites pass 33 script and five website tests; a verified preview
archive builds as a separate Linux consumer with its workspace-root vendor patches.
The updated local website passes [selected shell and theme checks](website/summary.json)
on both root and project-prefix mounts. The separate live site at `c343621` passes the
same selected interactions after a
frame-wait race in the checker was corrected. Current hosted CI and Windows checks were
not run for this continuation; final release validation is deferred by workspace policy.

The iOS backend is now vendored with its Apache license and provenance. A single-line digit
input requests a number pad with a Done accessory; number and telephone fields retain the
default keyboard to preserve their accepted character sets. The simulator bundle compiles,
but the accessory, Done, re-tap, focus switching, rotation and privacy have not been
verified at runtime. The simulator control layer returned `windowNotFoundAtPosition` in
this session. Password input remains deliberately fail-closed; no secure native password
entry is claimed. Physical-device, accessibility, IME and platform acceptance remain open.
