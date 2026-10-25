use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::{
    Theme,
    components::{
        button::Button,
        form::{FormCell, FormGroup},
        information_bar::ListTips,
        loading::Loading,
        msg::LinkText,
        panel::MediaBox,
        radio::RadioGroup,
        vcode::{VcodeButton, VcodeEvent},
    },
    input::TextInput,
};

struct States {
    input: Entity<TextInput>,
    vcode: Entity<VcodeButton>,
    busy: bool,
    saves: usize,
    sends: usize,
    panel_clicks: usize,
    link_clicks: [usize; 2],
    first: usize,
    second: usize,
    _subscription: Subscription,
}

impl States {
    fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| TextInput::new("Enter a display name", cx));
        let vcode = cx.new(|cx| {
            let mut button = VcodeButton::new(cx);
            button.set_label("Send code", cx);
            button.set_seconds(3);
            button
        });
        let subscription = cx.subscribe(&vcode, |this, _, _: &VcodeEvent, cx| {
            this.sends += 1;
            eprintln!("code sends={}", this.sends);
            cx.notify();
        });
        Self {
            input,
            vcode,
            busy: false,
            saves: 0,
            sends: 0,
            panel_clicks: 0,
            link_clicks: [0; 2],
            first: 0,
            second: 0,
            _subscription: subscription,
        }
    }
}

impl Render for States {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let first = RadioGroup::new("delivery")
            .option("Email")
            .option("SMS")
            .selected(Some(self.first))
            .on_change(cx.listener(|this, value, _, cx| {
                this.first = *value;
                eprintln!("delivery={value}");
                cx.notify();
            }))
            .rows(window, cx);
        let second = RadioGroup::new("frequency")
            .option("Daily")
            .option("Weekly")
            .selected(Some(self.second))
            .on_change(cx.listener(|this, value, _, cx| {
                this.second = *value;
                eprintln!("frequency={value}");
                cx.notify();
            }))
            .rows(window, cx);
        div()
            .id("component-states")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.bg_0)
            .text_color(theme.fg_0)
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(16.))
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
                FormGroup::new("profile").first().row(
                    FormCell::new("name")
                        .label("Display name")
                        .label_for(&self.input)
                        .child(self.input.clone()),
                ),
            )
            .child(
                Button::new("Save profile")
                    .id("save")
                    .cell()
                    .primary()
                    .loading(self.busy)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.saves += 1;
                        this.busy = true;
                        eprintln!("saves={}", this.saves);
                        cx.notify();
                        cx.spawn_in(window, async move |this, cx| {
                            cx.background_executor().timer(Duration::from_secs(3)).await;
                            this.update(cx, |this, cx| {
                                this.busy = false;
                                cx.notify();
                            })
                            .ok();
                        })
                        .detach();
                    })),
            )
            .child(
                Button::new("Unavailable")
                    .id("disabled")
                    .cell()
                    .disabled(true),
            )
            .child(self.vcode.clone())
            .child(
                Button::new("Start countdown twice")
                    .id("double-start")
                    .medium()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.vcode.update(cx, |button, cx| {
                            button.start(window, cx);
                            button.start(window, cx);
                            eprintln!("countdown remaining={}", button.remaining());
                        });
                    })),
            )
            .child(
                MediaBox::small_appmsg("details")
                    .aria_label("Profile details")
                    .child(div().p(px(12.)).child("Profile details"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.panel_clicks += 1;
                        eprintln!("panel clicks={}", this.panel_clicks);
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("radio-options")
                    .role(Role::Group)
                    .aria_label("Delivery and frequency")
                    .children(first)
                    .children(second),
            )
            .child(
                LinkText::new("inline-actions")
                    .text("Read ")
                    .link("the profile guide", {
                        let owner = cx.entity().downgrade();
                        move |_, cx| {
                            owner
                                .update(cx, |this, cx| {
                                    this.link_clicks[0] += 1;
                                    eprintln!("guide clicks={}", this.link_clicks[0]);
                                    cx.notify();
                                })
                                .ok();
                        }
                    })
                    .text(" or ")
                    .link(
                        "review the longer account preferences link that wraps onto another line",
                        {
                            let owner = cx.entity().downgrade();
                            move |_, cx| {
                                owner
                                    .update(cx, |this, cx| {
                                        this.link_clicks[1] += 1;
                                        eprintln!("preferences clicks={}", this.link_clicks[1]);
                                        cx.notify();
                                    })
                                    .ok();
                            }
                        },
                    )
                    .text(" before saving."),
            )
            .child(div().flex().gap(px(16.)).children([
                Loading::new().label("Standard spinner"),
                Loading::primary().label("Primary spinner"),
                Loading::mask().brand().label("Brand spinner"),
            ]))
            .child(
                ListTips::new()
                    .item("Your profile stays on this computer.")
                    .item("Sending is simulated."),
            )
            .child(
                div()
                    .id("results")
                    .role(Role::Label)
                    .aria_label(format!(
                        "Saved {}; sent {}; details {}",
                        self.saves, self.sends, self.panel_clicks
                    ))
                    .child(format!(
                        "Saved: {} · Sent: {} · Details: {}",
                        self.saves, self.sends, self.panel_clicks
                    )),
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
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(60.), px(60.)),
                        size(px(480.), px(900.)),
                    ))),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(States::new)
                },
            )
            .expect("open component states");
            cx.activate(true);
        });
}
