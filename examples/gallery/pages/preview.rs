use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::form_preview::{FormPreview, FormPreviewButton};

const LONG: &str = "很长很长的名字很长很长的名字很长很长的名字很长很长的名字很长很长的名字";

pub struct PreviewPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| PreviewPage { route }).into()
}

fn preview(id: &'static str) -> FormPreview {
    FormPreview::new(id)
        .header("付款金额", "¥2400.00")
        .item("商品", "电动打蛋机")
        .item("标题标题", "名字名字名字")
        .item("标题标题", LONG)
}

impl Render for PreviewPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Preview")
            .desc("表单预览")
            .child(
                preview("preview-1")
                    .button(FormPreviewButton::primary("操作").on_click(|_, _, _| {})),
            )
            .child(div().h(px(25.6)))
            .child(
                preview("preview-2")
                    .button(FormPreviewButton::default("辅助操作").on_click(|_, _, _| {}))
                    .button(FormPreviewButton::primary("操作").on_click(|_, _, _| {})),
            )
    }
}
