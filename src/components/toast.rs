use super::{
    Icon,
    loading::Loading,
    modal::{MaskKind, Modal, Motion, Placement, Presence},
};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::time::Duration;

pub const TOAST_DURATION: Duration = Duration::from_secs(2);
pub const TOAST_FADE: Duration = Duration::from_millis(100);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastKind {
    #[default]
    Success,
    Warn,
    Loading,
    Text,
}

#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    text: SharedString,
    kind: ToastKind,
    standalone: bool,
    presence: Option<Presence>,
}

impl Toast {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Name("weui-toast".into()),
            text: text.into(),
            kind: ToastKind::Success,
            standalone: false,
            presence: None,
        }
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn success(self) -> Self {
        self.kind(ToastKind::Success)
    }
    pub fn warn(self) -> Self {
        self.kind(ToastKind::Warn)
    }
    pub fn loading(self) -> Self {
        self.kind(ToastKind::Loading)
    }
    pub fn text_only(self) -> Self {
        self.kind(ToastKind::Text)
    }
    pub fn standalone(mut self) -> Self {
        self.standalone = true;
        self
    }
    pub fn presence(mut self, presence: &Presence) -> Self {
        self.presence = Some(presence.clone());
        self
    }
    pub fn presence_for(cx: &mut App) -> Presence {
        Presence::new(cx).duration(TOAST_FADE)
    }
}

impl RenderOnce for Toast {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let fg: Hsla = rgba(0xffffffe6).into();
        let text_kind = self.kind == ToastKind::Text;
        let body = div()
            .id(self.id.clone())
            .role(if self.kind == ToastKind::Loading {
                Role::Status
            } else {
                Role::Alert
            })
            .aria_label(self.text.clone())
            .min_w(px(if text_kind { 0. } else { 132. }))
            .max_w(px(320.))
            .px(px(20.))
            .py(px(if text_kind { 12. } else { 28. }))
            .rounded(px(8.))
            .bg(t.bg_4)
            .text_color(fg)
            .line_height(relative(1.4))
            .shadow(vec![BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(8.)),
                blur_radius: px(12.5),
                spread_radius: px(0.),
                inset: false,
            }])
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .text_center()
            .map(|el| match self.kind {
                ToastKind::Success => el.child(
                    div()
                        .mb(px(16.))
                        .child(Icon::new("success-no-circle").size(40.).color(fg)),
                ),
                ToastKind::Warn => el.child(
                    div()
                        .mb(px(16.))
                        .child(Icon::new("warn").size(40.).color(fg)),
                ),
                ToastKind::Loading => el.child(
                    div().mb(px(16.)).child(
                        Loading::primary()
                            .id(ElementId::Name("weui-toast-loading".into()))
                            .transparent()
                            .size(40.),
                    ),
                ),
                ToastKind::Text => el,
            })
            .child(
                div()
                    .max_w_full()
                    .text_size(px(14.))
                    .line_height(relative(1.4))
                    .child(self.text.clone()),
            );
        let body = Semantics::new(body).busy(self.kind == ToastKind::Loading);
        match self.presence {
            Some(presence) => {
                let viewport = window.viewport_size();
                let positioned = if self.standalone {
                    div().relative().top(viewport.height * -0.1)
                } else {
                    div().mt(viewport.width * -0.1)
                };
                Modal::new(self.id, &presence)
                    .mask(MaskKind::Transparent)
                    .placement(Placement::Center)
                    .motion(Motion::Fade)
                    .dismiss_on_mask(false)
                    .label(self.text)
                    .child(positioned.child(body))
                    .into_any_element()
            }
            None => body.into_any_element(),
        }
    }
}
