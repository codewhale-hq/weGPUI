use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    accessibility::ModalScope,
    components::{
        Cell, Cells, Presence,
        button::Button,
        dialog::{Dialog, DialogButton},
    },
};

struct Demo {
    dialog: Presence,
    confirmed: usize,
}

impl Demo {
    fn close(&mut self, confirmed: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.confirmed += confirmed as usize;
        self.dialog.hide(window, cx);
        cx.notify();
    }
}

impl Render for Demo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let page = cx.entity().downgrade();
        let close = move |confirmed: bool| {
            let page = page.clone();
            move |window: &mut Window, cx: &mut App| {
                page.update(cx, |this, cx| this.close(confirmed, window, cx))
                    .ok();
            }
        };
        ModalScope::new(
            div()
                .size_full()
                .flex()
                .flex_col()
                .gap(px(24.))
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
                .child(
                    Cells::new().child(
                        Cell::new("confirmed")
                            .title("Confirmed")
                            .value(self.confirmed.to_string()),
                    ),
                )
                .child(
                    div().px(px(16.)).child(
                        Button::new("Open dialog")
                            .id("open")
                            .primary()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dialog.show(window, cx);
                                cx.notify();
                            })),
                    ),
                )
                .child(
                    Dialog::new("demo-dialog")
                        .title("weGPUI")
                        .body("A WeUI dialog rendered by native GPUI components.")
                        .button(
                            DialogButton::new("Cancel")
                                .default_style()
                                .on_click(close(false)),
                        )
                        .button(DialogButton::new("OK").primary().on_click(close(true)))
                        .presence(&self.dialog)
                        .on_dismiss(close(false)),
                ),
        )
    }
}

fn main() {
    gpui_platform::application()
        .with_assets(wegpui::Assets)
        .run(|cx| {
            wegpui::init(cx);
            let bounds = Bounds::centered(None, size(px(375.), px(640.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(|cx| Demo {
                        dialog: Dialog::presence_for(cx),
                        confirmed: 0,
                    })
                },
            )
            .expect("open window");
            cx.activate(true);
        });
}
