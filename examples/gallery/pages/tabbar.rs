use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    badge::Badge,
    navbar::tab_panel,
    tabbar::{Tabbar, TabbarItem},
};

const PANELS: [&str; 4] = [
    "Lorem repellat eius consectetur ab pariatur! Facere cum dignissimos sequi error perferendis quis Ipsa reiciendis earum cumque reprehenderit tenetur ab vero modi. Soluta tempore minima tempora quae voluptate! Sunt delectus?",
    "Dolor eum rerum magnam voluptatem omnis fuga, dolore libero quibusdam cupiditate Sequi vero rem corrupti hic veritatis Atque exercitationem odit odit ducimus esse, sapiente laborum inventore! Ipsum harum quo minus",
    "Elit deserunt dolore nemo harum neque libero numquam, minima nam Atque nesciunt iusto aperiam eligendi praesentium ratione? Quasi iste odio itaque doloribus hic? Est aliquam recusandae dolore ratione quos. Saepe",
    "Amet dignissimos doloribus voluptate maxime dolorem quia Deleniti eligendi similique nisi corrupti eius aut Unde nesciunt quos quos sapiente dolorem? Odit soluta repudiandae accusantium ducimus totam accusamus. Rem ad numquam",
];
const TABS: [&str; 4] = ["微信", "通讯录", "发现", "我"];

pub struct TabbarPage {
    selected: usize,
}

pub fn build(_: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let selected = match args.state.as_deref() {
        Some("tab2") => 1,
        Some("tab3") => 2,
        Some("tab4") => 3,
        _ => 0,
    };
    cx.new(|_| TabbarPage { selected }).into()
}

fn icon() -> Img {
    img("icon_tabbar.png").size(px(28.))
}

impl Render for TabbarPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected;
        let entity = cx.entity().downgrade();
        PageFrame::new("tabbar").no_scroll().fixed(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    tab_panel(("panel", selected), TABS[selected])
                        .p(px(32.))
                        .child(PANELS[selected]),
                )
                .child(
                    Tabbar::new("tabbar")
                        .label("选项卡标题")
                        .item(
                            TabbarItem::new(TABS[0], icon())
                                .badge(Badge::new("8"), -2., -13.)
                                .description("8"),
                        )
                        .item(TabbarItem::new(TABS[1], icon()))
                        .item(
                            TabbarItem::new(TABS[2], icon())
                                .badge(Badge::dot(), 0., -6.)
                                .description("new"),
                        )
                        .item(TabbarItem::new(TABS[3], icon()))
                        .selected(selected)
                        .on_select(move |i, _, cx| {
                            entity
                                .update(cx, |this, cx| {
                                    this.selected = i;
                                    cx.notify();
                                })
                                .ok();
                        }),
                ),
        )
    }
}
