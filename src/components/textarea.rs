//! WeUI textarea (`.weui-textarea` and `.weui-textarea-counter` in
//! `src/style/widget/weui-cell/weui-form/weui-form_common.less`, example
//! `src/example/form/form_textarea.html`). [`TextArea`] wraps a multiline [`TextInput`] with an
//! optional length counter.
//! Known limitations: lengths count UTF-16 code units like HTML `maxlength`, and IME composition
//! text is only limited on commit (see [`crate::input`]).

use crate::{
    Theme,
    input::{InputEvent, TextInput},
};
use gpui::{prelude::*, *};

/// A multiline text field entity: an 80px-tall [`TextInput`] (or `rows` lines tall) with an
/// optional right-aligned counter below. It re-emits the inner input's [`InputEvent`]s, so
/// subscribe to the `TextArea` entity itself.
pub struct TextArea {
    input: Entity<TextInput>,
    show_count: bool,
    warn: bool,
    _subscription: Subscription,
}

impl TextArea {
    /// Creates an empty, enabled textarea showing `placeholder`, with no length limit and no
    /// counter; create the entity with `cx.new(|cx| TextArea::new("placeholder", cx))`.
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let placeholder = placeholder.into();
        let input = cx.new(|cx| TextInput::new_multiline(placeholder, cx));
        let subscription = cx.subscribe(&input, |_, _, event: &InputEvent, cx| {
            cx.emit(*event);
            cx.notify();
        });
        Self {
            input,
            show_count: false,
            warn: false,
            _subscription: subscription,
        }
    }

    /// The inner multiline [`TextInput`] entity, for settings this wrapper does not forward.
    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }

    /// The current text, including any IME text being composed.
    pub fn text<'a>(&self, cx: &'a App) -> &'a str {
        self.input.read(cx).text()
    }

    /// Text length in UTF-16 code units, the unit the counter and `set_max_len` use.
    pub fn len(&self, cx: &App) -> usize {
        self.input.read(cx).len()
    }

    /// Whether the text is empty.
    pub fn is_empty(&self, cx: &App) -> bool {
        self.input.read(cx).is_empty()
    }

    /// Replaces the text via [`TextInput::set_text`]: sanitised, truncated to the length limit,
    /// and with the undo history cleared.
    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.input.update(cx, |input, cx| input.set_text(text, cx));
    }

    /// Clears the text.
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.clear(cx));
    }

    /// Sets the placeholder shown while the text is empty.
    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.input
            .update(cx, |input, cx| input.set_placeholder(text, cx));
    }

    /// Overrides the placeholder colour; `None` restores the theme's placeholder colour.
    pub fn set_placeholder_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_placeholder_color(color, cx));
    }

    /// Sets the field's accessible name, usually the visible label of its cell.
    pub fn set_aria_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        let label = label.into();
        self.input
            .update(cx, |input, cx| input.set_aria_label(label, cx));
    }

    /// Disables or re-enables the field; a disabled field is not focusable or editable.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_disabled(disabled, cx));
    }

    /// Makes the text read-only (focusable and selectable, not editable) or editable again.
    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_readonly(readonly, cx));
    }

    /// Sets the maximum length in UTF-16 code units, or `None` for no limit (the default), and
    /// re-renders the counter. Longer existing text is truncated.
    pub fn set_max_len(&mut self, max_len: Option<usize>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_max_len(max_len, cx));
        cx.notify();
    }

    /// Sets the visible height to `rows` lines (minimum 1) instead of the default 80px.
    pub fn set_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_rows(Some(rows.max(1)), cx));
    }

    /// Shows or hides the counter (`.weui-textarea-counter`): "count/max" with a limit, else
    /// "count", 14px and right-aligned, exposed as `Role::Status`. Hidden by default.
    pub fn set_show_count(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_count = show;
        cx.notify();
    }

    /// Colours the counter red, as inside a `.weui-cell_warn` row; it also turns red on its own
    /// while the length exceeds the limit. The text colour is unchanged. Off by default.
    pub fn set_warn(&mut self, warn: bool, cx: &mut Context<Self>) {
        self.warn = warn;
        cx.notify();
    }

    /// Whether the inner input holds keyboard focus in `window`.
    pub fn is_focused(&self, window: &Window, cx: &App) -> bool {
        self.input.read(cx).is_focused(window)
    }

    /// Focuses the inner input unless it is disabled.
    pub fn focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.focus(window, cx));
    }
}

impl Focusable for TextArea {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl EventEmitter<InputEvent> for TextArea {}

impl Render for TextArea {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let input = self.input.read(cx);
            let count = input.len();
            let max = input.max_len();
            let over = max.is_some_and(|max| count > max);
            let counter = match max {
                Some(max) => format!("{count}/{max}"),
                None => count.to_string(),
            };
            let counter_color = if self.warn || over { t.red } else { t.fg_2 };
            div()
                .flex()
                .flex_col()
                .w_full()
                .child(self.input.clone())
                .when(self.show_count, |el| {
                    el.child(
                        div()
                            .id("textarea-counter")
                            .role(Role::Status)
                            .aria_label(SharedString::from(counter.clone()))
                            .w_full()
                            .text_right()
                            .text_size(px(14.))
                            .text_color(counter_color)
                            .child(counter),
                    )
                })
        })
    }
}
