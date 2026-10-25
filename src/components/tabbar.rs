//! WeUI bottom tab bar (`weui-tab/weui-tabbar.less`): icon-over-label tabs with optional badges.
//!
//! [`Tabbar`] is stateless like [`Navbar`](super::navbar::Navbar) and shares its arrow-key
//! handling, with focus contained to the bar and one Tab stop. [`TabbarItem`] describes one tab.

use super::navbar::{SelectHandler, tab_focus_handles, tab_keys};
use crate::{Theme, scrolling::RevealFocus};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// One tab: label, 28 px icon slot, optional badge and accessible description.
pub struct TabbarItem {
    label: SharedString,
    icon: AnyElement,
    badge: Option<(AnyElement, Pixels, Pixels)>,
    description: Option<SharedString>,
}

impl TabbarItem {
    /// Creates a tab with `label` under `icon`. The icon is drawn as given, so pass the selected or
    /// unselected variant yourself; only the label colour follows the selection.
    pub fn new(label: impl Into<SharedString>, icon: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            icon: icon.into_any_element(),
            badge: None,
            description: None,
        }
    }
    /// Overlays `badge` on the icon, offset `top` and `right` px from the icon box's top-right
    /// corner; negative values push it outward.
    pub fn badge(mut self, badge: impl IntoElement, top: f32, right: f32) -> Self {
        self.badge = Some((badge.into_any_element(), px(top), px(right)));
        self
    }
    /// Sets the tab's accessible description, such as an unread count.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Bottom tab list (`TabList` role) under a hairline; the selected tab's label uses the brand
/// colour. On a focused tab, Left/Up and Right/Down focus and select the neighbour (only when
/// `on_select` is set).
#[derive(IntoElement)]
pub struct Tabbar {
    id: ElementId,
    label: SharedString,
    items: Vec<TabbarItem>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Tabbar {
    /// Creates an empty tabbar with tab 0 selected.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            items: vec![],
            selected: 0,
            on_select: None,
        }
    }
    /// Sets the tab list's accessible name (default empty).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Appends a tab.
    pub fn item(mut self, item: TabbarItem) -> Self {
        self.items.push(item);
        self
    }
    /// Sets the selected tab index (default 0); an out-of-range index selects none.
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
    /// Called with a tab's index when it is clicked, activated with Enter or Space, or reached with
    /// an arrow key. Store the index and re-render.
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Tabbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let count = self.items.len();
        let selected = self.selected;
        let handles = tab_focus_handles(self.id.clone(), count, window, cx);
        let tab_stop = selected.min(count.saturating_sub(1));
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
                    .track_focus(&handles[i].clone().tab_stop(i == tab_stop))
                    .focusable()
                    .on_key_down(tab_keys(handles.clone(), i, self.on_select.clone()))
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
                    .reveal_focus()
            }))
    }
}
