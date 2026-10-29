use crate::text::tr;
use gpui::{prelude::*, *};
use wegpui::Theme;
use wegpui::scrolling::{FocusScroll, RevealFocus};

pub struct PendingScroll(pub f32);
impl Global for PendingScroll {}

pub fn go(route: &'static str) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    move |_, window, cx| crate::go(route, window, cx)
}

pub fn themed_image(name: &str, cx: &App) -> SharedString {
    if Theme::get(cx).is_dark {
        SharedString::from(name.replace(".png", "-dark.png"))
    } else {
        SharedString::from(name.to_string())
    }
}

const BG2_PAGES: &[&str] = &[
    "actionsheet",
    "dialog",
    "msg",
    "msg_text",
    "msg_text_primary",
    "msg_success",
    "msg_warn",
    "toast",
    "article",
    "form",
    "loadmore",
    "picker",
    "top-tips",
    "steps",
    "steps_vertical",
    "steps_horizonal",
    "progress",
    "button_bottom_fixed",
];

fn footer_hidden(route: &str) -> bool {
    route.starts_with("form_")
        || route.starts_with("msg_")
        || route.starts_with("button_")
        || matches!(route, "footer" | "gallery" | "navbar" | "tabbar")
}

#[derive(IntoElement)]
pub struct PageFrame {
    route: &'static str,
    title: Option<SharedString>,
    desc: Option<SharedString>,
    header: Option<AnyElement>,
    spacing: bool,
    bg: Option<Hsla>,
    scroll: bool,
    body: Vec<AnyElement>,
    fixed: Vec<AnyElement>,
}

impl PageFrame {
    pub fn new(route: &'static str) -> Self {
        Self {
            route,
            title: None,
            desc: None,
            header: None,
            spacing: false,
            bg: None,
            scroll: true,
            body: vec![],
            fixed: vec![],
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }
    pub fn spacing(mut self) -> Self {
        self.spacing = true;
        self
    }
    pub fn bg(mut self, bg: Hsla) -> Self {
        self.bg = Some(bg);
        self
    }
    pub fn no_scroll(mut self) -> Self {
        self.scroll = false;
        self
    }
    pub fn fixed(mut self, element: impl IntoElement) -> Self {
        self.fixed.push(element.into_any_element());
        self
    }
}

impl ParentElement for PageFrame {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

pub fn page_hd(title: impl IntoElement, desc: Option<SharedString>, cx: &App) -> Div {
    let t = Theme::get(cx);
    div()
        .p(px(40.))
        .child(
            div()
                .text_size(px(20.))
                .line_height(relative(1.6))
                .child(title),
        )
        .when_some(desc, |el, desc| {
            el.child(
                div()
                    .mt(px(4.))
                    .text_size(px(14.))
                    .line_height(relative(1.6))
                    .text_color(t.fg_1)
                    .child(desc),
            )
        })
}

pub fn page_ft(cx: &App) -> impl IntoElement {
    div().pt(px(40.)).pb(px(32.)).flex().justify_center().child(
        div()
            .id("page-footer-home")
            .role(Role::Link)
            .aria_label(tr("WeUI 首页"))
            .cursor_pointer()
            .focusable()
            .tab_stop(true)
            .rounded(px(4.))
            .border_2()
            .border_color(transparent_black())
            .focus_visible(|s| s.shadow(wegpui::components::form::dual_focus_ring()))
            .active(|s| s.opacity(0.5))
            .on_click(|_, w, cx| crate::home(w, cx))
            .child(
                img(themed_image("icon_footer_link.png", cx))
                    .w(px(84.))
                    .h(px(19.)),
            )
            .reveal_focus(),
    )
}

impl RenderOnce for PageFrame {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let bg = self.bg.unwrap_or(if BG2_PAGES.contains(&self.route) {
            t.bg_2
        } else {
            t.bg_0
        });
        let show_footer = !footer_hidden(self.route);
        let handle = window
            .use_keyed_state(
                SharedString::from(format!("scroll-{}", self.route)),
                cx,
                |_, _| ScrollHandle::new(),
            )
            .read(cx)
            .clone();
        if let Some(PendingScroll(y)) = cx.try_global::<PendingScroll>() {
            handle.set_offset(point(px(0.), px(-*y)));
            cx.remove_global::<PendingScroll>();
        }
        let header = self.header.or_else(|| {
            self.title
                .map(|title| page_hd(title, self.desc, cx).into_any_element())
        });
        let is_desktop = cfg!(not(target_os = "ios"))
            && window.viewport_size().width >= px(crate::desktop::BREAKPOINT);
        let content = div()
            .w_full()
            .when(is_desktop, |content| content.max_w(px(800.)).mx_auto())
            .min_h_full()
            .flex()
            .flex_col()
            .children(header)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .when(self.spacing, |el| el.px(px(16.)))
                    .children(self.body),
            )
            .when(show_footer, |el| el.child(page_ft(cx)));
        div()
            .relative()
            .size_full()
            .bg(bg)
            .child(FocusScroll::new(
                SharedString::from(format!("focus-scroll-{}", self.route)),
                &handle,
                div()
                    .id(SharedString::from(format!("page-{}", self.route)))
                    .size_full()
                    .when(self.scroll, |el| {
                        el.overflow_y_scroll().track_scroll(&handle)
                    })
                    .child(content),
            ))
            .children(self.fixed)
    }
}
