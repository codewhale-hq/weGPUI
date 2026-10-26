//! A desktop draft application with validation, controlled state, local images and modal focus.
//! Drafts stay in memory; selecting an image does not upload it.

use gpui::{prelude::*, *};
use wegpui::{
    Theme, ThemeMode,
    accessibility::ModalScope,
    components::{
        Cells, Presence,
        agree::Agree,
        button::Button,
        dialog::{Dialog, DialogButton},
        footer::{Footer, FooterLink},
        form::{CellKind, FormCell},
        gallery::{Gallery, gallery_presence},
        switch::Switch,
        textarea::TextArea,
        uploader::{Uploader, UploaderFile},
    },
    input::{InputEvent, TextInput},
    scrolling::FocusScroll,
};

const MAX_IMAGES: usize = 3;

struct Draft {
    name: Entity<TextInput>,
    notes: Entity<TextArea>,
    images: Vec<UploaderFile>,
    add_focus: FocusHandle,
    agreed: bool,
    notifications: bool,
    invalid: bool,
    attempted: bool,
    notice: SharedString,
    confirmed: usize,
    confirmation: Presence,
    viewer: Presence,
    preview: Option<usize>,
    scroll: ScrollHandle,
    _subscription: Subscription,
}

impl Draft {
    fn new(cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| {
            let mut input = TextInput::new("Your name", cx);
            input.set_aria_label("Name", cx);
            input.set_max_len(Some(60), cx);
            input
        });
        let notes = cx.new(|cx| {
            let mut area = TextArea::new("What would you like to do?", cx);
            area.set_aria_label("Notes", cx);
            area.set_rows(3, cx);
            area.set_max_len(Some(240), cx);
            area.set_show_count(true, cx);
            area
        });
        let subscription = cx.subscribe(&name, |this, input, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Changed) && this.invalid {
                this.invalid = false;
                this.notice = "".into();
                input.update(cx, |input, cx| input.set_invalid(false, cx));
                cx.notify();
            }
        });
        Self {
            name,
            notes,
            images: vec![],
            add_focus: cx.focus_handle(),
            agreed: false,
            notifications: true,
            invalid: false,
            attempted: false,
            notice: "Images remain on this device.".into(),
            confirmed: 0,
            confirmation: Dialog::presence_for(cx),
            viewer: gallery_presence(cx),
            preview: None,
            scroll: ScrollHandle::new(),
            _subscription: subscription,
        }
    }

    fn review(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.attempted = true;
        self.invalid = self.name.read(cx).text().trim().is_empty();
        self.name
            .update(cx, |input, cx| input.set_invalid(self.invalid, cx));
        if self.invalid {
            self.notice = "Enter your name before reviewing the draft.".into();
            self.name.update(cx, |input, cx| input.focus(window, cx));
        } else if !self.agreed {
            self.notice = "Accept the terms before reviewing the draft.".into();
        } else {
            self.notice = "".into();
            self.confirmation.show(window, cx);
        }
        cx.notify();
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.name.update(cx, |input, cx| {
            input.clear(cx);
            input.set_invalid(false, cx);
            input.focus(window, cx);
        });
        self.notes.update(cx, |area, cx| area.clear(cx));
        self.images.clear();
        self.agreed = false;
        self.invalid = false;
        self.attempted = false;
        self.notice = "Draft reset.".into();
        cx.notify();
    }

    fn choose_images(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.images.len() >= MAX_IMAGES {
            return;
        }
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some("Choose images".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            let result = paths.await;
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(Ok(Some(paths))) => {
                        let remaining = MAX_IMAGES.saturating_sub(this.images.len());
                        let images = paths.into_iter().filter(|path| {
                            path.extension()
                                .and_then(|ext| ext.to_str())
                                .is_some_and(|ext| {
                                    ["png", "jpg", "jpeg", "gif", "webp", "bmp"]
                                        .contains(&ext.to_ascii_lowercase().as_str())
                                })
                        });
                        this.images.extend(images.take(remaining).map(|path| {
                            let label = path
                                .file_name()
                                .map(|name| name.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "Image".into());
                            UploaderFile::new(path).label(label)
                        }));
                        this.notice =
                            format!("{} of {MAX_IMAGES} images selected.", this.images.len())
                                .into();
                    }
                    Ok(Ok(None)) | Err(_) => {}
                    Ok(Err(error)) => {
                        this.notice = format!("Could not open the file chooser: {error}").into();
                    }
                }
                if this.images.len() < MAX_IMAGES {
                    window.focus(&this.add_focus, cx);
                } else {
                    window.focus_next(cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for Draft {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let close = cx.entity().downgrade();
        let dismiss = close.clone();
        let confirm = close.clone();
        let viewer = close.clone();
        let confirmed = SharedString::from(format!("Confirmed drafts: {}", self.confirmed));
        let footer_color = theme.fg_1;
        let summary = format!(
            "{}\n{}\n{} images. Notifications {}.",
            self.name.read(cx).text().trim(),
            self.notes.read(cx).text(cx),
            self.images.len(),
            if self.notifications { "on" } else { "off" },
        );
        let page = div()
            .id("draft-scroll")
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .pb(px(140.))
            .child(
                div()
                    .p(px(20.))
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(div().text_size(px(24.)).child("Draft request"))
                    .child(
                        Button::new(if theme.is_dark { "Light mode" } else { "Dark mode" })
                            .id("theme")
                            .mini()
                            .on_click(move |_, _, cx| {
                                Theme::set_mode(
                                    if theme.is_dark { ThemeMode::Light } else { ThemeMode::Dark },
                                    cx,
                                );
                            }),
                    ),
            )
            .child(
                Cells::new()
                    .title("Your request")
                    .child(FormCell::new("name").label("Name").label_for(&self.name).child(self.name.clone()))
                    .child(FormCell::new("notes").label("Notes").kind(CellKind::Vertical).child(self.notes.clone()))
                    .child(
                        FormCell::new("notifications")
                            .label("Updates")
                            .kind(CellKind::Switch)
                            .child(
                                Switch::new("notifications-toggle")
                                    .label("Updates")
                                    .checked(self.notifications)
                                    .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                        this.notifications = *checked;
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
            .child(
                div().p(px(16.)).child(
                    Uploader::new("images", "Images")
                        .info(format!("{}/{MAX_IMAGES}", self.images.len()))
                        .max_files(MAX_IMAGES)
                        .track_add_focus(&self.add_focus)
                        .deletable(true)
                        .files(self.images.clone())
                        .on_add(cx.listener(|this, _, window, cx| this.choose_images(window, cx)))
                        .on_delete(cx.listener(|this, index: &usize, window, cx| {
                            if *index < this.images.len() {
                                this.images.remove(*index);
                                window.focus(&this.add_focus, cx);
                                cx.notify();
                            }
                        }))
                        .on_preview(cx.listener(|this, index: &usize, window, cx| {
                            this.preview = Some(*index);
                            this.viewer.show(window, cx);
                            cx.notify();
                        })),
                ),
            )
            .child(
                div().p(px(16.)).child(
                    Agree::new(
                        "consent",
                        "I have read the draft and understand that confirming it keeps this example's data in memory. ",
                    )
                    .checked(self.agreed)
                    .warn(!self.agreed && self.attempted)
                    .on_change(cx.listener(|this, checked: &bool, _, cx| {
                        this.agreed = *checked;
                        this.notice = "".into();
                        cx.notify();
                    }))
                    .link("Read the terms", cx.listener(|this, _, _, cx| {
                        this.notice = "This example sends no data and saves no files.".into();
                        cx.notify();
                    })),
                ),
            )
            .child(
                div()
                    .id("draft-status")
                    .p(px(16.))
                    .role(Role::Status)
                    .aria_label(self.notice.clone())
                    .child(self.notice.clone()),
            )
            .child(
                div().px(px(16.)).child(
                    Button::new("Review draft")
                        .id("review")
                        .primary()
                        .on_click(cx.listener(|this, _, window, cx| this.review(window, cx))),
                ),
            );
        let footer_owner = cx.entity().downgrade();
        let preview = self
            .preview
            .and_then(|index| self.images.get(index))
            .map(|f| f.src.clone());
        ModalScope::new(
            div()
                .size_full()
                .bg(Theme::get(cx).bg_0)
                .text_color(Theme::get(cx).fg_0)
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
                .child(FocusScroll::new("draft-focus", &self.scroll, page).inset_bottom(px(132.)))
                .child(
                    Footer::new()
                        .fixed_bottom()
                        .safe_area_bottom(px(24.))
                        .link(FooterLink::new("reset", "Reset draft").on_click(
                            move |_, window, cx| {
                                footer_owner
                                    .update(cx, |this, cx| this.reset(window, cx))
                                    .ok();
                            },
                        ))
                        .text(
                            div()
                                .id("confirmed-count")
                                .role(Role::Paragraph)
                                .aria_label(confirmed.clone())
                                .text_color(footer_color)
                                .child(confirmed),
                        ),
                )
                .child(
                    Dialog::new("confirm-draft")
                        .title("Confirm this draft?")
                        .body(summary)
                        .button(DialogButton::new("Cancel").default_style().on_click(
                            move |window, cx| {
                                close
                                    .update(cx, |this, cx| {
                                        this.confirmation.hide(window, cx);
                                        cx.notify();
                                    })
                                    .ok();
                            },
                        ))
                        .button(DialogButton::new("Confirm").primary().on_click(
                            move |window, cx| {
                                confirm
                                    .update(cx, |this, cx| {
                                        this.confirmed += 1;
                                        this.confirmation.hide(window, cx);
                                        this.notice = "Draft confirmed for this session.".into();
                                        cx.notify();
                                    })
                                    .ok();
                            },
                        ))
                        .presence(&self.confirmation)
                        .on_dismiss(move |window, cx| {
                            dismiss
                                .update(cx, |this, cx| {
                                    this.confirmation.hide(window, cx);
                                    cx.notify();
                                })
                                .ok();
                        }),
                )
                .child(
                    Gallery::new("image-preview", &self.viewer)
                        .when_some(preview, |gallery, src| gallery.src(src))
                        .on_dismiss(move |window, cx| {
                            viewer
                                .update(cx, |this, cx| {
                                    this.viewer.hide(window, cx);
                                    cx.notify();
                                })
                                .ok();
                        }),
                ),
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
                        point(px(0.), px(0.)),
                        size(px(420.), px(812.)),
                    ))),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(Draft::new)
                },
            )
            .expect("open application window");
            cx.activate(true);
        });
}
