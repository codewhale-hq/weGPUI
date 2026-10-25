//! WeUI top tab bar (`weui-tab/weui-navbar.less`) and tab panel (`weui-tab/weui-tab.less`).
//!
//! [`Navbar`] is stateless: the caller owns the selected index and updates it in `on_select`. Arrow
//! keys move focus and selection within this bar. Only the selected tab is in the Tab order;
//! Home/End select the first/last tab, and arrows wrap at the ends.

use crate::{Theme, scrolling::RevealFocus};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Shared callback receiving the index of the tab to select.
pub type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

pub(crate) fn tab_focus_handles(
    id: ElementId,
    count: usize,
    window: &mut Window,
    cx: &mut App,
) -> Vec<FocusHandle> {
    let handles = window.use_keyed_state((id, "tab-focus"), cx, |_, _| Vec::<FocusHandle>::new());
    handles.update(cx, |handles, cx| {
        handles.truncate(count);
        while handles.len() < count {
            handles.push(cx.focus_handle());
        }
        handles.clone()
    })
}

pub(crate) fn tab_keys(
    handles: Vec<FocusHandle>,
    selected: usize,
    handler: Option<SelectHandler>,
) -> impl Fn(&KeyDownEvent, &mut Window, &mut App) + 'static {
    move |event, window, cx| {
        let Some(handler) = handler.as_ref() else {
            return;
        };
        let count = handles.len();
        if count == 0 {
            return;
        }
        let next = match event.keystroke.key.as_str() {
            "right" | "down" => (selected + 1) % count,
            "left" | "up" => (selected + count - 1) % count,
            "home" => 0,
            "end" => count - 1,
            _ => return,
        };
        window.focus(&handles[next], cx);
        handler(next, window, cx);
        cx.stop_propagation();
    }
}

/// Equal-width tab list (`TabList` role) with hairline separators; the selected tab gets the
/// pressed background. On a focused tab, Left/Up and Right/Down focus and select the neighbour
/// (only when `on_select` is set).
#[derive(IntoElement)]
pub struct Navbar {
    id: ElementId,
    items: Vec<SharedString>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Navbar {
    /// Creates an empty navbar with tab 0 selected.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: vec![],
            selected: 0,
            on_select: None,
        }
    }
    /// Sets the tab labels, replacing any earlier ones.
    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.items = items.into_iter().map(Into::into).collect();
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

impl RenderOnce for Navbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let count = self.items.len();
        let selected = self.selected;
        let handles = tab_focus_handles(self.id.clone(), count, window, cx);
        let tab_stop = selected.min(count.saturating_sub(1));
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
                    .track_focus(&handles[i].clone().tab_stop(i == tab_stop))
                    .focusable()
                    .on_key_down(tab_keys(handles.clone(), i, self.on_select.clone()))
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
                    .reveal_focus()
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

/// Scrolling panel (`TabPanel` role, `weui-tab__panel`) that fills the remaining height; add the
/// selected tab's content as children.
/// To reveal focused descendants, call `track_scroll` with a [`ScrollHandle`] and wrap this
/// panel in [`crate::scrolling::FocusScroll`] using the same handle.
pub fn tab_panel(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .role(Role::TabPanel)
        .aria_label(label)
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
}
