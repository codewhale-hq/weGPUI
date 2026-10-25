# Modal accessibility isolation and draw order — Linux X11, 2026-10-25

## Result and final source

The final depth-ranked implementation passes the sampled two-level isolation/focus checks,
passive-toast checks and a three-layer composition that previously exposed the wrong modal.
This is keyboard/visual and accessibility-tree evidence, **not screen-reader acceptance**.
Actual Orca and Wayland followups are recorded separately.

Source is GitHub main `52650f3` plus the integrated continuation changes. The exact two-file
[final modal patch](source.patch), [production source/binary hashes](source-sha256.txt) and
[three-layer probe source/binary hashes](order-source-sha256.txt) identify these checks.
Rust 1.98.1, gpui-pre 0.3.9, Debian 13.5 x86_64, Xvfb :93 with Openbox 3.6.1,
`GPUI_X11_SCALE_FACTOR=1`. Package details match the [focus registration environment](../focus-reveal-20261025/environment.txt).
The integrating agent records the full Cargo gates separately. Both focused accessibility
contracts passed ([log](order-probe-after-build.log)); the integrated suite subsequently had
24 passing tests.

## Defects and ordering contract

The original implementation hid only the application's `ModalScope` group. Deferred outer
and inner modal roots both remained exposed. The unchanged [Wayland before tree](../wayland-20261025/nested-modals/inner-atspi.txt)
shows `Outer sheet` and its controls alongside `Inner dialog`.

The first fix tracked the last active modal in layout order. It passed the ordinary nested
example but failed this composition: root children `[outer A containing inner B, sibling C]`,
all open. Layout registers A→B→C; GPUI's deferred rounds paint A,C and then B. The
[last-layout before tree](order-before/all-open-atspi.txt) exposed C's content while B's title/body/buttons
were hidden, despite [B visibly painting on top](order-before/all-open.png). AccessKit retained
B's focused root without its content. This failed [runtime verdict](order-before-validation.log)
is preserved.

The final implementation ranks active layers by **modal nesting depth, then layout order**.
A per-window layout-depth guard counts every crate `Modal` deferred layer, including passive or
closing parents, and restores depth on scope exit. `Semantics` reads each shared hidden flag
when GPUI writes its accessibility node at prepaint, after registrations from layout are known.
The deepest active modal wins; among peers at that depth the last registration wins, matching
these equal-priority GPUI deferred rounds. Covered layers remain painted. Closing nonpassive
layers become hidden immediately; passive toasts do not hide the page or displace an active modal.

Applications still need one root `ModalScope` covering all modal elements. Arbitrary
application-supplied `deferred` wrappers or different overlay priorities are **outside this
ordering contract**. This receipt does not qualify those compositions, macOS/Windows, full
state/theme coverage, or actual screen-reader operation.

## Final three-layer runtime

[Probe source](order-probe.rs.txt) was temporarily copied to `examples/modal_order_probe.rs`,
compiled and then removed. [Driver](run-order.sh) and [validator](validate-order.py) preserve
both the failed [before](order-before/) and passing [after](order-after/) captures.
[Final validation](order-after-validation.log) passes:

- While A, B and C are open, only B's dialog, title, body and buttons are exposed.
- Tab reaches Cancel B then OK B, with visible rings.
- Escape closes B and exposes/focuses C; Tab reaches Close C.
- Further Escape presses expose A, then the page.

The before/after visuals retain B on top; the accessibility exposure now agrees with the
painted order. Both probe unit-contract tests and actual runtime observations are required
here: the earlier two-level run alone missed the sibling composition.

```sh
# Copy the receipt probe to examples/modal_order_probe.rs before building.
cargo build --example modal_order_probe --locked --offline
OUT=/tmp/modal-order dbus-run-session -- bash docs/receipts/nested-modal-a11y-20261025/run-order.sh
python3 -I docs/receipts/nested-modal-a11y-20261025/validate-order.py /tmp/modal-order
```

## Final two-level and passive-toast rerun

The final-source rerun is in [depth-final](depth-final/); [driver output](depth-final/driver.log)
ends in PASS. [Driver](run-x11.sh) sends real XTEST input and records native-window screenshots
and AT-SPI trees. The [validator](validate.py) checks:

- Outer open hides page controls; inner open hides outer and page controls.
- Inner Tab/Shift+Tab wrap between Cancel and OK.
- Escape restores `Open inner dialog`, then a second Escape restores `Open sheet`.
- A passive success toast retains page controls and a notification; Tab reaches the first
  trigger without creating a modal dialog.

The original linear-policy two-level evidence remains at this receipt's root with its
[original patch](linear-policy-source.patch) and [hashes](linear-policy-source-sha256.txt).
It is historical evidence of that narrower case, superseded by the final rerun.

```sh
cargo build --all-targets --locked --offline
OUT=/tmp/nested-final dbus-run-session -- bash docs/receipts/nested-modal-a11y-20261025/run-x11.sh
```

## Failed harness/build attempts

[failed-window-coordinates](failed-window-coordinates/) preserves the first input attempt. It
used window-relative coordinates as root-screen coordinates, so its click missed the centred
window, no modal opened, and validation failed. The final helper requests window coordinates
and sends real `mousemove --window` clicks. This was a harness failure, not a component verdict.

The temporary three-layer probe's [first build](order-probe-build-initial.log) supplied click-event
closures to DialogButton's two-argument callback API. Corrected probe source then built; no
production API or dependency change was needed.
