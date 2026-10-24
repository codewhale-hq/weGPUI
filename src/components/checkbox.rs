use super::{ChangeHandler, Icon};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};

pub fn check_icon(checked: bool, size: f32, cx: &App) -> Div {
    let t = Theme::get(cx);
    let ring = size * 22.8 / 24.;
    div()
        .size(px(size))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .when(checked, |el| {
            el.child(
                div()
                    .size(px(size))
                    .rounded_full()
                    .bg(t.brand)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div().mt(px(size * 0.04)).child(
                            Icon::new("success-no-circle")
                                .size(size * 2. / 3.)
                                .color(white()),
                        ),
                    ),
            )
        })
        .when(!checked, |el| {
            el.child(
                div()
                    .size(px(ring))
                    .rounded_full()
                    .border(px(size * 1.2 / 24.))
                    .border_color(t.fg_2),
            )
        })
}

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    size: f32,
    on_change: Option<ChangeHandler<bool>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "多选".into(),
            checked: false,
            disabled: false,
            size: 24.,
            on_change: None,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let brand = Theme::get(cx).brand;
        let checked = self.checked;
        let mut el = div()
            .id(self.id)
            .role(Role::CheckBox)
            .aria_label(self.label)
            .aria_toggled(if checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .relative()
            .flex_shrink_0()
            .child(check_icon(checked, self.size, cx).when(self.disabled, |el| el.opacity(0.3)))
            .child(
                div()
                    .absolute()
                    .top(px(-2.))
                    .left(px(-2.))
                    .size(px(self.size + 4.))
                    .rounded_full()
                    .border_2()
                    .border_color(transparent_black()),
            );
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.rounded_full().bg(brand.opacity(0.15)));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| {
                    cx.stop_propagation();
                    handler(&!checked, w, cx)
                });
            }
        }
        Semantics::new(el).disabled(self.disabled)
    }
}

#[derive(IntoElement)]
pub struct CheckboxCell {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}

impl CheckboxCell {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for CheckboxCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let checked = self.checked;
        let disabled = self.disabled;
        let pressed = t.fg_3;
        let brand = t.brand;
        let mut el = super::form::cell_frame(self.id)
            .role(Role::CheckBox)
            .aria_label(self.label.clone())
            .aria_toggled(if checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .text_color(if disabled { t.fg_2 } else { t.fg_0 })
            .child(
                div()
                    .pr(px(16.))
                    .child(check_icon(checked, 24., cx).when(disabled, |el| el.opacity(0.3))),
            )
            .child(div().flex_1().min_w_0().child(self.label));
        if !disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .active(move |s| s.bg(pressed))
                .focus_visible(move |s| s.border_color(brand));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| handler(&!checked, w, cx));
            }
        }
        Semantics::new(el).disabled(disabled)
    }
}
