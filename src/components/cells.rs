use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Cells {
    title: Option<SharedString>,
    footer: Option<SharedString>,
    children: Vec<AnyElement>,
}
impl Default for Cells {
    fn default() -> Self {
        Self::new()
    }
}
impl Cells {
    pub fn new() -> Self {
        Self {
            title: None,
            footer: None,
            children: vec![],
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}
impl ParentElement for Cells {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl RenderOnce for Cells {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        div()
            .w_full()
            .flex()
            .flex_col()
            .mt(px(8.))
            .when_some(self.title, |el, title| {
                el.child(
                    div()
                        .mt(px(8.))
                        .mb(px(3.))
                        .px(px(16.))
                        .text_size(px(14.))
                        .line_height(px(19.6))
                        .text_color(t.fg_1)
                        .child(title),
                )
            })
            .child(
                div()
                    .bg(t.bg_2)
                    .border_y(px(0.5))
                    .border_color(t.fg_3)
                    .children(self.children.into_iter().enumerate().map(|(i, item)| {
                        div()
                            .when(i > 0, |el| {
                                el.child(div().ml(px(16.)).h(px(0.5)).bg(t.fg_3))
                            })
                            .child(item)
                    })),
            )
            .when_some(self.footer, |el, footer| {
                el.child(
                    div()
                        .mt(px(8.))
                        .px(px(16.))
                        .text_size(px(14.))
                        .line_height(px(19.6))
                        .text_color(t.fg_1)
                        .child(footer),
                )
            })
    }
}
#[derive(IntoElement)]
pub struct Cell {
    id: ElementId,
    body: AnyElement,
    label: SharedString,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    access: bool,
    link: bool,
    on_click: Option<ClickHandler>,
}
impl Cell {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            id: label.clone().into(),
            body: label.clone().into_any_element(),
            label,
            header: None,
            footer: None,
            access: false,
            link: false,
            on_click: None,
        }
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = body.into_any_element();
        self
    }
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    pub fn access(mut self) -> Self {
        self.access = true;
        self
    }
    pub fn link(mut self) -> Self {
        self.link = true;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for Cell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let mut el = div()
            .id(self.id)
            .role(if self.link {
                Role::Link
            } else {
                Role::ListItem
            })
            .aria_label(self.label)
            .p(px(16.))
            .min_h(px(56.))
            .flex()
            .items_center()
            .gap(px(16.))
            .text_size(px(17.))
            .line_height(px(24.))
            .text_color(if self.link { t.link } else { t.fg_0 })
            .children(self.header)
            .child(div().flex_1().min_w_0().child(self.body))
            .when_some(self.footer, |el, footer| {
                el.child(div().text_color(t.fg_1).child(footer))
            })
            .when(self.access, |el| {
                el.child(Icon::new("arrow").size(12.).color(t.fg_2))
            });
        if let Some(handler) = self.on_click {
            let bg = t.bg_color_active;
            let brand = t.brand;
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .hover(move |s| s.bg(bg))
                .active(move |s| s.bg(bg))
                .focus_visible(move |s| s.border_2().border_color(brand))
                .on_click(handler);
        }
        el
    }
}
