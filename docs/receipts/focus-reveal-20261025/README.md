# Focus-target registrations — Linux X11, 2026-10-25

Source: GitHub main `52650f3` plus the exact eight-file [source patch](source.patch).
[File hashes](source-sha256.txt) include the unchanged scrolling/input code and Cargo lockfile.
Rust 1.98.1 / Cargo 1.98.1, gpui-pre 0.3.9, Debian 13.5 x86_64, Xvfb :93 with Openbox
3.6.1, `GPUI_X11_SCALE_FACTOR=1`, 480×420 client windows. See [environment](environment.txt)
and [X server package versions](xserver-version.txt). The `Xvfb -version` probe in the environment
log was unsupported; package versions were recorded separately.

This is keyboard/focus and accessibility-tree evidence. No Orca or other screen reader was run
for this receipt. It does not establish cross-platform, theme/state, or complete component acceptance.

## Source change

Register individual Navbar/Tabbar tabs and the ClearableInput clear button with `RevealFocus`.
Register the gallery's raw article link, home category/layers controls, and form footer, help,
agreement and verification-code tip links. The article link also gains the standard 2px brand
focus ring; it previously had no visible keyboard-focus style.

`form_link` and `navbar::tab_panel` keep their `Stateful<Div>` return types. Their rustdoc now
explains how callers wrap the styled element/scroller. The gallery wraps its form-link callsites.

## Reusable-component probe

[Probe source](probe.rs.txt) places Navbar, Tabbar and ClearableInput between 600px spacers in
one `FocusScroll`, without a FormCell wrapper that could conceal a missing leaf registration.
The source was temporarily copied to `examples/focus_reveal_probe.rs`, compiled, then removed.
The corrected build passed ([log](probe-build-final.log)).

Run from a checkout with the receipt source copied into that transient example:

```sh
cargo build --example focus_reveal_probe --locked --offline
dbus-run-session -- bash docs/receipts/focus-reveal-20261025/run-x11.sh
```

The [driver](run-x11.sh) sends actual XTEST clicks/keys/wheel events, captures native windows,
and checks the [runtime offset log](probe.log). [Driver output](probe-driver.log) ends in PASS.
All 18 final captures were reviewed in the [contact sheet](probe-contact.png), with full-size
Navbar, Tabbar, clear and endpoint captures also inspected.

Observed forward focus reveals: Navbar `-324px`, Tabbar `-984px`, input `-1626px`, clear button
`-2005px`, End `-2306px`. Reverse focus returns through input `-2004px`, Tabbar `-1328px`,
Navbar `-672px` and Start `-8px`. Navbar/Tabbar Right selects the second item with a visible
ring and no additional scroll. A delayed redraw leaves visible focus at the same offset.

Wheel scrolling while focus remains on Navbar changes the offset to `-714px`; it stays there
in the delayed sample. The same input sequence preserves `-2016px`. These intentional manual
scrolls can move the focused control partly offscreen. The next Tab onto the clear button then
moves only 11px, exposing its own ring at `-2005px` rather than revealing a larger form row.

## Gallery checks

[Gallery driver](run-gallery.sh) runs article, home, form_checkbox, form_vcode and form_page at
480×420, forwards then backwards through the recorded Tab sequences. [AT-SPI focus logs](gallery-validation.log)
validate all 68 sampled leaf bounds inside that client area. Individual logs retain the actual
names, roles and screen bounds. Some sequences wrap because the route has fewer stops than
steps; these are retained explicitly.

The [review sheet](gallery-review.png) contains the twelve visually inspected gallery frames:
article forward/backward link, home lower categories/layers and return to the first category,
checkbox agreement/submit, vcode help/tip, and form footer forward/backward. Their rings and
text are visible. These checks cover the focus changes; they are not upstream visual-parity
comparisons. Vcode's clear button starts hidden here because its field is empty; the reusable
probe covers that leaf separately.

```sh
dbus-run-session -- bash docs/receipts/focus-reveal-20261025/run-gallery.sh
```

## Failed attempts retained

- [Initial build](probe-build-initial.log): the temporary probe used a borrowed-index listener
  where the component API requires an owned index, and an unsupported `WindowOptions.title`
  field. Corrected in the preserved final source; no production-component build defect.
- [First input run](failed-initial-focus/): click x=80 missed the Start button; no focus was
  established, no diagnostic samples were logged, and Tab did not exercise the probe. No pass.
- [Second input run](failed-probe-tab-stops/): custom Start/End handles omitted `tab_stop(true)`.
  Intermediate component reveals worked, but endpoint/backward labels were wrong. The final
  probe sets those handles correctly and passes its full runtime assertions.

Final source gates are recorded by the integrating agent separately; the temporary probe and
these sampled runtime observations do not substitute for the five required Cargo gates.
