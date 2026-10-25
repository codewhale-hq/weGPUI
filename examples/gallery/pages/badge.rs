use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Direction, I18n, Theme,
    components::{Cell, Cells, badge::Badge},
    logical::Logical,
};

pub struct BadgePage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| BadgePage { route }).into()
}

fn titled(title: &'static str, badge: Badge, dir: Direction) -> Div {
    div()
        .flex()
        .row(dir)
        .items_center()
        .h(px(25.33))
        .child(title)
        .child(div().ms(dir, px(5.)).child(badge))
}

impl Render for BadgePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dir = I18n::get(cx).direction();
        let t = Theme::get(cx);
        let fg1 = t.fg_1;
        PageFrame::new(self.route)
            .title("Badge")
            .desc(tr("徽章"))
            .child(
                Cells::new()
                    .title(tr("新消息提示跟摘要信息后，统一在列表右侧"))
                    .child(
                        Cell::new("badge-l1")
                            .role(Role::Link)
                            .aria_label(tr("单行列表 详细信息，有更新"))
                            .access()
                            .bd(tr("单行列表"))
                            .ft(div()
                                .flex()
                                .items_center()
                                .text_color(fg1)
                                .child(tr("详细信息"))
                                .child(div().mx(px(5.)).child(Badge::dot())))
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("未读数红点跟在主题信息后，统一在列表左侧"))
                    .child(
                        Cell::new("badge-b1")
                            .role(Role::ListBoxOption)
                            .aria_label(tr("联系人名称，8个新通知"))
                            .active()
                            .hd(div()
                                .relative()
                                .me(dir, px(10.))
                                .child(img("pic_160.png").size(px(50.)))
                                .child(
                                    div()
                                        .absolute()
                                        .top(px(-4.8))
                                        .inset_e(dir, px(-4.8))
                                        .child(Badge::count(8)),
                                ))
                            .bd(div().flex().flex_col().child(tr("联系人名称")).child(
                                div()
                                    .text_size(px(13.))
                                    .line_height(relative(1.6))
                                    .text_color(rgb(0x888888))
                                    .child(tr("摘要信息")),
                            ))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b2")
                            .role(Role::Link)
                            .aria_label(tr("单行列表，8个新通知"))
                            .access()
                            .bd(titled(tr("单行列表"), Badge::count(8), dir))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b3")
                            .role(Role::Link)
                            .aria_label(tr("单行列表 详细信息，8个新通知"))
                            .access()
                            .bd(titled(tr("单行列表"), Badge::count(8), dir))
                            .value(tr("详细信息"))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("badge-b4")
                            .role(Role::Link)
                            .aria_label(tr("单行列表，New"))
                            .access()
                            .bd(titled(tr("单行列表"), Badge::new("New"), dir))
                            .on_click(|_, _, _| {}),
                    ),
            )
    }
}
