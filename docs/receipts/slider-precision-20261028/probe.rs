//! Native contract fixture for controlled data/navigation components. Events are printed to stderr.
use gpui::{prelude::*, *};
use wegpui::{
    Theme, ThemeMode,
    accessibility::ModalScope,
    components::{
        Cell, Cells, Modal, Presence,
        article::{Article, ArticleList, h6, paragraph_text},
        button::Button,
        footer::{Footer, FooterLink},
        grid::{Grid, Grids},
        information_bar::InformationBar,
        loadmore::LoadMore,
        navbar::Navbar,
        panel::{MediaBox, Panel},
        progress::Progress,
        slider::Slider,
        tabbar::{Tabbar, TabbarItem},
    },
    scrolling::FocusScroll,
};

struct Probe {
    value: f32,
    cancelable: bool,
    notice: bool,
    navbar: usize,
    tabbar: usize,
    events: usize,
    announcement: SharedString,
    sliders: [f32; 2],
    modal: Presence,
    scroll: ScrollHandle,
}

impl Probe {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: 25.,
            cancelable: true,
            notice: true,
            navbar: 0,
            tabbar: 0,
            events: 0,
            announcement: "Upload 25 percent".into(),
            sliders: [6., 0.125],
            modal: Presence::new(cx),
            scroll: ScrollHandle::new(),
        }
    }

    fn record(&mut self, event: &str, cx: &mut Context<Self>) {
        self.events += 1;
        eprintln!("event={event} count={}", self.events);
        cx.notify();
    }
}

fn select(
    navbar: bool,
    cx: &mut Context<Probe>,
) -> impl Fn(usize, &mut Window, &mut App) + 'static {
    let owner = cx.entity().downgrade();
    move |index, _, cx| {
        owner
            .update(cx, |this, cx| {
                if navbar {
                    this.navbar = index;
                } else {
                    this.tabbar = index;
                }
                this.record(
                    &format!("{}-{index}", if navbar { "navbar" } else { "tabbar" }),
                    cx,
                );
            })
            .ok();
    }
}

impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let scroll = self.scroll.clone();
        let owner = cx.entity().downgrade();
        let content = div()
            .id("data-content")
            .flex()
            .flex_col()
            .gap(px(16.))
            .p(px(16.))
            .bg(theme.bg_0)
            .text_color(theme.fg_0)
            .on_key_down(|event, window, cx| {
                if event.keystroke.key == "tab" {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    cx.stop_propagation();
                }
            })
            .child(
                div()
                    .id("inset-parent")
                    .role(Role::Group)
                    .aria_label("Inset container")
                    .relative()
                    .ml(px(64.))
                    .mt(px(64.))
                    .h(px(48.))
                    .flex_shrink_0()
                    .bg(theme.bg_2)
                    .child("Inset parent: notice must remain at window inset 8")
                    .child(
                        Footer::new()
                            .id("fixed-footer")
                            .fixed_bottom()
                            .link(
                                FooterLink::new("fixed-footer-link", "Fixed footer callback")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.record("fixed-footer-open", cx)
                                    })),
                            )
                            .text_line("Viewport footer"),
                    )
                    .when(self.notice, |el| {
                        el.child(
                            InformationBar::new("notice", "Floating data notice")
                                .floating(true)
                                .link(
                                    "Notice details",
                                    cx.listener(|this, _, _, cx| this.record("notice-link", cx)),
                                )
                                .closable("Close notice")
                                .on_close(cx.listener(|this, _, _, cx| {
                                    this.notice = false;
                                    this.record("notice-close", cx);
                                })),
                        )
                    }),
            )
            .child(
                Progress::new("upload-progress")
                    .label("Controlled upload")
                    .value(self.value)
                    .cancelable(self.cancelable)
                    .cancel_label("Cancel upload")
                    .announcement(self.announcement.clone())
                    .on_cancel(cx.listener(|this, _, _, cx| {
                        this.announcement = "Upload canceled".into();
                        this.record("progress-cancel", cx);
                    })),
            )
            .child(LoadMore::loading("Loading data records").id("data-loadmore"))
            .child(
                div()
                    .flex()
                    .gap(px(12.))
                    .child(
                        Button::new("Advance progress")
                            .id("advance")
                            .medium()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.value = (this.value + 25.).min(100.);
                                this.announcement = format!("Upload {} percent", this.value).into();
                                this.record(&format!("progress-{}", this.value), cx);
                            })),
                    )
                    .child(
                        Button::new(if self.cancelable {
                            "Hide cancel"
                        } else {
                            "Show cancel"
                        })
                        .id("toggle-cancel")
                        .medium()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.cancelable = !this.cancelable;
                            this.record(
                                if this.cancelable {
                                    "cancel-visible"
                                } else {
                                    "cancel-hidden"
                                },
                                cx,
                            );
                        })),
                    ),
            )
            .child(
                Navbar::new("data-navbar")
                    .label("Dataset navigation")
                    .items(["Overview", "Details", "History"])
                    .selected(self.navbar)
                    .on_select(select(true, cx)),
            )
            .child(
                Tabbar::new("data-tabbar")
                    .label("Workspace navigation")
                    .item(TabbarItem::new("Home", div().child("H")))
                    .item(TabbarItem::new("Files", div().child("F")))
                    .item(TabbarItem::new("Settings", div().child("S")))
                    .selected(self.tabbar)
                    .on_select(select(false, cx)),
            )
            .child(
                Slider::new("integer-slider")
                    .label("Step three slider")
                    .range(0., 10.)
                    .step(3.)
                    .value(self.sliders[0])
                    .show_value(true)
                    .on_change(cx.listener(|this, value: &f32, _, cx| {
                        this.sliders[0] = *value;
                        this.record(&format!("slider-integer-{value}"), cx);
                    })),
            )
            .child(
                Slider::new("fractional-slider")
                    .label("Fractional slider")
                    .range(-1., 1.)
                    .step(0.125)
                    .value(self.sliders[1])
                    .show_value(true)
                    .on_change(cx.listener(|this, value: &f32, _, cx| {
                        this.sliders[1] = *value;
                        this.record(&format!("slider-fractional-{value}"), cx);
                    })),
            )
            .child(
                Slider::new("disabled-slider")
                    .label("Disabled slider")
                    .value(60.)
                    .disabled(true)
                    .on_change(
                        cx.listener(|this, _, _, cx| this.record("UNEXPECTED-disabled-slider", cx)),
                    ),
            )
            .child(
                Slider::new("readonly-slider")
                    .label("Read-only slider")
                    .value(40.),
            )
            .child(
                Panel::new("data-panel")
                    .first()
                    .title("Panel callbacks")
                    .child(
                        MediaBox::new("panel-item")
                            .title("Open record")
                            .desc("Controlled reusable MediaBox callback")
                            .on_click(cx.listener(|this, _, _, cx| this.record("panel-open", cx))),
                    ),
            )
            .child(
                Cells::new()
                    .id("data-cells")
                    .title_text("Accessible record group")
                    .tips_text("Application-owned record actions")
                    .child(
                        Cell::new("record-cell")
                            .title("Open cell record")
                            .access()
                            .on_click(cx.listener(|this, _, _, cx| this.record("cell-open", cx))),
                    ),
            )
            .child(
                Grids::new("data-grid").child(
                    Grid::new("grid-item")
                        .label("Open grid record")
                        .on_click(cx.listener(|this, _, _, cx| this.record("grid-open", cx))),
                ),
            )
            .child(
                Footer::new()
                    .link(
                        FooterLink::new("footer-item", "Footer callback")
                            .on_click(cx.listener(|this, _, _, cx| this.record("footer-open", cx))),
                    )
                    .text_line("Application-owned data transport"),
            )
            .child(
                Button::new("Open data modal")
                    .id("open-data-modal")
                    .medium()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.modal.show(window, cx);
                        this.record("modal-open", cx);
                    })),
            )
            .child(
                Article::new("data-article")
                    .child(h6("Repeated heading"))
                    .child(h6("Repeated heading"))
                    .child(paragraph_text(
                        "plain-paragraph",
                        "Accessible article paragraph.",
                    ))
                    .child(ArticleList::unordered().item("Repeated list entry"))
                    .child(ArticleList::unordered().item("Repeated list entry")),
            )
            .child(
                div()
                    .id("contract-state")
                    .role(Role::Status)
                    .aria_label(format!(
                        "Events {}; progress {}; navbar {}; tabbar {}",
                        self.events, self.value, self.navbar, self.tabbar
                    ))
                    .child(format!(
                        "Events {}; progress {}; navbar {}; tabbar {}",
                        self.events, self.value, self.navbar, self.tabbar
                    )),
            )
            .child(div().h(px(650.)).flex_shrink_0().child("Scroll tail"))
            .child(
                Modal::new("data-modal", &self.modal)
                    .label("Data modal")
                    .on_dismiss(move |window, cx| {
                        owner
                            .update(cx, |this, cx| {
                                this.modal.hide(window, cx);
                                this.record("modal-dismiss", cx);
                            })
                            .ok();
                    })
                    .child(
                        div().p(px(24.)).bg(theme.bg_2).child(
                            Button::new("Close data modal")
                                .id("close-data-modal")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.modal.hide(window, cx);
                                    this.record("modal-close", cx);
                                })),
                        ),
                    ),
            );
        let page = div()
            .id("data-contracts")
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .bg(theme.bg_0)
            .child(content);
        ModalScope::new(FocusScroll::new("data-focus", &scroll, page).inset_bottom(px(80.)))
    }
}

#[path = "/src/examples/support/platform.rs"]
mod platform;

fn main() {
    platform::application()
        .with_assets(wegpui::Assets)
        .run(|cx| {
            wegpui::init(cx);
            if std::env::args().any(|arg| arg == "--dark") {
                Theme::set_mode(ThemeMode::Dark, cx);
            } else {
                Theme::set_mode(ThemeMode::Light, cx);
            }
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(0.), px(0.)),
                        size(px(600.), px(900.)),
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI data contracts".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(Probe::new)
                },
            )
            .expect("open data contracts");
            cx.activate(true);
        });
}
