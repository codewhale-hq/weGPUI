//! One application for a desktop window and a phone screen.
//!
//! Wide windows get a navigation rail and a resizable list beside the open conversation.
//! Narrow windows and phones get a bottom tab bar and one pane at a time with a Back control.
//! Nothing here checks the platform: `wegpui::adaptive` reads the room the window has.
//!
//! ```sh
//! cargo run --example adaptive --locked
//! cargo run --example adaptive --locked -- --size 390x800
//! bash examples/ios/build-simulator.sh adaptive
//! ```

use gpui::{prelude::*, *};
use wegpui::{
    Theme, ThemeMode,
    accessibility::ModalScope,
    adaptive::{AdaptiveShell, AdaptiveSplit, Destination, SafeArea, SizeClass},
    components::{
        Cell, Cells, Presence, cells_title,
        dialog::{Dialog, DialogButton},
        switch::Switch,
        textarea::TextArea,
    },
    desktop::{
        ContactRow, ConversationRow, DesktopAvatar, DesktopComposer, DesktopEmptyState,
        DesktopIconKind, DesktopPalette, MessageBubble,
    },
};

const PEOPLE: [(&str, &str, u32, &str); 4] = [
    (
        "Morgan Chen",
        "MC",
        0x4f8ef7,
        "The same view tree is running on both.",
    ),
    (
        "Lena Park",
        "LP",
        0xe8704a,
        "Try dragging the window narrower.",
    ),
    (
        "Design review",
        "DR",
        0x8f6fe8,
        "Rail on the desk, tab bar in the hand.",
    ),
    (
        "Sam Rivera",
        "SR",
        0x3aa981,
        "Back returns to the list on a phone.",
    ),
];

struct Demo {
    tab: usize,
    open: Option<usize>,
    list_width: f32,
    draft: Entity<TextArea>,
    sent: Vec<(usize, SharedString)>,
    unread: [usize; 4],
    about: Presence,
}

impl Demo {
    fn new(cx: &mut Context<Self>) -> Self {
        let draft = cx.new(|cx| {
            let mut area = TextArea::new("Message", cx);
            area.set_aria_label("Message", cx);
            area.set_rows(2, cx);
            area
        });
        Self {
            tab: 0,
            open: None,
            list_width: 280.,
            draft,
            sent: vec![],
            unread: [2, 0, 1, 0],
            about: Presence::new(cx),
        }
    }

    fn avatar(person: usize) -> DesktopAvatar {
        let (_, initials, color, _) = PEOPLE[person];
        DesktopAvatar::new(initials, rgb(color))
    }

    fn send(&mut self, cx: &mut Context<Self>) {
        let text = self.draft.read(cx).text(cx).trim().to_string();
        if let (Some(person), false) = (self.open, text.is_empty()) {
            self.sent.push((person, text.into()));
            self.draft.update(cx, |area, cx| area.clear(cx));
            cx.notify();
        }
    }

    fn conversations(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let mut rows = div()
            .id("conversations")
            .size_full()
            .overflow_y_scroll()
            .bg(DesktopPalette::get(cx).list);
        for (i, (name, _, _, preview)) in PEOPLE.iter().enumerate() {
            rows = rows.child(
                ConversationRow::new(("conversation", i), *name, Self::avatar(i))
                    .preview(*preview)
                    .time("09:30")
                    .unread(self.unread[i])
                    .selected(self.open == Some(i))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.open = Some(i);
                        this.unread[i] = 0;
                        cx.notify();
                    })),
            );
        }
        rows
    }

    fn conversation(&self, person: usize, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let (name, _, _, preview) = PEOPLE[person];
        let mut thread = div()
            .id("thread")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .p(px(16.))
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                MessageBubble::new("first")
                    .sender(name)
                    .avatar(Self::avatar(person))
                    .child(preview),
            );
        for (i, (_, text)) in self.sent.iter().filter(|(to, _)| *to == person).enumerate() {
            thread = thread.child(
                MessageBubble::new(("sent", i))
                    .outgoing()
                    .child(text.clone()),
            );
        }
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(DesktopPalette::get(cx).canvas)
            .child(thread)
            .child(
                DesktopComposer::new(self.draft.clone())
                    .on_send(cx.listener(|this, _, _, cx| this.send(cx))),
            )
    }

    fn chats(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        AdaptiveSplit::new("chats", self.conversations(cx))
            .detail(self.open.map(|person| self.conversation(person, cx)))
            .title(self.open.map_or("", |person| PEOPLE[person].0))
            .on_back(cx.listener(|this, _, _, cx| {
                this.open = None;
                cx.notify();
            }))
            .placeholder(DesktopEmptyState::new(
                DesktopIconKind::Chat,
                "No conversation open",
                "Choose a conversation from the list.",
            ))
            .width(self.list_width)
            .on_resize(cx.listener(|this, width: &f32, _, cx| {
                this.list_width = *width;
                cx.notify();
            }))
    }

    fn contacts(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let mut rows = div()
            .id("contacts")
            .size_full()
            .overflow_y_scroll()
            .bg(DesktopPalette::get(cx).canvas);
        for (i, (name, ..)) in PEOPLE.iter().enumerate() {
            rows = rows.child(
                ContactRow::new(("contact", i), *name, Self::avatar(i)).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.tab = 0;
                        this.open = Some(i);
                        cx.notify();
                    },
                )),
            );
        }
        rows
    }

    fn settings(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::get(cx);
        let layout = match SizeClass::of(window) {
            SizeClass::Compact => "Compact: tab bar, one pane",
            SizeClass::Regular => "Regular: rail, two panes",
        };
        div()
            .id("settings")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.bg_0)
            .child(cells_title("Appearance", cx))
            .child(
                Cells::new().child(
                    Cell::new("dark")
                        .title("Dark mode")
                        .ft(Switch::new("dark-switch")
                            .label("Dark mode")
                            .checked(theme.is_dark)
                            .on_change(|dark, _, cx| {
                                Theme::set_mode(
                                    if *dark {
                                        ThemeMode::Dark
                                    } else {
                                        ThemeMode::Light
                                    },
                                    cx,
                                )
                            })),
                ),
            )
            .child(cells_title("This window", cx))
            .child(
                Cells::new()
                    .child(Cell::new("layout").title("Layout").value(layout))
                    .child(
                        Cell::new("about")
                            .title("About")
                            .access()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.about.show(window, cx);
                                cx.notify();
                            })),
                    ),
            )
    }
}

impl Render for Demo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.tab {
            0 => self.chats(cx).into_any_element(),
            1 => self.contacts(cx).into_any_element(),
            _ => self.settings(window, cx).into_any_element(),
        };
        let page = cx.entity().downgrade();
        let close = move |window: &mut Window, cx: &mut App| {
            page.update(cx, |this, cx| {
                this.about.hide(window, cx);
                cx.notify();
            })
            .ok();
        };
        let shell = AdaptiveShell::new("shell", content)
            .label("Sections")
            .destination(
                Destination::new("Chats", DesktopIconKind::Chat).badge(self.unread.iter().sum()),
            )
            .destination(Destination::new("Contacts", DesktopIconKind::Contacts))
            .destination(Destination::new("Settings", DesktopIconKind::Settings))
            .selected(self.tab)
            .on_select(cx.listener(|this, tab: &usize, _, cx| {
                this.tab = *tab;
                cx.notify();
            }))
            .tab_bar_hidden(self.tab == 0 && self.open.is_some())
            .rail_header(Self::avatar(3).size(36.));
        let theme = Theme::get(cx);
        let root = div()
            .size_full()
            .text_size(px(16.))
            .text_color(theme.fg_0)
            .bg(theme.bg_0)
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
            .child(shell)
            .child(
                Dialog::new("about-dialog")
                    .title("weGPUI")
                    .body("One Rust view tree, laid out for the window it is given.")
                    .button(DialogButton::new("OK").primary().on_click(close.clone()))
                    .presence(&self.about)
                    .on_dismiss(close),
            );
        ModalScope::new(SafeArea::new(root))
    }
}

#[path = "support/platform.rs"]
mod platform;

fn main() {
    let mut size = (1040., 720.);
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--size"
            && let Some((w, h)) = args.next().as_deref().and_then(|v| v.split_once('x'))
            && let (Ok(w), Ok(h)) = (w.parse(), h.parse())
        {
            size = (w, h);
        }
    }
    platform::application()
        .with_assets(wegpui::Assets)
        .run(move |cx| {
            wegpui::init(cx);
            let bounds = Bounds::centered(None, gpui::size(px(size.0), px(size.1)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI adaptive".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    cx.new(Demo::new)
                },
            )
            .expect("adaptive window");
            cx.on_window_closed(|cx, _| cx.quit()).detach();
            cx.activate(true);
        });
}
