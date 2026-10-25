// Adapted from GPUI examples/input.rs, Copyright Zed Industries.
// Licensed under Apache-2.0; see LICENSE-APACHE and NOTICE.

//! Single-line and multiline text editing: the native replacement for HTML `<input>` and
//! `<textarea>` inside WeUI form cells.
//!
//! [`TextInput`] is a GPUI entity that owns its text, selection, IME marked range, scroll position
//! and undo history. Create it with `cx.new(|cx| TextInput::new("placeholder", cx))`, render the
//! entity as a child, observe [`InputEvent`]s with `cx.subscribe`, and call [`init`] once (done by
//! [`crate::init`]) to install the key bindings, which are scoped to the `TextInput` key context.
//! It implements GPUI's `EntityInputHandler`, so platform IME composition and candidate windows
//! work where the GPUI backend supports them.
//!
//! Known limitations: `maxlength` counts UTF-16 code units, as HTML does, but IME composition text
//! is only limited on commit. [`InputKind`] filters characters and is not HTML form validation.
//! There is no spell checking or context menu. Undo history is cleared by
//! [`TextInput::set_text`]. Linux XIM preedit was observed in the input method's own window
//! rather than inline at the caret.

use std::{ops::Range, time::Duration};

use crate::Theme;
use gpui::{prelude::*, *};
use unicode_segmentation::*;

actions!(
    text_input,
    [
        /// Deletes the selection, or the grapheme before the caret.
        Backspace,
        /// Deletes the selection, or the grapheme after the caret.
        Delete,
        /// Deletes the selection, or back to the previous word boundary.
        DeleteWordLeft,
        /// Deletes the selection, or back to the start of the current line.
        DeleteToLineStart,
        /// Moves the caret one grapheme left, collapsing any selection.
        Left,
        /// Moves the caret one grapheme right, collapsing any selection.
        Right,
        /// Moves the caret up one visual line (multiline), or to the start (single line).
        Up,
        /// Moves the caret down one visual line (multiline), or to the end (single line).
        Down,
        /// Extends the selection one grapheme left.
        SelectLeft,
        /// Extends the selection one grapheme right.
        SelectRight,
        /// Extends the selection up one visual line.
        SelectUp,
        /// Extends the selection down one visual line.
        SelectDown,
        /// Moves the caret to the previous word boundary.
        WordLeft,
        /// Moves the caret to the next word boundary.
        WordRight,
        /// Extends the selection to the previous word boundary.
        SelectWordLeft,
        /// Extends the selection to the next word boundary.
        SelectWordRight,
        /// Selects the whole text.
        SelectAll,
        /// Moves the caret to the start of the current line.
        Home,
        /// Moves the caret to the end of the current line.
        End,
        /// Extends the selection to the start of the current line.
        SelectHome,
        /// Extends the selection to the end of the current line.
        SelectEnd,
        /// Moves the caret to the start of the text.
        DocStart,
        /// Moves the caret to the end of the text.
        DocEnd,
        /// Extends the selection to the start of the text.
        SelectDocStart,
        /// Extends the selection to the end of the text.
        SelectDocEnd,
        /// Inserts a newline (multiline) or emits [`InputEvent::Submit`] (single line).
        Enter,
        /// Opens the platform's character/emoji palette where supported.
        ShowCharacterPalette,
        /// Replaces the selection with clipboard text, sanitised and length-limited.
        Paste,
        /// Copies the selection to the clipboard and deletes it (not for passwords).
        Cut,
        /// Copies the selection to the clipboard (not for passwords).
        Copy,
        /// Restores the text and selection before the last edit.
        Undo,
        /// Re-applies the last undone edit.
        Redo,
    ]
);

const BULLET: char = '•';
const UNDO_LIMIT: usize = 100;

#[derive(Clone)]
struct Snapshot {
    content: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EditKind {
    Insert,
    Delete,
    Replace,
}
const BLINK_INTERVAL: Duration = Duration::from_millis(500);

/// Character filter and presentation for a [`TextInput`], mirroring HTML `type` values.
///
/// Typing, pasting and committed IME text are filtered per character. This is not validation:
/// a `Number` field can still hold `"1-e"`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputKind {
    /// Any printable text (`type="text"`). The default.
    #[default]
    Text,
    /// ASCII digits plus `.`, `-`, `+`, `e`, `E` (`type="number"`). [`TextInput::set_text`] clears
    /// values that do not parse as a number.
    Number,
    /// ASCII digits only (the `pattern="[0-9]*"` examples).
    Digit,
    /// Digits plus `+ - ( ) * #` and spaces (`type="tel"`).
    Tel,
    /// Any text, rendered as bullets. Copy and cut are disabled (`type="password"`).
    Password,
}

impl InputKind {
    fn accepts(self, ch: char) -> bool {
        match self {
            InputKind::Text | InputKind::Password => true,
            InputKind::Number => ch.is_ascii_digit() || matches!(ch, '.' | '-' | '+' | 'e' | 'E'),
            InputKind::Digit => ch.is_ascii_digit(),
            InputKind::Tel => {
                ch.is_ascii_digit() || matches!(ch, '+' | '-' | ' ' | '(' | ')' | '*' | '#')
            }
        }
    }
}

/// Notifications emitted by [`TextInput`] (and re-emitted by `TextArea`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEvent {
    /// The text changed through editing, paste, undo/redo, `set_text`, or truncation by
    /// `set_max_len`. Not emitted while IME text is only being composed.
    Changed,
    /// The input gained keyboard focus.
    Focus,
    /// The input lost keyboard focus.
    Blur,
    /// Enter was pressed in a single-line input.
    Submit,
}

/// An editable text field entity. See the [module documentation](self) for usage.
///
/// Lengths and offsets in the public API are in UTF-16 code units, matching HTML. The entity
/// renders WeUI input typography and colours from [`crate::Theme`] and exposes a
/// `TextInput`/multiline accessibility node with its value, placeholder and label.
pub struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    placeholder_color: Option<Hsla>,
    label: Option<SharedString>,
    role: Option<Role>,
    kind: InputKind,
    max_len: Option<usize>,
    disabled: bool,
    readonly: bool,
    multiline: bool,
    rows: Option<usize>,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    preferred_x: Option<Pixels>,
    is_selecting: bool,
    layout: Option<TextLayout>,
    scroll: Point<Pixels>,
    autoscroll: bool,
    focused: bool,
    cursor_visible: bool,
    blink_epoch: usize,
    blink_task: Option<Task<()>>,
    focus_subscriptions: Vec<Subscription>,
    undo_stack: Vec<Snapshot>,
    redo_stack: Vec<Snapshot>,
    last_edit: Option<(EditKind, usize)>,
    composition_base: Option<Snapshot>,
}

impl TextInput {
    /// Creates an empty, enabled, single-line [`InputKind::Text`] input with the given placeholder.
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle().tab_stop(true),
            content: "".into(),
            placeholder: placeholder.into(),
            placeholder_color: None,
            label: None,
            role: None,
            kind: InputKind::Text,
            max_len: None,
            disabled: false,
            readonly: false,
            multiline: false,
            rows: None,
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            preferred_x: None,
            is_selecting: false,
            layout: None,
            scroll: Point::default(),
            autoscroll: true,
            focused: false,
            cursor_visible: true,
            blink_epoch: 0,
            blink_task: None,
            focus_subscriptions: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_edit: None,
            composition_base: None,
        }
    }

    pub(crate) fn new_multiline(
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            multiline: true,
            ..Self::new(placeholder, cx)
        }
    }

    /// The current committed text, including any IME text being composed.
    pub fn text(&self) -> &str {
        &self.content
    }

    /// Text length in UTF-16 code units, the unit HTML `maxlength` counts.
    pub fn len(&self) -> usize {
        self.content.encode_utf16().count()
    }

    /// Whether the text is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// The placeholder shown while the text is empty.
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// The character filter and presentation in effect.
    pub fn kind(&self) -> InputKind {
        self.kind
    }

    /// The maximum length in UTF-16 code units, if any.
    pub fn max_len(&self) -> Option<usize> {
        self.max_len
    }

    /// Whether the input is disabled (not focusable, not editable).
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Whether the input is read-only (focusable and selectable, not editable).
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Replaces the text programmatically.
    ///
    /// The text is sanitised (newlines become spaces in single-line inputs; control characters
    /// and characters rejected by the [`InputKind`] are dropped) and truncated to the maximum
    /// length. The caret moves to the end, any composition ends, and the undo history is cleared.
    /// Emits [`InputEvent::Changed`] if the text differs.
    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text: SharedString = text.into();
        let text = if self.kind == InputKind::Number
            && !text.trim().is_empty()
            && text.trim().parse::<f64>().is_err()
        {
            String::new()
        } else {
            self.sanitize(&text)
        };
        let text = self.limit(&text, 0..self.content.len());
        let changed = *self.content != *text;
        self.content = text.into();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_edit = None;
        self.composition_base = None;
        self.selected_range = self.content.len()..self.content.len();
        self.selection_reversed = false;
        self.marked_range = None;
        self.preferred_x = None;
        self.autoscroll = true;
        if changed {
            cx.emit(InputEvent::Changed);
        }
        cx.notify();
    }

    /// Clears the text and scroll position, as `set_text("")`.
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.set_text("", cx);
        self.scroll = Point::default();
    }

    /// Sets the placeholder, also used as the accessible placeholder.
    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.placeholder = text.into();
        cx.notify();
    }

    /// Overrides the placeholder colour; `None` restores the theme's placeholder colour.
    pub fn set_placeholder_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.placeholder_color = color;
        cx.notify();
    }

    /// Sets the accessible name, usually the visible form label of the cell.
    pub fn set_aria_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        let label = Some(label.into());
        if self.label != label {
            self.label = label;
            cx.notify();
        }
    }

    /// Overrides the accessibility role (for example `Role::SearchInput`); `None` uses the
    /// default text-input role.
    pub fn set_accessible_role(&mut self, role: Option<Role>, cx: &mut Context<Self>) {
        self.role = role;
        cx.notify();
    }

    /// Enables or disables the input. Disabling removes it from the tab order, ends any
    /// selection drag or composition, and stops the caret blink.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        self.focus_handle = self.focus_handle.clone().tab_stop(!disabled);
        if disabled {
            self.is_selecting = false;
            self.marked_range = None;
            self.stop_blink();
        }
        cx.notify();
    }

    /// Makes the input read-only or editable. Read-only inputs keep focus and selection.
    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.readonly = readonly;
        self.marked_range = None;
        cx.notify();
    }

    /// Sets the maximum length in UTF-16 code units (HTML `maxlength`); `None` removes it.
    ///
    /// Longer existing text is truncated at a grapheme boundary and emits
    /// [`InputEvent::Changed`]. Later edits that would exceed the limit insert only the graphemes
    /// that fit.
    pub fn set_max_len(&mut self, max_len: Option<usize>, cx: &mut Context<Self>) {
        self.max_len = max_len;
        if let Some(max) = max_len
            && self.len() > max
        {
            let end = self.limit(&self.content, 0..self.content.len()).len();
            self.content = self.content[..end].to_string().into();
            self.selected_range = end..end;
            self.selection_reversed = false;
            self.marked_range = None;
            cx.emit(InputEvent::Changed);
        }
        cx.notify();
    }

    /// Changes the [`InputKind`] and re-filters the current text through `set_text`.
    pub fn set_kind(&mut self, kind: InputKind, cx: &mut Context<Self>) {
        self.kind = kind;
        let text = self.content.clone();
        self.set_text(text, cx);
    }

    pub(crate) fn set_rows(&mut self, rows: Option<usize>, cx: &mut Context<Self>) {
        self.rows = rows;
        cx.notify();
    }

    /// Whether this input holds keyboard focus in `window`.
    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    /// Focuses the input unless it is disabled, and starts the caret blink.
    pub fn focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.ensure_focus_subscriptions(window, cx);
        window.focus(&self.focus_handle, cx);
    }

    fn editable(&self) -> bool {
        !self.disabled && !self.readonly
    }

    fn sanitize(&self, text: &str) -> String {
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        let text = if self.multiline {
            text
        } else {
            text.replace('\n', " ")
        };
        text.chars()
            .filter(|ch| *ch == '\n' || !ch.is_control())
            .filter(|ch| self.kind.accepts(*ch) || (*ch == '\n' && self.multiline))
            .collect()
    }

    fn limit(&self, text: &str, replaced: Range<usize>) -> String {
        let Some(max) = self.max_len else {
            return text.to_string();
        };
        let kept = self.content[..replaced.start].encode_utf16().count()
            + self.content[replaced.end..].encode_utf16().count();
        let mut budget = max.saturating_sub(kept);
        text.graphemes(true)
            .take_while(|grapheme| {
                let units = grapheme.encode_utf16().count();
                let fits = units <= budget;
                budget = budget.saturating_sub(units);
                fits
            })
            .collect()
    }

    fn ensure_focus_subscriptions(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_subscriptions.is_empty() {
            return;
        }
        let handle = self.focus_handle.clone();
        self.focus_subscriptions
            .push(cx.on_focus(&handle, window, |this, _, cx| {
                this.focused = true;
                this.restart_blink(cx);
                cx.emit(InputEvent::Focus);
                cx.notify();
            }));
        self.focus_subscriptions
            .push(cx.on_blur(&handle, window, |this, _, cx| {
                this.focused = false;
                this.is_selecting = false;
                this.stop_blink();
                cx.emit(InputEvent::Blur);
                cx.notify();
            }));
    }

    fn restart_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self.blink_epoch = self.blink_epoch.wrapping_add(1);
        if !self.focused || cx.reduce_motion() {
            self.blink_task = None;
            return;
        }
        let epoch = self.blink_epoch;
        self.blink_task = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(BLINK_INTERVAL).await;
                let alive = this.update(cx, |this, cx| {
                    if this.blink_epoch != epoch {
                        return false;
                    }
                    this.cursor_visible = !this.cursor_visible;
                    cx.notify();
                    true
                });
                if !matches!(alive, Ok(true)) {
                    break;
                }
            }
        }));
    }

    fn stop_blink(&mut self) {
        self.blink_task = None;
        self.blink_epoch = self.blink_epoch.wrapping_add(1);
        self.cursor_visible = true;
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(-1., false, cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(1., false, cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(-1., true, cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(1., true, cx);
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_word(self.cursor_offset()), cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_word(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.row_range(self.cursor_offset()).start, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.row_range(self.cursor_offset()).end, cx);
    }

    fn select_home(&mut self, _: &SelectHome, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.row_range(self.cursor_offset()).start, cx);
    }

    fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.row_range(self.cursor_offset()).end, cx);
    }

    fn doc_start(&mut self, _: &DocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn doc_end(&mut self, _: &DocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn select_doc_start(&mut self, _: &SelectDocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(0, cx);
    }

    fn select_doc_end(&mut self, _: &SelectDocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.content.len(), cx);
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            if self.editable() {
                self.replace_text_in_range(None, "\n", window, cx);
            }
        } else {
            cx.emit(InputEvent::Submit);
        }
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if self.selected_range.is_empty() {
            let prev = self.previous_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                window.play_system_bell();
                return;
            }
            self.select_to(prev, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                window.play_system_bell();
                return;
            }
            self.select_to(next, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete_word_left(
        &mut self,
        _: &DeleteWordLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        if self.selected_range.is_empty() {
            self.select_to(self.previous_word(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete_to_line_start(
        &mut self,
        _: &DeleteToLineStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        if self.selected_range.is_empty() {
            self.select_to(self.row_range(self.cursor_offset()).start, cx);
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        self.ensure_focus_subscriptions(window, cx);
        window.focus(&self.focus_handle, cx);
        let index = self.index_for_mouse_position(event.position);
        if event.click_count >= 3 {
            self.move_to(0, cx);
            self.select_to(self.content.len(), cx);
            return;
        }
        if event.click_count == 2 {
            let word = self.word_range(index);
            self.move_to(word.start, cx);
            self.select_to(word.end, cx);
            return;
        }
        self.is_selecting = true;
        if event.modifiers.shift {
            self.select_to(index, cx);
        } else {
            self.move_to(index, cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn on_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(layout) = self.layout.as_ref() else {
            return;
        };
        let max = (layout.total_height - layout.bounds.size.height).max(px(0.));
        if max <= px(0.) {
            return;
        }
        let delta = event.delta.pixel_delta(layout.line_height);
        self.scroll.y = (self.scroll.y - delta.y).max(px(0.)).min(max);
        self.autoscroll = false;
        cx.stop_propagation();
        cx.notify();
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && self.kind != InputKind::Password {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && self.editable() && self.kind != InputKind::Password {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            content: self.content.clone(),
            selected_range: self.selected_range.clone(),
            selection_reversed: self.selection_reversed,
        }
    }

    fn restore(&mut self, snapshot: Snapshot, cx: &mut Context<Self>) {
        self.content = snapshot.content;
        self.selected_range = snapshot.selected_range;
        self.selection_reversed = snapshot.selection_reversed;
        self.marked_range = None;
        self.composition_base = None;
        self.last_edit = None;
        self.after_cursor_change(cx);
        cx.emit(InputEvent::Changed);
    }

    fn record_edit(&mut self, before: Snapshot, range: &Range<usize>, text: &str) {
        let kind = match (range.is_empty(), text.is_empty()) {
            (_, true) => EditKind::Delete,
            (true, false) => EditKind::Insert,
            (false, false) => EditKind::Replace,
        };
        let coalesce = match (self.last_edit, kind) {
            (Some((EditKind::Insert, at)), EditKind::Insert) => {
                at == range.start && !text.chars().any(char::is_whitespace)
            }
            (Some((EditKind::Delete, at)), EditKind::Delete) => {
                at == range.start || at == range.end
            }
            _ => false,
        };
        if !coalesce {
            self.undo_stack.push(before);
            if self.undo_stack.len() > UNDO_LIMIT {
                self.undo_stack.remove(0);
            }
        }
        self.redo_stack.clear();
        self.last_edit = Some((kind, range.start + text.len()));
    }

    fn undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if let Some(snapshot) = self.undo_stack.pop() {
            self.redo_stack.push(self.snapshot());
            self.restore(snapshot, cx);
        }
    }

    fn redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if let Some(snapshot) = self.redo_stack.pop() {
            self.undo_stack.push(self.snapshot());
            self.restore(snapshot, cx);
        }
    }

    fn vertical(&mut self, direction: f32, select: bool, cx: &mut Context<Self>) {
        if !self.multiline {
            let target = if direction < 0. {
                0
            } else {
                self.content.len()
            };
            if select {
                self.select_to(target, cx)
            } else {
                self.move_to(target, cx)
            }
            return;
        }
        let from = if select || self.selected_range.is_empty() {
            self.cursor_offset()
        } else if direction < 0. {
            self.selected_range.start
        } else {
            self.selected_range.end
        };
        let Some(layout) = self.layout.as_ref() else {
            return;
        };
        let position = layout.position(self.to_display(from));
        let x = self.preferred_x.unwrap_or(position.x);
        let lh = layout.line_height;
        let y = position.y + lh * direction + lh / 2.;
        let target = if y < px(0.) {
            0
        } else if y >= layout.total_height {
            self.content.len()
        } else {
            self.display_to_content(layout.index_at(point(x, y)))
        };
        if select {
            self.select_to(target, cx)
        } else {
            self.move_to(target, cx)
        }
        self.preferred_x = Some(x);
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        self.after_cursor_change(cx);
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.after_cursor_change(cx);
    }

    fn after_cursor_change(&mut self, cx: &mut Context<Self>) {
        self.preferred_x = None;
        self.autoscroll = true;
        self.restart_blink(cx);
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn row_range(&self, offset: usize) -> Range<usize> {
        if !self.multiline {
            return 0..self.content.len();
        }
        match self.layout.as_ref() {
            Some(layout) if !self.content.is_empty() => {
                let range = layout.row_range(offset);
                range.start.min(self.content.len())..range.end.min(self.content.len())
            }
            _ => 0..self.content.len(),
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let Some(layout) = self.layout.as_ref() else {
            return 0;
        };
        let local = position - layout.bounds.origin + self.scroll;
        self.display_to_content(layout.index_at(local))
    }

    fn to_display(&self, offset: usize) -> usize {
        if self.content.is_empty() {
            0
        } else if self.kind == InputKind::Password {
            self.content[..offset.min(self.content.len())]
                .graphemes(true)
                .count()
                * BULLET.len_utf8()
        } else {
            offset
        }
    }

    fn display_to_content(&self, offset: usize) -> usize {
        if self.content.is_empty() {
            0
        } else if self.kind == InputKind::Password {
            self.content
                .grapheme_indices(true)
                .nth(offset / BULLET.len_utf8())
                .map_or(self.content.len(), |(i, _)| i)
        } else {
            offset.min(self.content.len())
        }
    }

    fn display_text(&self) -> SharedString {
        if self.kind == InputKind::Password {
            BULLET
                .to_string()
                .repeat(self.content.graphemes(true).count())
                .into()
        } else {
            self.content.clone()
        }
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    fn previous_word(&self, offset: usize) -> usize {
        if self.kind == InputKind::Password {
            return 0;
        }
        self.content[..offset]
            .unicode_word_indices()
            .next_back()
            .map_or(0, |(i, _)| i)
    }

    fn next_word(&self, offset: usize) -> usize {
        if self.kind == InputKind::Password {
            return self.content.len();
        }
        self.content[offset..]
            .unicode_word_indices()
            .next()
            .map_or(self.content.len(), |(i, word)| offset + i + word.len())
    }

    fn word_range(&self, offset: usize) -> Range<usize> {
        if self.kind == InputKind::Password {
            return 0..self.content.len();
        }
        let mut last = offset..offset;
        for (i, word) in self.content.split_word_bound_indices() {
            last = i..i + word.len();
            if offset < i + word.len() {
                return last;
            }
        }
        last
    }

    fn replace(&mut self, range: Range<usize>, text: &str) -> Range<usize> {
        self.content =
            (self.content[0..range.start].to_owned() + text + &self.content[range.end..]).into();
        range.start..range.start + text.len()
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
        if let Some(base) = self.composition_base.take()
            && *base.content != *self.content
        {
            self.undo_stack.push(base);
            self.redo_stack.clear();
            self.last_edit = None;
        }
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let text = self.sanitize(new_text);
        let text = self.limit(&text, range.clone());
        let had_marked = self.marked_range.take().is_some();
        let base = self.composition_base.take();
        if text.is_empty() && range.is_empty() && !had_marked {
            cx.notify();
            return;
        }
        let before = base.unwrap_or_else(|| self.snapshot());
        if *before.content
            == *format!(
                "{}{}{}",
                &self.content[..range.start],
                text,
                &self.content[range.end..]
            )
        {
            let inserted = self.replace(range, &text);
            self.selected_range = inserted.end..inserted.end;
            self.selection_reversed = false;
            self.after_cursor_change(cx);
            return;
        }
        self.record_edit(before, &range, &text);
        let inserted = self.replace(range, &text);
        self.selected_range = inserted.end..inserted.end;
        self.selection_reversed = false;
        self.after_cursor_change(cx);
        cx.emit(InputEvent::Changed);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let new_text = if self.multiline {
            new_text.replace("\r\n", "\n").replace('\r', "\n")
        } else {
            new_text.replace(['\r', '\n'], " ")
        };
        if self.marked_range.is_none() && self.composition_base.is_none() {
            self.composition_base = Some(self.snapshot());
        }
        let inserted = self.replace(range, &new_text);
        self.marked_range = (!new_text.is_empty()).then_some(inserted.clone());
        if new_text.is_empty() {
            self.composition_base = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| {
                let start = utf16_offset(&new_text, range_utf16.start);
                let end = utf16_offset(&new_text, range_utf16.end).max(start);
                inserted.start + start..inserted.start + end
            })
            .unwrap_or_else(|| inserted.end..inserted.end);
        self.selection_reversed = false;
        self.after_cursor_change(cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layout = self.layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        let start = layout.position(self.to_display(range.start));
        let end = layout.position(self.to_display(range.end));
        let origin = bounds.origin - self.scroll;
        Some(Bounds::from_corners(
            origin + start,
            origin + point(end.x.max(start.x), start.y + layout.line_height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let layout = self.layout.as_ref()?;
        let local = layout.bounds.localize(&point)? + self.scroll;
        if self.content.is_empty() {
            return Some(0);
        }
        let index = self.display_to_content(layout.index_at(local));
        Some(self.offset_to_utf16(index))
    }
}

struct TextLayout {
    lines: Vec<WrappedLine>,
    starts: Vec<usize>,
    tops: Vec<Pixels>,
    line_height: Pixels,
    total_height: Pixels,
    bounds: Bounds<Pixels>,
}

impl TextLayout {
    fn locate(&self, index: usize) -> Option<(usize, usize)> {
        if self.lines.is_empty() {
            return None;
        }
        let line = self
            .starts
            .partition_point(|start| *start <= index)
            .saturating_sub(1);
        Some((
            line,
            (index - self.starts[line]).min(self.lines[line].len()),
        ))
    }

    fn row_starts(line: &WrappedLine) -> Vec<usize> {
        let runs = &line.unwrapped_layout.runs;
        std::iter::once(0)
            .chain(
                line.wrap_boundaries()
                    .iter()
                    .map(|b| runs[b.run_ix].glyphs[b.glyph_ix].index),
            )
            .collect()
    }

    fn row_of(starts: &[usize], local: usize) -> usize {
        starts[1..].iter().filter(|start| **start < local).count()
    }

    fn position(&self, index: usize) -> Point<Pixels> {
        let Some((line, local)) = self.locate(index) else {
            return Point::default();
        };
        self.lines[line]
            .position_for_index(local, self.line_height)
            .unwrap_or_default()
            + point(px(0.), self.tops[line])
    }

    fn index_at(&self, position: Point<Pixels>) -> usize {
        if self.lines.is_empty() {
            return 0;
        }
        let line = self
            .tops
            .partition_point(|top| *top <= position.y)
            .saturating_sub(1);
        let wrapped = &self.lines[line];
        let rows = wrapped.wrap_boundaries().len() + 1;
        let y = (position.y - self.tops[line])
            .max(px(0.))
            .min(self.line_height * rows as f32 - px(0.5));
        let local = match wrapped
            .closest_index_for_position(point(position.x.max(px(0.)), y), self.line_height)
        {
            Ok(index) | Err(index) => index,
        };
        self.starts[line] + local
    }

    fn row_range(&self, index: usize) -> Range<usize> {
        let Some((line, local)) = self.locate(index) else {
            return 0..0;
        };
        let starts = Self::row_starts(&self.lines[line]);
        let row = Self::row_of(&starts, local);
        let base = self.starts[line];
        base + starts[row]
            ..base
                + starts
                    .get(row + 1)
                    .copied()
                    .unwrap_or(self.lines[line].len())
    }

    fn selection_rects(&self, range: Range<usize>, newline_width: Pixels) -> Vec<Bounds<Pixels>> {
        let mut rects = Vec::new();
        for (i, line) in self.lines.iter().enumerate() {
            let base = self.starts[i];
            let len = line.len();
            if range.end < base || range.start > base + len {
                continue;
            }
            let starts = Self::row_starts(line);
            let layout = &line.unwrapped_layout;
            for (row, row_start) in starts.iter().enumerate() {
                let row_end = starts.get(row + 1).copied().unwrap_or(len);
                let lo = range.start.max(base + row_start);
                let hi = range.end.min(base + row_end);
                if lo > hi {
                    continue;
                }
                let origin_x = layout.x_for_index(*row_start);
                let x0 = layout.x_for_index(lo - base) - origin_x;
                let x1 = layout.x_for_index(hi - base) - origin_x;
                let extra = if row + 1 == starts.len() && range.end > base + len {
                    newline_width
                } else {
                    px(0.)
                };
                if x1 + extra <= x0 {
                    continue;
                }
                let y = self.tops[i] + self.line_height * row as f32;
                rects.push(Bounds::from_corners(
                    point(x0, y),
                    point(x1 + extra, y + self.line_height),
                ));
            }
        }
        rects
    }
}

struct TextElement {
    input: Entity<TextInput>,
}

struct PrepaintState {
    layout: Option<TextLayout>,
    cursor: Option<PaintQuad>,
    selections: Vec<PaintQuad>,
    scroll: Point<Pixels>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let input = self.input.read(cx);
        let line_height = window.line_height();
        let height = if input.multiline {
            input.rows.map_or(px(80.), |rows| line_height * rows as f32)
        } else {
            line_height
        };
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = height.into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let theme = Theme::get(cx);
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let empty = input.content.is_empty();
        let (display_text, text_color) = if empty {
            (
                input.placeholder.clone(),
                input.placeholder_color.unwrap_or(theme.fg_2),
            )
        } else {
            (input.display_text(), style.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = match input.marked_range.as_ref().filter(|_| !empty) {
            Some(marked) => {
                let start = input.to_display(marked.start);
                let end = input.to_display(marked.end);
                vec![
                    TextRun {
                        len: start,
                        ..run.clone()
                    },
                    TextRun {
                        len: end - start,
                        underline: Some(UnderlineStyle {
                            color: Some(run.color),
                            thickness: px(1.0),
                            wavy: false,
                        }),
                        ..run.clone()
                    },
                    TextRun {
                        len: display_text.len() - end,
                        ..run
                    },
                ]
                .into_iter()
                .filter(|run| run.len > 0)
                .collect()
            }
            None => vec![run],
        };

        let wrap_width = input.multiline.then_some(bounds.size.width);
        let lines: Vec<WrappedLine> = window
            .text_system()
            .shape_text(display_text, font_size, &runs, wrap_width, None)
            .map(|lines| lines.into_iter().collect())
            .unwrap_or_default();
        let mut starts = Vec::with_capacity(lines.len());
        let mut tops = Vec::with_capacity(lines.len());
        let mut offset = 0;
        let mut y = px(0.);
        for line in &lines {
            starts.push(offset);
            tops.push(y);
            offset += line.len() + 1;
            y += line_height * (line.wrap_boundaries().len() + 1) as f32;
        }
        let caret_height = lines.first().map_or(line_height, |line| {
            (line.ascent() + line.descent()).min(line_height)
        });
        let layout = TextLayout {
            lines,
            starts,
            tops,
            line_height,
            total_height: y,
            bounds,
        };

        let cursor_display = input.to_display(input.cursor_offset());
        let cursor_pos = layout.position(cursor_display);
        let mut scroll = input.scroll;
        if input.multiline {
            scroll.x = px(0.);
            if input.autoscroll {
                if cursor_pos.y < scroll.y {
                    scroll.y = cursor_pos.y;
                }
                if cursor_pos.y + line_height > scroll.y + bounds.size.height {
                    scroll.y = cursor_pos.y + line_height - bounds.size.height;
                }
            }
            scroll.y = scroll
                .y
                .min((layout.total_height - bounds.size.height).max(px(0.)))
                .max(px(0.));
        } else {
            scroll.y = px(0.);
            if input.autoscroll {
                if cursor_pos.x < scroll.x {
                    scroll.x = cursor_pos.x;
                }
                if cursor_pos.x > scroll.x + bounds.size.width - px(1.) {
                    scroll.x = cursor_pos.x - bounds.size.width + px(1.);
                }
            }
            let width = layout
                .lines
                .first()
                .map_or(px(0.), |line| line.unwrapped_layout.width);
            scroll.x = scroll
                .x
                .min((width + px(1.) - bounds.size.width).max(px(0.)))
                .max(px(0.));
        }
        let origin = bounds.origin - scroll;
        let brand = theme.brand;
        let selections = if input.selected_range.is_empty() || empty {
            Vec::new()
        } else {
            let range = input.to_display(input.selected_range.start)
                ..input.to_display(input.selected_range.end);
            layout
                .selection_rects(range, font_size * 0.3)
                .into_iter()
                .map(|rect| fill(rect + origin, brand.opacity(0.25)))
                .collect()
        };
        let cursor = (input.selected_range.is_empty()
            && input.focused
            && input.cursor_visible
            && !input.disabled)
            .then(|| {
                fill(
                    Bounds::new(
                        origin + cursor_pos + point(px(0.), (line_height - caret_height) / 2.),
                        size(px(1.), caret_height),
                    ),
                    brand,
                )
            });
        PrepaintState {
            layout: Some(layout),
            cursor,
            selections,
            scroll,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let input = self.input.read(cx);
        let focus_handle = input.focus_handle.clone();
        if input.editable() {
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.input.clone()),
                cx,
            );
        }
        let Some(layout) = prepaint.layout.take() else {
            return;
        };
        let origin = bounds.origin - prepaint.scroll;
        let selections = std::mem::take(&mut prepaint.selections);
        let cursor = prepaint.cursor.take();
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            for selection in selections {
                window.paint_quad(selection);
            }
            for (line, top) in layout.lines.iter().zip(&layout.tops) {
                line.paint(
                    origin + point(px(0.), *top),
                    layout.line_height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .ok();
            }
        });
        if let Some(cursor) = cursor
            && focus_handle.is_focused(window)
        {
            window.paint_quad(cursor);
        }

        let scroll = prepaint.scroll;
        self.input.update(cx, |input, _cx| {
            input.scroll = scroll;
            input.autoscroll = false;
            input.layout = Some(layout);
        });
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            self.ensure_focus_subscriptions(window, cx);
            let role = self.role.unwrap_or(if self.multiline {
                Role::MultilineTextInput
            } else if self.kind == InputKind::Password {
                Role::PasswordInput
            } else {
                Role::TextInput
            });
            let label = self
                .label
                .clone()
                .unwrap_or_else(|| self.placeholder.clone());
            let el = div()
                .id("text-input")
                .role(role)
                .aria_label(label)
                .aria_placeholder(self.placeholder.clone())
                .aria_value(self.display_text())
                .flex()
                .w_full()
                .min_w_0()
                .when(!self.disabled, |el| {
                    el.key_context("TextInput")
                        .track_focus(&self.focus_handle)
                        .cursor(CursorStyle::IBeam)
                        .on_action(cx.listener(Self::undo))
                        .on_action(cx.listener(Self::redo))
                        .on_action(cx.listener(Self::backspace))
                        .on_action(cx.listener(Self::delete))
                        .on_action(cx.listener(Self::delete_word_left))
                        .on_action(cx.listener(Self::delete_to_line_start))
                        .on_action(cx.listener(Self::left))
                        .on_action(cx.listener(Self::right))
                        .on_action(cx.listener(Self::up))
                        .on_action(cx.listener(Self::down))
                        .on_action(cx.listener(Self::select_left))
                        .on_action(cx.listener(Self::select_right))
                        .on_action(cx.listener(Self::select_up))
                        .on_action(cx.listener(Self::select_down))
                        .on_action(cx.listener(Self::word_left))
                        .on_action(cx.listener(Self::word_right))
                        .on_action(cx.listener(Self::select_word_left))
                        .on_action(cx.listener(Self::select_word_right))
                        .on_action(cx.listener(Self::select_all))
                        .on_action(cx.listener(Self::home))
                        .on_action(cx.listener(Self::end))
                        .on_action(cx.listener(Self::select_home))
                        .on_action(cx.listener(Self::select_end))
                        .on_action(cx.listener(Self::doc_start))
                        .on_action(cx.listener(Self::doc_end))
                        .on_action(cx.listener(Self::select_doc_start))
                        .on_action(cx.listener(Self::select_doc_end))
                        .on_action(cx.listener(Self::enter))
                        .on_action(cx.listener(Self::show_character_palette))
                        .on_action(cx.listener(Self::paste))
                        .on_action(cx.listener(Self::cut))
                        .on_action(cx.listener(Self::copy))
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                        .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
                        .on_mouse_move(cx.listener(Self::on_mouse_move))
                        .when(self.multiline, |el| {
                            el.on_scroll_wheel(cx.listener(Self::on_scroll_wheel))
                        })
                })
                .child(TextElement { input: cx.entity() });
            FieldSemantics {
                inner: el,
                disabled: self.disabled,
                readonly: self.readonly,
            }
        })
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<InputEvent> for TextInput {}

pub(crate) struct FieldSemantics<E: Element> {
    pub(crate) inner: E,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
}

impl<E: Element> IntoElement for FieldSemantics<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for FieldSemantics<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }
    fn a11y_role(&self) -> Option<Role> {
        self.inner.a11y_role()
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        if self.disabled {
            node.set_disabled();
        }
        if self.readonly {
            node.set_read_only();
        }
    }
    fn a11y_synthetic_children(
        &mut self,
        prepaint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner.a11y_synthetic_children(prepaint, builder);
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.inner.request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.inner
            .prepaint(id, inspector, bounds, state, window, cx)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner
            .paint(id, inspector, bounds, state, prepaint, window, cx);
    }
}

fn utf16_offset(text: &str, offset: usize) -> usize {
    let mut units = 0;
    for (byte, ch) in text.char_indices() {
        if units >= offset {
            return byte;
        }
        units += ch.len_utf16();
    }
    text.len()
}

/// Installs the text-editing key bindings for the `TextInput` key context. Both `ctrl-` and
/// `cmd-` variants are bound for clipboard, select-all and undo/redo. Called by [`crate::init`].
pub fn init(cx: &mut App) {
    let ctx = Some("TextInput");
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, ctx),
        KeyBinding::new("shift-backspace", Backspace, ctx),
        KeyBinding::new("delete", Delete, ctx),
        KeyBinding::new("left", Left, ctx),
        KeyBinding::new("right", Right, ctx),
        KeyBinding::new("up", Up, ctx),
        KeyBinding::new("down", Down, ctx),
        KeyBinding::new("shift-left", SelectLeft, ctx),
        KeyBinding::new("shift-right", SelectRight, ctx),
        KeyBinding::new("shift-up", SelectUp, ctx),
        KeyBinding::new("shift-down", SelectDown, ctx),
        KeyBinding::new("home", Home, ctx),
        KeyBinding::new("end", End, ctx),
        KeyBinding::new("shift-home", SelectHome, ctx),
        KeyBinding::new("shift-end", SelectEnd, ctx),
        KeyBinding::new("enter", Enter, ctx),
        KeyBinding::new("ctrl-a", SelectAll, ctx),
        KeyBinding::new("ctrl-v", Paste, ctx),
        KeyBinding::new("ctrl-c", Copy, ctx),
        KeyBinding::new("ctrl-x", Cut, ctx),
        KeyBinding::new("cmd-a", SelectAll, ctx),
        KeyBinding::new("cmd-v", Paste, ctx),
        KeyBinding::new("cmd-c", Copy, ctx),
        KeyBinding::new("cmd-x", Cut, ctx),
        KeyBinding::new("ctrl-z", Undo, ctx),
        KeyBinding::new("ctrl-shift-z", Redo, ctx),
        KeyBinding::new("ctrl-y", Redo, ctx),
        KeyBinding::new("cmd-z", Undo, ctx),
        KeyBinding::new("cmd-shift-z", Redo, ctx),
        KeyBinding::new("alt-left", WordLeft, ctx),
        KeyBinding::new("alt-right", WordRight, ctx),
        KeyBinding::new("shift-alt-left", SelectWordLeft, ctx),
        KeyBinding::new("shift-alt-right", SelectWordRight, ctx),
        KeyBinding::new("alt-backspace", DeleteWordLeft, ctx),
        KeyBinding::new("cmd-backspace", DeleteToLineStart, ctx),
        KeyBinding::new("cmd-left", Home, ctx),
        KeyBinding::new("cmd-right", End, ctx),
        KeyBinding::new("shift-cmd-left", SelectHome, ctx),
        KeyBinding::new("shift-cmd-right", SelectEnd, ctx),
        KeyBinding::new("cmd-up", DocStart, ctx),
        KeyBinding::new("cmd-down", DocEnd, ctx),
        KeyBinding::new("shift-cmd-up", SelectDocStart, ctx),
        KeyBinding::new("shift-cmd-down", SelectDocEnd, ctx),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, ctx),
    ]);
}

#[cfg(test)]
mod tests {
    use super::utf16_offset;
    use core::prelude::v1::test;

    #[test]
    fn utf16_offsets_stay_on_utf8_boundaries() {
        let text = "A😀中";
        // An offset inside the surrogate pair rounds to the next scalar boundary.
        for (units, bytes) in [(0, 0), (1, 1), (2, 5), (3, 5), (4, 8), (99, 8)] {
            assert_eq!(utf16_offset(text, units), bytes);
            assert!(text.is_char_boundary(bytes));
        }
        assert_eq!(utf16_offset("", 99), 0);
    }
}
