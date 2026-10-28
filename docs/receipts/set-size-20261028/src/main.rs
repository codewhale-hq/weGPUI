use gpui::{prelude::*, *};
use wegpui::{Theme, accessibility::ModalScope, components::{
    Presence,
    picker::{Picker, PickerItem}, select::SelectMenu,
}};
#[path = "/src/examples/support/platform.rs"] mod platform;

struct Probe {
    mode: String,
    presence: Presence,
    picker: Entity<Picker>,
}
impl Probe {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mode = std::env::var("WEGPUI_SET_SIZE_MODE").unwrap_or_else(|_| "select".into());
        let picker = if mode == "date" {
            cx.new(|cx| Picker::new("probe-date", cx).date(1900, 2100, (2000, 1, 1)).title("Date picker"))
        } else {
            cx.new(|cx| Picker::single("probe-picker", [
                PickerItem::new("Picker one", "1"),
                PickerItem::new("Picker disabled", "2").disabled(true),
                PickerItem::new("Picker three", "3"),
            ], cx).title("Picker"))
        };
        if mode == "picker" || mode == "date" {
            let target = picker.clone();
            window.defer(cx, move |window, cx| target.update(cx, |picker, cx| picker.open_instantly(window, cx)));
        }
        let mut presence = Presence::new(cx);
        if mode == "select" { presence.show_instantly(window, cx); }
        Self { mode, presence, picker }
    }
}
impl Render for Probe {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let mut page = div().id("set-size-root").size_full().flex().flex_col().bg(theme.bg_0).text_color(theme.fg_0);
        if self.mode == "select" {
            page = page.child(SelectMenu::new("set-size-select", &self.presence).title("Select options")
                .option("Select one").disabled_option("Select disabled").option("Select three").selected(Some(0)));
        } else {
            page = page.child(self.picker.clone());
        }
        ModalScope::new(page)
    }
}
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size(px(600.), px(900.))))),
            titlebar: Some(TitlebarOptions { title: Some("weGPUI set size probe".into()), ..Default::default() }),
            ..Default::default()
        }, |window, cx| { Theme::observe(window); cx.new(|cx| Probe::new(window, cx)) }).unwrap();
        cx.activate(true);
    });
}
