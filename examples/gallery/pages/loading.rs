use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::loading::Loading;

pub struct LoadingPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| LoadingPage { route }).into()
}

impl Render for LoadingPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let items = [
            Loading::new().id("l-default"),
            Loading::mask().id("l-mask"),
            Loading::mask().id("l-brand").brand(),
            Loading::mask().id("l-transparent").transparent(),
        ];
        PageFrame::new(self.route)
            .title("Loading")
            .desc(tr("正在加载"))
            .child(
                div()
                    .my(px(10.))
                    .mx(px(40.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(25.6))
                    .children(items.into_iter().enumerate().map(|(i, l)| {
                        div()
                            .mx(px(5.))
                            .when(i > 0, |el| el.ml(px(5. + 4.4)))
                            .child(l.label(tr("正在加载")))
                    })),
            )
    }
}
