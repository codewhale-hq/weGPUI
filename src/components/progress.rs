use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct Progress {
    id: ElementId,
    value: f32,
    label: SharedString,
    cancel_label: SharedString,
    on_cancel: Option<ClickHandler>,
}

impl Progress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            label: "进度".into(),
            cancel_label: "取消".into(),
            on_cancel: None,
        }
    }
    pub fn value(mut self, percent: f32) -> Self {
        self.value = percent.clamp(0., 100.);
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn cancel_label(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel_label = label.into();
        self
    }
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Progress {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let brand = t.brand;
        let bar = div()
            .id("progress-bar")
            .role(Role::ProgressIndicator)
            .aria_label(self.label)
            .aria_numeric_value(self.value.round() as f64)
            .aria_min_numeric_value(0.)
            .aria_max_numeric_value(100.)
            .aria_value(SharedString::from(format!("{}%", self.value.round())))
            .flex_1()
            .h(px(3.))
            .bg(t.bg_0)
            .child(div().h_full().w(relative(self.value / 100.)).bg(brand));
        let opr = div()
            .relative()
            .ml(px(15.))
            .size(px(24.))
            .flex_shrink_0()
            .child(Icon::new("cancel").size(24.).color(t.red))
            .child(
                div()
                    .id("progress-cancel")
                    .role(Role::Button)
                    .aria_label(self.cancel_label)
                    .absolute()
                    .top(px(-10.))
                    .left(px(-10.))
                    .size(px(44.))
                    .rounded_full()
                    .border_2()
                    .border_color(transparent_black())
                    .cursor_pointer()
                    .focusable()
                    .tab_stop(true)
                    .focus_visible(move |s| s.border_color(brand))
                    .when_some(self.on_cancel, |el, h| el.on_click(h)),
            );
        div()
            .id(self.id)
            .flex()
            .items_center()
            .child(bar)
            .child(opr)
    }
}
