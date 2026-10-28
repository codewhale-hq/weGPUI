# Fractional slider value — 28 October 2026

The source is `ba6ec829e8c4b92c8f24241979df6990ff115f42` plus the slider change
identified in [identity.json](identity.json). The earlier [data-contract receipt](../data-contracts-20261025/README.md)
observed numeric value `0.125` while the status label showed `0.1`. The formatter now
uses the shortest round-trippable `f32` representation and normalizes negative zero;
visible and accessible value strings use the same formatter.

The existing six slider unit tests pass, including extended cases for fractional, negative,
zero and large values. A Linux ARM64 X11/Mesa fixture and AT-SPI readback show numeric
and status values agreeing at `0.125`, `0.375` after SetValue, and `0.5` after Right.
[Initial](initial.json), [SetValue](set.json), [keyboard](key.json), [runtime log](runtime.log),
[fixture source](probe.rs) and [screenshot](slider.png) are retained. This is a sampled
native and AT-SPI check, not a complete screen-reader, hardware or cross-platform pass.

The updated local site passes selected [shell and theme checks](website/summary.json)
at both mount paths. An initial attempt pointed the container at a non-mounted output
directory and returned 404s; correcting the fixture path produced this pass.
