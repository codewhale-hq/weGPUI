use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::slider::Slider;

pub struct SliderPage {
    route: &'static str,
    value: f32,
}

pub fn build(route: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let value = args
        .state
        .as_deref()
        .and_then(|s| s.strip_prefix("value"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(50.);
    cx.new(|_| SliderPage { route, value }).into()
}

impl Render for SliderPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Slider")
            .desc(tr("滑块"))
            .spacing()
            .child(
                Slider::new("slider-fixed")
                    .label(tr("不可调滑块"))
                    .value(0.),
            )
            .child(div().h(px(25.6)))
            .child(
                Slider::new("slider-adjustable")
                    .label(tr("可调滑块"))
                    .value(self.value)
                    .show_value(true)
                    .on_change(cx.listener(|this, value: &f32, _, cx| {
                        this.value = *value;
                        cx.notify();
                    })),
            )
    }
}
