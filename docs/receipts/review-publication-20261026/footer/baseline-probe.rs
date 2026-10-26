use gpui::{prelude::*, *};
use wegpui::{Theme, components::footer::{Footer, FooterLink}};
struct Probe;
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p(px(24.)).bg(Theme::get(cx).bg_0).text_color(Theme::get(cx).fg_0)
            .child(div().w(px(90.)).child(Footer::new()
                .link(FooterLink::new("help", "Help").on_click(|_, _, _| eprintln!("callback=Help")))
                .link(FooterLink::new("terms", "Terms").on_click(|_, _, _| eprintln!("callback=Terms")))))
    }
}
#[path = "support/platform.rs"] mod platform;
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(40.),px(40.)),size(px(240.),px(320.))))), titlebar: Some(TitlebarOptions { title: Some("Footer review".into()), ..Default::default() }), ..Default::default() }, |window,cx| { Theme::observe(window);cx.new(|_| Probe) }).unwrap();
        cx.activate(true);
    });
}
