use super::Args;
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{button::Button, switch::Switch},
    desktop::FormRow,
    input::TextInput,
};

pub struct DesktopForm {
    name: Entity<TextInput>,
    email: Entity<TextInput>,
    notifications: bool,
    saved: bool,
}

pub fn build(_: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let saved = args.state.as_deref() == Some("saved");
    cx.new(|cx| {
        let name = cx.new(|cx| {
            let mut input = TextInput::new("Your name", cx);
            input.set_aria_label("Display name", cx);
            input.set_text("Morgan Chen", cx);
            input
        });
        let email = cx.new(|cx| {
            let mut input = TextInput::new("you@example.com", cx);
            input.set_aria_label("Email address", cx);
            input.set_text("morgan@example.com", cx);
            input
        });
        DesktopForm {
            name,
            email,
            notifications: true,
            saved,
        }
    })
    .into()
}

impl Render for DesktopForm {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let stacked = window.viewport_size().width < px(crate::desktop::BREAKPOINT);
        let field = |input: Entity<TextInput>| {
            div()
                .w_full()
                .min_w_0()
                .px(px(12.))
                .py(px(9.))
                .rounded(px(6.))
                .border_1()
                .border_color(theme.separator_0)
                .bg(theme.bg_2)
                .child(input)
        };
        let name_focus = self.name.read(cx).focus_handle(cx);
        let email_focus = self.email.read(cx).focus_handle(cx);
        PageFrame::new("desktop-form")
            .title("Profile settings")
            .desc("A desktop form with aligned labels, compact actions and caller-owned controls.")
            .spacing()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.))
                    .px(px(24.))
                    .py(px(8.))
                    .child(
                        FormRow::new("desktop-name-row", "Display name")
                            .label_width(px(180.))
                            .focus_target(name_focus)
                            .stacked(stacked)
                            .child(field(self.name.clone())),
                    )
                    .child(
                        FormRow::new("desktop-email-row", "Email address")
                            .label_width(px(180.))
                            .focus_target(email_focus)
                            .stacked(stacked)
                            .child(field(self.email.clone())),
                    )
                    .child(
                        FormRow::new("desktop-notification-row", "Notifications")
                            .label_width(px(180.))
                            .stacked(stacked)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(16.))
                                    .child(
                                        Switch::new("desktop-notifications")
                                            .checked(self.notifications)
                                            .label("Enable notifications")
                                            .on_change(cx.listener(
                                                |this, checked: &bool, _, cx| {
                                                    this.notifications = *checked;
                                                    this.saved = false;
                                                    cx.notify();
                                                },
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.))
                                            .text_color(theme.fg_1)
                                            .child("Notify me about updates"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .items_center()
                            .gap(px(12.))
                            .pt(px(20.))
                            .border_t_1()
                            .border_color(theme.separator_0)
                            .child(
                                Button::new("Save changes")
                                    .id("desktop-save")
                                    .primary()
                                    .mini()
                                    .inline()
                                    .margin_reset()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.saved = true;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("Reset")
                                    .id("desktop-reset")
                                    .mini()
                                    .inline()
                                    .margin_reset()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.name.update(cx, |input, cx| {
                                            input.set_text("Morgan Chen", cx)
                                        });
                                        this.email.update(cx, |input, cx| {
                                            input.set_text("morgan@example.com", cx)
                                        });
                                        this.notifications = true;
                                        this.saved = false;
                                        cx.notify();
                                    })),
                            )
                            .when(self.saved, |row| {
                                row.child(
                                    div()
                                        .id("desktop-saved-status")
                                        .role(Role::Status)
                                        .aria_label("Changes saved locally")
                                        .text_size(px(13.))
                                        .text_color(theme.brand)
                                        .child("Changes saved locally"),
                                )
                            }),
                    ),
            )
    }
}
