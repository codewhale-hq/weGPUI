use super::navbar::{SelectHandler, tab_keys};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

pub struct TabbarItem {
    label: SharedString,
    icon: AnyElement,
    badge: Option<(AnyElement, Pixels, Pixels)>,
    description: Option<SharedString>,
}

impl TabbarItem {
    pub fn new(label: impl Into<SharedString>, icon: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            icon: icon.into_any_element(),
            badge: None,
            description: None,
        }
    }
    pub fn badge(mut self, badge: impl IntoElement, top: f32, right: f32) -> Self {
        self.badge = Some((badge.into_any_element(), px(top), px(right)));
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(IntoElement)]
pub struct Tabbar {
    id: ElementId,
    label: SharedString,
    items: Vec<TabbarItem>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Tabbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            items: vec![],
            selected: 0,
            on_select: None,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn item(mut self, item: TabbarItem) -> Self {
        self.items.push(item);
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

impl RenderOnce for Tabbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let count = self.items.len();
        let selected = self.selected;
        div()
            .id(self.id)
            .role(Role::TabList)
            .aria_label(self.label)
            .relative()
            .flex()
            .flex_shrink_0()
            .bg(t.bg_1)
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top_0()
                    .h(px(0.5))
                    .bg(t.fg_3),
            )
            .children(self.items.into_iter().enumerate().map(|(i, item)| {
                let handler = self.on_select.clone();
                let on = i == selected;
                let color = if on { t.brand } else { t.fg_0 };
                div()
                    .id(("weui-tabbar-item", i))
                    .role(Role::Tab)
                    .aria_label(item.label.clone())
                    .when_some(item.description, |el, d| el.aria_description(d))
                    .aria_selected(on)
                    .focusable()
                    .tab_stop(true)
                    .on_key_down(tab_keys(count, i, self.on_select.clone()))
                    .flex_1()
                    .pt(px(6.))
                    .pb(px(6.))
                    .border_2()
                    .border_color(transparent_black())
                    .flex()
                    .flex_col()
                    .items_center()
                    .cursor_pointer()
                    .focus_visible(|s| s.border_color(t.brand))
                    .child(
                        div()
                            .relative()
                            .size(px(28.))
                            .mb(px(2.))
                            .child(item.icon)
                            .when_some(item.badge, |el, (badge, top, right)| {
                                el.child(div().absolute().top(top).right(right).child(badge))
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .line_height(relative(1.4))
                            .text_color(color)
                            .child(item.label),
                    )
                    .on_click(move |_, window, cx| {
                        if let Some(handler) = handler.as_ref() {
                            handler(i, window, cx);
                        }
                    })
            }))
    }
}
