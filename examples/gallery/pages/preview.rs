use crate::text::tr;
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
        .header(tr("付款金额"), "¥2400.00")
        .item(tr("商品"), tr("电动打蛋机"))
        .item(tr("标题标题"), tr("名字名字名字"))
        .item(tr("标题标题"), tr(LONG))
}

impl Render for PreviewPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Preview")
            .desc(tr("表单预览"))
            .child(
                preview("preview-1")
                    .button(FormPreviewButton::primary(tr("操作")).on_click(|_, _, _| {})),
            )
            .child(div().h(px(25.6)))
            .child(
                preview("preview-2")
                    .button(FormPreviewButton::default(tr("辅助操作")).on_click(|_, _, _| {}))
                    .button(FormPreviewButton::primary(tr("操作")).on_click(|_, _, _| {})),
            )
    }
}
