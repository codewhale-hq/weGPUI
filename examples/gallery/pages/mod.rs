use gpui::*;

pub mod actionsheet;
pub mod article;
pub mod badge;
pub mod button;
pub mod dialog;
pub mod flex;
pub mod footer;
pub mod form;
pub mod gallery;
pub mod grid;
pub mod half_screen_dialog;
pub mod home;
pub mod icons;
pub mod information_bar;
pub mod input;
pub mod layers;
pub mod list;
pub mod loading;
pub mod loadmore;
pub mod msg;
pub mod navbar;
pub mod panel;
pub mod picker;
pub mod preview;
pub mod progress;
pub mod searchbar;
pub mod slider;
pub mod steps;
pub mod tabbar;
pub mod toast;
pub mod uploader;

#[derive(Default, Clone)]
pub struct Args {
    pub state: Option<String>,
}

pub type Build = fn(&'static str, &Args, &mut Window, &mut App) -> AnyView;

pub const ROUTES: &[(&str, Build)] = &[
    ("home", home::build),
    ("layers", layers::build),
    ("button", button::build),
    ("button_default", button::build),
    ("button_bottom_fixed", button::build),
    ("list", list::build),
    ("input", input::build),
    ("form", form::build),
    ("form_primary", form::build),
    ("form_page", form::build),
    ("form_page_old", form::build),
    ("form_input_status", form::build),
    ("form_vcode", form::build),
    ("form_bottom_fixed", form::build),
    ("form_access", form::build),
    ("form_checkbox", form::build),
    ("form_radio", form::build),
    ("form_switch", form::build),
    ("form_select", form::build),
    ("form_select_primary", form::build),
    ("form_textarea", form::build),
    ("form_vertical", form::build),
    ("toast", toast::build),
    ("dialog", dialog::build),
    ("half-screen-dialog", half_screen_dialog::build),
    ("progress", progress::build),
    ("steps", steps::build),
    ("steps_vertical", steps::build),
    ("steps_horizonal", steps::build),
    ("msg", msg::build),
    ("msg_success", msg::build),
    ("msg_warn", msg::build),
    ("msg_text", msg::build),
    ("msg_text_primary", msg::build),
    ("msg_custom_area_preview", msg::build),
    ("msg_custom_area_tips", msg::build),
    ("msg_custom_area_cell", msg::build),
    ("article", article::build),
    ("navbar", navbar::build),
    ("tabbar", tabbar::build),
    ("panel", panel::build),
    ("actionsheet", actionsheet::build),
    ("icons", icons::build),
    ("searchbar", searchbar::build),
    ("searchbar_filled", searchbar::build),
    ("searchbar_grey", searchbar::build),
    ("searchbar_outlined", searchbar::build),
    ("searchbar_homepage", searchbar::build),
    ("picker", picker::build),
    ("footer", footer::build),
    ("gallery", gallery::build),
    ("flex", flex::build),
    ("loading", loading::build),
    ("loadmore", loadmore::build),
    ("uploader", uploader::build),
    ("preview", preview::build),
    ("grid", grid::build),
    ("badge", badge::build),
    ("slider", slider::build),
    ("information-bar", information_bar::build),
];

pub fn build(route: &str, args: &Args, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    ROUTES
        .iter()
        .find(|(r, _)| *r == route)
        .map(|(r, build)| build(r, args, window, cx))
}
