use super::Icon;
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

type LinkHandler = Rc<dyn Fn(&mut Window, &mut App)>;

enum Segment {
    Text(SharedString),
    Link(SharedString, Option<LinkHandler>),
}

#[derive(IntoElement)]
pub struct LinkText {
    id: ElementId,
    segments: Vec<Segment>,
}

impl LinkText {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            segments: vec![],
        }
    }
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.segments.push(Segment::Text(text.into()));
        self
    }
    pub fn link(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.segments
            .push(Segment::Link(label.into(), Some(Rc::new(handler))));
        self
    }
    pub fn inert_link(mut self, label: impl Into<SharedString>) -> Self {
        self.segments.push(Segment::Link(label.into(), None));
        self
    }
}

impl RenderOnce for LinkText {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let mut full = String::new();
        let mut ranges = vec![];
        let mut highlights = vec![];
        let mut handlers: Vec<Option<LinkHandler>> = vec![];
        let mut first_link: Option<SharedString> = None;
        for segment in self.segments {
            match segment {
                Segment::Text(text) => full.push_str(&text),
                Segment::Link(label, handler) => {
                    let start = full.len();
                    full.push_str(&label);
                    let range = start..full.len();
                    highlights.push((
                        range.clone(),
                        HighlightStyle {
                            color: Some(t.link),
                            ..Default::default()
                        },
                    ));
                    if first_link.is_none() && handler.is_some() {
                        first_link = Some(label.clone());
                    }
                    ranges.push(range);
                    handlers.push(handler);
                }
            }
        }
        let full = SharedString::from(full);
        let handlers = Rc::new(handlers);
        let click_handlers = handlers.clone();
        let text = InteractiveText::new(
            "link-text",
            StyledText::new(full.clone()).with_highlights(highlights),
        )
        .on_click(ranges, move |ix, window, cx| {
            if let Some(Some(handler)) = click_handlers.get(ix) {
                handler(window, cx);
            }
        });
        let brand = t.brand;
        let key_handler = handlers.iter().flatten().next().cloned();
        div()
            .id(self.id)
            .when_some(first_link.clone(), |el, label| {
                el.role(Role::Link)
                    .aria_label(label)
                    .aria_description(full.clone())
                    .rounded(px(4.))
                    .focusable()
                    .tab_stop(true)
                    .focus_visible(move |s| s.bg(brand.opacity(0.15)))
            })
            .when(first_link.is_none(), |el| {
                el.role(Role::Paragraph).aria_label(full.clone())
            })
            .when_some(key_handler, |el, handler| {
                el.on_key_down(move |event, window, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        cx.stop_propagation();
                        handler(window, cx);
                    }
                })
            })
            .child(text)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MsgIcon {
    Success,
    Warn,
    WarnPrimary,
    Info,
    Waiting,
}

enum Area {
    Icon(AnyElement),
    Text(Vec<(TextKind, AnyElement)>),
    Opr(AnyElement),
    Tips(AnyElement),
    Extra(AnyElement),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextKind {
    Title,
    Desc,
    DescPrimary,
    Custom,
}

#[derive(IntoElement)]
pub struct Msg {
    id: ElementId,
    areas: Vec<Area>,
    align_top: bool,
}

impl Msg {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            areas: vec![],
            align_top: false,
        }
    }
    pub fn icon(self, icon: MsgIcon) -> Self {
        let (name, primary) = match icon {
            MsgIcon::Success => ("success", false),
            MsgIcon::Warn => ("warn", false),
            MsgIcon::WarnPrimary => ("warn", true),
            MsgIcon::Info => ("info-circle", false),
            MsgIcon::Waiting => ("waiting", false),
        };
        self.icon_element(MsgIconView { name, primary })
    }
    pub fn icon_element(mut self, icon: impl IntoElement) -> Self {
        self.areas.push(Area::Icon(icon.into_any_element()));
        self
    }
    fn push_text(mut self, kind: TextKind, el: AnyElement) -> Self {
        if let Some(Area::Text(items)) = self.areas.last_mut() {
            items.push((kind, el));
        } else {
            self.areas.push(Area::Text(vec![(kind, el)]));
        }
        self
    }
    pub fn title(self, title: impl Into<SharedString>) -> Self {
        let title: SharedString = title.into();
        self.push_text(TextKind::Title, title.into_any_element())
    }
    pub fn desc(self, desc: impl IntoElement) -> Self {
        self.push_text(TextKind::Desc, desc.into_any_element())
    }
    pub fn desc_primary(self, desc: impl IntoElement) -> Self {
        self.push_text(TextKind::DescPrimary, desc.into_any_element())
    }
    pub fn custom_area(self, custom: impl IntoElement) -> Self {
        self.push_text(TextKind::Custom, custom.into_any_element())
    }
    pub fn opr(mut self, opr: impl IntoElement) -> Self {
        self.areas.push(Area::Opr(opr.into_any_element()));
        self
    }
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.areas.push(Area::Tips(tips.into_any_element()));
        self
    }
    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.areas.push(Area::Extra(extra.into_any_element()));
        self
    }
    pub fn align_top(mut self, align_top: bool) -> Self {
        self.align_top = align_top;
        self
    }
}

#[derive(IntoElement)]
struct MsgIconView {
    name: &'static str,
    primary: bool,
}

impl RenderOnce for MsgIconView {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let color = match (self.name, self.primary) {
            ("success", _) => t.brand,
            ("warn", true) => t.yellow,
            ("warn", false) => t.red,
            _ => t.blue,
        };
        div()
            .id("msg-icon")
            .role(Role::Image)
            .aria_label(match (self.name, self.primary) {
                ("success", _) => "成功",
                ("warn", _) => "警告",
                ("waiting", _) => "等待",
                _ => "提示",
            })
            .flex()
            .justify_center()
            .child(Icon::new(self.name).size(64.).color(color))
    }
}

impl RenderOnce for Msg {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let n = self.areas.len();
        let kinds: Vec<u8> = self
            .areas
            .iter()
            .map(|a| match a {
                Area::Icon(_) => 0,
                Area::Text(_) => 1,
                Area::Opr(_) => 2,
                Area::Tips(_) => 3,
                Area::Extra(_) => 4,
            })
            .collect();
        let align_top = self.align_top;
        let areas = self.areas.into_iter().enumerate().map(|(i, area)| {
            let last = i + 1 == n;
            let prev = i.checked_sub(1).map(|p| kinds[p]);
            match area {
                Area::Icon(icon) => div().mb(px(32.)).child(icon),
                Area::Text(items) => {
                    let mut prev_kind: Option<TextKind> = None;
                    let children: Vec<AnyElement> = items
                        .into_iter()
                        .map(|(kind, el)| {
                            let wrapper = match kind {
                                TextKind::Title => div()
                                    .mb(px(16.))
                                    .text_size(px(22.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(t.fg_0),
                                TextKind::Desc => {
                                    div().mb(px(16.)).text_size(px(17.)).text_color(t.fg_0)
                                }
                                TextKind::DescPrimary => {
                                    div().mb(px(16.)).text_size(px(14.)).text_color(t.fg_1)
                                }
                                TextKind::Custom => div()
                                    .flex()
                                    .flex_col()
                                    .mb(px(16.))
                                    .text_left()
                                    .when(prev_kind == Some(TextKind::Title), |el| el.mt(px(32.)))
                                    .when(
                                        matches!(
                                            prev_kind,
                                            Some(TextKind::Desc | TextKind::DescPrimary)
                                        ),
                                        |el| el.mt(px(24.)),
                                    ),
                            };
                            prev_kind = Some(kind);
                            wrapper.child(el).into_any_element()
                        })
                        .collect();
                    div()
                        .flex_1()
                        .mb(px(32.))
                        .px(px(32.))
                        .line_height(relative(1.6))
                        .when(i == 0 && !align_top, |el| el.pt(px(96.)))
                        .flex()
                        .flex_col()
                        .children(children)
                }
                Area::Opr(opr) => div()
                    .mb(px(if last { 96. } else { 16. }))
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(opr),
                Area::Tips(tips) => div()
                    .mb(px(if last {
                        64.
                    } else if prev == Some(2) {
                        48.
                    } else {
                        16.
                    }))
                    .px(px(40.))
                    .text_size(px(14.))
                    .text_color(t.fg_1)
                    .child(tips),
                Area::Extra(extra) => div()
                    .when(prev == Some(2), |el| el.mt(px(48.)))
                    .mb(px(24.))
                    .px(px(32.))
                    .text_size(px(12.))
                    .text_color(t.fg_1)
                    .child(extra),
            }
        });
        div()
            .id(self.id)
            .min_h_full()
            .flex_grow(1.)
            .w_full()
            .flex()
            .flex_col()
            .pt(px(48.))
            .text_center()
            .line_height(relative(1.4))
            .bg(t.bg_2)
            .children(areas)
    }
}
