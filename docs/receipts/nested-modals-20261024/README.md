# Nested modals — Linux X11 (Xvfb + openbox), 2026-10-24

`examples/nested_modals.rs`: a half-screen dialog whose content renders a centred `Dialog`.
Driven by `drive.sh` (xdotool): click "Open sheet", Tab, Enter (inner opens), Tab ×5,
Shift-Tab, Escape, Tab, Escape, Enter.

- `before/` (Tab trap in the capture phase): the outer sheet's trap ran first, so Tab from
  the inner dialog's OK went to "Open inner dialog" and "Close sheet" behind the inner mask
  (05-inner-tab3/4), and the next Escape closed the outer sheet (log `outer-close`) with the
  inner dialog.
- `after/` (trap moved to the bubble phase, innermost layer first): Tab cycles Cancel ↔ OK
  only; Escape closes the inner dialog only (log `inner-close`) and focus returns to
  "Open inner dialog"; Tab stays in the sheet; the second Escape closes the sheet and focus
  returns to "Open sheet"; Enter reopens it.
