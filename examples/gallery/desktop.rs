use gpui::{prelude::*, *};
use wegpui::{
    I18n, Theme, ThemeMode,
    components::{Cell, button::Button},
    logical::Logical,
    scrolling::FocusScroll,
};

pub const BREAKPOINT: f32 = 960.;
pub const SIDEBAR: f32 = 224.;

pub fn title(route: &str) -> String {
    match route {
        route
            if super::pages::macos::EXAMPLES
                .iter()
                .any(|(name, _)| *name == route) =>
        {
            super::pages::macos::EXAMPLES
                .iter()
                .find(|(name, _)| *name == route)
                .unwrap()
                .1
                .into()
        }
        "home" => "Overview".into(),
        "desktop-form" => "Desktop form".into(),
        "button_default" => "Button styles".into(),
        "button_bottom_fixed" => "Fixed actions".into(),
        "form_primary" => "Primary form".into(),
        "form_vcode" => "Verification code".into(),
        "form_input_status" => "Input states".into(),
        "form_page_old" => "Classic form".into(),
        "form_select_primary" => "Select controls".into(),
        "msg" => "Messages".into(),
        "msg_success" => "Success message".into(),
        "msg_warn" => "Warning message".into(),
        "languages" => "Languages & typography".into(),
        "navbar" => "Navigation tabs".into(),
        "tabbar" => "Tab bar".into(),
        "actionsheet" => "Action sheet".into(),
        "searchbar" => "Search".into(),
        "loadmore" => "Load more".into(),
        _ => {
            let words = route.replace(['_', '-'], " ").replace("msg ", "Message ");
            let mut chars = words.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        }
    }
}

fn category(route: &str) -> &'static str {
    match route {
        route
            if super::pages::macos::EXAMPLES
                .iter()
                .any(|(name, _)| *name == route) =>
        {
            "Mac desktop"
        }
        "home" | "desktop-form" => "Start here",
        "button" | "button_default" | "button_bottom_fixed" => "Actions",
        "dialog" | "actionsheet" | "half-screen-dialog" | "picker" => "Overlays & pickers",
        "navbar" | "tabbar" | "steps" | "steps_vertical" | "steps_horizonal" => "Navigation",
        "article" | "languages" | "icons" | "footer" => "Content",
        "uploader" | "gallery" => "Media",
        "list" | "panel" | "preview" | "grid" | "flex" | "layers" => "Layout",
        _ if route.starts_with("form")
            || route.starts_with("searchbar")
            || matches!(route, "input" | "slider") =>
        {
            "Forms & inputs"
        }
        _ => "Feedback",
    }
}

pub fn frame(
    content: Stateful<Div>,
    selected: &str,
    window: &mut Window,
    cx: &mut App,
) -> Stateful<Div> {
    let theme = Theme::get(cx).clone();
    let state = window.use_keyed_state("desktop-navigation-scroll", cx, |_, _| {
        (String::new(), ScrollHandle::new())
    });
    let scroll = state.read(cx).1.clone();
    let mut selected_index = 0;
    let mut index = 0;
    let mut navigation = div()
        .id("desktop-component-list")
        .role(Role::Navigation)
        .aria_label("Component catalog")
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .track_scroll(&scroll)
        .px(px(12.))
        .pb(px(20.));
    for group in [
        "Start here",
        "Mac desktop",
        "Actions",
        "Forms & inputs",
        "Navigation",
        "Overlays & pickers",
        "Feedback",
        "Layout",
        "Media",
        "Content",
    ] {
        navigation = navigation.child(
            div()
                .px(px(10.))
                .pt(px(12.))
                .pb(px(7.))
                .text_size(px(11.))
                .text_color(theme.fg_1)
                .child(group),
        );
        index += 1;
        for &(route, _) in super::pages::ROUTES
            .iter()
            .filter(|(route, _)| category(route) == group)
        {
            let label: SharedString = title(route).into();
            if route == selected {
                selected_index = index;
            }
            index += 1;
            navigation = navigation.child(
                div()
                    .rounded(px(6.))
                    .overflow_hidden()
                    .when(route == selected, |row| row.bg(theme.brand.opacity(0.1)))
                    .child(
                        Cell::new(SharedString::from(format!("desktop-nav-{route}")))
                            .bd(div()
                                .text_size(px(13.))
                                .line_height(relative(1.4))
                                .child(label.clone()))
                            .aria_label(label)
                            .role(Role::Link)
                            .selected(route == selected)
                            .padding(Edges {
                                top: px(8.),
                                bottom: px(8.),
                                left: px(10.),
                                right: px(10.),
                            })
                            .on_click(super::chrome::go(route)),
                    ),
            );
        }
    }
    state.update(cx, |state, _| {
        if state.0 != selected {
            state.0 = selected.to_string();
            state.1.scroll_to_item(selected_index);
        }
    });
    div()
        .id("desktop-gallery-shell")
        .size_full()
        .flex()
        .row(I18n::get(cx).direction())
        .bg(theme.bg_0)
        .text_size(px(14.))
        .text_color(theme.fg_0)
        .font_family(".SystemUIFont")
        .child(
            div()
                .w(px(SIDEBAR))
                .h_full()
                .flex_shrink_0()
                .flex()
                .flex_col()
                .bg(theme.bg_1)
                .border_r_1()
                .border_color(theme.separator_0)
                .child(
                    div()
                        .h(px(72.))
                        .flex_shrink_0()
                        .px(px(22.))
                        .flex()
                        .items_center()
                        .text_size(px(23.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("weGPUI"),
                )
                .child(FocusScroll::new(
                    "desktop-navigation-focus",
                    &scroll,
                    navigation,
                )),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .h_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h(px(72.))
                        .flex_shrink_0()
                        .px(px(28.))
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .border_b_1()
                        .border_color(theme.separator_0)
                        .child(div().flex_1().text_size(px(14.)).child(title(selected)))
                        .child(
                            Button::new("Light")
                                .id("desktop-theme-light")
                                .mini()
                                .inline()
                                .margin_reset()
                                .when(!theme.is_dark, |button| button.primary())
                                .on_click(|_, _, cx| Theme::set_mode(ThemeMode::Light, cx)),
                        )
                        .child(
                            Button::new("Dark")
                                .id("desktop-theme-dark")
                                .mini()
                                .inline()
                                .margin_reset()
                                .when(theme.is_dark, |button| button.primary())
                                .on_click(|_, _, cx| Theme::set_mode(ThemeMode::Dark, cx)),
                        ),
                )
                .child(
                    div()
                        .relative()
                        .flex_1()
                        .min_h_0()
                        .overflow_hidden()
                        .child(content),
                ),
        )
}
