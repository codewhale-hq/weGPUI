//! WeUI gallery image viewer (`weui-cell/weui-gallery.less`): a full-window black overlay showing
//! one image, with an optional delete bar (`weui-gallery__opr`).
//!
//! [`Gallery`] renders in a [`Modal`] driven by a caller-owned [`Presence`], created with
//! [`gallery_presence`]. Known limitations: no pinch, zoom, swipe or multi-image paging, and the
//! image is not focusable, so keyboard users close the viewer with Escape.

use super::{
    ClickHandler, Icon,
    modal::{MaskKind, Modal, Motion, Placement, Presence},
};
use crate::Theme;
use gpui::{prelude::*, *};
use std::{rc::Rc, time::Duration};

type DismissHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// Viewer fade length, matching the upstream example's `fadeIn(100)`.
pub const GALLERY_FADE: Duration = Duration::from_millis(100);

/// Creates a [`Presence`] with the [`GALLERY_FADE`] transition.
pub fn gallery_presence(cx: &mut App) -> Presence {
    Presence::new(cx).duration(GALLERY_FADE)
}

/// Full-window image viewer exposed as a modal `Dialog`. Clicking the image area or pressing Escape
/// calls `on_dismiss`.
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
    /// Creates a viewer for `presence` (cloned) with no image, label "大图" and no delete bar.
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
    /// Sets the image, any GPUI `ImageSource` (asset name, URI or path), scaled to fit. Without one
    /// only the black background is drawn.
    pub fn src(mut self, src: impl Into<ImageSource>) -> Self {
        self.src = Some(src.into());
        self
    }
    /// Sets the accessible name of the image and the dialog (default "大图").
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Sets the delete button's accessible label (default "删除").
    pub fn delete_label(mut self, label: impl Into<SharedString>) -> Self {
        self.delete_label = label.into();
        self
    }
    /// Shows the 60 px bottom bar with a focusable delete button; the handler receives its click
    /// event. Without it there is no bar.
    pub fn on_delete(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_delete = Some(Box::new(handler));
        self
    }
    /// Called on an image click or Escape; hide the presence here.
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
