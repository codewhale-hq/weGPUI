use gpui::{prelude::*, *};
use wegpui::{components::progress::Progress, Theme, ThemeMode};

struct Probe;
impl Render for Probe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p(px(32.)).flex().flex_col().gap(px(20.)).bg(white())
            .children([
                ("zero", 0.), ("fraction", 0.4), ("middle", 49.6), ("fine", 0.125),
                ("integer", 25.), ("negative-zero", -0.0), ("nan", f32::NAN),
                ("positive-infinity", f32::INFINITY), ("below", -1.), ("above", 101.),
            ].into_iter().map(|(name, value)| div().w(px(400.)).h(px(22.))
                .child(Progress::new(name).label(name).value(value).cancelable(false))))
    }
}
#[path = "/src/examples/support/platform.rs"]
mod platform;
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        Theme::set_mode(ThemeMode::Light, cx);
        cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size(px(520.), px(520.))))),
            titlebar: Some(TitlebarOptions { title: Some("weGPUI progress precision".into()), ..Default::default() }),
            ..Default::default()
        }, |window, cx| { Theme::observe(window); cx.new(|_| Probe) }).expect("open progress probe");
        cx.activate(true);
    });
}
