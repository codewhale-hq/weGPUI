//! Progress bars from WeUI's `src/style/widget/weui-progress/`: [`Progress`] draws a 3px bar
//! followed by an optional cancel icon button (`weui-progress__opr`). The caller owns progress,
//! cancellation and the timing of optional live announcements, as in the upstream example.

use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

/// A horizontal progress bar filled with the brand colour, optionally followed by a red cancel
/// [`Icon`].
///
/// The bar is exposed as `Role::ProgressIndicator` with a 0–100 value and "N%" value text; the
/// cancel icon is a Tab-stop button with a 44px hit area. It holds no state: re-render it with the
/// new value.
#[derive(IntoElement)]
pub struct Progress {
    id: ElementId,
    value: f32,
    label: SharedString,
    cancel_label: SharedString,
    cancelable: bool,
    announcement: Option<SharedString>,
    on_cancel: Option<ClickHandler>,
}

impl Progress {
    /// Creates a bar at 0% with element id `id`, labelled "进度", with a cancel button labelled
    /// "取消".
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            label: "进度".into(),
            cancel_label: "取消".into(),
            cancelable: true,
            announcement: None,
            on_cancel: None,
        }
    }
    /// Sets the filled percentage, clamped to 0–100; NaN is treated as 0. Default: 0.
    pub fn value(mut self, percent: f32) -> Self {
        self.value = if percent.is_nan() {
            0.
        } else {
            percent.clamp(0., 100.)
        };
        self
    }
    /// Sets the progress indicator's accessible name. Default: "进度" (progress).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Sets the cancel button's accessible name. Default: "取消" (cancel).
    pub fn cancel_label(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel_label = label.into();
        self
    }
    /// Shows or removes the cancel icon and button (default `true`). When `false`, no cancel
    /// control is drawn, focusable or exposed to assistive technology; the bar fills its width.
    pub fn cancelable(mut self, cancelable: bool) -> Self {
        self.cancelable = cancelable;
        self
    }
    /// Adds an invisible alert with caller-controlled announcement text, for example
    /// "已上传百分之50". Default: none. Keep this text unchanged between milestones to avoid
    /// announcing every animation frame; an empty string keeps the alert available silently.
    /// The numeric indicator value always remains available independently of this text.
    pub fn announcement(mut self, text: impl Into<SharedString>) -> Self {
        self.announcement = Some(text.into());
        self
    }
    /// Sets the cancel callback. It receives the click event when the cancel button is clicked, or
    /// activated with Enter or Space while focused. Without it the button still renders and takes
    /// focus.
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
        crate::scrolling::FocusTarget::new({
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
                        .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
                        .when_some(self.on_cancel, |el, h| el.on_click(h)),
                );
            div()
                .id(self.id)
                .relative()
                .flex()
                .items_center()
                .child(bar)
                .when(self.cancelable, |el| el.child(opr))
                .when_some(self.announcement, |el, text| {
                    el.child(
                        crate::accessibility::Semantics::new(
                            div()
                                .id("progress-announcement")
                                .role(Role::Alert)
                                .aria_label(text)
                                .absolute()
                                .size(px(1.))
                                .opacity(0.),
                        )
                        .live(accesskit::Live::Assertive),
                    )
                })
        })
    }
}
