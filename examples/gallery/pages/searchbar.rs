use crate::{
    chrome::{PageFrame, go},
    pages::Args,
};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{
        button::Button,
        search_bar::{SearchBar, SearchBarEvent, SearchBarVariant, SearchResults},
    },
};

pub struct SearchbarPage {
    route: &'static str,
    bar: Option<Entity<SearchBar>>,
    _subscription: Option<Subscription>,
}

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| SearchbarPage::new(route, state.as_deref(), window, cx))
        .into()
}

impl SearchbarPage {
    fn new(
        route: &'static str,
        state: Option<&str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let variant = match route {
            "searchbar_filled" => Some(SearchBarVariant::Filled),
            "searchbar_grey" => Some(SearchBarVariant::FilledGrey),
            "searchbar_outlined" => Some(SearchBarVariant::Outlined),
            "searchbar_homepage" => Some(SearchBarVariant::Homepage),
            _ => None,
        };
        let bar = variant.map(|variant| {
            cx.new(|cx| {
                let bar = SearchBar::new(cx).variant(variant);
                match variant {
                    SearchBarVariant::Filled => bar.words("微信"),
                    SearchBarVariant::FilledGrey => bar.words("微信").clear_on_cancel(false),
                    _ => bar,
                }
            })
        });
        let subscription = bar
            .as_ref()
            .map(|bar| cx.subscribe(bar, |_, _, _: &SearchBarEvent, cx| cx.notify()));
        if let Some(bar) = bar.as_ref() {
            match state {
                Some("focused") => bar.update(cx, |bar, cx| bar.activate(window, cx)),
                Some("typed") => bar.update(cx, |bar, cx| {
                    bar.activate(window, cx);
                    bar.set_text("weui", cx);
                }),
                Some("filled") => bar.update(cx, |bar, cx| bar.set_text("weui", cx)),
                _ => {}
            }
        }
        Self {
            route,
            bar,
            _subscription: subscription,
        }
    }
}

impl Render for SearchbarPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let desc = match self.route {
            "searchbar_filled" => "搜索栏 Filled",
            "searchbar_grey" => "搜索栏 Grey",
            "searchbar_outlined" => "搜索栏 Outlined",
            "searchbar_homepage" => "搜索栏 Homepage",
            _ => "搜索栏",
        };
        let frame = PageFrame::new(self.route).title("SearchBar").desc(desc);
        let Some(bar) = self.bar.clone() else {
            return frame.spacing().child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        Button::new("Filled (default)")
                            .id("to-filled")
                            .on_click(go("searchbar_filled")),
                    )
                    .child(
                        Button::new("Filled Grey")
                            .id("to-grey")
                            .on_click(go("searchbar_grey")),
                    )
                    .child(
                        Button::new("Outlined")
                            .id("to-outlined")
                            .on_click(go("searchbar_outlined")),
                    )
                    .child(
                        Button::new("Homepage")
                            .id("to-homepage")
                            .on_click(go("searchbar_homepage")),
                    ),
            );
        };
        let show_results = self.route == "searchbar_filled" && bar.read(cx).is_expanded(cx);
        let frame = if matches!(self.route, "searchbar_filled" | "searchbar_homepage") {
            frame.bg(t.bg_3)
        } else {
            frame
        };
        frame.child(bar).when(show_results, |el| {
            el.child(
                SearchResults::new("searchResult").items(std::iter::repeat_n("实时搜索文本", 4)),
            )
        })
    }
}
