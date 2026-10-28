use gpui::{prelude::*, *};
use wegpui::{components::vcode::{VcodeButton, VcodeEvent}, Theme, ThemeMode};

struct Probe { button: Entity<VcodeButton>, sends: usize, subscription: Subscription }
impl Probe {
    fn new(cx: &mut Context<Self>) -> Self {
        let button = cx.new(|cx| { let mut b = VcodeButton::new(cx); b.set_label("Send code", cx); b.set_seconds(3); b });
        let subscription = cx.subscribe(&button, |this, _, _: &VcodeEvent, cx| { this.sends += 1; cx.notify(); });
        Self { button, sends: 0, subscription }
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = &self.subscription;
        div().size_full().p(px(32.)).flex().flex_col().gap(px(20.)).bg(white())
            .child(self.button.clone())
            .child(div().id("reset").role(Role::Button).aria_label("Reset").focusable().tab_stop(true)
                .on_click(cx.listener(|this, _, _, cx| { this.button.update(cx, |button, cx| button.reset(cx)); }))
                .child("Reset"))
            .child(div().id("event-count").role(Role::Status).aria_label(format!("Sends {}", self.sends)).child(format!("Sends {}", self.sends)))
    }
}
#[path = "/src/examples/support/platform.rs"]
mod platform;
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx); Theme::set_mode(ThemeMode::Light, cx);
        cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.),px(0.)),size(px(420.),px(300.))))),
            titlebar: Some(TitlebarOptions {title: Some("weGPUI vcode deadline".into()), ..Default::default()}),
            ..Default::default()
        }, |window, cx| { Theme::observe(window); cx.new(Probe::new) }).expect("open vcode fixture");
        cx.activate(true);
    });
}
