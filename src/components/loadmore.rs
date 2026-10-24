use super::loading::Loading;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadMoreKind {
    #[default]
    Loading,
    Line,
    Dot,
}

#[derive(IntoElement)]
pub struct LoadMore {
    id: ElementId,
    text: SharedString,
    kind: LoadMoreKind,
    margin: Option<(f32, f32)>,
}

impl LoadMore {
    pub fn new(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            id: ElementId::Name(format!("loadmore-{text}").into()),
            text,
            kind: LoadMoreKind::Loading,
            margin: None,
        }
    }
    pub fn loading(text: impl Into<SharedString>) -> Self {
        Self::new(text)
    }
    pub fn line(text: impl Into<SharedString>) -> Self {
        Self::new(text).kind(LoadMoreKind::Line)
    }
    pub fn dot(text: impl Into<SharedString>) -> Self {
        Self::new(text).kind(LoadMoreKind::Dot)
    }
    pub fn kind(mut self, kind: LoadMoreKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn finished(self) -> Self {
        self.kind(LoadMoreKind::Line)
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn margin(mut self, top: f32, bottom: f32) -> Self {
        self.margin = Some((top, bottom));
        self
    }
}

impl RenderOnce for LoadMore {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let (fg, line, bg) = (t.fg_1, t.fg_3, t.bg_2);
        let (mt, mb) = self.margin.unwrap_or(match self.kind {
            LoadMoreKind::Loading => (20., 20.),
            _ => (32., 20.),
        });
        let lh = 14. * 1.6;
        let base = div()
            .id(self.id)
            .w(relative(0.65))
            .mx_auto()
            .mt(px(mt))
            .mb(px(mb))
            .flex()
            .justify_center()
            .text_size(px(14.))
            .line_height(px(lh))
            .text_color(fg);
        match self.kind {
            LoadMoreKind::Loading => base
                .role(Role::Alert)
                .aria_label(self.text.clone())
                .items_center()
                .h(px(lh))
                .child(div().mr(px(8.)).child(Loading::primary()))
                .child(self.text),
            LoadMoreKind::Line => base
                .role(Role::Paragraph)
                .aria_label(self.text.clone())
                .h(px(lh + 1.))
                .pt(px(1.))
                .relative()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(1.))
                        .bg(line),
                )
                .child(
                    div()
                        .relative()
                        .top(px(-0.9 * 14.))
                        .px(px(8.))
                        .bg(bg)
                        .child(self.text),
                ),
            LoadMoreKind::Dot => base
                .role(Role::ListItem)
                .aria_label(self.text)
                .h(px(lh + 1.))
                .pt(px(1.))
                .relative()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(1.))
                        .bg(line),
                )
                .child(
                    div()
                        .relative()
                        .top(px(-0.9 * 14.))
                        .h(px(lh))
                        .px(px(8.))
                        .bg(bg)
                        .child(
                            div()
                                .mt(px(lh * 0.5 - 1.))
                                .size(px(4.))
                                .rounded_full()
                                .bg(line),
                        ),
                ),
        }
    }
}
