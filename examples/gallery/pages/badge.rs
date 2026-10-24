use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{Cell, Cells, badge::Badge},
};

pub struct BadgePage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| BadgePage { route }).into()
}

fn titled(title: &'static str, badge: Badge) -> Div {
    div()
        .flex()
        .items_center()
        .h(px(25.33))
        .child(title)
        .child(div().ml(px(5.)).child(badge))
}

impl Render for BadgePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let fg1 = t.fg_1;
        PageFrame::new(self.route)
            .title("Badge")
            .desc("徽章")
            .child(
                Cells::new()
                    .title("新消息提示跟摘要信息后，统一在列表右侧")
                    .child(
                        Cell::new("badge-l1")
                            .role(Role::Link)
                            .aria_label("单行列表 详细信息，有更新")
                            .access()
                            .bd("单行列表")
                            .ft(div()
                                .flex()
                                .items_center()
                                .text_color(fg1)
                                .child("详细信息")
                                .child(div().mx(px(5.)).child(Badge::dot())))
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title("未读数红点跟在主题信息后，统一在列表左侧")
                    .child(
                        Cell::new("badge-b1")
                            .role(Role::ListBoxOption)
                            .aria_label("联系人名称，8个新通知")
                            .active()
                            .hd(div()
                                .relative()
                                .mr(px(10.))
                                .child(img("pic_160.png").size(px(50.)))
                                .child(
                                    div()
                                        .absolute()
                                        .top(px(-4.8))
                                        .right(px(-4.8))
                                        .child(Badge::count(8)),
                                ))
                            .bd(div().flex().flex_col().child("联系人名称").child(
                                div()
                                    .text_size(px(13.))
                                    .line_height(relative(1.6))
                                    .text_color(rgb(0x888888))
                                    .child("摘要信息"),
                            ))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b2")
                            .role(Role::Link)
                            .aria_label("单行列表，8个新通知")
                            .access()
                            .bd(titled("单行列表", Badge::count(8)))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b3")
                            .role(Role::Link)
                            .aria_label("单行列表 详细信息，8个新通知")
                            .access()
                            .bd(titled("单行列表", Badge::count(8)))
                            .value("详细信息")
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b4")
                            .role(Role::Link)
                            .aria_label("单行列表，New")
                            .access()
                            .bd(titled("单行列表", Badge::new("New")))
                            .on_click(|_, _, _| {}),
                    ),
            )
    }
}
