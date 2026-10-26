use gpui::{prelude::*, *};
use wegpui::{Theme, ThemeMode, components::footer::{Footer, FooterLink}};
struct Probe;
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p(px(24.)).bg(Theme::get(cx).bg_0).text_color(Theme::get(cx).fg_0)
            .on_key_down(|event,window,cx| { if event.keystroke.key == "tab" { if event.keystroke.modifiers.shift { window.focus_prev(cx); } else { window.focus_next(cx); } cx.stop_propagation(); } })
            .child(div().w(px(90.)).child(Footer::new()
                .link(FooterLink::new("help", "Help").on_click(|_, _, _| eprintln!("callback=Help")))
                .link(FooterLink::new("terms", "Terms").on_click(|_, _, _| eprintln!("callback=Terms")))))
    }
}
#[path = "support/platform.rs"] mod platform;
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    platform::application().with_assets(wegpui::Assets).run(move |cx| {
        wegpui::init(cx);
        Theme::set_mode(if mode.starts_with("dark") { ThemeMode::Dark } else { ThemeMode::Light },cx);
        Theme::set_care(mode.contains("care"),cx);
        if mode.contains("brand") { Theme::set_brand(Some(rgb(0x7c3aed).into()),cx); }
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(40.),px(40.)),size(px(240.),px(320.))))), titlebar: Some(TitlebarOptions { title: Some("Footer review".into()), ..Default::default() }), ..Default::default() }, |window,cx| { Theme::observe(window);cx.new(|_| Probe) }).unwrap();
        cx.activate(true);
    });
}
