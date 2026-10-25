use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    Cell, Cells, CellsStyle,
    panel::{MediaBox, Panel},
};
use wegpui::{I18n, logical::Logical};

const DESC: &str = "由各种物质组成的巨型球状天体，叫做星球。星球有一定的形状，有自己的运行轨道。";

pub struct PanelPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| PanelPage { route }).into()
}

fn more(id: &'static str) -> Cell {
    Cell::new(id)
        .title(tr("查看更多"))
        .link()
        .access()
        .on_click(|_, _, _| {})
}

fn small_cell(id: &'static str, dir: wegpui::Direction) -> Cell {
    Cell::new(id)
        .access()
        .aria_label(tr("文字标题"))
        .hd(img("pic_placeholder_46.png").size(px(20.)).me(dir, px(16.)))
        .bd(tr("文字标题"))
        .on_click(|_, _, _| {})
}

impl Render for PanelPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dir = I18n::get(cx).direction();
        PageFrame::new(self.route)
            .title("Panel")
            .desc(tr("面板"))
            .child(
                div()
                    .pb(px(20.))
                    .flex()
                    .flex_col()
                    .child(
                        Panel::new("panel-appmsg")
                            .first()
                            .title(tr("图文组合列表"))
                            .child(
                                MediaBox::appmsg("p1m1", "pic_placeholder_120.png")
                                    .title(tr("标题一"))
                                    .desc(tr(DESC))
                                    .on_click(|_, _, _| {}),
                            )
                            .child(
                                MediaBox::appmsg("p1m2", "pic_placeholder_120.png")
                                    .title(tr("标题二"))
                                    .desc(tr(DESC))
                                    .on_click(|_, _, _| {}),
                            )
                            .footer(more("p1-more")),
                    )
                    .child(
                        Panel::new("panel-text")
                            .title(tr("文字组合列表"))
                            .child(MediaBox::text("p2m1").title(tr("标题一")).desc(tr(DESC)))
                            .child(MediaBox::text("p2m2").title(tr("标题二")).desc(tr(DESC)))
                            .footer(more("p2-more")),
                    )
                    .child(
                        Panel::new("panel-small").title(tr("小图文组合列表")).child(
                            MediaBox::small_appmsg("p3m1").child(
                                Cells::new()
                                    .style(CellsStyle::Bare)
                                    .separator_inset(52., 0.)
                                    .margin_top(0.)
                                    .child(small_cell("t1", dir))
                                    .child(small_cell("t2", dir)),
                            ),
                        ),
                    )
                    .child(
                        Panel::new("panel-source")
                            .title(tr("文字列表附来源"))
                            .child(
                                MediaBox::text("p4m1")
                                    .role(Role::Link)
                                    .title(tr("标题一"))
                                    .desc(tr(DESC))
                                    .meta(tr("文字来源"))
                                    .meta(tr("时间"))
                                    .meta_extra(tr("其它信息"))
                                    .aria_description(tr("文字来源，时间，其它信息")),
                            ),
                    ),
            )
    }
}
