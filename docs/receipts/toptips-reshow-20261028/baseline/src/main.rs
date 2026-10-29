use gpui::{prelude::*, *};
use std::time::{Duration, Instant};
use wegpui::{Theme, ThemeMode, components::toptips::{Toptips, ToptipsState}};

struct Probe { state: ToptipsState, started: Instant, shows: usize, closes: usize, opens: usize }
impl Probe {
    fn record(&self, stage: &str) {
        println!("{{\"stage\":\"{stage}\",\"elapsed_ms\":{},\"open\":{},\"shows\":{},\"closes\":{},\"opens\":{}}}",self.started.elapsed().as_millis(),self.state.is_open(),self.shows,self.closes,self.opens);
    }
    fn show(&mut self, label: &str, window: &mut Window, cx: &mut Context<Self>) {
        if !self.state.is_open() { self.opens += 1; }
        self.shows += 1;
        self.state.show(window);
        self.state.hide_after(Duration::from_secs(2),window,cx,|this| &mut this.state);
        self.record(label); cx.notify();
    }
    fn hide(&mut self, label: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.closes += 1; self.state.hide(window); self.record(label); cx.notify();
    }
    fn start(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let this=Self { state:ToptipsState::new(), started:Instant::now(), shows:0, closes:0, opens:0 };
        cx.spawn_in(window,async move |this,cx| {
            this.update_in(cx,|s,w,c| s.show("first",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(800)).await;
            this.update_in(cx,|s,w,c| s.show("second",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(1350)).await;
            this.update_in(cx,|s,_,_| s.record("overlap")).unwrap();
            cx.background_executor().timer(Duration::from_millis(900)).await;
            this.update_in(cx,|s,_,_| s.record("second_done")).unwrap();
            this.update_in(cx,|s,w,c| s.show("single",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(2250)).await;
            this.update_in(cx,|s,_,_| s.record("single_done")).unwrap();
            this.update_in(cx,|s,w,c| s.show("before_hide",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(600)).await;
            this.update_in(cx,|s,w,c| s.hide("hide",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(600)).await;
            this.update_in(cx,|s,w,c| s.show("reopen",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(1000)).await;
            this.update_in(cx,|s,_,_| s.record("reopen_overlap")).unwrap();
            cx.background_executor().timer(Duration::from_millis(1200)).await;
            this.update_in(cx,|s,_,_| s.record("reopen_done")).unwrap();
            this.update_in(cx,|s,w,c| s.show("cancel_show",w,c)).unwrap();
            this.update_in(cx,|s,w,c| s.hide("cancel_hide",w,c)).unwrap();
            cx.background_executor().timer(Duration::from_millis(2250)).await;
            this.update_in(cx,|s,_,c| { s.record("cancel_done"); c.quit(); }).unwrap();
        }).detach();
        this
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(white()).child(Toptips::new("tip",&self.state,"Saved tip"))
    }
}
fn main() {
    gpui_platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx); Theme::set_mode(ThemeMode::Light,cx);
        cx.set_reduce_motion(std::env::var_os("TOPTIPS_REDUCED").is_some());
        cx.open_window(WindowOptions { window_bounds:Some(WindowBounds::Windowed(Bounds::new(point(px(50.),px(50.)),size(px(420.),px(240.))))), titlebar:Some(TitlebarOptions{title:Some("weGPUI Toptips timer probe".into()),..Default::default()}),..Default::default()}, |window,cx| {Theme::observe(window); cx.new(|cx|Probe::start(window,cx))}).unwrap();
        cx.activate(true);
    });
}
