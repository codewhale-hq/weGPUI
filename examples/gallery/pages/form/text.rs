use super::primary_button;
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::components::{
    form::{CellKind, Form, FormCell, FormGroup},
    textarea::TextArea,
};

pub struct Text {
    route: &'static str,
    area: Entity<TextArea>,
}

impl Text {
    pub fn new(route: &'static str, _: &mut Window, cx: &mut Context<Self>) -> Self {
        let area = cx.new(|cx| {
            let mut area = TextArea::new("请描述你所发生的问题", cx);
            area.set_aria_label("问题描述", cx);
            area.set_max_len(Some(200), cx);
            area.set_show_count(true, cx);
            area
        });
        cx.observe(&area, |_, _, cx| cx.notify()).detach();
        Self { route, area }
    }
}

impl Render for Text {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let vertical = self.route == "form_vertical";
        let cell = FormCell::new("cell-textarea")
            .when(vertical, |c| c.kind(CellKind::Vertical).label("问题描述"))
            .label_for(self.area.read(cx).input())
            .child(self.area.clone());
        let group = FormGroup::new("group-1").first();
        let group = if vertical {
            group.row(cell)
        } else {
            group.title("问题描述").row(cell)
        };
        let form = Form::new("weui-form")
            .title(if vertical {
                "上下结构"
            } else {
                "文本域"
            })
            .desc(if vertical {
                "上下结构样式"
            } else {
                "输入更多内容的输入区域样式展示"
            })
            .child(group)
            .opr_area(primary_button("showTooltips", "确定", false, |_, _, _| {}));
        PageFrame::new(self.route).child(form)
    }
}
