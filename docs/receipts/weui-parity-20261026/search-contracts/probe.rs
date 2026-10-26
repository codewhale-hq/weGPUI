use gpui::{prelude::*, *};
use wegpui::{Theme, ThemeMode, components::search_bar::{SearchBar, SearchBarEvent, SearchBarVariant}};
struct Probe { bar: Entity<SearchBar>, subscription: Subscription, root_focus: FocusHandle }
impl Probe {
    fn new(variant: SearchBarVariant, cx: &mut Context<Self>) -> Self {
        let bar = cx.new(|cx| {
            let mut bar = SearchBar::new(cx).variant(variant).words("Scope");
            bar.set_placeholder("Contract search", cx);
            bar
        });
        let subscription = cx.subscribe(&bar, |_, _, event: &SearchBarEvent, _| eprintln!("event={event:?}"));
        Self { bar, subscription, root_focus: cx.focus_handle().tab_stop(false) }
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = &self.subscription;
        let t = Theme::get(cx).clone();
        div().id("search-contracts").track_focus(&self.root_focus).size_full().p(px(24.)).bg(t.bg_0).text_color(t.fg_0)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                eprintln!("root-key={}", event.keystroke.key);
                match event.keystroke.key.as_str() {
                    "tab" => if event.keystroke.modifiers.shift { window.focus_prev(cx); } else { window.focus_next(cx); },
                    "f5" => this.bar.update(cx, |bar, cx| bar.activate(window, cx)),
                    "f6" => { this.bar.read(cx).input().clone().update(cx, |input, cx| input.set_disabled(true, cx)); window.focus(&this.root_focus, cx); },
                    "f7" => { eprintln!("reenable"); this.bar.read(cx).input().clone().update(cx, |input, cx| input.set_disabled(false, cx)); },
                    "f8" => this.bar.read(cx).input().clone().update(cx, |input, cx| input.set_readonly(true, cx)),
                    "f9" => this.bar.read(cx).input().clone().update(cx, |input, cx| input.set_readonly(false, cx)),
                    "f10" => eprintln!("snapshot text={:?} focusing={} disabled={} readonly={} focused={}", this.bar.read(cx).text(cx), this.bar.read(cx).is_focusing(), this.bar.read(cx).input().read(cx).is_disabled(), this.bar.read(cx).input().read(cx).is_readonly(), this.bar.focus_handle(cx).is_focused(window)),
                    _ => return,
                }
                cx.stop_propagation();
            })).child(self.bar.clone())
    }
}
#[path = "support/platform.rs"] mod platform;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).cloned().unwrap_or_else(|| "light".into());
    let variant = match args.get(2).map(String::as_str) { Some("outlined") => SearchBarVariant::Outlined, Some("filled") => SearchBarVariant::Filled, _ => SearchBarVariant::FilledGrey };
    platform::application().with_assets(wegpui::Assets).run(move |cx| {
        wegpui::init(cx);
        Theme::set_mode(if mode.starts_with("dark") { ThemeMode::Dark } else { ThemeMode::Light }, cx);
        Theme::set_care(mode.contains("care"), cx);
        if mode.contains("brand") { Theme::set_brand(Some(rgb(0xe64340).into()), cx); }
        cx.set_reduce_motion(true);
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(40.), px(40.)), size(px(600.), px(300.))))), titlebar: Some(TitlebarOptions { title: Some("Search contracts".into()), ..Default::default() }), ..Default::default() }, |window, cx| { Theme::observe(window); let probe = cx.new(|cx| Probe::new(variant, cx)); let focus = probe.read(cx).root_focus.clone(); window.focus(&focus, cx); probe }).expect("search window");
        cx.activate(true);
    });
}
