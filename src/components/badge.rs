//! Badges from WeUI's `src/style/widget/weui-tips/weui-badge.less`: [`Badge`] draws a red pill with
//! short text or a count, or a small dot (`weui-badge_dot`).
//!
//! Known limitations: a badge sets no role or accessible label, so describe it on the owning cell
//! or link (upstream uses labels such as ", 8 new notifications"). Counts are not capped (no "99+").

use crate::Theme;
use gpui::{prelude::*, *};
/// A WeUI badge: white 12px text on the theme red with an 18px corner radius, at least 17.6px wide.
#[derive(IntoElement)]
pub struct Badge {
    text: Option<SharedString>,
}
impl Badge {
    /// Creates a text badge showing `text`, for example "New".
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
        }
    }
    /// Creates a badge showing `value` in decimal.
    pub fn count(value: usize) -> Self {
        Self::new(value.to_string())
    }
    /// Creates a text-less 9.6px dot badge (`weui-badge_dot`).
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
