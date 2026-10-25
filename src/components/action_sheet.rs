//! WeUI action sheet (`weui-tips/weui-actionsheet.less`): an iOS bottom sheet with optional title,
//! menu cells and a separate action group, or the Android skin's centred 274 px menu card.
//!
//! [`ActionSheet`] renders in a [`Modal`] when given a [`Presence`]; [`ActionSheetItem`] handlers
//! do not close it. Known limitations: in Android style the title is only the accessible name and
//! is not drawn, and the action group is omitted (upstream hides `weui-actionsheet__action` there).

use super::modal::{MaskKind, Modal, Motion, Placement, Presence};
use crate::{Theme, logical::Logical, scrolling::RevealFocus};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Shared callback for item activation and dismissal.
pub type ActionHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// One cell of an [`ActionSheet`] menu or action group, exposed as a focusable `Button`.
#[derive(Clone)]
pub struct ActionSheetItem {
    label: SharedString,
    tips: Option<SharedString>,
    warn: bool,
    focus: Option<FocusHandle>,
    handler: Option<ActionHandler>,
}

impl ActionSheetItem {
    /// Creates a plain cell showing `label`, with no handler.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tips: None,
            warn: false,
            focus: None,
            handler: None,
        }
    }
    /// Adds a 12 px secondary line under the label (`weui-actionsheet__cell__tips`); it is included
    /// in the accessible name.
    pub fn tips(mut self, tips: impl Into<SharedString>) -> Self {
        self.tips = Some(tips.into());
        self
    }
    /// Draws the label in the warning red (`weui-actionsheet__cell_warn`) in iOS style.
    /// The Android skin's cell colour overrides this modifier, matching upstream CSS.
    pub fn warn(mut self) -> Self {
        self.warn = true;
        self
    }
    /// Uses `focus` as the cell's focus handle, so the caller can focus it, for example on open.
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Called when the cell is clicked or activated with Enter or Space; it does not close the
    /// sheet.
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handler = Some(Rc::new(handler));
        self
    }
}

/// Action sheet listing [`ActionSheetItem`]s.
///
/// With [`ActionSheet::presence`] it renders in a [`Modal`] with a dark mask: iOS style slides up
/// from the bottom, Android style fades in centred. Escape and mask clicks call `on_dismiss`.
/// Without a presence it renders inline with the `Dialog` role.
#[derive(IntoElement)]
pub struct ActionSheet {
    id: ElementId,
    title: Option<SharedString>,
    label: SharedString,
    items: Vec<ActionSheetItem>,
    actions: Vec<ActionSheetItem>,
    android: bool,
    presence: Option<Presence>,
    on_dismiss: Option<ActionHandler>,
}

impl ActionSheet {
    /// Creates an empty iOS-style sheet with no presence.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            label: SharedString::default(),
            items: vec![],
            actions: vec![],
            android: false,
            presence: None,
            on_dismiss: None,
        }
    }
    /// Sets the 12 px title row (up to two lines, iOS style only) and uses it as the accessible
    /// name.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        self.label = title.clone();
        self.title = Some(title);
        self
    }
    /// Sets the accessible name explicitly; a later `title` call replaces it.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Appends a menu cell.
    pub fn item(mut self, item: ActionSheetItem) -> Self {
        self.items.push(item);
        self
    }
    /// Appends several menu cells, in order.
    pub fn items(mut self, items: impl IntoIterator<Item = ActionSheetItem>) -> Self {
        self.items.extend(items);
        self
    }
    /// Appends a cell to the action group drawn 8 px below the menu, such as "Cancel" (iOS style only).
    pub fn action(mut self, item: ActionSheetItem) -> Self {
        self.actions.push(item);
        self
    }
    /// Uses the Android skin (`weui-skin_android`): a centred 274 px card with left-aligned cells.
    /// Default `false`.
    pub fn android(mut self, value: bool) -> Self {
        self.android = value;
        self
    }
    /// Renders the sheet as a modal driven by `presence` (cloned); without it the sheet is inline.
    pub fn presence(mut self, presence: &Presence) -> Self {
        self.presence = Some(presence.clone());
        self
    }
    /// Called on Escape or a mask click in the open modal; hide the presence here.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

fn cell(
    id: ElementId,
    item: ActionSheetItem,
    first: bool,
    android: bool,
    t: &Theme,
    dir: crate::i18n::Direction,
    presence: Option<&Presence>,
) -> Stateful<Div> {
    let handler = item.handler.clone();
    let presence = presence.cloned();
    let has_tips = item.tips.is_some();
    let line = t.fg_3;
    div()
        .id(id)
        .role(Role::Button)
        .aria_label(match &item.tips {
            Some(tips) => SharedString::from(format!("{} {}", item.label, tips)),
            None => item.label.clone(),
        })
        .focusable()
        .tab_stop(true)
        .when_some(item.focus.as_ref(), |el, focus| el.track_focus(focus))
        .relative()
        .overflow_hidden()
        .border_2()
        .border_color(transparent_black())
        .px(px(14.))
        .py(px(if has_tips { 10. } else { 14. }))
        .text_size(px(17.))
        .line_height(relative(24. / 17.))
        .bg(t.bg_2)
        .text_color(if item.warn && !android { t.red } else { t.fg_0 })
        .when(android, |el| el.text_start(dir))
        .when(!android, |el| el.text_center())
        .cursor_pointer()
        .active(|s| s.bg(t.bg_color_active))
        .focus_visible(|s| s.border_color(t.focus_color(t.bg_2)))
        .when(!first, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(-2.))
                    .left(px(-2.))
                    .right(px(-2.))
                    .h(px(0.5))
                    .bg(line),
            )
        })
        .child(item.label)
        .when_some(item.tips, |el, tips| {
            el.child(
                div()
                    .text_size(px(12.))
                    .line_height(relative(24. / 17.))
                    .text_color(t.fg_1)
                    .child(tips),
            )
        })
        .on_click(move |_, window, cx| {
            if presence.as_ref().is_none_or(Presence::accepts_input)
                && let Some(handler) = handler.as_ref()
            {
                handler(window, cx);
            }
        })
}

impl RenderOnce for ActionSheet {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let dir = crate::i18n::direction(cx);
        let android = self.android;
        let has_title = self.title.is_some();
        let count = self.items.len();
        let presence = self.presence.clone();
        let menu = div()
            .flex()
            .flex_col()
            .bg(t.bg_2)
            .when(!android && !has_title, |el| el.rounded_t(px(12.)))
            .when(android, |el| {
                el.rounded(px(2.)).overflow_hidden().shadow(vec![BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(6.)),
                    blur_radius: px(15.),
                    spread_radius: px(0.),
                    inset: false,
                }])
            })
            .children(self.items.into_iter().enumerate().map(|(i, item)| {
                cell(
                    ("weui-actionsheet-cell", i).into(),
                    item,
                    i == 0,
                    android,
                    &t,
                    dir,
                    presence.as_ref(),
                )
                .when(i == 0, |el| {
                    el.rounded_t(px(if android {
                        2.
                    } else if has_title {
                        0.
                    } else {
                        12.
                    }))
                })
                .when(android && i + 1 == count, |el| el.rounded_b(px(2.)))
                .reveal_focus()
            }));
        let sheet = if android {
            div().w(px(274.)).child(menu)
        } else {
            div()
                .w_full()
                .bg(t.bg_1)
                .rounded_t(px(12.))
                .overflow_hidden()
                .flex()
                .flex_col()
                .when_some(self.title, |el, title| {
                    el.child(
                        div()
                            .relative()
                            .h(px(56.))
                            .py(px(8.))
                            .px(px(24.))
                            .flex()
                            .flex_col()
                            .justify_center()
                            .text_center()
                            .text_size(px(12.))
                            .line_height(relative(1.4))
                            .text_color(t.fg_1)
                            .bg(t.bg_2)
                            .rounded_t(px(12.))
                            .child(div().line_clamp(2).child(title))
                            .child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .left_0()
                                    .right_0()
                                    .h(px(0.5))
                                    .bg(t.fg_3),
                            ),
                    )
                })
                .child(menu)
                .when(!self.actions.is_empty(), |el| {
                    el.child(div().mt(px(8.)).flex().flex_col().bg(t.bg_2).children(
                        self.actions.into_iter().enumerate().map(|(i, item)| {
                            cell(
                                ("weui-actionsheet-action", i).into(),
                                item,
                                i == 0,
                                false,
                                &t,
                                dir,
                                presence.as_ref(),
                            )
                            .reveal_focus()
                        }),
                    ))
                })
        };
        match self.presence {
            Some(presence) => {
                let dismiss = self.on_dismiss.clone();
                Modal::new(self.id, &presence)
                    .mask(MaskKind::Dark)
                    .placement(if android {
                        Placement::Center
                    } else {
                        Placement::Bottom
                    })
                    .motion(if android {
                        Motion::Fade
                    } else {
                        Motion::SlideUp
                    })
                    .label(self.label)
                    .on_dismiss(move |window, cx| {
                        if let Some(dismiss) = dismiss.as_ref() {
                            dismiss(window, cx);
                        }
                    })
                    .child(sheet)
                    .into_any_element()
            }
            None => div()
                .id(self.id)
                .role(Role::Dialog)
                .aria_label(self.label)
                .child(sheet)
                .into_any_element(),
        }
    }
}
