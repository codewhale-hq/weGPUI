use crate::text::tr;
use crate::{
    chrome::{PageFrame, themed_image},
    pages::Args,
};
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};
use wegpui::{
    Theme,
    components::{Cell, Cells, CellsStyle, modal::ease},
    scrolling::RevealFocus,
};

type Category = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
);

const CATEGORIES: &[Category] = &[
    (
        "表单",
        "icon_nav_form.png",
        &[
            ("button", "Button"),
            ("form", "Form"),
            ("list", "List"),
            ("slider", "Slider"),
            ("uploader", "Uploader"),
        ],
    ),
    (
        "基础组件",
        "icon_nav_layout.png",
        &[
            ("article", "Article"),
            ("badge", "Badge"),
            ("flex", "Flex"),
            ("footer", "Footer"),
            ("gallery", "Gallery"),
            ("grid", "Grid"),
            ("icons", "Icons"),
            ("loading", "Loading"),
            ("loadmore", "Loadmore"),
            ("panel", "Panel"),
            ("preview", "Preview"),
            ("progress", "Progress"),
            ("steps", "Steps"),
            ("languages", "Languages"),
        ],
    ),
    (
        "操作反馈",
        "icon_nav_feedback.png",
        &[
            ("actionsheet", "Actionsheet"),
            ("dialog", "Dialog"),
            ("half-screen-dialog", "Half-screen Dialog"),
            ("msg", "Msg"),
            ("picker", "Picker"),
            ("toast", "Toast"),
            ("information-bar", "Information Bar"),
        ],
    ),
    (
        "导航相关",
        "icon_nav_nav.png",
        &[("navbar", "Navbar"), ("tabbar", "Tabbar")],
    ),
    (
        "搜索相关",
        "icon_nav_search.png",
        &[("searchbar", "Search Bar")],
    ),
];

pub struct Home {
    open: Option<usize>,
    changed: Option<Instant>,
}

pub fn build(_: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let open = args
        .state
        .as_deref()
        .and_then(|s| s.strip_prefix("category-"))
        .and_then(|s| s.parse().ok());
    cx.new(|_| Home {
        open,
        changed: None,
    })
    .into()
}

impl Home {
    fn toggle(&mut self, index: usize, cx: &mut Context<Self>) {
        self.open = if self.open == Some(index) {
            None
        } else {
            Some(index)
        };
        self.changed = Some(Instant::now());
        cx.notify();
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let p = match self.changed {
            Some(at) if !cx.reduce_motion() => {
                let linear =
                    (at.elapsed().as_secs_f32() / Duration::from_millis(300).as_secs_f32()).min(1.);
                if linear < 1. {
                    window.request_animation_frame();
                }
                ease(linear)
            }
            _ => 1.,
        };
        let header = div()
            .p(px(40.))
            .child(
                div().mb(px(15.)).child(
                    img(themed_image("logo.png", cx))
                        .w(px(21. * 273. / 93.))
                        .h(px(21.)),
                ),
            )
            .child(
                div()
                    .mt(px(4.))
                    .text_size(px(14.))
                    .line_height(relative(1.6))
                    .text_color(t.fg_1)
                    .child(tr("WeUI 是一套同微信原生视觉体验一致的基础样式库，由微信官方设计团队为微信内网页和微信小程序量身设计，令用户的使用感知更加统一。")),
            );
        let brand = t.brand;
        let category = |i: usize,
                        label: &'static str,
                        icon: &'static str,
                        expanded: bool,
                        cx: &mut Context<Self>| {
            div()
                .id(("category", i))
                .role(Role::Button)
                .aria_label(label)
                .aria_expanded(expanded)
                .focusable()
                .tab_stop(true)
                .flex()
                .items_center()
                .p(px(18.))
                .border_2()
                .border_color(transparent_black())
                .focus_visible(move |s| s.border_color(brand))
                .when(expanded, |el| el.opacity(0.5))
                .cursor_pointer()
                .child(div().flex_1().child(label))
                .child(img(themed_image(icon, cx)).size(px(30.)))
        };
        let mut list = div().flex().flex_col();
        for (i, (label, icon, items)) in CATEGORIES.iter().enumerate() {
            let expanded = self.open == Some(i);
            let mut li = div()
                .mt(px(if i == 0 { 0. } else { 8. }))
                .bg(t.bg_2)
                .rounded(px(2.))
                .overflow_hidden()
                .child(
                    category(i, tr(label), icon, expanded, cx)
                        .on_click(cx.listener(move |this, _, _, cx| this.toggle(i, cx)))
                        .reveal_focus(),
                );
            if expanded {
                let cells = Cells::new()
                    .style(CellsStyle::Bare)
                    .margin_top(0.)
                    .separator_inset(20., 20.)
                    .role(Role::Menu)
                    .children(items.iter().map(|(route, title)| {
                        let route: &'static str = route;
                        Cell::new(SharedString::from(format!("home-{route}")))
                            .role(Role::MenuItem)
                            .title(*title)
                            .access()
                            .padding(Edges {
                                top: px(16.),
                                bottom: px(16.),
                                left: px(20.),
                                right: px(20.),
                            })
                            .on_click(move |_, w, cx| crate::go(route, w, cx))
                    }));
                li = li.child(
                    div()
                        .relative()
                        .top(px(-(1. - p) * 0.5 * 56. * items.len() as f32))
                        .opacity(p)
                        .child(cells),
                );
            }
            list = list.child(li);
        }
        list = list.child(
            div()
                .mt(px(8.))
                .bg(t.bg_2)
                .rounded(px(2.))
                .overflow_hidden()
                .child(
                    div()
                        .id("layers")
                        .role(Role::MenuItem)
                        .aria_label(tr("层级规范"))
                        .focusable()
                        .tab_stop(true)
                        .flex()
                        .items_center()
                        .p(px(18.))
                        .border_2()
                        .border_color(transparent_black())
                        .focus_visible(move |s| s.border_color(brand))
                        .cursor_pointer()
                        .child(div().flex_1().child(tr("层级规范")))
                        .child(img(themed_image("icon_nav_z-index.png", cx)).size(px(30.)))
                        .on_click(|_, w, cx| crate::go("layers", w, cx))
                        .reveal_focus(),
                ),
        );
        PageFrame::new("home").header(header).spacing().child(
            div()
                .id("menubar")
                .role(Role::MenuBar)
                .aria_label(tr("WeUI组件列表"))
                .child(list),
        )
    }
}
