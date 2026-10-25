use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::footer::{Footer, FooterLink},
};

pub struct FooterPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| FooterPage { route }).into()
}

const COPYRIGHT: &str = "Copyright © 2008-2022 weui.io";
const BR: f32 = 16. * 1.6 * 2.;

impl Render for FooterPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let link = Theme::get(cx).link;
        PageFrame::new(self.route)
            .title("Footer")
            .desc(tr("页脚"))
            .spacing()
            .child(Footer::new().text(COPYRIGHT))
            .child(div().h(px(BR)))
            .child(
                Footer::new()
                    .link(FooterLink::new("footer-link-1", tr("底部链接")).on_click(|_, _, _| {}))
                    .text(COPYRIGHT),
            )
            .child(div().h(px(BR)))
            .child(
                Footer::new()
                    .link(FooterLink::new("footer-link-2a", tr("底部链接")).on_click(|_, _, _| {}))
                    .link(FooterLink::new("footer-link-2b", tr("底部链接")).on_click(|_, _, _| {}))
                    .text(COPYRIGHT),
            )
            .fixed(
                Footer::new()
                    .fixed_bottom()
                    .link(
                        FooterLink::new("footer-home", tr("WeUI首页"))
                            .on_click(|_, w, cx| crate::home(w, cx)),
                    )
                    .text(COPYRIGHT)
                    .child(
                        div()
                            .id("page-copyright")
                            .role(Role::Link)
                            .aria_label(tr("备案号：粤B2-20090059"))
                            .text_color(link)
                            .child(tr("备案号：粤B2-20090059")),
                    ),
            )
    }
}
