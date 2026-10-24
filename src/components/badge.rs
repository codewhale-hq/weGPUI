use crate::Theme;
use gpui::{prelude::*, *};
#[derive(IntoElement)]
pub struct Badge {
    text: Option<SharedString>,
}
impl Badge {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
        }
    }
    pub fn count(value: usize) -> Self {
        Self::new(value.to_string())
    }
    pub fn dot() -> Self {
        Self { text: None }
    }
}
impl RenderOnce for Badge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let base = div()
            .rounded(px(18.))
            .bg(Theme::get(cx).red)
            .text_color(white())
            .flex_shrink_0();
        match self.text {
            Some(text) => base
                .min_w(px(17.6))
                .px(px(4.8))
                .py(px(1.8))
                .text_size(px(12.))
                .line_height(px(14.4))
                .text_center()
                .child(text),
            None => base.size(px(9.6)),
        }
    }
}
