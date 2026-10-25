//! Nine-grid navigation from WeUI's `src/style/widget/weui-grid/`: [`Grids`] (`weui-grids`) holds
//! [`Grid`] cells (`weui-grid`), each an icon above a label, separated by hairlines.
//!
//! Known limitations: the column count is fixed at WeUI's three, and cells have no disabled state;
//! they stay focusable even without a click handler.

use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

/// One grid cell: a third of the row wide, padded 20px vertically and 10px horizontally, with a
/// 28px icon above a 14px label.
///
/// It exposes `Role::Button` named by the label, is a Tab stop and activates on click, Enter or
/// Space. Pressing shows theme `bg_color_active`; keyboard focus tints it with the brand colour.
#[derive(IntoElement)]
pub struct Grid {
    id: ElementId,
    icon: Option<AnyElement>,
    label: Option<SharedString>,
    columns: usize,
    on_click: Option<ClickHandler>,
}

impl Grid {
    /// Creates an empty cell with element id `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            label: None,
            columns: 3,
            on_click: None,
        }
    }
    /// Sets the element drawn in the centred 28px icon box (`weui-grid__icon`). Default: none.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
    /// Uses an image filling the icon box. `path` is a URL or the name of an asset served by the
    /// app's `AssetSource`, such as [`crate::Assets`].
    pub fn image(self, path: impl Into<SharedString>) -> Self {
        let path: SharedString = path.into();
        self.icon(img(path).size_full())
    }
    /// Sets the single-line label (`weui-grid__label`), truncated with an ellipsis; it is also the
    /// accessible name. Default: none.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the activation callback. It receives the click event on a pointer click, or on Enter or
    /// Space while the cell is focused.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Grid {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let (line, active, ring, fg) = (t.fg_3, t.bg_color_active, t.brand, t.fg_0);
            let has_icon = self.icon.is_some();
            div()
                .id(self.id)
                .role(Role::Button)
                .when_some(self.label.clone(), |el, l| el.aria_label(l))
                .relative()
                .w(relative(1. / self.columns as f32))
                .pt(px(20.))
                .pb(px(20.))
                .px(px(10.))
                .focusable()
                .tab_stop(true)
                .active(move |s| s.bg(active))
                .when_some(self.icon, |el, icon| {
                    el.child(div().size(px(28.)).mx_auto().child(icon))
                })
                .when_some(self.label, |el, label| {
                    el.child(
                        div()
                            .when(has_icon, |el| el.mt(px(4.)))
                            .text_center()
                            .text_color(fg)
                            .text_size(px(14.))
                            .line_height(relative(1.6))
                            .truncate()
                            .child(label),
                    )
                })
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .right_0()
                        .w(px(0.5))
                        .bg(line),
                )
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .h(px(0.5))
                        .bg(line),
                )
                .focus_visible(move |s| s.bg(ring.opacity(0.1)))
                .when_some(self.on_click, |el, h| el.on_click(h))
        })
    }
}

/// The grid container: a wrapping row of [`Grid`] cells with top and left hairlines, exposed as
/// `Role::Group`. Add cells with `child` or `children`.
#[derive(IntoElement)]
pub struct Grids {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl Grids {
    /// Creates an empty container with element id `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
        }
    }
}

impl ParentElement for Grids {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Grids {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let line = Theme::get(cx).fg_3;
        div()
            .id(self.id)
            .role(Role::Group)
            .relative()
            .overflow_hidden()
            .flex()
            .flex_wrap()
            .children(self.children)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(0.5))
                    .bg(line),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .bottom_0()
                    .w(px(0.5))
                    .bg(line),
            )
    }
}
