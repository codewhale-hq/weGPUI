use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct FooterLink {
    id: ElementId,
    text: SharedString,
    first: bool,
    on_click: Option<ClickHandler>,
}

impl FooterLink {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            first: true,
            on_click: None,
        }
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for FooterLink {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let (link, line, ring) = (t.link, t.fg_3, t.brand);
        div()
            .relative()
            .mx(px(8.))
            .when(!self.first, |el| {
                el.child(
                    div()
                        .absolute()
                        .left(px(-8.))
                        .top(px(14. * 0.36))
                        .bottom(px(14. * 0.36))
                        .w(px(0.5))
                        .bg(line),
                )
            })
            .child(
                div()
                    .id(self.id)
                    .role(Role::Link)
                    .aria_label(self.text.clone())
                    .relative()
                    .text_size(px(14.))
                    .text_color(link)
                    .rounded(px(2.))
                    .focusable()
                    .tab_stop(true)
                    .active(|s| s.opacity(0.5))
                    .child(self.text)
                    .focus_visible(move |s| s.bg(ring.opacity(0.15)))
                    .when_some(self.on_click, |el, h| el.on_click(h)),
            )
    }
}

#[derive(IntoElement)]
pub struct Footer {
    links: Vec<FooterLink>,
    texts: Vec<AnyElement>,
    extra: Vec<AnyElement>,
    fixed_bottom: bool,
}

impl Default for Footer {
    fn default() -> Self {
        Self::new()
    }
}

impl Footer {
    pub fn new() -> Self {
        Self {
            links: vec![],
            texts: vec![],
            extra: vec![],
            fixed_bottom: false,
        }
    }
    pub fn link(mut self, mut link: FooterLink) -> Self {
        link.first = self.links.is_empty();
        self.links.push(link);
        self
    }
    pub fn text(mut self, text: impl IntoElement) -> Self {
        self.texts.push(text.into_any_element());
        self
    }
    pub fn fixed_bottom(mut self) -> Self {
        self.fixed_bottom = true;
        self
    }
}

impl ParentElement for Footer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.extra.extend(elements);
    }
}

pub fn footer_color(cx: &App) -> Hsla {
    if Theme::get(cx).is_dark {
        rgba(0xffffff33).into()
    } else {
        rgba(0x00000033).into()
    }
}

impl RenderOnce for Footer {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = footer_color(cx);
        let fixed = self.fixed_bottom;
        div()
            .flex()
            .flex_col()
            .items_center()
            .text_center()
            .text_color(color)
            .text_size(px(14.))
            .line_height(relative(1.4))
            .when(fixed, |el| {
                el.absolute().bottom_0().left_0().right_0().py(px(16.))
            })
            .when(!self.links.is_empty(), |el| {
                el.child(div().flex().justify_center().children(self.links))
            })
            .children(
                self.texts
                    .into_iter()
                    .map(|t| div().px(px(16.)).text_size(px(12.)).child(t)),
            )
            .children(self.extra)
    }
}
