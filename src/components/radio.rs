use super::{ChangeHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}
impl Radio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "单选".into(),
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
impl RenderOnce for Radio {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let brand = t.brand;
        let mut el = div()
            .id(self.id)
            .role(Role::RadioButton)
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
        el = el.when(self.checked, |el| {
            el.child(Icon::new("success-no-circle").size(16.).color(brand))
        });
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.border_2().border_color(brand));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| handler(&true, w, cx));
            }
        } else {
            el = el.aria_description("不可用");
        }
        crate::accessibility::Semantics::new(el).disabled(self.disabled)
    }
}
