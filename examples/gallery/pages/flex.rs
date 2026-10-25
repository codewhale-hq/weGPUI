use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::flex::{flex, flex_item},
};

pub struct FlexPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| FlexPage { route }).into()
}

impl Render for FlexPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let (bg, fg) = (t.bg_1, t.fg_1);
        let placeholder = move || {
            div()
                .m(px(5.))
                .px(px(10.))
                .bg(bg)
                .h(px(16. * 2.3))
                .line_height(px(16. * 2.3))
                .text_center()
                .text_color(fg)
                .child("weui")
        };
        let row = |n: usize| flex().children((0..n).map(|_| flex_item().child(placeholder())));
        PageFrame::new(self.route)
            .title("Flex")
            .desc(tr("Flex布局"))
            .spacing()
            .child(row(1))
            .child(row(2))
            .child(row(3))
            .child(row(4))
            .child(
                flex()
                    .child(div().child(placeholder()))
                    .child(flex_item().child(placeholder()))
                    .child(div().child(placeholder())),
            )
    }
}
