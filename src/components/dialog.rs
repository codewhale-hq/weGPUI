use super::modal::{MaskKind, Modal, Motion, Placement, Presence};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::{rc::Rc, time::Duration};

pub const DIALOG_FADE: Duration = Duration::from_millis(200);

pub type ActionHandler = Rc<dyn Fn(&mut Window, &mut App)>;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogButtonKind {
    #[default]
    Primary,
    Default,
    Warn,
    Disabled,
}

#[derive(Clone)]
pub struct DialogButton {
    label: SharedString,
    kind: DialogButtonKind,
    handler: Option<ActionHandler>,
}

impl DialogButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: DialogButtonKind::Primary,
            handler: None,
        }
    }
    pub fn kind(mut self, kind: DialogButtonKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn primary(self) -> Self {
        self.kind(DialogButtonKind::Primary)
    }
    pub fn default_style(self) -> Self {
        self.kind(DialogButtonKind::Default)
    }
    pub fn warn(self) -> Self {
        self.kind(DialogButtonKind::Warn)
    }
    pub fn disabled(self) -> Self {
        self.kind(DialogButtonKind::Disabled)
    }
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handler = Some(Rc::new(handler));
        self
    }
}

#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    title: Option<SharedString>,
    body: Vec<AnyElement>,
    label: SharedString,
    buttons: Vec<DialogButton>,
    android: bool,
    btn_wrap: bool,
    presence: Option<Presence>,
    on_dismiss: Option<ActionHandler>,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            body: vec![],
            label: SharedString::default(),
            buttons: vec![],
            android: false,
            btn_wrap: false,
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
    pub fn body(mut self, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        if self.label.is_empty() {
            self.label = text.clone();
        }
        self.body.push(text.into_any_element());
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn button(mut self, button: DialogButton) -> Self {
        self.buttons.push(button);
        self
    }
    pub fn buttons(mut self, buttons: impl IntoIterator<Item = DialogButton>) -> Self {
        self.buttons.extend(buttons);
        self
    }
    pub fn android(mut self, value: bool) -> Self {
        self.android = value;
        self
    }
    pub fn btn_wrap(mut self, value: bool) -> Self {
        self.btn_wrap = value;
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
    pub fn presence_for(cx: &mut App) -> Presence {
        Presence::new(cx).duration(DIALOG_FADE)
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let viewport = window.viewport_size();
        let width = (viewport.width - px(32.)).min(px(320.));
        let android = self.android;
        let has_title = self.title.is_some();
        let line = t.dialog_line_color;
        let count = self.buttons.len();
        let buttons = self.buttons.into_iter().enumerate().map(|(i, button)| {
            let color = match button.kind {
                DialogButtonKind::Primary => t.link,
                DialogButtonKind::Default => t.fg_half,
                DialogButtonKind::Warn => t.red,
                DialogButtonKind::Disabled => t.fg_2,
            };
            let disabled = button.kind == DialogButtonKind::Disabled;
            let handler = button.handler.clone();
            let el = div()
                .id(("weui-dialog-btn", i))
                .role(Role::Button)
                .aria_label(button.label.clone())
                .relative()
                .text_size(px(17.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color)
                .overflow_hidden()
                .cursor_pointer()
                .border_2()
                .border_color(transparent_black())
                .map(|el| {
                    if android {
                        el.px(px(13.6 - 2.))
                            .mt(px(-2.))
                            .line_height(relative(24. / 17.))
                            .when(i + 1 == count, |el| el.mr(px(-13.6)))
                    } else {
                        el.flex_1()
                            .px(px(8. - 2.))
                            .py(px(20. - 2.))
                            .line_height(relative(24. / 17.))
                            .text_center()
                    }
                })
                .when(!android && i > 0, |el| {
                    el.child(if self.btn_wrap {
                        div()
                            .absolute()
                            .top(px(-2.))
                            .left(px(-2.))
                            .right(px(-2.))
                            .h(px(0.5))
                            .bg(line)
                    } else {
                        div()
                            .absolute()
                            .top(px(-2.))
                            .bottom(px(-2.))
                            .left(px(-2.))
                            .w(px(0.5))
                            .bg(line)
                    })
                })
                .child(button.label.clone());
            let el = if disabled {
                el
            } else {
                el.focusable()
                    .tab_stop(true)
                    .active(|s| s.bg(t.bg_color_active))
                    .focus_visible(|s| s.border_color(t.brand))
                    .on_click(move |_, window, cx| {
                        if let Some(handler) = handler.as_ref() {
                            handler(window, cx);
                        }
                    })
            };
            Semantics::new(el).disabled(disabled)
        });
        let footer = if android {
            div()
                .flex()
                .justify_end()
                .items_start()
                .min_h(px(56.))
                .px(px(24.))
                .pb(px(16.))
                .children(buttons)
        } else {
            div()
                .relative()
                .flex()
                .when(self.btn_wrap, |el| el.flex_col())
                .when(!self.btn_wrap, |el| el.items_center())
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(0.5))
                        .bg(line),
                )
                .children(buttons)
        };
        let body = div()
            .id("weui-dialog-bd")
            .min_h_0()
            .overflow_y_scroll()
            .px(px(24.))
            .mb(px(32.))
            .text_size(px(17.))
            .line_height(relative(1.4))
            .when(android, |el| el.text_left())
            .map(|el| {
                if has_title {
                    el.text_color(t.fg_1)
                } else {
                    el.min_h(px(40.))
                        .pt(px(32.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(t.fg_0)
                        .flex()
                        .flex_col()
                        .justify_center()
                }
            })
            .children(self.body);
        let dialog = div()
            .id(self.id.clone())
            .w(width)
            .max_h(viewport.height * 0.9)
            .rounded(px(12.))
            .overflow_hidden()
            .bg(t.bg_2)
            .text_center()
            .flex()
            .flex_col()
            .when_some(self.title, |el, title| {
                el.child(
                    div()
                        .flex_shrink_0()
                        .pt(px(32.))
                        .pb(px(16.))
                        .px(px(24.))
                        .when(android, |el| el.text_left())
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(if android { 22. } else { 17. }))
                        .line_height(relative(1.4))
                        .text_color(t.fg_0)
                        .child(title),
                )
            })
            .child(body)
            .child(footer.flex_shrink_0());
        match self.presence {
            Some(presence) => {
                let dismiss = self.on_dismiss.clone();
                Modal::new(self.id, &presence)
                    .mask(MaskKind::Dark)
                    .placement(Placement::Center)
                    .motion(Motion::Fade)
                    .dismiss_on_mask(false)
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
