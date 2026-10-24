use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Panel {
    id: ElementId,
    title: Option<SharedString>,
    footer: Option<AnyElement>,
    margin_top: f32,
    children: Vec<AnyElement>,
}

impl Panel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            footer: None,
            margin_top: 10.,
            children: vec![],
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    pub fn first(mut self) -> Self {
        self.margin_top = 0.;
        self
    }
    pub fn margin_top(mut self, margin: f32) -> Self {
        self.margin_top = margin;
        self
    }
}

impl ParentElement for Panel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

impl RenderOnce for Panel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let line = t.fg_3;
        let rows = self.children.into_iter().enumerate().map(|(i, child)| {
            div().relative().child(child).when(i > 0, |el| {
                el.child(hairline(line).top_0().left(px(16.)).right_0())
            })
        });
        div()
            .id(self.id)
            .role(Role::Group)
            .when_some(self.title.clone(), |el, title| el.aria_label(title))
            .relative()
            .mt(px(self.margin_top))
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(t.bg_2)
            .when_some(self.title, |el, title| {
                el.child(
                    div()
                        .relative()
                        .pt(px(16.))
                        .px(px(16.))
                        .pb(px(13.))
                        .text_size(px(15.))
                        .line_height(px(24.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(t.fg_0)
                        .child(title)
                        .child(hairline(line).bottom_0().left(px(15.)).right_0()),
                )
            })
            .child(div().flex().flex_col().children(rows))
            .when_some(self.footer, |el, footer| {
                el.child(
                    div()
                        .relative()
                        .child(footer)
                        .child(hairline(line).top_0().left(px(16.)).right_0()),
                )
            })
            .child(hairline(line).top_0().left_0().right_0())
            .child(hairline(line).bottom_0().left_0().right_0())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MediaBoxKind {
    #[default]
    Text,
    Appmsg,
    SmallAppmsg,
}

pub struct MediaMeta {
    text: SharedString,
    extra: bool,
}

#[derive(IntoElement)]
pub struct MediaBox {
    id: ElementId,
    kind: MediaBoxKind,
    title: Option<SharedString>,
    desc: Option<SharedString>,
    thumb: Option<SharedString>,
    meta: Vec<MediaMeta>,
    label: Option<SharedString>,
    description: Option<SharedString>,
    role: Option<Role>,
    on_click: Option<ClickHandler>,
    children: Vec<AnyElement>,
}

impl MediaBox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            kind: MediaBoxKind::Text,
            title: None,
            desc: None,
            thumb: None,
            meta: vec![],
            label: None,
            description: None,
            role: None,
            on_click: None,
            children: vec![],
        }
    }
    pub fn text(id: impl Into<ElementId>) -> Self {
        Self::new(id)
    }
    pub fn appmsg(id: impl Into<ElementId>, thumb: impl Into<SharedString>) -> Self {
        let mut this = Self::new(id);
        this.kind = MediaBoxKind::Appmsg;
        this.thumb = Some(thumb.into());
        this
    }
    pub fn small_appmsg(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id);
        this.kind = MediaBoxKind::SmallAppmsg;
        this
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta.push(MediaMeta {
            text: meta.into(),
            extra: false,
        });
        self
    }
    pub fn meta_extra(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta.push(MediaMeta {
            text: meta.into(),
            extra: true,
        });
        self
    }
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
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

impl ParentElement for MediaBox {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MediaBox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let label = self.label.clone().or_else(|| {
            let parts: Vec<&str> = [self.title.as_deref(), self.desc.as_deref()]
                .into_iter()
                .flatten()
                .collect();
            (!parts.is_empty()).then(|| SharedString::from(parts.join("，")))
        });
        let interactive = self.on_click.is_some();
        let role = self.role.unwrap_or(if interactive {
            Role::Link
        } else {
            Role::ListItem
        });
        let title = self.title.map(|title| {
            div()
                .w_full()
                .text_size(px(17.))
                .line_height(relative(1.4))
                .text_color(t.fg_0)
                .truncate()
                .child(title)
        });
        let desc = self.desc.map(|desc| {
            div()
                .pt(px(4.))
                .text_size(px(14.))
                .line_height(relative(1.4))
                .text_color(t.fg_2)
                .overflow_hidden()
                .text_ellipsis()
                .line_clamp(2)
                .child(desc)
        });
        let meta_color = t.fg_2;
        let info = (!self.meta.is_empty()).then(|| {
            div()
                .mt(px(16.))
                .pb(px(4.))
                .flex()
                .text_size(px(13.))
                .line_height(px(13.))
                .text_color(meta_color)
                .children(self.meta.into_iter().map(move |m| {
                    div()
                        .pr(px(13.))
                        .when(m.extra, |el| {
                            el.pl(px(13.)).border_l_1().border_color(meta_color)
                        })
                        .child(m.text)
                }))
        });
        let body = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .children(title)
            .children(desc)
            .children(info);
        let mut el = div()
            .id(self.id)
            .role(role)
            .when_some(label, |el, label| el.aria_label(label))
            .when_some(self.description, |el, d| el.aria_description(d))
            .relative()
            .border_2()
            .border_color(transparent_black());
        el = match self.kind {
            MediaBoxKind::Text => el.p(px(14.)).child(body),
            MediaBoxKind::Appmsg => el
                .p(px(14.))
                .flex()
                .items_center()
                .when_some(self.thumb, |el, thumb| {
                    el.child(
                        div()
                            .mr(px(16.))
                            .size(px(60.))
                            .flex_shrink_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(img(thumb).size(px(60.)).object_fit(ObjectFit::Contain)),
                    )
                })
                .child(body),
            MediaBoxKind::SmallAppmsg => el.border_0(),
        };
        el = el.children(self.children);
        if let Some(handler) = self.on_click {
            let pressed = t.bg_color_active;
            let brand = t.brand;
            el = el
                .cursor_pointer()
                .focusable()
                .tab_stop(true)
                .active(move |s| s.bg(pressed))
                .focus_visible(move |s| s.border_color(brand))
                .on_click(handler);
        }
        el
    }
}
