# Linux EditableText feasibility — 2026-10-25

A complete, contained `org.a11y.atspi.EditableText` patch is **not viable under the current
locked action/host APIs**. Do not advertise the interface after implementing only
`SetTextContents`. This note records the precise boundary; it does not propose or apply a
partial interface, dependency upgrade, registry edit or additional vendored change.

The existing native keyboard and AT-SPI Text caret/selection paths remain useful and are
verified separately in [input contracts](../../receipts/input-contracts-20261025/README.md).
Their success does not establish EditableText method coverage.

## Existing source and protocol

- Locked `atspi-common 0.13.0/xml/EditableText.xml` and `atspi-proxies 0.13.0/src/editable_text.rs`
  define six methods: SetTextContents, InsertText, CopyText, CutText, DeleteText and PasteText.
- `vendor/accesskit_atspi_common/src/node.rs` exposes Text and numeric Value interfaces;
  its interface selection does not expose EditableText. `PlatformNode::do_action_internal`
  sends one AccessKit ActionRequest and returns without an application result.
- `vendor/accesskit_unix/src/atspi/interfaces/` has no EditableText implementation or
  registration in `bus.rs`.
- Locked AccessKit 0.24.1 has `SetValue(Value)`, `SetTextSelection(TextSelection)` and
  **ReplaceSelectedText(Value)**. It has no Copy, Cut or Paste action, and no single action
  payload carrying both an explicit range and replacement text.
- weGPUI currently handles SetValue and SetTextSelection. A ReplaceSelectedText handler
  would be feasible as a component-only follow-up through the existing API, but would not
  make the full Linux interface safe or complete.

Exact source versions, method locations and hashes are retained in `source-evidence.txt`.
No Cargo command or source mutation was needed for this feasibility check.

## Method coverage boundary

| Wire method | Existing route | Remaining requirement |
| --- | --- | --- |
| SetTextContents(newContents) → bool | One textual SetValue action can replace the entire value. | Check enabled/editable state in the application; retain its filtering, maxlength, callback and undo rules. A queued request has no application acknowledgment. This method alone is insufficient to advertise the interface. |
| InsertText(position, text, length) → bool | SetTextSelection followed by ReplaceSelectedText could express the intent as two actions. | There is no atomic range-plus-text request or acknowledgment linking them. Rejection or intervening input between actions can leave replacement aimed at another selection. Full-value read/modify/write is incorrect for passwords and insertion limits. |
| DeleteText(startPos, endPos) → bool | The same two-action sequence with an empty replacement. | Needs atomic range validation/application, safe stale-request handling, and one undo/callback transaction. |
| CopyText(startPos, endPos) → void | No standard AccessKit clipboard action. | Must reach the host application's clipboard provider and preserve its selection/privacy policy. A successful no-op would violate the method contract. |
| CutText(startPos, endPos) → bool | No standard clipboard action or atomic copy-plus-range-delete request. | Copy and delete must form one application transaction; copy failure must not silently delete text. |
| PasteText(position) → bool | No standard paste action or clipboard provider at this generic bridge boundary. | The application must read its clipboard, validate the insertion range, apply filtering/limits, and create the correct undo/callback transaction. |

CustomAction provides an integer identifier, not a standard clipboard/range-edit protocol.
Assigning private identifiers, recognizing translated labels or encoding ranges in identifiers
would add a new host agreement; it is not an existing generic AccessKit capability.

## Correctness constraints for a future design

**Unicode and graphemes.** The current bridge converts external signed offsets through
`text_position_from_global_usv_index`. weGPUI internally edits UTF-8 byte ranges, moves by
native graphemes, and limits HTML-style UTF-16 units. A future range operation must preserve
these distinct units, validate negative/out-of-range endpoints and the InsertText length
contract, and never split a selectable grapheme. It must also retain the current handling of
soft wraps, trailing empty lines and the documented oversized-grapheme metadata limit.

**Passwords.** The accessibility tree intentionally contains masking bullets. It cannot be
used to reconstruct the original value. For example, inserting `X` into the exposed `•••`
and sending the result as SetValue would replace the real password with bullet characters.
Range edits must execute in the application, which owns the real text. Copy/cut must honor
its password policy; clipboard support must not expose the underlying secret.

**Read-only and disabled state.** Guard in the application as well as in advertised bridge
capabilities, including when an already-held proxy becomes stale after a state change.
Read-only text can permit selection/copy while rejecting mutations; disabled fields reject
mutation/focus requests. A future interface must define this method matrix explicitly.

**Maximum length and filtering.** Native insertion limits only the replacement range while
preserving untouched suffix text. Consider a six-UTF-16-unit field containing `A😀B`. Inserting
`😀界` after A should admit the two-unit emoji and preserve the existing suffix. Constructing
a whole replacement from the tree and passing it through full-value truncation can instead
keep the extra character and discard suffix text. The generic bridge does not own these
application rules.

**Undo, composition and callbacks.** One successful edit should produce one intended undo
transaction and one Changed event; selection/copy alone must not produce a value change.
Pending IME composition and stale requests need an application-owned policy. Separate action
requests do not provide a transaction or a success token: if selection validation rejects a
request, a subsequent unqualified replacement must not edit the user's previous selection.

## Safe next boundary

A future implementation needs an agreed application-side range-edit/clipboard transaction
contract, or equivalent upstream AccessKit support, with method-level capability/guard
semantics. It must be exercised on real X11/Wayland sessions with Unicode, passwords,
read-only/disabled transitions, maxlength, clipboard failures, undo/composition and competing
input. Adding display-specific clipboard ownership to this generic adapter would also require
backend/lifecycle work; using X11 alone would not qualify Wayland.

That is broader than the approved cache/state/window-bounds compatibility corrections.
There is no safe limited patch attached here. No additional approval question or external
issue is needed merely to record this remaining feature gap.
