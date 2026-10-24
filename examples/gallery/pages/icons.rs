use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::Icon;

pub struct IconsPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| IconsPage { route }).into()
}

const BOXES: &[(&str, bool, &str, &str)] = &[
    ("success", false, "成功", "用于表示操作顺利达成"),
    (
        "info",
        false,
        "提示",
        "用于表示信息提示；也常用于缺乏条件的操作拦截，提示用户所需信息",
    ),
    (
        "warn",
        true,
        "普通警告",
        "用于表示操作后将引起一定后果的情况",
    ),
    (
        "warn",
        false,
        "强烈警告",
        "用于表示操作后将引起严重的不可挽回的后果的情况",
    ),
    ("waiting", false, "等待", "用于表示等待"),
];

impl Render for IconsPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fg = wegpui::Theme::get(cx).fg_0;
        PageFrame::new(self.route)
            .title("Icons")
            .desc("图标")
            .child(
                div()
                    .px(px(40.))
                    .children(BOXES.iter().map(|&(name, primary, title, desc)| {
                        let icon = Icon::new(name).label(title);
                        let icon = if primary {
                            icon.msg_primary()
                        } else {
                            icon.msg()
                        };
                        div()
                            .mb(px(25.))
                            .flex()
                            .items_center()
                            .child(div().mr(px(18.)).child(icon))
                            .child(
                                div()
                                    .flex_shrink(100.)
                                    .min_w_0()
                                    .child(
                                        div()
                                            .text_size(px(18.72))
                                            .line_height(relative(1.6))
                                            .text_color(fg)
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .mt(px(6.))
                                            .text_size(px(12.))
                                            .line_height(relative(1.6))
                                            .text_color(rgb(0x888888))
                                            .child(desc),
                                    ),
                            )
                    })),
            )
    }
}
