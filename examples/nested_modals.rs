//! Nested-modal check: a half-screen dialog whose content opens a centred dialog. Run it and use
//! Tab, Shift-Tab and Escape: focus stays inside the topmost modal, Escape closes only the top one,
//! and each close returns focus to the button that opened it.
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    accessibility::ModalScope,
    components::{
        Presence,
        button::Button,
        dialog::{Dialog, DialogButton},
        half_screen_dialog::HalfScreenDialog,
    },
};

struct Demo {
    outer: Presence,
    inner: Presence,
    log: Vec<&'static str>,
}

impl Demo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            outer: Presence::new(cx),
            inner: Dialog::presence_for(cx),
            log: Vec::new(),
        }
    }
}

impl Render for Demo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let page = cx.entity().downgrade();
        let act = move |which: &'static str| {
            let page = page.clone();
            move |window: &mut Window, cx: &mut App| {
                page.update(cx, |this, cx| {
                    this.log.push(which);
                    match which {
                        "inner-close" => this.inner.hide(window, cx),
                        _ => this.outer.hide(window, cx),
                    }
                    cx.notify();
                })
                .ok();
            }
        };
        let inner = Dialog::new("inner")
            .title("Inner dialog")
            .body("Opened from inside the half-screen dialog.")
            .button(
                DialogButton::new("Cancel")
                    .default_style()
                    .on_click(act("inner-close")),
            )
            .button(
                DialogButton::new("OK")
                    .primary()
                    .on_click(act("inner-close")),
            )
            .presence(&self.inner)
            .on_dismiss(act("inner-close"));
        let outer = HalfScreenDialog::new("outer")
            .title("Outer sheet")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .py(px(16.))
                    .child(
                        Button::new("Open inner dialog")
                            .id("open-inner")
                            .primary()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.inner.show(window, cx);
                                cx.notify();
                            })),
                    )
                    .child(Button::new("Close sheet").id("close-outer").on_click({
                        let close = act("outer-close");
                        move |_, window, cx| close(window, cx)
                    }))
                    .child(inner),
            )
            .presence(&self.outer)
            .on_dismiss(act("outer-close"));
        ModalScope::new(
            div()
                .size_full()
                .flex()
                .flex_col()
                .gap(px(24.))
                .p(px(16.))
                .bg(t.bg_0)
                .text_color(t.fg_0)
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
                .child(format!("Log: {}", self.log.join(", ")))
                .child(
                    Button::new("Open sheet")
                        .id("open-outer")
                        .primary()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.outer.show(window, cx);
                            cx.notify();
                        })),
                )
                .child(Button::new("Background button").id("background"))
                .child(outer),
        )
    }
}

#[path = "support/platform.rs"]
mod platform;

fn main() {
    platform::application()
        .with_assets(wegpui::Assets)
        .run(|cx| {
            wegpui::init(cx);
            let bounds = Bounds::centered(None, size(px(375.), px(640.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI nested modals".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(Demo::new)
                },
            )
            .expect("open nested modals");
            cx.activate(true);
        });
}
