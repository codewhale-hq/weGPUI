use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

pub type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

pub(crate) fn tab_keys(
    count: usize,
    selected: usize,
    handler: Option<SelectHandler>,
) -> impl Fn(&KeyDownEvent, &mut Window, &mut App) + 'static {
    move |event, window, cx| {
        let Some(handler) = handler.as_ref() else {
            return;
        };
        let next = match event.keystroke.key.as_str() {
            "right" | "down" if selected + 1 < count => {
                window.focus_next(cx);
                selected + 1
            }
            "left" | "up" if selected > 0 => {
                window.focus_prev(cx);
                selected - 1
            }
            _ => return,
        };
        handler(next, window, cx);
        cx.stop_propagation();
    }
}

#[derive(IntoElement)]
pub struct Navbar {
    id: ElementId,
    items: Vec<SharedString>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Navbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: 0,
            on_select: None,
        }
    }
    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Navbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let count = self.items.len();
        let selected = self.selected;
        div()
            .id(self.id)
            .role(Role::TabList)
            .relative()
            .flex()
            .flex_shrink_0()
            .bg(t.bg_2)
            .children(self.items.into_iter().enumerate().map(|(i, label)| {
                let handler = self.on_select.clone();
                let on = i == selected;
                div()
                    .id(("weui-navbar-item", i))
                    .role(Role::Tab)
                    .aria_label(label.clone())
                    .aria_selected(on)
                    .focusable()
                    .tab_stop(true)
                    .on_key_down(tab_keys(count, i, self.on_select.clone()))
                    .relative()
                    .flex_1()
                    .py(px(14.))
                    .border_2()
                    .border_color(transparent_black())
                    .text_center()
                    .text_size(px(17.))
                    .line_height(px(24.))
                    .cursor_pointer()
                    .when(on, |el| el.bg(t.bg_color_active))
                    .active(|s| s.bg(t.bg_color_active))
                    .focus_visible(|s| s.border_color(t.brand))
                    .when(i + 1 < count, |el| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(-2.))
                                .bottom(px(-2.))
                                .right(px(-2.))
                                .w(px(0.5))
                                .bg(t.fg_3),
                        )
                    })
                    .child(label)
                    .on_click(move |_, window, cx| {
                        if let Some(handler) = handler.as_ref() {
                            handler(i, window, cx);
                        }
                    })
            }))
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .h(px(0.5))
                    .bg(t.fg_3),
            )
    }
}

pub fn tab_panel(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .role(Role::TabPanel)
        .aria_label(label)
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
}
