use crate::{chrome::PageFrame, pages::Args};
use gpui::*;
use wegpui::components::loadmore::LoadMore;

pub struct LoadMorePage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| LoadMorePage { route }).into()
}

impl Render for LoadMorePage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Loadmore")
            .desc("加载更多")
            .child(LoadMore::loading("正在加载").margin(20., 0.))
            .child(LoadMore::line("暂无数据").margin(32., 0.))
            .child(LoadMore::dot("已无更多数据").margin(32., 20.))
    }
}
