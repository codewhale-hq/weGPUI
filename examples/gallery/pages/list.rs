use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    Cell, Cells,
    swiped::{SwipedAction, SwipedCell},
};
use wegpui::{I18n, logical::Logical};

pub struct ListPage {
    route: &'static str,
    open: bool,
}

pub fn build(route: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let open = args.state.as_deref() != Some("closed");
    cx.new(|_| ListPage { route, open }).into()
}

fn icon(dir: wegpui::Direction) -> impl IntoElement {
    img("pic_placeholder_46.png").size(px(20.)).me(dir, px(16.))
}

impl Render for ListPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dir = I18n::get(cx).direction();
        PageFrame::new(self.route)
            .title("List")
            .desc(tr("列表"))
            .child(
                Cells::new()
                    .title(tr("带说明的列表项"))
                    .child(
                        Cell::new("l1")
                            .role(Role::ListBoxOption)
                            .title(tr("标题文字"))
                            .value(tr("说明文字")),
                    )
                    .child(
                        SwipedCell::new("swiped")
                            .aria_label(tr("标题文字 说明文字"))
                            .default_open(self.open)
                            .content(Cell::new("l2").title(tr("标题文字")).value(tr("说明文字")))
                            .action(
                                SwipedAction::new("swiped-delete", tr("删除"))
                                    .warn()
                                    .on_click(|_, _, _| {}),
                            ),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("带图标、说明的列表项"))
                    .separator_inset(52., 0.)
                    .children((0..2).map(|i| {
                        Cell::new(("icon-cell", i as usize))
                            .role(Role::ListBoxOption)
                            .hd(icon(dir))
                            .title(tr("标题文字"))
                            .value(tr("说明文字"))
                    })),
            )
            .child(
                Cells::new()
                    .title(tr("带跳转的列表项"))
                    .child(
                        Cell::new("a1")
                            .access()
                            .title("cell standard")
                            .desc(tr("副标题"))
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Cell::new("a2")
                            .access()
                            .title("cell standard")
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("带说明、跳转的列表项"))
                    .children((0..2).map(|i| {
                        Cell::new(("tl", i as usize))
                            .access()
                            .aria_label(tr("cell standard 说明文字"))
                            .title("cell standard")
                            .value(tr("说明文字"))
                            .on_click(|_, _, _| {})
                    })),
            )
            .child(
                Cells::new()
                    .title(tr("带图标、说明、跳转的列表项"))
                    .separator_inset(52., 0.)
                    .children((0..2).map(|i| {
                        Cell::new(("itl", i as usize))
                            .access()
                            .aria_label(tr("cell standard 说明文字"))
                            .hd(icon(dir))
                            .title("cell standard")
                            .value(tr("说明文字"))
                            .on_click(|_, _, _| {})
                    })),
            )
    }
}
