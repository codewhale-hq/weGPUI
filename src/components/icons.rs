use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Icon {
    name: SharedString,
    size: f32,
    color: Option<Hsla>,
}
impl Icon {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            size: 24.,
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
impl RenderOnce for Icon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        svg()
            .path(SharedString::from(format!("icon-{}.svg", self.name)))
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(self.color.unwrap_or(Theme::get(cx).fg_0))
    }
}
