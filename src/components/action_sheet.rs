use super::modal::{MaskKind, Modal, Motion, Placement, Presence};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

pub type ActionHandler = Rc<dyn Fn(&mut Window, &mut App)>;

#[derive(Clone)]
pub struct ActionSheetItem {
    label: SharedString,
    tips: Option<SharedString>,
    warn: bool,
    focus: Option<FocusHandle>,
    handler: Option<ActionHandler>,
}

impl ActionSheetItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tips: None,
            warn: false,
            focus: None,
            handler: None,
        }
    }
    pub fn tips(mut self, tips: impl Into<SharedString>) -> Self {
        self.tips = Some(tips.into());
        self
    }
    pub fn warn(mut self) -> Self {
        self.warn = true;
        self
    }
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handler = Some(Rc::new(handler));
        self
    }
}

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
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        self.label = title.clone();
        self.title = Some(title);
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn item(mut self, item: ActionSheetItem) -> Self {
        self.items.push(item);
        self
    }
    pub fn items(mut self, items: impl IntoIterator<Item = ActionSheetItem>) -> Self {
        self.items.extend(items);
        self
    }
    pub fn action(mut self, item: ActionSheetItem) -> Self {
        self.actions.push(item);
        self
    }
    pub fn android(mut self, value: bool) -> Self {
        self.android = value;
        self
    }
    pub fn presence(mut self, presence: &Presence) -> Self {
        self.presence = Some(presence.clone());
        self
    }
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
) -> Stateful<Div> {
    let handler = item.handler.clone();
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
        .text_color(if item.warn { t.red } else { t.fg_0 })
        .when(android, |el| el.text_left())
        .when(!android, |el| el.text_center())
        .cursor_pointer()
        .active(|s| s.bg(t.bg_color_active))
        .focus_visible(|s| s.border_color(t.brand))
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
            if let Some(handler) = handler.as_ref() {
                handler(window, cx);
            }
        })
}

impl RenderOnce for ActionSheet {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let android = self.android;
        let has_title = self.title.is_some();
        let count = self.items.len();
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
                            )
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
