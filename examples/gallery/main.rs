mod chrome;
mod desktop;
mod pages;
mod text;

use gpui::{prelude::*, *};
use std::time::{Duration, Instant};
use wegpui::{Assets, I18n, Locale, Theme, ThemeMode, logical::Logical};

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
        LangEn,
        LangZhHans,
        LangZhHant,
        LangJa,
        LangKo,
        LangEs,
        LangFr,
        LangDe,
        LangPt,
        LangRu,
        LangAr,
        LangHe,
        LangHi,
        LangTh,
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
    font_family: SharedString,
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
            font_family: args
                .font_family
                .clone()
                .unwrap_or_else(|| ".SystemUIFont".into())
                .into(),
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
    fn rebuild(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for entry in &mut self.stack {
            if let Some(view) = pages::build(&entry.route, &pages::Args::default(), window, cx) {
                entry.view = view;
            }
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
        #[cfg(not(target_os = "ios"))]
        let width = window.viewport_size().width;
        #[cfg(target_os = "ios")]
        let width = window.fully_visible_bounds().size.width;
        let is_desktop = cfg!(not(target_os = "ios")) && width >= px(desktop::BREAKPOINT);
        let width = width - px(if is_desktop { desktop::SIDEBAR } else { 0. });
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
            .font_family(self.font_family.clone())
            .text_size(px(16.))
            .line_height(relative(1.6))
            .text_color(t.fg_0)
            .bg(t.bg_0);
        root = root.text_start(I18n::get(cx).direction());
        let fallbacks: Vec<String> = I18n::get(cx)
            .locale()
            .font_fallbacks()
            .iter()
            .map(|name| name.to_string())
            .collect();
        if !fallbacks.is_empty() {
            root.text_style().font_fallbacks = Some(FontFallbacks::from_fonts(fallbacks));
        }
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
        if is_desktop {
            let route = self
                .stack
                .last()
                .map(|entry| entry.route.as_ref())
                .unwrap_or("home");
            root = desktop::frame(root, route, window, cx);
        }
        let root = root
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
        #[cfg(target_os = "ios")]
        let root = platform::safe_area(
            div()
                .size_full()
                .flex()
                .flex_col()
                .when(n > 1, |el| {
                    el.child(
                        div()
                            .h(px(56.))
                            .flex_shrink_0()
                            .px(px(16.))
                            .flex()
                            .items_center()
                            .child(
                                wegpui::components::button::Button::new(
                                    I18n::get(cx).messages().back.clone(),
                                )
                                .inline()
                                .medium()
                                .margin_reset()
                                .on_click(|_, window, cx| back(window, cx)),
                            ),
                    )
                })
                .child(div().relative().flex_1().min_h_0().child(root)),
            window,
            cx,
        );
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
    lang: Option<String>,
    dir: Option<String>,
    scroll: Option<f32>,
    size: Option<(f32, f32)>,
    position: Option<(f32, f32)>,
    font_family: Option<String>,
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
            "--lang" => args.lang = Some(value()),
            "--font-family" => args.font_family = Some(value()),
            "--dir" => args.dir = Some(value()),
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
    let lang = I18n::get(cx).locale().catalog_tag();
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
        Menu::new("Language").items([
            MenuItem::action("English", LangEn).checked(lang == "en"),
            MenuItem::action("简体中文", LangZhHans).checked(lang == "zh-Hans"),
            MenuItem::action("繁體中文", LangZhHant).checked(lang == "zh-Hant"),
            MenuItem::action("日本語", LangJa).checked(lang == "ja"),
            MenuItem::action("한국어", LangKo).checked(lang == "ko"),
            MenuItem::action("Español", LangEs).checked(lang == "es"),
            MenuItem::action("Français", LangFr).checked(lang == "fr"),
            MenuItem::action("Deutsch", LangDe).checked(lang == "de"),
            MenuItem::action("Português", LangPt).checked(lang == "pt"),
            MenuItem::action("Русский", LangRu).checked(lang == "ru"),
            MenuItem::action("العربية", LangAr).checked(lang == "ar"),
            MenuItem::action("עברית", LangHe).checked(lang == "he"),
            MenuItem::action("हिन्दी", LangHi).checked(lang == "hi"),
            MenuItem::action("ไทย", LangTh).checked(lang == "th"),
        ]),
    ]);
}

fn set_language(tag: &str, cx: &mut App) {
    if let Some(locale) = Locale::parse(tag) {
        text::set_locale(&locale);
        I18n::set_locale(locale, cx);
        set_menus(cx);
        rebuild_pages(cx);
    }
}

fn rebuild_pages(cx: &mut App) {
    let Some(nav) = cx.try_global::<Nav>().map(|n| n.0.clone()) else {
        return;
    };
    if let Some(handle) = cx.active_window() {
        handle
            .update(cx, |_, window, cx| {
                nav.update(cx, |gallery, cx| gallery.rebuild(window, cx))
                    .ok();
            })
            .ok();
    }
}

#[path = "../support/platform.rs"]
mod platform;

fn main() {
    let args = parse_args();
    platform::application().with_assets(Assets).run(move |cx| {
        wegpui::init(cx);
        let locale = args
            .lang
            .as_deref()
            .and_then(Locale::parse)
            .unwrap_or_else(Locale::en);
        text::set_locale(&locale);
        I18n::set_locale(locale, cx);
        match args.dir.as_deref() {
            Some("rtl") => I18n::set_direction(Some(wegpui::Direction::Rtl), cx),
            Some("ltr") => I18n::set_direction(Some(wegpui::Direction::Ltr), cx),
            _ => {}
        }
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
        cx.on_action(|_: &LangEn, cx| set_language("en", cx));
        cx.on_action(|_: &LangZhHans, cx| set_language("zh-Hans", cx));
        cx.on_action(|_: &LangZhHant, cx| set_language("zh-Hant", cx));
        cx.on_action(|_: &LangJa, cx| set_language("ja", cx));
        cx.on_action(|_: &LangKo, cx| set_language("ko", cx));
        cx.on_action(|_: &LangEs, cx| set_language("es", cx));
        cx.on_action(|_: &LangFr, cx| set_language("fr", cx));
        cx.on_action(|_: &LangDe, cx| set_language("de", cx));
        cx.on_action(|_: &LangPt, cx| set_language("pt", cx));
        cx.on_action(|_: &LangRu, cx| set_language("ru", cx));
        cx.on_action(|_: &LangAr, cx| set_language("ar", cx));
        cx.on_action(|_: &LangHe, cx| set_language("he", cx));
        cx.on_action(|_: &LangHi, cx| set_language("hi", cx));
        cx.on_action(|_: &LangTh, cx| set_language("th", cx));
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-[", Back, None),
            KeyBinding::new("alt-left", Back, None),
            KeyBinding::new("cmd-shift-h", Home, None),
        ]);
        set_menus(cx);
        let (w, h) = args.size.unwrap_or(if cfg!(target_os = "ios") {
            (375., 812.)
        } else {
            (1120., 760.)
        });
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
