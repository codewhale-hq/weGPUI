# Accessible text corrections — Linux, 2026-10-25

The Wayland continuation exposed an empty dialog paragraph (`label ''`) and a Confirmed row
whose visible count had no exported textual value. The copied `before-*-atspi.txt` snapshots
are tree observations from that run, not screen-reader results. See the Wayland receipt for
its source, binary, host and interaction provenance.

Source inspection explains the paragraph: the pinned `accesskit_consumer` treats `Role::Label`
as a name derived from its value; `accesskit_atspi_common 0.19.1` uses that value for the
AT-SPI name. GPUI's `aria_label` only sets the AccessKit label. `Dialog::body`, the
`FormPreview` header and the component-state example's result summary now use `aria_value`
for these static text nodes.

`Cell::value` now retains its textual value and exposes it through the row's accessible
description, followed by `Cell::desc` when present. The title or explicit accessible name
stays unchanged. Replacing the footer with arbitrary `ft(...)` content clears the previous
textual value, leaving that content responsible for its own semantics. Two regression tests
cover explicit-name preservation, value/description ordering, stale-value replacement and
empty text. This does not add a live announcement when a nonfocused row changes.

The component-state example also now wraps the Delivery and Frequency options in separate
named `RadioGroup` containers. Its previous single generic group caused Orca to report
positions across all four options, despite each public builder having two options. The
`RadioGroup` API already documents the caller's container responsibility. This corrects the
example without changing radio selection or keyboard behavior. The Orca receipt records
the speech observations and reruns.

The AT-SPI dumper now prints nonempty descriptions so the exported value can be checked.
Source formatting, Python compilation and `git diff --check` were checked while editing;
the integrated Linux continuation receipts record the final Cargo gates and runtime reruns.

## Editable text readback

The Orca continuation also found that the Display name field visibly contained typed text but
Orca's focused-item summary omitted that text. A field-level AccessKit value alone does not
activate `accesskit_consumer` text ranges: a text-input node must contain `TextRun` children.
GPUI 0.3.9 documents the supported synthetic-child API in its `_accessibility` guide.

`TextInput` now uses that API to export displayed text as stable text-run children, including
UTF-8 character lengths and the current anchor/focus selection. Hard newlines separate runs;
a trailing newline retains an empty final run for its caret. Password fields supply the same
bullets they draw, with one bullet per grapheme; secret text never reaches the synthetic run.
Five focused regression tests cover native grapheme units (including combining marks and
ZWJ emoji), forward/reversed selection, hard-line/trailing-empty positions, oversized-grapheme
fallback and masked exported values. Source formatting and diff checks pass; the integrated
continuation gates and Orca rerun establish compilation and observed readback separately.

This adds readable text and keyboard-driven caret/selection state. It does not supply character
geometry, soft-wrap line segmentation or assistive-technology editing actions. Those limits
remain open for complete input accessibility acceptance. AccessKit stores each character's
UTF-8 length in a byte. Graphemes longer than 255 bytes therefore use a lossless scalar-boundary
fallback with matching caret indices, rather than truncating the length. That rare case cannot
represent the native grapheme as one AccessKit character and remains an explicit limitation.

## Runtime followups

Actual Orca speech now reads back entered text and announces keyboard-driven caret/selection
changes; the separate seeded probe covers combining marks, a family ZWJ emoji, reversed
selection, masked password text, multiline/trailing-newline text and an empty field. See
`../orca-20261025/README.md` for exact artifacts, transcripts and limits; emoji glyphs fell
back to squares on this host, and pronunciation/intelligibility was not reviewed by listening.
The final Wayland receipt independently queries Text/caret/selection after real input.

The seeded probe also exposed missing Ctrl+Home/End bindings: the intended Ctrl+Home action
left the multiline caret at 24. Four bindings now call the already-existing DocStart/DocEnd
and SelectDocStart/SelectDocEnd actions. The failed sample and targeted rerun are retained
in the Orca receipt; document-boundary aliases do not change line-only Home/End.

The temporary probe was built only for runtime evidence; `probe.rs.txt` preserves its source.
It is removed from examples before the final clean-tree gate run. Neither these fixes nor the
reader samples accept the entire input family; assistive editing actions/geometry, bridge
cache compatibility and the full platform/theme/state matrix remain open.
