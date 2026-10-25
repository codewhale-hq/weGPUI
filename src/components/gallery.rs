use super::{
    ClickHandler, Icon,
    modal::{MaskKind, Modal, Motion, Placement, Presence},
};
use crate::Theme;
use gpui::{prelude::*, *};
use std::{rc::Rc, time::Duration};

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub const GALLERY_FADE: Duration = Duration::from_millis(100);

pub fn gallery_presence(cx: &mut App) -> Presence {
    Presence::new(cx).duration(GALLERY_FADE)
}

#[derive(IntoElement)]
pub struct Gallery {
    id: ElementId,
    presence: Presence,
    src: Option<ImageSource>,
    label: SharedString,
    delete_label: SharedString,
    on_delete: Option<ClickHandler>,
    on_dismiss: Option<DismissHandler>,
}

impl Gallery {
    pub fn new(id: impl Into<ElementId>, presence: &Presence) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            src: None,
            label: "大图".into(),
            delete_label: "删除".into(),
            on_delete: None,
            on_dismiss: None,
        }
    }
    pub fn src(mut self, src: impl Into<ImageSource>) -> Self {
        self.src = Some(src.into());
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn delete_label(mut self, label: impl Into<SharedString>) -> Self {
        self.delete_label = label.into();
        self
    }
    pub fn on_delete(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_delete = Some(Box::new(handler));
        self
    }
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Gallery {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let white = t.white;
        let has_opr = self.on_delete.is_some();
        let dismiss_click = self.on_dismiss.clone();
        let image = div()
            .id("gallery-img")
            .role(Role::Image)
            .aria_label(self.label.clone())
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom(px(if has_opr { 60. } else { 0. }))
            .when_some(self.src, |el, src| {
                el.child(img(src).size_full().object_fit(ObjectFit::Contain))
            })
            .on_click(move |_, window, cx| {
                cx.stop_propagation();
                if let Some(handler) = dismiss_click.as_ref() {
                    handler(window, cx);
                }
            });
        let opr = self.on_delete.map(|handler| {
            div()
                .absolute()
                .left_0()
                .right_0()
                .bottom_0()
                .h(px(60.))
                .bg(rgb(0x0d0d0d))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .id("gallery-del")
                        .relative()
                        .top(px(2.))
                        .role(Role::Button)
                        .aria_label(self.delete_label)
                        .p(px(8.))
                        .rounded(px(8.))
                        .border_2()
                        .border_color(transparent_black())
                        .cursor_pointer()
                        .focusable()
                        .tab_stop(true)
                        .active(|s| s.opacity(0.5))
                        .focus_visible(move |s| s.border_color(white))
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx)
                        })
                        .child(Icon::new("delete").size(24.).color(white)),
                )
        });
        let dismiss = self.on_dismiss.clone();
        Modal::new(self.id, &self.presence)
            .mask(MaskKind::None)
            .placement(Placement::Fill)
            .motion(Motion::Fade)
            .dismiss_on_mask(false)
            .label(self.label)
            .on_dismiss(move |window, cx| {
                if let Some(handler) = dismiss.as_ref() {
                    handler(window, cx);
                }
            })
            .child(
                div()
                    .relative()
                    .size_full()
                    .bg(black())
                    .child(image)
                    .children(opr),
            )
    }
}
