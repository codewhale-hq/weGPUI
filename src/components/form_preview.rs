use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FormPreviewButtonKind {
    #[default]
    Primary,
    Default,
}

pub struct FormPreviewButton {
    label: SharedString,
    kind: FormPreviewButtonKind,
    on_click: Option<ClickHandler>,
}

impl FormPreviewButton {
    pub fn primary(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: FormPreviewButtonKind::Primary,
            on_click: None,
        }
    }
    pub fn default(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: FormPreviewButtonKind::Default,
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

#[derive(IntoElement)]
pub struct FormPreview {
    id: ElementId,
    header: Option<(SharedString, SharedString)>,
    items: Vec<(SharedString, SharedString)>,
    buttons: Vec<FormPreviewButton>,
}

impl FormPreview {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            items: vec![],
            buttons: vec![],
        }
    }
    pub fn header(
        mut self,
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        self.header = Some((label.into(), value.into()));
        self
    }
    pub fn item(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.items.push((label.into(), value.into()));
        self
    }
    pub fn button(mut self, button: FormPreviewButton) -> Self {
        self.buttons.push(button);
        self
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

fn preview_item(
    label: SharedString,
    value: impl IntoElement,
    label_width: f32,
    gap: f32,
    label_color: Hsla,
) -> Div {
    div()
        .flex()
        .items_start()
        .child(
            div()
                .flex_shrink_0()
                .w(px(label_width))
                .mr(px(gap))
                .text_left()
                .text_color(label_color)
                .child(label),
        )
        .child(div().flex_1().min_w_0().child(value))
}

impl RenderOnce for FormPreview {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let id = self.id.clone();
        let header = self.header.map(|(label, value)| {
            let aria = SharedString::from(format!("{label} {value}"));
            div()
                .id("form-preview-hd")
                .role(Role::ListItem)
                .aria_label(aria)
                .relative()
                .p(px(16.))
                .text_size(px(16.))
                .line_height(px(40.))
                .child(preview_item(
                    label,
                    div()
                        .text_right()
                        .text_size(px(25.6))
                        .text_color(t.fg_0)
                        .child(value),
                    67.2,
                    16.,
                    t.fg_1,
                ))
                .child(hairline(t.fg_3).bottom_0().left(px(16.)).right_0())
        });
        let body_label = SharedString::from(
            self.items
                .iter()
                .map(|(l, v)| format!("{l} {v}"))
                .collect::<Vec<_>>()
                .join("，"),
        );
        let fg0 = t.fg_0;
        let fg1 = t.fg_1;
        let body = (!self.items.is_empty()).then(|| {
            div()
                .id("form-preview-bd")
                .role(Role::ListItem)
                .aria_label(body_label)
                .p(px(16.))
                .text_size(px(14.4))
                .line_height(px(28.8))
                .text_color(fg1)
                .children(self.items.into_iter().map(move |(label, value)| {
                    preview_item(
                        label,
                        div().text_right().text_color(fg0).child(value),
                        60.48,
                        14.4,
                        fg1,
                    )
                }))
        });
        let line = t.dialog_line_color;
        let pressed = t.bg_color_active;
        let brand = t.brand;
        let footer = (!self.buttons.is_empty()).then(|| {
            div()
                .relative()
                .flex()
                .text_size(px(16.))
                .line_height(px(50.))
                .children(self.buttons.into_iter().enumerate().map(|(i, b)| {
                    let color = match b.kind {
                        FormPreviewButtonKind::Primary => t.link,
                        FormPreviewButtonKind::Default => t.fg_half,
                    };
                    div()
                        .id(ElementId::NamedInteger("form-preview-btn".into(), i as u64))
                        .role(Role::Button)
                        .aria_label(b.label.clone())
                        .relative()
                        .flex_1()
                        .text_center()
                        .text_color(color)
                        .border_2()
                        .border_color(transparent_black())
                        .my(px(-2.))
                        .cursor_pointer()
                        .focusable()
                        .tab_stop(true)
                        .active(move |s| s.bg(pressed))
                        .focus_visible(move |s| s.border_color(brand))
                        .when_some(b.on_click, |el, h| el.on_click(h))
                        .child(b.label)
                        .when(i > 0, |el| {
                            el.child(
                                div()
                                    .absolute()
                                    .left(px(-2.))
                                    .top(px(-2.))
                                    .bottom(px(-2.))
                                    .w(px(0.5))
                                    .bg(line),
                            )
                        })
                }))
                .child(hairline(line).top_0().left_0().right_0())
        });
        div()
            .id(id)
            .relative()
            .flex()
            .flex_col()
            .bg(t.bg_2)
            .children(header)
            .children(body)
            .children(footer)
            .child(hairline(t.fg_3).top_0().left_0().right_0())
            .child(hairline(t.fg_3).bottom_0().left_0().right_0())
    }
}

#[derive(IntoElement)]
pub struct FormPreviewList {
    items: Vec<(SharedString, SharedString)>,
    last: bool,
}

impl Default for FormPreviewList {
    fn default() -> Self {
        Self::new()
    }
}

impl FormPreviewList {
    pub fn new() -> Self {
        Self {
            items: vec![],
            last: true,
        }
    }
    pub fn item(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.items.push((label.into(), value.into()));
        self
    }
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }
}

impl RenderOnce for FormPreviewList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let (fg0, fg1) = (t.fg_0, t.fg_1);
        div()
            .relative()
            .pt(px(24.))
            .when(!self.last, |el| el.pb(px(24.)))
            .flex()
            .flex_col()
            .gap(px(12.))
            .text_size(px(14.))
            .line_height(relative(1.4))
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(move |(i, (label, value))| {
                        div()
                            .id(ElementId::NamedInteger(
                                "form-preview-list-item".into(),
                                i as u64,
                            ))
                            .role(Role::ListItem)
                            .aria_label(SharedString::from(format!("{label} {value}")))
                            .child(preview_item(
                                label,
                                div().text_color(fg0).child(value),
                                84.,
                                14.,
                                fg1,
                            ))
                    }),
            )
            .child(hairline(t.fg_3).top_0().left_0().right_0())
    }
}
