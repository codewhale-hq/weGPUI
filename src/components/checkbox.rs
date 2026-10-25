//! WeUI checkboxes (`src/style/widget/weui-cell/weui-check/weui-checkbox.less`, example
//! `src/example/form/form_checkbox.html`). [`Checkbox`] is a bare circular check,
//! [`CheckboxCell`] a full `.weui-check__label` form row, and [`check_icon`] the shared glyph.
//! Both controls are controlled: they draw the `checked` value they are given and report the
//! requested new value through `on_change`.
use crate::scrolling::RevealFocus;

use super::{ChangeHandler, Icon};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};

/// The WeUI check glyph, `size` px across: a brand-filled circle with a white tick when
/// `checked`, otherwise a thin `fg_2` ring. Purely visual, with no role or state.
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

/// A standalone checkbox: a [`check_icon`] exposed as `Role::CheckBox` with a toggled state and
/// two contrasting 2px outer bands on keyboard focus. Controlled; the caller stores the value.
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
    /// Creates an unchecked, enabled 24px checkbox whose accessible name is "多选".
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
    /// Sets the accessible name. It is not drawn; place visible text next to the checkbox.
    /// Default "多选".
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Sets whether the box is drawn and exposed as checked. Default `false`.
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    /// Disables the checkbox: drawn at 30% opacity, not focusable, `on_change` never called, and
    /// flagged disabled for assistive technology. Default `false`.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Sets the glyph diameter in logical pixels. Default 24.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    /// Calls `handler` with the requested new value (`!checked`) on click or Enter/Space; the
    /// click does not propagate to the parent. Re-render with [`Checkbox::checked`] to show it.
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let checked = self.checked;
        let focus = window
            .use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "focus".into()),
                cx,
                |_, cx| cx.focus_handle().tab_stop(true),
            )
            .read(cx)
            .clone();
        let show_focus =
            !self.disabled && focus.is_focused(window) && window.last_input_was_keyboard();
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
                    .top(px(-4.))
                    .left(px(-4.))
                    .size(px(self.size + 8.))
                    .rounded_full()
                    // An empty overlay paints only its inset edges. Drop shadows underneath
                    // the glyph would fill its transparent unchecked centre on keyboard focus.
                    .when(show_focus, |el| el.shadow(super::form::dual_focus_ring())),
            );
        if !self.disabled {
            el = el
                .focusable()
                .track_focus(&focus)
                .tab_stop(true)
                .cursor_pointer();
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| {
                    cx.stop_propagation();
                    handler(&!checked, w, cx)
                });
            }
        }
        Semantics::new(el).disabled(self.disabled).reveal_focus()
    }
}

/// A checkbox row (`.weui-cells_checkbox .weui-check__label`): a 24px [`check_icon`] then the
/// label. The whole row is one `Role::CheckBox` tab stop named by the label, with a pressed
/// highlight and two contrasting inset focus bands. Controlled. In a form group, add it with
/// [`crate::components::form::FormGroup::row_inset`] and 56 so separators align with the label.
#[derive(IntoElement)]
pub struct CheckboxCell {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}

impl CheckboxCell {
    /// Creates an unchecked, enabled row; `label` is both the visible text and the accessible
    /// name.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }
    /// Sets whether the row is drawn and exposed as checked. Default `false`.
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    /// Disables the row: `fg_2` text, icon at 30% opacity, no focus or pressed state,
    /// `on_change` never called, and flagged disabled. Default `false`.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Calls `handler` with the requested new value (`!checked`) when the row is clicked or
    /// activated with Enter/Space. Re-render with [`CheckboxCell::checked`] to show it.
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
                .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| handler(&!checked, w, cx));
            }
        }
        Semantics::new(el).disabled(disabled).reveal_focus()
    }
}
