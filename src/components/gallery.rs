//! WeUI gallery image viewer (`weui-cell/weui-gallery.less`): a full-window black overlay showing
//! one image, with an optional delete bar (`weui-gallery__opr`).
//!
//! [`Gallery`] renders in a [`Modal`] driven by a caller-owned [`Presence`], created with
//! [`gallery_presence`]. The image is a keyboard stop and Enter/Space dismiss it; Escape also
//! dismisses. The pinned upstream viewer has no pinch, zoom, swipe or multi-image paging.

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

/// Full-window image viewer exposed as a modal `Dialog`. Clicking the image area, activating it
/// with Enter/Space, or pressing Escape calls `on_dismiss`. State remains caller-owned.
#[derive(IntoElement)]
pub struct Gallery {
    id: ElementId,
    presence: Presence,
    src: Option<ImageSource>,
    label: Option<SharedString>,
    delete_label: Option<SharedString>,
    on_delete: Option<ClickHandler>,
    on_dismiss: Option<DismissHandler>,
}

impl Gallery {
    /// Creates a viewer for `presence` (cloned) with no image, the localized
    /// [`crate::Messages::large_image`] label and no delete bar.
    pub fn new(id: impl Into<ElementId>, presence: &Presence) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            src: None,
            label: None,
            delete_label: None,
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
    /// Sets the accessible name of the image and the dialog (default: the localized
    /// [`crate::Messages::large_image`]).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the delete button's accessible label (default: the localized
    /// [`crate::Messages::delete`]).
    pub fn delete_label(mut self, label: impl Into<SharedString>) -> Self {
        self.delete_label = Some(label.into());
        self
    }
    /// Shows the 60 px bottom bar, whose full area is a focusable delete button; the handler
    /// receives its click event. Without it there is no bar.
    pub fn on_delete(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_delete = Some(Box::new(handler));
        self
    }
    /// Called on an image click, image Enter/Space activation, or Escape; hide the presence here.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Gallery {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let m = crate::i18n::messages_arc(cx);
        let label = self.label.clone().unwrap_or_else(|| m.large_image.clone());
        let delete_label = self
            .delete_label
            .clone()
            .unwrap_or_else(|| m.delete.clone());
        let white = t.white;
        let delete_focus = t.focus_color_with(white, rgb(0x0d0d0d).into());
        let has_opr = self.on_delete.is_some();
        let dismiss_click = self.on_dismiss.clone();
        let image_focus = window
            .use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "focus".into()),
                cx,
                |_, cx| cx.focus_handle().tab_stop(true),
            )
            .read(cx)
            .clone();
        let show_image_focus = image_focus.is_focused(window) && window.last_input_was_keyboard();
        let image = div()
            .id("gallery-img")
            .role(Role::Image)
            .aria_label(label.clone())
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom(px(if has_opr { 60. } else { 0. }))
            .when(self.on_dismiss.is_some(), |el| {
                el.focusable()
                    .track_focus(&image_focus)
                    .tab_stop(true)
                    .cursor_pointer()
                    .aria_description(m.close_preview_hint.clone())
            })
            .when_some(self.src, |el, src| {
                el.child(img(src).size_full().object_fit(ObjectFit::Contain))
            })
            .when(show_image_focus, |el| {
                // Paint after the image so neither band can be covered by its pixels. Both
                // bands are inset and remain visible at the window's clipped edges.
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .shadow(super::form::dual_focus_ring()),
                )
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
                        .role(Role::Button)
                        .aria_label(delete_label)
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_2()
                        .border_color(transparent_black())
                        .cursor_pointer()
                        .focusable()
                        .tab_stop(true)
                        .active(|s| s.opacity(0.5))
                        .focus_visible(move |s| s.border_color(delete_focus))
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx)
                        })
                        .child(
                            div()
                                .relative()
                                .top(px(2.))
                                .child(Icon::new("delete").size(24.).color(white)),
                        ),
                )
        });
        let dismiss = self.on_dismiss.clone();
        Modal::new(self.id, &self.presence)
            .mask(MaskKind::None)
            .placement(Placement::Fill)
            .motion(Motion::Fade)
            .dismiss_on_mask(false)
            .label(label)
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
