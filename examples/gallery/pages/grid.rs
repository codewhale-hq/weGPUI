use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::*;
use wegpui::components::grid::{Grid, Grids};

pub struct GridPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| GridPage { route }).into()
}

impl Render for GridPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Grid")
            .desc(tr("九宫格"))
            .child(Grids::new("grids").children((0..9).map(|i| {
                Grid::new(("grid", i as usize))
                    .image("icon_tabbar.png")
                    .label("Grid")
                    .on_click(|_, _, _| {})
            })))
    }
}
