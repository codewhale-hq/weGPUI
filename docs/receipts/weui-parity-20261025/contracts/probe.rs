use gpui::{prelude::*, *};
use wegpui::{
    Theme, ThemeMode,
    accessibility::ModalScope,
    components::{
        agree::Agree,
        footer::{Footer, FooterLink},
        swiped::SwipedAction,
    },
};
struct Probe {
    actions: usize,
    links: usize,
    checked: bool,
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        ModalScope::new(div().size_full().bg(theme.bg_0).text_color(white()).flex().flex_col()
            .child(SwipedAction::new("neutral", "Neutral action").default_kind().on_click(cx.listener(|this,_,_,cx| {this.actions+=1;cx.notify();})))
            .child(div().p(px(16.)).text_color(theme.fg_0).child(format!("Actions: {}",self.actions)))
            .child(div().p(px(16.)).child(Agree::new("agree", "A long agreement sentence should wrap inside the available width while the checkbox keeps its size and the link remains independently usable. ").checked(self.checked).on_change(cx.listener(|this, value: &bool,_,cx| {this.checked=*value;cx.notify();})).link("Terms",|_,_,_| {})))
            .child(Footer::new().fixed_bottom().safe_area_bottom(px(24.)).link(FooterLink::new("help","Help").on_click(cx.listener(|this,_,_,cx| {this.links+=1;cx.notify();}))).text_line(format!("Footer calls: {}",self.links))))
    }
}
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    gpui_platform::application()
        .with_assets(wegpui::Assets)
        .run(move |cx| {
            wegpui::init(cx);
            Theme::set_mode(
                if mode.contains("dark") {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                },
                cx,
            );
            Theme::set_care(mode.contains("care"), cx);
            if mode.contains("brand") {
                Theme::set_brand(Some(rgb(0x7c3aed).into()), cx);
            }
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(0.), px(0.)),
                        size(px(375.), px(480.)),
                    ))),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(|_| Probe {
                        actions: 0,
                        links: 0,
                        checked: false,
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
