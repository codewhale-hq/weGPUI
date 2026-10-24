use super::{ChangeHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}
impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "多选".into(),
            checked: false,
            disabled: false,
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
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let brand = t.brand;
        let mut el = div()
            .id(self.id)
            .role(Role::CheckBox)
            .aria_label(self.label)
            .aria_toggled(if self.checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .size(px(24.))
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .when(self.disabled, |el| el.opacity(0.3));
        el = el.child(
            div()
                .size(px(22.))
                .rounded_full()
                .when(self.checked, |el| {
                    el.bg(brand)
                        .child(Icon::new("success-no-circle").size(22.).color(white()))
                })
                .when(!self.checked, |el| el.border_1().border_color(t.fg_2)),
        );
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.border_2().border_color(brand));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| handler(&!self.checked, w, cx));
            }
        } else {
            el = el.aria_description("不可用");
        }
        crate::accessibility::Semantics::new(el).disabled(self.disabled)
    }
}
