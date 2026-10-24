use super::{Icon, Loading};
use crate::Theme;
use gpui::{prelude::*, *};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Loading,
    Text,
}
#[derive(Clone, IntoElement)]
pub struct Toast {
    text: SharedString,
    kind: ToastKind,
}
impl Toast {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            kind: ToastKind::Success,
        }
    }
    pub fn loading(mut self) -> Self {
        self.kind = ToastKind::Loading;
        self
    }
    pub fn text_only(mut self) -> Self {
        self.kind = ToastKind::Text;
        self
    }
}
impl RenderOnce for Toast {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let fg = white().opacity(0.9);
        div()
            .id("toast")
            .role(Role::Status)
            .aria_label(self.text.clone())
            .min_w(px(if self.kind == ToastKind::Text {
                0.
            } else {
                132.
            }))
            .max_w(px(320.))
            .px(px(20.))
            .py(px(if self.kind == ToastKind::Text {
                12.
            } else {
                28.
            }))
            .rounded(px(8.))
            .bg(Theme::get(cx).bg_4)
            .text_color(fg)
            .shadow_lg()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .text_center()
            .when(self.kind == ToastKind::Success, |el| {
                el.child(
                    div()
                        .mb(px(16.))
                        .child(Icon::new("success-no-circle").size(64.).color(fg)),
                )
            })
            .when(self.kind == ToastKind::Loading, |el| {
                el.child(
                    div()
                        .mb(px(16.))
                        .child(Loading::new().size(40.).color(rgb(0xededed))),
                )
            })
            .child(
                div()
                    .text_size(px(14.))
                    .line_height(px(19.6))
                    .child(self.text),
            )
    }
}
