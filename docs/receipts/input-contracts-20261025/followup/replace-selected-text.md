# ReplaceSelectedText follow-up design — 2026-10-25

This is a **proposed component-only change**, prepared while the integrated batch is frozen.
It has not been applied, compiled or exercised. It needs no dependency, registry or vendored
adapter change, and no public API addition. The accompanying patch is against the input source
whose exact hash appears in `source-evidence.txt`.

## What it would close

AccessKit 0.24.1 defines `Action::ReplaceSelectedText` as deleting the selected text and
inserting `ActionData::Value` in its place, like typing/pasting. TextInput currently registers
SetValue and SetTextSelection but has no handler for that action. TextArea delegates editing
to TextInput, so one handler would cover both components.

The pinned native adapters do **not** currently emit this action:

| Adapter | Existing textual mutation/selection route | Selected-text replacement route |
| --- | --- | --- |
| accesskit_macos 0.26.3 | `setAccessibilityValue:` sends SetValue; `setAccessibilitySelectedTextRange:` sends SetTextSelection. | No `setAccessibilitySelectedText:` implementation and no ReplaceSelectedText occurrence in the adapter source. The selected-text getter is present. |
| accesskit_windows 0.34.0 | UIA IValueProvider SetValue sends SetValue; TextRange Select sends SetTextSelection. | No ReplaceSelectedText occurrence or ITextEditProvider implementation in the adapter source. |
| Existing Linux bridge | AT-SPI Text selection/caret operations send SetTextSelection. Value sends numeric SetValue. | No ReplaceSelectedText route and no EditableText interface. |

Consequently this closes a real missing **component action contract** for a direct or future
AccessKit caller; it does not establish a newly reachable native AT workflow on these pinned
adapters. Native macOS/Windows source was inspected only. No Mac/Windows build or runtime was
performed, and VoiceOver remains off. The broader Linux limitation is recorded in
[linux-editable-text](../../../proposals/linux-editable-text/README.md).

## Minimal delta

Add a private `replace_accessible_selected_text` helper beside `set_accessible_value`:

```rust
fn replace_accessible_selected_text(
    &mut self,
    value: &str,
    window: &mut Window,
    cx: &mut Context<Self>,
) {
    if !self.editable() {
        return;
    }
    self.last_edit = None;
    self.replace_text_in_range(None, value, window, cx);
    self.last_edit = None;
}
```

Register ReplaceSelectedText beside SetValue, only when editable. Accept only
`Some(ActionData::Value(value))`; ignore missing or other payload variants. Capture a separate
WeakEntity, as the existing setters do. Keep the helper's enabled/read-only guard even though
the handler is conditionally registered, so an already-rendered listener cannot edit after a
state transition. The handler neither steals focus nor changes public selection APIs.

Reuse the native `replace_text_in_range(None, ...)` path. Do not reconstruct text from the
accessibility tree, turn the operation into full SetValue, clear history with set_text, or
precommit using unmark_text.

## Required semantics

- **Selection:** use the current native range, including a reversed selection; collapse to the
  inserted end. An empty replacement deletes the selection. Empty replacement at an empty
  caret is a value no-op. This action intentionally has no explicit-range/snapshot payload:
  it edits the current selection when delivered, not a historic external range.
- **Pending IME:** None selects the whole marked range before the ordinary selection. The
  existing composition base supplies the undo snapshot; replacement commits that preedit in
  one native transaction. Calling unmark_text first could produce two commits/events. This is
  the same marked-range policy already used by native paste/typing, not a new IME rule.
- **Undo/callbacks:** isolate the request from adjacent typing by clearing last_edit before
  and after it. One effective committed change emits one Changed and creates the existing
  bounded undo entry. An unchanged committed value emits no Changed and creates no extra
  undo entry. Undo restores the preedit base or prior selection/range direction as applicable.
- **Filtering/maxlength:** apply current InputKind filtering and CR/LF normalization; count
  UTF-16 units, accept whole graphemes, and limit only the replacement range while preserving
  untouched suffix text. Sanitized-empty input still deletes a nonempty selection, matching
  the existing native replacement path.
- **Passwords:** edit the app-owned raw text/range. Export only the existing masking bullets;
  no accessible string is used to reconstruct a secret. No copy/cut/clipboard behavior changes.
- **State:** reject mutations for disabled and read-only fields, including a request delivered
  through a stale listener. A rejected request changes no value, selection, composition,
  Changed count or undo/redo history.

This does not make a separate SetTextSelection plus ReplaceSelectedText pair atomic, add
request acknowledgments, or implement Linux EditableText's explicit-range/clipboard methods.

## Verification proposal for a later implementation

Run all five repository gates on the resulting source. Exercise the private helper in a
temporary native probe using a real TextInput entity, Context and Window; retain the exact
probe/source and binary hashes, callback traces, accessibility tree and screenshots. No
production test API or dependency feature is needed: a scratch copy of the input module can
append a receipt-only child probe that calls its private helper. Label that evidence as
**direct component dispatch**, not an OS accessibility request.

| Case | Expected assertion |
| --- | --- |
| Caret insertion, forward/reversed range, empty replacement | Correct value/caret and one Changed for each effective edit; Undo restores the prior value and selection direction. |
| Neighboring native typing → action → typing | Three intended undo groups; one action cannot merge into either neighboring typing group. |
| `A😀B`, maxlength 6, insertion of `😀界` after A | Admit only the new emoji; retain the original emoji and B; no UTF-16/grapheme split. |
| Combining/ZWJ selected grapheme, multiline CRLF, filtered input | Reuse existing native boundaries, sanitation and whole-grapheme insertion limit. |
| Pending preedit, with and without truncation | Replace the complete marked range, preserve suffix, one Changed/undo transaction, no retained marked range; repeated unmark does not change the result. |
| Same-content, empty-at-caret, sanitized-empty nonempty selection | No event/undo for true value no-ops; deletion has one event and a reversible edit. |
| Password selection replacement | Raw value changes correctly; every exported value/run stays masked. |
| Read-only/disabled before delivery | Helper rejects without changing value, selection, preedit, callback or history. |
| Missing/NumericValue/other payload | Listener ignores it without invoking the edit helper. |

The current GPUI `Window::handle_a11y_action` entry point is crate-private. The public API
allows registration but not arbitrary external injection, and the pinned OS adapters above
have no wire route for ReplaceSelectedText. Direct helper tests plus source inspection can
qualify this component contract; they cannot honestly be described as a native AT pass.
Continue actual Linux native keyboard and AT-SPI Text regression checks for the reused path.
Full native dispatch evidence needs an adapter that really delivers the action.

No production edits, Cargo commands, platform builds or screen-reader runs were performed
for this follow-up design.
