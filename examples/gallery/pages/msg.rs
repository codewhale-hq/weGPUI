use crate::text::tr;
use crate::{
    chrome::{self, PageFrame},
    pages::Args,
};
use gpui::{prelude::*, *};
use wegpui::components::{
    Cell, Cells, CellsStyle,
    button::Button,
    footer::{Footer, FooterLink},
    form_preview::FormPreviewList,
    information_bar::ListTips,
    msg::{LinkText, Msg, MsgIcon},
};

const DESC: &str = "内容详情，可根据实际需要安排，如果换行则不超过规定长度，居中展现";
const TIPS: &str = "提示详情，可根据实际需要安排，如果换行则不超过规定长度，居中展现";

pub struct MsgPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| MsgPage { route }).into()
}

fn desc_link() -> LinkText {
    LinkText::new("msg-desc")
        .text(tr(DESC))
        .link(tr("文字链接"), |_, _| {})
}

fn tips_link() -> LinkText {
    LinkText::new("msg-tips")
        .text(tr(TIPS))
        .link(tr("文字链接"), |_, _| {})
}

fn primary() -> Button {
    Button::new(tr("推荐操作"))
        .id("msg-primary")
        .primary()
        .on_click(|_, window, cx| crate::back(window, cx))
}

fn footer() -> Footer {
    Footer::new()
        .link(FooterLink::new("msg-footer-link", tr("底部链接文本")).on_click(|_, _, _| {}))
        .text("Copyright © 2008-2016 weui.io")
}

fn custom_cells() -> impl IntoElement {
    div().flex().flex_col().mt(px(-24.)).mx(px(-16.)).child(
        Cells::new()
            .style(CellsStyle::Form)
            .child(
                Cell::new("msg-cell-1")
                    .title("cell standard")
                    .access()
                    .form()
                    .on_click(|_, _, _| {}),
            )
            .child(
                Cell::new("msg-cell-2")
                    .title("cell standard")
                    .access()
                    .form()
                    .on_click(|_, _, _| {}),
            ),
    )
}

impl Render for MsgPage {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let msg = match self.route {
            "msg_success" => Msg::new("msg")
                .icon(MsgIcon::Success)
                .title(tr("操作成功"))
                .desc(desc_link())
                .opr(primary())
                .tips(tips_link())
                .extra(footer()),
            "msg_warn" => Msg::new("msg")
                .icon(MsgIcon::Warn)
                .title(tr("操作失败"))
                .desc(desc_link())
                .tips(tips_link())
                .opr(
                    Button::new(tr("辅助操作"))
                        .id("msg-default")
                        .on_click(|_, window, cx| crate::back(window, cx)),
                )
                .extra(footer()),
            "msg_text" => Msg::new("msg")
                .title(tr("操作成功"))
                .desc(desc_link())
                .opr(primary()),
            "msg_text_primary" => Msg::new("msg")
                .desc(tr(DESC))
                .desc_primary(tr(DESC))
                .opr(primary())
                .tips(tips_link()),
            "msg_custom_area_preview" => Msg::new("msg")
                .icon(MsgIcon::Success)
                .title(tr("操作成功"))
                .desc(desc_link())
                .custom_area(
                    FormPreviewList::new()
                        .item(tr("姓名"), tr("张三"))
                        .item(tr("微信号"), "zhangsan"),
                )
                .opr(primary()),
            "msg_custom_area_tips" => Msg::new("msg")
                .icon(MsgIcon::Success)
                .title(tr("操作成功"))
                .desc(desc_link())
                .custom_area(
                    ListTips::new()
                        .item(tr("列表提示项"))
                        .item(tr("列表提示项"))
                        .item(tr("列表提示项")),
                )
                .opr(primary()),
            "msg_custom_area_cell" => Msg::new("msg")
                .icon(MsgIcon::Success)
                .title(tr("操作成功"))
                .desc(desc_link())
                .custom_area(custom_cells())
                .opr(primary()),
            _ => return index(self.route).into_any_element(),
        };
        PageFrame::new(self.route)
            .child(
                div()
                    .min_h(window.viewport_size().height)
                    .flex()
                    .flex_col()
                    .child(msg),
            )
            .into_any_element()
    }
}

fn index(route: &'static str) -> PageFrame {
    let links: [(&str, &'static str); 7] = [
        (tr("成功提示页"), "msg_success"),
        (tr("失败提示页"), "msg_warn"),
        (tr("无图标提示页"), "msg_text"),
        (tr("无图标提示页"), "msg_text_primary"),
        (tr("key-value场景"), "msg_custom_area_preview"),
        (tr("描述列表场景"), "msg_custom_area_tips"),
        (tr("跳转列表场景"), "msg_custom_area_cell"),
    ];
    PageFrame::new(route)
        .title("Msg")
        .desc(tr("提示页"))
        .spacing()
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(16.))
                .children(links.into_iter().map(|(label, target)| {
                    Button::new(label).id(target).on_click(chrome::go(target))
                })),
        )
}
