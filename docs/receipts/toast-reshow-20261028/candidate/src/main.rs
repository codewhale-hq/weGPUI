use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::{Theme, ThemeMode, accessibility::ModalScope, components::{modal::Presence, toast::Toast}};

struct Probe { presence: Presence, shows: usize, closes: usize }
impl Probe {
    fn new(cx: &mut Context<Self>) -> Self { Self { presence: Toast::presence_for(cx), shows: 0, closes: 0 } }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status=format!("Open {}; generation {}; shows {}; closes {}",self.presence.is_open(),self.presence.generation(),self.shows,self.closes);
        let content=div().size_full().p(px(32.)).flex().flex_col().gap(px(20.)).bg(white())
            .child(div().id("show").role(Role::Button).aria_label("Show toast").focusable().tab_stop(true)
                .on_click(cx.listener(|this,_,window,cx| {
                    this.shows += 1;
                    this.presence.show(window,cx);
                    this.presence.hide_after(Duration::from_secs(3),window,cx,|this| &mut this.presence);
                    cx.notify();
                })).child("Show toast"))
            .child(div().id("hide").role(Role::Button).aria_label("Hide toast").focusable().tab_stop(true)
                .on_click(cx.listener(|this,_,window,cx| {this.closes+=1;this.presence.hide(window,cx);cx.notify();})).child("Hide toast"))
            .child(div().id("status").role(Role::Status).aria_label(status.clone()).child(status))
            .child(Toast::new("Saved toast").text_only().presence(&self.presence));
        ModalScope::new(content)
    }
}
#[path = "/src/examples/support/platform.rs"] mod platform;
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx); Theme::set_mode(ThemeMode::Light,cx);
        cx.set_reduce_motion(std::env::var_os("TOAST_REDUCED").is_some());
        cx.open_window(WindowOptions {window_bounds:Some(WindowBounds::Windowed(Bounds::new(point(px(0.),px(0.)),size(px(420.),px(320.))))),
            titlebar:Some(TitlebarOptions{title:Some("weGPUI toast reshow".into()),..Default::default()}),..Default::default()},
            |window,cx|{Theme::observe(window);cx.new(Probe::new)}).expect("open toast probe");
        cx.activate(true);
    });
}
