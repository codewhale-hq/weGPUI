use crate::Theme;
use gpui::{prelude::*, *};
use std::time::Duration;

#[derive(IntoElement)]
pub struct Loading {
    size: f32,
    color: Option<Hsla>,
}
impl Default for Loading {
    fn default() -> Self {
        Self::new()
    }
}
impl Loading {
    pub fn new() -> Self {
        Self {
            size: 16.,
            color: None,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}
impl RenderOnce for Loading {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Theme::get(cx).brand);
        svg()
            .path("loading.svg")
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(color)
            .with_animation(
                "loading",
                Animation::new(Duration::from_secs(1))
                    .repeat()
                    .with_max_fps(60.)
                    .with_easing(|phase| (phase * 60.).floor() / 60.),
                |icon, delta| icon.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}
#[derive(IntoElement)]
pub struct LoadMore {
    text: SharedString,
    loading: bool,
    dot: bool,
}
impl LoadMore {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            loading: true,
            dot: false,
        }
    }
    pub fn finished(mut self) -> Self {
        self.loading = false;
        self
    }
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self.loading = false;
        self
    }
}
impl RenderOnce for LoadMore {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        div()
            .w(relative(0.65))
            .mx_auto()
            .my(px(20.))
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(22.4))
            .text_color(t.fg_1)
            .when(self.loading, |el| el.child(Loading::new().color(t.fg_1)))
            .when(!self.loading, |el| {
                el.child(div().flex_1().h(px(0.5)).bg(t.fg_3))
            })
            .when(self.dot, |el| {
                el.child(div().size(px(4.)).rounded_full().bg(t.fg_3))
            })
            .when(!self.dot, |el| el.child(self.text))
            .when(!self.loading, |el| {
                el.child(div().flex_1().h(px(0.5)).bg(t.fg_3))
            })
    }
}
