use crate::{
    Theme,
    input::{InputEvent, TextInput},
};
use gpui::{prelude::*, *};

pub struct TextArea {
    input: Entity<TextInput>,
    show_count: bool,
    warn: bool,
    _subscription: Subscription,
}

impl TextArea {
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

    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }

    pub fn text<'a>(&self, cx: &'a App) -> &'a str {
        self.input.read(cx).text()
    }

    pub fn len(&self, cx: &App) -> usize {
        self.input.read(cx).len()
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.input.read(cx).is_empty()
    }

    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.input.update(cx, |input, cx| input.set_text(text, cx));
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.clear(cx));
    }

    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.input
            .update(cx, |input, cx| input.set_placeholder(text, cx));
    }

    pub fn set_placeholder_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_placeholder_color(color, cx));
    }

    pub fn set_aria_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        let label = label.into();
        self.input
            .update(cx, |input, cx| input.set_aria_label(label, cx));
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_disabled(disabled, cx));
    }

    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_readonly(readonly, cx));
    }

    pub fn set_max_len(&mut self, max_len: Option<usize>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_max_len(max_len, cx));
        cx.notify();
    }

    pub fn set_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_rows(Some(rows.max(1)), cx));
    }

    pub fn set_show_count(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_count = show;
        cx.notify();
    }

    pub fn set_warn(&mut self, warn: bool, cx: &mut Context<Self>) {
        self.warn = warn;
        cx.notify();
    }

    pub fn is_focused(&self, window: &Window, cx: &App) -> bool {
        self.input.read(cx).is_focused(window)
    }

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
    }
}
