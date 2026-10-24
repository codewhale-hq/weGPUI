use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Icon {
    name: SharedString,
    w: f32,
    h: f32,
    color: Option<Hsla>,
}
impl Icon {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            w: 24.,
            h: 24.,
            color: None,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.w = size;
        self.h = size;
        self
    }
    pub fn w(mut self, w: f32) -> Self {
        self.w = w;
        self
    }
    pub fn h(mut self, h: f32) -> Self {
        self.h = h;
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}
impl RenderOnce for Icon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        svg()
            .path(SharedString::from(format!("icon-{}.svg", self.name)))
            .w(px(self.w))
            .h(px(self.h))
            .flex_shrink_0()
            .text_color(self.color.unwrap_or(Theme::get(cx).fg_0))
    }
}
