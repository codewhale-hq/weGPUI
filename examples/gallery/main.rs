mod chrome;
mod pages;

use gpui::{prelude::*, *};
use std::time::{Duration, Instant};
use wegpui::{Assets, Theme, ThemeMode};

actions!(
    gallery,
    [
        Quit,
        Back,
        Home,
        ThemeLight,
        ThemeDark,
        ThemeSystem,
        ToggleCare,
        ToggleBrand,
        ToggleReduceMotion,
    ]
);

const SLIDE: Duration = Duration::from_millis(200);
const CUSTOM_BRAND: u32 = 0x1485ee;

struct Entry {
    route: SharedString,
    view: AnyView,
    focus: FocusHandle,
    pushed: Option<Instant>,
}

pub struct Gallery {
    stack: Vec<Entry>,
    leaving: Option<(Entry, Instant)>,
}

struct Nav(WeakEntity<Gallery>);
impl Global for Nav {}

pub fn go(route: &str, window: &mut Window, cx: &mut App) {
    let Some(nav) = cx.try_global::<Nav>().map(|n| n.0.clone()) else {
        return;
    };
    let route = route.to_string();
    window.defer(cx, move |window, cx| {
        nav.update(cx, |g, cx| {
            g.push(&route, &pages::Args::default(), true, window, cx)
        })
        .ok();
    });
}

pub fn back(window: &mut Window, cx: &mut App) {
    let Some(nav) = cx.try_global::<Nav>().map(|n| n.0.clone()) else {
        return;
    };
    window.defer(cx, move |window, cx| {
        nav.update(cx, |g, cx| g.pop(window, cx)).ok();
    });
}

pub fn home(window: &mut Window, cx: &mut App) {
    let Some(nav) = cx.try_global::<Nav>().map(|n| n.0.clone()) else {
        return;
    };
    window.defer(cx, move |window, cx| {
        nav.update(cx, |g, cx| {
            while g.stack.len() > 1 {
                g.pop(window, cx);
            }
        })
        .ok();
    });
}

impl Gallery {
    fn new(args: &CliArgs, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Theme::observe(window);
        let mut this = Self {
            stack: vec![],
            leaving: None,
        };
        let page_args = pages::Args {
            state: args.state.clone(),
        };
        let page = args.page.as_deref().unwrap_or("home");
        if page == "home" {
            this.push("home", &page_args, false, window, cx);
        } else {
            this.push("home", &pages::Args::default(), false, window, cx);
            this.push(page, &page_args, false, window, cx);
        }
        if let Some(y) = args.scroll {
            cx.set_global(chrome::PendingScroll(y));
        }
        this
    }
    fn push(
        &mut self,
        route: &str,
        args: &pages::Args,
        animate: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = pages::build(route, args, window, cx) else {
            return;
        };
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        self.stack.push(Entry {
            route: SharedString::from(route.to_string()),
            view,
            focus,
            pushed: animate.then(Instant::now),
        });
        cx.notify();
    }
    fn pop(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.stack.len() <= 1 {
            return;
        }
        let entry = self.stack.pop().unwrap();
        self.leaving = Some((entry, Instant::now()));
        if let Some(top) = self.stack.last() {
            window.focus(&top.focus, cx);
        }
        cx.notify();
    }
    fn progress(at: Option<Instant>, window: &Window, cx: &App) -> f32 {
        match at {
            Some(at) if !cx.reduce_motion() => {
                let p = (at.elapsed().as_secs_f32() / SLIDE.as_secs_f32()).min(1.);
                if p < 1. {
                    window.request_animation_frame();
                }
                wegpui::components::modal::ease(p)
            }
            _ => 1.,
        }
    }
}

fn layer(entry: &Entry, offset: f32, opacity: f32, width: Pixels) -> impl IntoElement {
    div()
        .id(ElementId::Name(entry.route.clone()))
        .absolute()
        .top_0()
        .bottom_0()
        .w(width)
        .left(width * offset)
        .opacity(opacity)
        .track_focus(&entry.focus)
        .child(entry.view.clone())
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let width = window.viewport_size().width;
        let n = self.stack.len();
        let top = self
            .stack
            .last()
            .map(|e| Self::progress(e.pushed, window, cx));
        let leaving_p = self
            .leaving
            .as_ref()
            .map(|(_, at)| Self::progress(Some(*at), window, cx));
        if leaving_p.is_some_and(|p| p >= 1.) {
            self.leaving = None;
        }
        let mut root = div()
            .id("gallery")
            .key_context("Gallery")
            .relative()
            .size_full()
            .overflow_hidden()
            .font_family(".SystemUIFont")
            .text_size(px(16.))
            .line_height(relative(1.6))
            .text_color(t.fg_0)
            .bg(t.bg_0)
            .on_key_down(|event, window, cx| {
                if event.keystroke.key == "tab" && !event.keystroke.modifiers.platform {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx)
                    } else {
                        window.focus_next(cx)
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .on_action(|_: &Back, w, cx| back(w, cx))
            .on_action(|_: &Home, w, cx| home(w, cx));
        for (i, entry) in self.stack.iter().enumerate() {
            let is_top = i + 1 == n;
            let covered = i + 2 < n || (i + 1 < n && top == Some(1.) && self.leaving.is_none());
            if covered {
                continue;
            }
            let p = if is_top { top.unwrap_or(1.) } else { 1. };
            root = root.child(layer(entry, 1. - p, p, width));
        }
        if let Some((entry, _)) = self.leaving.as_ref() {
            let p = leaving_p.unwrap_or(1.);
            root = root.child(layer(entry, p, 1. - p, width));
        }
        wegpui::accessibility::ModalScope::new(root)
    }
}

#[derive(Default)]
struct CliArgs {
    page: Option<String>,
    state: Option<String>,
    theme: Option<ThemeMode>,
    care: bool,
    brand: Option<Hsla>,
    reduce_motion: Option<bool>,
    scroll: Option<f32>,
    size: Option<(f32, f32)>,
    position: Option<(f32, f32)>,
}

fn parse_args() -> CliArgs {
    let mut args = CliArgs::default();
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = || it.next().unwrap_or_default();
        match arg.as_str() {
            "--page" => args.page = Some(value()),
            "--state" => args.state = Some(value()),
            "--theme" => {
                args.theme = Some(match value().as_str() {
                    "dark" => ThemeMode::Dark,
                    "light" => ThemeMode::Light,
                    _ => ThemeMode::System,
                })
            }
            "--care" => args.care = true,
            "--brand" => {
                let v = value();
                args.brand = u32::from_str_radix(v.trim_start_matches('#'), 16)
                    .ok()
                    .map(|c| rgb(c).into());
            }
            "--reduce-motion" => args.reduce_motion = Some(true),
            "--motion" => args.reduce_motion = Some(false),
            "--scroll" => args.scroll = value().parse().ok(),
            "--size" => {
                let v = value();
                if let Some((w, h)) = v.split_once('x') {
                    args.size = w.parse().ok().zip(h.parse().ok());
                }
            }
            "--position" => {
                let v = value();
                if let Some((x, y)) = v.split_once(',') {
                    args.position = x.parse().ok().zip(y.parse().ok());
                }
            }
            _ => {}
        }
    }
    args
}

fn set_menus(cx: &mut App) {
    let t = Theme::get(cx);
    let (mode, care, branded) = (t.mode, t.care, t.brand_override.is_some());
    let reduce = cx.reduce_motion();
    cx.set_menus([
        Menu::new("weGPUI").items([MenuItem::action("Quit weGPUI", Quit)]),
        Menu::new("Navigate").items([
            MenuItem::action("Back", Back),
            MenuItem::action("Home", Home),
        ]),
        Menu::new("Appearance").items([
            MenuItem::action("System", ThemeSystem).checked(mode == ThemeMode::System),
            MenuItem::action("Light", ThemeLight).checked(mode == ThemeMode::Light),
            MenuItem::action("Dark", ThemeDark).checked(mode == ThemeMode::Dark),
            MenuItem::separator(),
            MenuItem::action("Care Mode", ToggleCare).checked(care),
            MenuItem::action("Custom Brand Colour", ToggleBrand).checked(branded),
            MenuItem::action("Reduce Motion", ToggleReduceMotion).checked(reduce),
        ]),
    ]);
}

#[path = "../support/platform.rs"]
mod platform;

fn main() {
    let args = parse_args();
    platform::application().with_assets(Assets).run(move |cx| {
        wegpui::init(cx);
        if let Some(mode) = args.theme {
            Theme::set_mode(mode, cx);
        }
        if args.care {
            Theme::set_care(true, cx);
        }
        if args.brand.is_some() {
            Theme::set_brand(args.brand, cx);
        }
        if let Some(reduce) = args.reduce_motion {
            cx.set_reduce_motion(reduce);
        }
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &ThemeLight, cx| {
            Theme::set_mode(ThemeMode::Light, cx);
            set_menus(cx);
        });
        cx.on_action(|_: &ThemeDark, cx| {
            Theme::set_mode(ThemeMode::Dark, cx);
            set_menus(cx);
        });
        cx.on_action(|_: &ThemeSystem, cx| {
            Theme::set_mode(ThemeMode::System, cx);
            set_menus(cx);
        });
        cx.on_action(|_: &ToggleCare, cx| {
            let care = !Theme::get(cx).care;
            Theme::set_care(care, cx);
            set_menus(cx);
        });
        cx.on_action(|_: &ToggleBrand, cx| {
            let brand = match Theme::get(cx).brand_override {
                Some(_) => None,
                None => Some(rgb(CUSTOM_BRAND).into()),
            };
            Theme::set_brand(brand, cx);
            set_menus(cx);
        });
        cx.on_action(|_: &ToggleReduceMotion, cx| {
            let reduce = !cx.reduce_motion();
            cx.set_reduce_motion(reduce);
            set_menus(cx);
        });
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-[", Back, None),
            KeyBinding::new("alt-left", Back, None),
            KeyBinding::new("cmd-shift-h", Home, None),
        ]);
        set_menus(cx);
        let (w, h) = args.size.unwrap_or((375., 812.));
        let bounds = match args.position {
            Some((x, y)) => Bounds::new(point(px(x), px(y)), size(px(w), px(h))),
            None => Bounds::centered(None, size(px(w), px(h)), cx),
        };
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI".into()),
                        ..Default::default()
                    }),
                    app_id: Some("weGPUI".into()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Gallery::new(&args, window, cx));
                    cx.set_global(Nav(view.downgrade()));
                    view
                },
            )
            .expect("gallery window");
        window
            .update(cx, |_, window, cx| {
                cx.observe_window_activation(window, |_, window, cx| {
                    if window.is_window_active() {
                        wegpui::motion::sync_reduce_motion(cx);
                    }
                })
                .detach();
            })
            .ok();
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.activate(true);
    });
}
