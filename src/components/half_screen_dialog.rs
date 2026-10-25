//! WeUI half-screen dialog (`weui-tips/weui-half-screen-dialog.less`): a bottom sheet with a header
//! (side actions, title and subtitle), scrolling body and footer, plus the large, slide (`grab`)
//! and bottom-fixed (`weui-bottom-fixed-opr-page`) variants.
//!
//! [`HalfScreenDialog`] renders inside a sliding [`Modal`] when given a [`Presence`]. The helpers
//! [`HdIconButton`], [`action_group`], [`hd_link`] and [`attachment_area`] build header and footer
//! parts. Known limitation: a grab drag shorter than 56 px snaps back instantly, without the
//! upstream 0.3 s transition.

use super::{
    ClickHandler, Icon,
    modal::{MaskKind, Modal, Motion, Placement, Presence},
};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Shared callback invoked when the dialog asks to be dismissed.
pub type DismissHandler = Rc<dyn Fn(&mut Window, &mut App)>;

const GRAB_DISTANCE: f32 = 56.;

/// 24 px header icon button (`weui-btn_icon` in `__hd__side`) with a focusable 44 px hit area and
/// the `Button` role.
#[derive(IntoElement)]
pub struct HdIconButton {
    id: ElementId,
    icon: SharedString,
    label: SharedString,
    on_click: Option<ClickHandler>,
}

impl HdIconButton {
    /// Creates an icon button from an icon asset name (as for [`Icon`]) and an accessible label.
    /// The `"search"` icon uses the secondary text colour, others the primary one.
    pub fn new(
        id: impl Into<ElementId>,
        icon: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            on_click: None,
        }
    }
    /// Called with the click event when the button is clicked or activated with Enter or Space.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for HdIconButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let color = if self.icon.as_ref() == "search" {
            t.fg_1
        } else {
            t.fg_0
        };
        div()
            .relative()
            .size(px(24.))
            .flex_shrink_0()
            .child(Icon::new(self.icon.clone()).size(24.).color(color))
            .child(
                div()
                    .id(self.id)
                    .role(Role::Button)
                    .aria_label(self.label)
                    .focusable()
                    .tab_stop(true)
                    .absolute()
                    .top(px(-10.))
                    .left(px(-10.))
                    .size(px(44.))
                    .rounded(px(6.))
                    .border_2()
                    .border_color(transparent_black())
                    .cursor_pointer()
                    .focus_visible(|s| s.border_color(t.brand))
                    .active(|s| s.bg(t.bg_2.opacity(0.5)))
                    .when_some(self.on_click, |el, handler| el.on_click(handler)),
            )
    }
}

/// Lays out header actions in a non-wrapping row with 16 px gaps (`__hd__action-group`).
pub fn action_group(children: impl IntoIterator<Item = AnyElement>) -> Div {
    div()
        .flex()
        .flex_shrink_0()
        .items_center()
        .gap(px(16.))
        .whitespace_nowrap()
        .children(children)
}

/// Text link for the header (`weui-link`): 15 px, link colour, `Link` role and focusable; `handler`
/// receives the click. Returns the element for further styling.
pub fn hd_link(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    cx: &App,
) -> Stateful<Div> {
    let t = Theme::get(cx);
    let label = label.into();
    let brand = t.brand;
    div()
        .id(id)
        .role(Role::Link)
        .aria_label(label.clone())
        .focusable()
        .tab_stop(true)
        .border_b_2()
        .mb(px(-2.))
        .border_color(transparent_black())
        .text_size(px(15.))
        .text_color(t.link)
        .cursor_pointer()
        .active(|s| s.opacity(0.5))
        .focus_visible(move |s| s.border_color(brand))
        .on_click(handler)
        .child(label)
}

/// Centres `child` below the footer with a 24 px top and -34 px bottom margin
/// (`__attachment-area`).
pub fn attachment_area(child: impl IntoElement) -> Div {
    div()
        .mt(px(24.))
        .mb(px(-34.))
        .flex()
        .justify_center()
        .child(child)
}

#[derive(Default, Clone)]
struct GrabState {
    start: Option<f32>,
    offset: f32,
}

/// Bottom sheet with a header, scrolling body and optional footer; children go in the body.
///
/// Unless `large`, its height ranges from 255 px to 75 % of the window. With
/// [`HalfScreenDialog::presence`] it renders in a [`Modal`] at the bottom with a dark mask and
/// slide-up motion, and Escape or a mask click calls `on_dismiss`. Without a presence it renders
/// inline with the `Dialog` role.
#[derive(IntoElement)]
pub struct HalfScreenDialog {
    id: ElementId,
    title: Option<SharedString>,
    subtitle: Option<SharedString>,
    main: Option<AnyElement>,
    left: Option<AnyElement>,
    right: Option<AnyElement>,
    label: SharedString,
    grab: bool,
    large: bool,
    bottom_fixed: bool,
    flush: bool,
    body: Vec<AnyElement>,
    body_padding_bottom: Option<f32>,
    footer: Option<AnyElement>,
    presence: Option<Presence>,
    on_dismiss: Option<DismissHandler>,
}

impl HalfScreenDialog {
    /// Creates an empty dialog with 24 px horizontal padding and no presence.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            subtitle: None,
            main: None,
            left: None,
            right: None,
            label: SharedString::default(),
            grab: false,
            large: false,
            bottom_fixed: false,
            flush: false,
            body: vec![],
            body_padding_bottom: None,
            footer: None,
            presence: None,
            on_dismiss: None,
        }
    }
    /// Sets the 15 px semibold title (`__title`) and uses it as the accessible name. Not drawn when
    /// [`HalfScreenDialog::main`] is set.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        self.label = title.clone();
        self.title = Some(title);
        self
    }
    /// Sets the 10 px line under the title (`__subtitle`); drawn only with a title and no `main`.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
    /// Replaces the title block in the header (`__hd__main`). Set [`HalfScreenDialog::label`] to
    /// give the dialog an accessible name.
    pub fn main(mut self, main: impl IntoElement) -> Self {
        self.main = Some(main.into_any_element());
        self
    }
    /// Places content on the header's left side (`__hd__side`), such as a close [`HdIconButton`];
    /// the main block is then centred.
    pub fn left(mut self, side: impl IntoElement) -> Self {
        self.left = Some(side.into_any_element());
        self
    }
    /// Places content on the header's right side, aligned to the end, such as an [`action_group`].
    pub fn right(mut self, side: impl IntoElement) -> Self {
        self.right = Some(side.into_any_element());
        self
    }
    /// Sets the accessible name explicitly; a later `title` call replaces it.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Enables the slide variant (`weui-half-screen-dialog_slide`): a grab handle above the header,
    /// and dragging the dialog down at least 56 px then releasing calls `on_dismiss`. Default
    /// `false`.
    pub fn grab(mut self, value: bool) -> Self {
        self.grab = value;
        self
    }
    /// Uses the large variant (`weui-half-screen-dialog_large`): the window height minus 16 px.
    /// Default `false`.
    pub fn large(mut self, value: bool) -> Self {
        self.large = value;
        self
    }
    /// Pins the footer at the bottom over a fading gradient (`weui-bottom-fixed-opr-page`) and
    /// moves the 24 px side padding onto the header, body and footer. Default `false`.
    pub fn bottom_fixed(mut self, value: bool) -> Self {
        self.bottom_fixed = value;
        self
    }
    /// Drops the dialog's side padding so the body runs edge to edge, keeping 24 px on the header.
    /// Default `false`; the picker uses it.
    pub fn flush(mut self, value: bool) -> Self {
        self.flush = value;
        self
    }
    /// Overrides the body's bottom padding in px (default 56, or 80 with `bottom_fixed`).
    pub fn body_padding_bottom(mut self, padding: f32) -> Self {
        self.body_padding_bottom = Some(padding);
        self
    }
    /// Sets the footer (`__ft`), drawn below the body with 64 px bottom padding.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    /// Renders the dialog as a modal driven by `presence` (cloned); without it the dialog is
    /// inline.
    pub fn presence(mut self, presence: &Presence) -> Self {
        self.presence = Some(presence.clone());
        self
    }
    /// Called on Escape, a mask click or a completed grab drag; hide the presence here.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for HalfScreenDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for HalfScreenDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let viewport = window.viewport_size();
        let grab_state = window.use_keyed_state(
            SharedString::from(format!("hsd-grab-{:?}", self.id)),
            cx,
            |_, _| GrabState::default(),
        );
        if self.presence.as_ref().is_some_and(|p| !p.is_open()) {
            grab_state.update(cx, |s, _| *s = GrabState::default());
        }
        let grab = grab_state.read(cx).clone();
        let pct = (grab.offset / GRAB_DISTANCE).clamp(0., 1.);
        let inset = if self.flush || self.bottom_fixed {
            px(24.)
        } else {
            px(0.)
        };
        let body_inset = if self.bottom_fixed { px(24.) } else { px(0.) };
        let has_left = self.left.is_some();
        let main = self
            .main
            .or_else(|| {
                self.title.clone().map(|title| {
                    div()
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_size(px(15.))
                                .text_color(t.fg_0)
                                .child(title),
                        )
                        .when_some(self.subtitle.clone(), |el, sub| {
                            el.child(div().text_size(px(10.)).text_color(t.fg_1).child(sub))
                        })
                        .into_any_element()
                })
            })
            .unwrap_or_else(|| div().into_any_element());
        let nav = div()
            .relative()
            .w_full()
            .min_h(px(64.))
            .flex()
            .items_center()
            .flex_shrink_0()
            .when_some(self.left, |el, side| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .left(px(-8.))
                        .flex()
                        .items_center()
                        .child(side),
                )
            })
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .when(has_left, |el| el.px(px(40.)).text_center())
                    .child(main),
            )
            .when_some(self.right, |el, side| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .left_0()
                        .right(px(-8.))
                        .flex()
                        .items_center()
                        .justify_end()
                        .child(side),
                )
            });
        let slide_bg = if t.is_dark { t.fg_3 } else { t.bg_0 };
        let header = if self.grab {
            div()
                .flex_shrink_0()
                .px(inset)
                .flex()
                .flex_col()
                .child(div().h(px(28.)))
                .child(nav)
        } else {
            div().flex_shrink_0().px(inset).child(nav)
        };
        let body_pb = self
            .body_padding_bottom
            .unwrap_or(if self.bottom_fixed { 80. } else { 56. });
        let body = div()
            .id("weui-half-screen-dialog-bd")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px(body_inset)
            .pb(px(body_pb))
            .text_size(px(14.))
            .text_color(t.fg_0)
            .children(self.body);
        let footer = self.footer.map(|footer| {
            if self.bottom_fixed {
                let solid: Hsla = if t.is_dark {
                    rgb(0x191919).into()
                } else {
                    rgb(0xffffff).into()
                };
                div()
                    .relative()
                    .flex_shrink_0()
                    .px(px(24.))
                    .pb(px(64.))
                    .bg(solid)
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .bottom(relative(1.))
                            .mb(px(-1.))
                            .h(px(80.))
                            .bg(linear_gradient(
                                0.,
                                linear_color_stop(solid, 0.),
                                linear_color_stop(solid.opacity(0.), 1.),
                            )),
                    )
                    .child(footer)
            } else {
                div()
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .px(body_inset)
                    .pb(px(64.))
                    .text_center()
                    .child(footer)
            }
        });
        let dismiss = self.on_dismiss.clone();
        let mut dialog = div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .when(!self.large, |el| {
                el.min_h(px(255.)).max_h(viewport.height * 0.75)
            })
            .when(self.large, |el| el.h(viewport.height - px(16.)))
            .top(px(grab.offset.max(0.)))
            .line_height(relative(1.4))
            .bg(t.bg_2)
            .text_color(t.fg_0)
            .rounded_t(px(12.))
            .overflow_hidden()
            .when(!self.flush && !self.bottom_fixed, |el| el.px(px(24.)))
            .flex()
            .flex_col()
            .child(header)
            .child(body)
            .children(footer);
        if self.grab {
            let line_h = 4. + 12. * pct;
            let arrow = if pct >= 0.5 { (pct - 0.5) / 0.5 } else { 0. };
            dialog = dialog.child(
                div()
                    .absolute()
                    .top(px(12.))
                    .left_0()
                    .right_0()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .w(px(40.))
                            .h(px(line_h))
                            .rounded(px(2. + 10. * pct))
                            .bg(slide_bg)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path("icon-arrow.svg")
                                    .w(px(8.))
                                    .h(px(16.))
                                    .text_color(t.fg_2)
                                    .opacity(arrow)
                                    .with_transformation(Transformation::rotate(percentage(0.25))),
                            ),
                    ),
            );
            let down = grab_state.clone();
            let moved = grab_state.clone();
            let up = grab_state.clone();
            let up_out = grab_state.clone();
            let dismiss_up = dismiss.clone();
            let finish = move |state: &Entity<GrabState>,
                               dismiss: &Option<DismissHandler>,
                               window: &mut Window,
                               cx: &mut App| {
                let s = state.read(cx).clone();
                if s.start.is_none() {
                    return;
                }
                state.update(cx, |s, cx| {
                    let close = s.offset >= GRAB_DISTANCE;
                    s.start = None;
                    if !close {
                        s.offset = 0.;
                    }
                    cx.notify();
                });
                if s.offset >= GRAB_DISTANCE
                    && let Some(dismiss) = dismiss.as_ref()
                {
                    dismiss(window, cx);
                }
            };
            let finish_out = finish;
            dialog = dialog
                .on_mouse_down(MouseButton::Left, move |e, _, cx| {
                    down.update(cx, |s, cx| {
                        s.start = Some(e.position.y.as_f32());
                        s.offset = 0.;
                        cx.notify();
                    });
                })
                .on_mouse_move(move |e, _, cx| {
                    if e.pressed_button != Some(MouseButton::Left) {
                        return;
                    }
                    moved.update(cx, |s, cx| {
                        if let Some(start) = s.start {
                            s.offset = (e.position.y.as_f32() - start).max(0.);
                            cx.notify();
                        }
                    });
                })
                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                    finish(&up, &dismiss_up, window, cx)
                })
                .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                    finish_out(&up_out, &dismiss, window, cx)
                });
        }
        match self.presence {
            Some(presence) => {
                let dismiss = self.on_dismiss.clone();
                Modal::new(
                    SharedString::from(format!("{:?}-modal", self.id)),
                    &presence,
                )
                .mask(MaskKind::Dark)
                .placement(Placement::Bottom)
                .motion(Motion::SlideUp)
                .label(self.label)
                .on_dismiss(move |window, cx| {
                    if let Some(dismiss) = dismiss.as_ref() {
                        dismiss(window, cx);
                    }
                })
                .child(dialog)
                .into_any_element()
            }
            None => dialog
                .role(Role::Dialog)
                .aria_label(self.label)
                .into_any_element(),
        }
    }
}
