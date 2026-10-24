use super::ChangeHandler;
use crate::Theme;
use gpui::{prelude::*, *};
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}
impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "开关".into(),
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
impl RenderOnce for Switch {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let brand = t.brand;
        let mut el = div()
            .id(self.id)
            .role(Role::Switch)
            .aria_label(self.label)
            .aria_toggled(if self.checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .w(px(52.))
            .h(px(32.))
            .flex_shrink_0()
            .rounded(px(16.))
            .bg(if self.checked { brand } else { t.fg_3 })
            .p(px(2.))
            .when(self.disabled, |el| el.opacity(0.1))
            .child(
                div()
                    .size(px(28.))
                    .rounded(px(15.))
                    .bg(white())
                    .shadow_sm()
                    .ml(px(if self.checked { 20. } else { 0. })),
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
