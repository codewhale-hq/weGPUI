use super::Args;
use gpui::{prelude::*, *};
use wegpui::{
    Theme, ThemeMode,
    components::{Modal, Presence, textarea::TextArea},
    desktop::*,
    input::{InputEvent, TextInput},
    scrolling::FocusScroll,
};

pub const EXAMPLES: &[(&str, &str)] = &[
    ("desktop-chat", "Chat"),
    ("desktop-controls", "Controls"),
    ("desktop-messages", "Messages"),
    ("desktop-contacts", "Contacts"),
    ("desktop-settings", "Settings"),
    ("desktop-overlays", "Menus & dialogs"),
    ("desktop-history", "Chat history"),
    ("desktop-chat-info", "Chat info"),
    ("desktop-favorites", "Favorites"),
];

const PEOPLE: &[(&str, u32, &str)] = &[
    (
        "Morgan Chen",
        0x657f72,
        "The prototype is ready for a look.",
    ),
    ("Lena Park", 0xa87562, "I’ve added the notes to the file."),
    ("Alex Rivera", 0x687d9b, "See you at the studio tomorrow."),
    ("Sophie Wu", 0x9a7b99, "这版的细节很舒服。"),
    ("Noah Brooks", 0x8b916c, "[Photo] Afternoon light"),
    ("Hana Mori", 0x598e98, "Thanks! This is looking good."),
];
const CHATS: &[(&str, &str, &str)] = &[
    (
        "Design studio",
        "Morgan: The prototype is ready for a look.",
        "09:41",
    ),
    ("Lena Park", "I’ve added the notes to the file.", "09:32"),
    ("Weekend plans", "Alex: Shall we meet at ten?", "08:50"),
    ("Sophie Wu", "这版的细节很舒服。", "Yesterday"),
    ("Noah Brooks", "[Photo] Afternoon light", "Yesterday"),
    ("Product team", "Hana: Thanks everyone", "Tuesday"),
    ("File transfer", "Components.fig", "Tuesday"),
    ("Hana Mori", "Thanks! This is looking good.", "Monday"),
];
const EMOJI: &[&str] = &[
    "😀", "😊", "🥰", "😎", "🤔", "😅", "😂", "🥹", "🙃", "🙂", "😉", "😍", "🤩", "🥳", "😌", "😴",
    "😮", "🤭", "🫡", "🤝", "👍", "👏", "🙌", "👌", "✌️", "🫶", "❤️", "💚", "✨", "🎉", "☕", "🌿",
    "🌻", "🍊", "🐳", "🐈", "🌤️", "🌙", "🔥", "💡", "🎨", "📚", "🎧", "🌊", "🏔️", "🍵", "✅", "🚀",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Popup {
    None,
    Navigation,
    More,
    Emoji,
    Appearance,
    Language,
    Message,
    Confirm,
    Media,
}

pub struct MacGallery {
    route: &'static str,
    state: String,
    search: Entity<TextInput>,
    field: Entity<TextInput>,
    draft: Entity<TextArea>,
    _subscriptions: Vec<Subscription>,
    selected: usize,
    contact: usize,
    sent: Vec<(usize, String)>,
    unread: [usize; 8],
    sidebar: f32,
    settings_tab: usize,
    flags: [bool; 8],
    history_tab: usize,
    info: bool,
    compact_list: bool,
    overlay: Presence,
    popup: Popup,
    popup_position: Point<Pixels>,
    notice: Option<String>,
    saved: bool,
    language: usize,
}

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone().unwrap_or_default();
    cx.new(|cx| {
        let search = cx.new(|cx| {
            let mut input = TextInput::new("Search", cx);
            input.set_aria_label("Search conversations, contacts or history", cx);
            input
        });
        let field = cx.new(|cx| {
            let mut input = TextInput::new("Display name", cx);
            input.set_aria_label("Display name", cx);
            input.set_text("Morgan Chen", cx);
            input
        });
        let draft = cx.new(|cx| {
            let mut area = TextArea::new("Message", cx);
            area.set_aria_label("Message draft", cx);
            area.set_rows(3, cx);
            area
        });
        let mut subscriptions = vec![
            cx.subscribe(&search, |_, _, _: &InputEvent, cx| cx.notify()),
            cx.subscribe(&field, |_, _, _: &InputEvent, cx| cx.notify()),
        ];
        subscriptions.push(cx.subscribe(&draft, |_, _, _: &InputEvent, cx| cx.notify()));
        let popup = match (route, state.as_str()) {
            ("desktop-overlays", "menu") => Popup::Message,
            ("desktop-overlays", "dialog") => Popup::Confirm,
            ("desktop-overlays", "emoji") => Popup::Emoji,
            ("desktop-settings", "appearance") => Popup::Appearance,
            _ => Popup::None,
        };
        if popup != Popup::None {
            let weak = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                weak.update(cx, |this: &mut MacGallery, cx| {
                    this.overlay.show_instantly(window, cx);
                    cx.notify();
                })
                .ok();
            });
        }
        let settings_tab = match state.as_str() {
            "shortcuts" => 1,
            "notifications" => 2,
            _ => 0,
        };
        let history_tab = match state.as_str() {
            "files" => 1,
            "media" => 2,
            "links" => 3,
            _ => 0,
        };
        let notice = (state == "toast").then(|| "Saved to Favorites".to_string());
        MacGallery {
            route,
            state,
            search,
            field,
            draft,
            _subscriptions: subscriptions,
            selected: 0,
            contact: 0,
            sent: vec![],
            unread: [0, 2, 5, 0, 0, 0, 0, 0],
            sidebar: 270.,
            settings_tab,
            flags: [true, true, false, true, false, true, false, false],
            history_tab,
            info: route == "desktop-chat-info",
            compact_list: false,
            overlay: Presence::new(cx),
            popup,
            popup_position: point(px(370.), px(170.)),
            notice,
            saved: false,
            language: 0,
        }
    })
    .into()
}

impl MacGallery {
    fn show(&mut self, popup: Popup, window: &mut Window, cx: &mut Context<Self>) {
        self.popup = popup;
        self.popup_position = if popup == Popup::Navigation {
            point(px(12.), px(40.))
        } else {
            window.mouse_position()
        };
        self.overlay.show(window, cx);
        cx.notify();
    }
    fn hide(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.overlay.hide(window, cx);
        cx.notify();
    }
    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input = self.draft.read(cx).input().clone();
        if input
            .update(cx, |input, cx| input.marked_text_range(window, cx))
            .is_some()
        {
            return;
        }
        let text = self.draft.read(cx).text(cx).trim().to_string();
        if text.is_empty() {
            return;
        }
        self.sent.push((self.selected, text));
        self.draft.update(cx, |area, cx| area.clear(cx));
        self.draft.read(cx).focus_handle(cx).focus(window, cx);
        cx.notify();
    }
    fn avatar(index: usize) -> DesktopAvatar {
        let (name, color, _) = PEOPLE[index % PEOPLE.len()];
        DesktopAvatar::new(name, rgb(color))
    }
    fn group_avatar() -> DesktopAvatar {
        DesktopAvatar::new("Design studio", rgb(0xc8c8c8)).members(
            PEOPLE
                .iter()
                .take(4)
                .map(|(name, color, _)| ((*name).into(), rgb(*color).into())),
        )
    }
    fn button(
        &self,
        id: &'static str,
        label: &'static str,
        icon: DesktopIconKind,
        popup: Popup,
        cx: &Context<Self>,
    ) -> DesktopButton {
        DesktopButton::new(id, label)
            .icon(icon)
            .icon_only()
            .on_click(cx.listener(move |this, _, w, cx| this.show(popup, w, cx)))
    }
    fn scroll(
        &self,
        id: &'static str,
        body: Div,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let handle = window
            .use_keyed_state(id, cx, |_, _| ScrollHandle::new())
            .read(cx)
            .clone();
        FocusScroll::new(
            SharedString::from(format!("{id}-focus")),
            &handle,
            body.id(id)
                .h_full()
                .overflow_y_scroll()
                .track_scroll(&handle),
        )
        .into_any_element()
    }
    fn toolbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let title = EXAMPLES
            .iter()
            .find(|(route, _)| *route == self.route)
            .unwrap()
            .1;
        div()
            .h(px(38.))
            .flex_shrink_0()
            .px(px(10.))
            .flex()
            .items_center()
            .gap(px(8.))
            .border_b_1()
            .border_color(p.border)
            .bg(p.panel)
            .child(
                DesktopButton::new("all-components", "All components")
                    .icon(DesktopIconKind::Back)
                    .compact()
                    .on_click(|_, w, cx| crate::home(w, cx)),
            )
            .child(div().w(px(1.)).h(px(14.)).bg(p.border))
            .child(
                DesktopButton::new("desktop-examples", title)
                    .icon(DesktopIconKind::ChevronDown)
                    .compact()
                    .on_click(cx.listener(|this, _, w, cx| this.show(Popup::Navigation, w, cx))),
            )
            .child(div().flex_1())
            .child(
                DesktopButton::new(
                    "toggle-appearance",
                    if Theme::get(cx).is_dark {
                        "Light"
                    } else {
                        "Dark"
                    },
                )
                .compact()
                .on_click(|_, _, cx| {
                    let mode = if Theme::get(cx).is_dark {
                        ThemeMode::Light
                    } else {
                        ThemeMode::Dark
                    };
                    Theme::set_mode(mode, cx);
                }),
            )
            .into_any_element()
    }
    fn rail(&self, cx: &mut Context<Self>) -> DesktopRail {
        let active = if self.route == "desktop-contacts" {
            1
        } else if self.route == "desktop-favorites" {
            2
        } else {
            0
        };
        let mut rail = DesktopRail::new().child(Self::avatar(0).size(36.));
        for (i, (route, label, icon)) in [
            ("desktop-chat", "Chats", DesktopIconKind::Chat),
            ("desktop-contacts", "Contacts", DesktopIconKind::Contacts),
            ("desktop-favorites", "Favorites", DesktopIconKind::Favorites),
        ]
        .into_iter()
        .enumerate()
        {
            rail = rail.child(
                DesktopButton::new(("rail", i), label)
                    .icon(icon)
                    .icon_only()
                    .selected(active == i)
                    .on_click(move |_, w, cx| crate::go(route, w, cx)),
            );
        }
        rail.footer(
            DesktopButton::new("rail-settings", "Settings")
                .icon(DesktopIconKind::Settings)
                .icon_only()
                .on_click(|_, w, cx| crate::go("desktop-settings", w, cx)),
        )
        .footer(self.button("rail-more", "More", DesktopIconKind::More, Popup::More, cx))
    }
    fn conversation_list(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let query = self.search.read(cx).text().to_lowercase();
        let mut rows = div();
        let mut count = 0;
        for (i, (title, preview, time)) in CHATS.iter().enumerate() {
            if !format!("{title} {preview}").to_lowercase().contains(&query) {
                continue;
            }
            count += 1;
            let avatar = if matches!(i, 0 | 2 | 5) {
                Self::group_avatar()
            } else {
                Self::avatar(i)
            };
            rows = rows.child(
                ConversationRow::new(("conversation", i), *title, avatar)
                    .preview(*preview)
                    .time(*time)
                    .selected(self.selected == i)
                    .unread(self.unread[i])
                    .pinned(i < 2)
                    .muted(i == 5)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected = i;
                        this.unread[i] = 0;
                        this.compact_list = false;
                        cx.notify();
                    })),
            );
        }
        if count == 0 {
            rows = rows.child(DesktopEmptyState::new(
                DesktopIconKind::Search,
                "No conversations",
                "Try a different name or clear the search.",
            ));
        }
        div()
            .size_full()
            .bg(p.list)
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(52.))
                    .px(px(10.))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .gap(px(5.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(DesktopField::new("chat-search", self.search.clone()).search()),
                    )
                    .child(
                        DesktopButton::new("new-chat", "New chat")
                            .icon(DesktopIconKind::Add)
                            .icon_only()
                            .compact()
                            .on_click(|_, w, cx| crate::go("desktop-contacts", w, cx)),
                    ),
            )
            .child(div().flex_1().min_h_0().child(self.scroll(
                "conversation-scroll",
                rows,
                window,
                cx,
            )))
            .into_any_element()
    }
    fn transcript(
        &self,
        expanded: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let mut body = div().p(px(24.)).flex().flex_col().gap(px(22.));
        if self.state == "empty" {
            body = body.child(DesktopEmptyState::new(
                DesktopIconKind::Chat,
                "Start a conversation",
                "Messages you send in this gallery stay in this window.",
            ));
        } else if self.selected == 0 || expanded {
            body = body
                .child(
                    div()
                        .text_center()
                        .text_size(px(11.))
                        .text_color(p.secondary)
                        .child("Today 09:30"),
                )
                .child(
                    MessageBubble::new("message-1")
                        .sender("Morgan Chen")
                        .avatar(Self::avatar(0))
                        .child("The desktop components are ready for a look."),
                )
                .child(
                    MessageBubble::new("message-2")
                        .outgoing()
                        .avatar(Self::avatar(2))
                        .child("Nice. How do the compact controls feel on the Mac?"),
                )
                .child(
                    MessageBubble::new("message-3")
                        .sender("Lena Park")
                        .avatar(Self::avatar(1))
                        .child("Quiet, familiar, and much better with a little room to breathe.")
                        .child(div().mt(px(8.)).child(MessageQuote::new(
                            "Alex",
                            "How do the compact controls feel?",
                        ))),
                )
                .child(
                    MessageBubble::new("message-4")
                        .sender("Sophie Wu")
                        .avatar(Self::avatar(3))
                        .child("这版的细节很舒服。")
                        .child(MessageTranslation::new(
                            "The details in this version feel just right.",
                        )),
                )
                .child(
                    MessageBubble::new("message-5")
                        .sender("Morgan Chen")
                        .avatar(Self::avatar(0))
                        .child(MessageAttachment::new(
                            DesktopAttachmentKind::File,
                            "Desktop components.pdf",
                            "PDF · 2.4 MB",
                        )),
                );
            if expanded {
                body = body
                    .child(
                        MessageBubble::new("message-6")
                            .sender("Hana Mori")
                            .avatar(Self::avatar(5))
                            .child(MessageAttachment::new(
                                DesktopAttachmentKind::Link,
                                "A field guide to good interfaces",
                                "studio.example · Link preview",
                            )),
                    )
                    .child(
                        MessageBubble::new("message-7")
                            .sender("Noah Brooks")
                            .avatar(Self::avatar(4))
                            .child(MessageAttachment::new(
                                DesktopAttachmentKind::Voice,
                                "Voice message",
                                "0:12 · Select Play below",
                            )),
                    )
                    .child(
                        MessageBubble::new("message-8")
                            .outgoing()
                            .avatar(Self::avatar(2))
                            .status("Couldn’t send · Offline example")
                            .child("A message that needs another try."),
                    )
                    .child(
                        DesktopButton::new("retry-message", "Retry message")
                            .icon(DesktopIconKind::Send)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.notice =
                                    Some("Message added to the local conversation".into());
                                cx.notify();
                            })),
                    );
            }
        } else {
            let person = self.selected % PEOPLE.len();
            body = body.child(
                MessageBubble::new("message-9")
                    .sender(PEOPLE[person].0)
                    .avatar(Self::avatar(person))
                    .child(PEOPLE[person].2),
            );
        }
        for (index, (i, text)) in self.sent.iter().enumerate() {
            if *i == self.selected {
                body = body.child(
                    MessageBubble::new(("sent-message", index))
                        .outgoing()
                        .avatar(Self::avatar(2))
                        .status("Local preview")
                        .child(text.clone()),
                );
            }
        }
        self.scroll("transcript-scroll", body, window, cx)
    }
    fn composer(&self, cx: &mut Context<Self>) -> AnyElement {
        DesktopComposer::new(self.draft.clone())
            .on_send(cx.listener(|this, _, w, cx| this.send(w, cx)))
            .child(self.button(
                "open-emoji",
                "Emoji",
                DesktopIconKind::Emoji,
                Popup::Emoji,
                cx,
            ))
            .child(
                DesktopButton::new("attach-sample", "Attach sample file")
                    .icon(DesktopIconKind::Folder)
                    .icon_only()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.draft.update(cx, |area, cx| {
                            area.set_text("[File] Desktop components.pdf", cx)
                        });
                        cx.notify();
                    })),
            )
            .child(
                DesktopButton::new("capture-sample", "Insert sample image")
                    .icon(DesktopIconKind::Scissors)
                    .icon_only()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.draft
                            .update(cx, |area, cx| area.set_text("[Image] Studio reference", cx));
                        cx.notify();
                    })),
            )
            .child(
                DesktopButton::new(
                    "voice-input",
                    "Voice recording requires an application handler",
                )
                .icon(DesktopIconKind::Microphone)
                .icon_only()
                .disabled(true),
            )
            .into_any_element()
    }
    fn chat(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let compact = window.viewport_size().width < px(760.);
        let header = ChatHeader::new(CHATS[self.selected].0)
            .when(compact, |h| {
                h.child(
                    DesktopButton::new("show-conversations", "Conversations")
                        .icon(DesktopIconKind::Back)
                        .icon_only()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.compact_list = true;
                            cx.notify();
                        })),
                )
            })
            .child(
                DesktopButton::new("open-history", "Chat history")
                    .icon(DesktopIconKind::History)
                    .icon_only()
                    .on_click(|_, w, cx| crate::go("desktop-history", w, cx)),
            )
            .child(
                DesktopButton::new("call", "Voice calls require an application handler")
                    .icon(DesktopIconKind::Phone)
                    .icon_only()
                    .disabled(true),
            )
            .child(
                DesktopButton::new("chat-info", "Chat info")
                    .icon(DesktopIconKind::More)
                    .icon_only()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.info = !this.info;
                        cx.notify();
                    })),
            );
        let main = div()
            .size_full()
            .flex()
            .flex_col()
            .child(header)
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .child(self.transcript(false, window, cx)),
            )
            .child(self.composer(cx));
        let main = div()
            .size_full()
            .flex()
            .child(div().flex_1().min_w_0().child(main))
            .when(self.info && !compact, |el| {
                el.child(
                    div()
                        .w(px(268.))
                        .h_full()
                        .flex_shrink_0()
                        .child(self.info_panel(window, cx)),
                )
            });
        if compact {
            return if self.compact_list {
                self.conversation_list(window, cx)
            } else {
                main.into_any_element()
            };
        }
        div()
            .size_full()
            .flex()
            .child(self.rail(cx))
            .child(
                div().flex_1().min_w_0().child(
                    DesktopSplit::new(
                        "chat-sidebar-divider",
                        self.sidebar,
                        self.conversation_list(window, cx),
                        main,
                    )
                    .limits(230., 360.)
                    .on_resize(cx.listener(|this, width: &f32, _, cx| {
                        this.sidebar = *width;
                        cx.notify();
                    })),
                ),
            )
            .into_any_element()
    }
    fn info_panel(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let mut members = DesktopMemberGrid::new();
        for (i, (name, _, _)) in PEOPLE.iter().enumerate() {
            members = members.member(*name, Self::avatar(i));
        }
        let body = div()
            .p(px(18.))
            .child(members)
            .child(
                DesktopSettingRow::new("Group name")
                    .child(div().text_size(px(12.)).child("Design studio")),
            )
            .child(
                DesktopSettingRow::new("Group notice")
                    .detail("Share your work, leave thoughtful notes."),
            )
            .child(DesktopSettingRow::new("My alias").child(div().text_size(px(12.)).child("Alex")))
            .child(
                DesktopSettingRow::new("Mute notifications").child(
                    DesktopSwitch::new("mute-group", "Mute notifications")
                        .checked(self.flags[4])
                        .on_change(cx.listener(|this, value: &bool, _, cx| {
                            this.flags[4] = *value;
                            cx.notify();
                        })),
                ),
            )
            .child(
                DesktopSettingRow::new("Pin chat").child(
                    DesktopSwitch::new("pin-chat", "Pin chat")
                        .checked(self.flags[5])
                        .on_change(cx.listener(|this, value: &bool, _, cx| {
                            this.flags[5] = *value;
                            cx.notify();
                        })),
                ),
            )
            .child(
                DesktopSettingRow::new("Save to Contacts").child(
                    DesktopSwitch::new("save-contact", "Save to Contacts")
                        .checked(self.flags[6])
                        .on_change(cx.listener(|this, value: &bool, _, cx| {
                            this.flags[6] = *value;
                            cx.notify();
                        })),
                ),
            )
            .child(
                div().mt(px(18.)).child(
                    DesktopButton::new("clear-conversation", "Clear local messages")
                        .style(DesktopButtonStyle::Danger)
                        .on_click(cx.listener(|this, _, w, cx| this.show(Popup::Confirm, w, cx))),
                ),
            );
        div()
            .size_full()
            .border_l_1()
            .border_color(p.border)
            .bg(p.canvas)
            .flex()
            .flex_col()
            .child(ChatHeader::new("Chat info"))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .child(self.scroll("info-scroll", body, window, cx)),
            )
            .into_any_element()
    }
    fn controls(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let disabled = self.state == "disabled";
        let mut body = div()
            .p(px(32.))
            .max_w(px(850.))
            .flex()
            .flex_col()
            .gap(px(30.));
        body = body.child(
            div().child(section("Buttons", cx)).child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(10.))
                    .children(
                        [
                            (DesktopButtonStyle::Primary, "Primary"),
                            (DesktopButtonStyle::Filled, "Default"),
                            (DesktopButtonStyle::Quiet, "Quiet"),
                            (DesktopButtonStyle::Danger, "Destructive"),
                        ]
                        .into_iter()
                        .enumerate()
                        .map(|(i, (style, label))| {
                            DesktopButton::new(("control-button", i), label)
                                .style(style)
                                .disabled(disabled)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.notice = Some(format!("{label} action selected"));
                                    cx.notify();
                                }))
                        }),
                    )
                    .child(
                        div()
                            .mt(px(12.))
                            .flex()
                            .gap(px(10.))
                            .child(
                                DesktopButton::new("disabled-example", "Disabled")
                                    .style(DesktopButtonStyle::Filled)
                                    .disabled(true),
                            )
                            .child(DesktopButton::new("loading-example", "Working").loading(true)),
                    ),
            ),
        );
        body = body.child(
            div()
                .child(section("Fields & selection", cx))
                .child(
                    DesktopSettingRow::new("Display name").child(
                        div().w(px(240.)).max_w_full().child(
                            DesktopField::new("control-field", self.field.clone())
                                .when(self.state == "error", |field| {
                                    field.error("Enter a display name.")
                                }),
                        ),
                    ),
                )
                .child(
                    DesktopSettingRow::new("Search").child(
                        div().w(px(240.)).max_w_full().child(
                            DesktopField::new("control-search", self.search.clone()).search(),
                        ),
                    ),
                )
                .child(
                    DesktopSettingRow::new("Appearance").child(
                        DesktopSelect::new(
                            "control-select",
                            "Appearance",
                            if Theme::get(cx).is_dark {
                                "Dark Mode"
                            } else {
                                "Light Mode"
                            },
                        )
                        .on_open(cx.listener(|this, _, w, cx| this.show(Popup::Appearance, w, cx))),
                    ),
                )
                .child(
                    DesktopSettingRow::new("Notifications").child(
                        DesktopSwitch::new("control-switch", "Notifications")
                            .disabled(disabled)
                            .checked(self.flags[0])
                            .on_change(cx.listener(|this, value: &bool, _, cx| {
                                this.flags[0] = *value;
                                cx.notify();
                            })),
                    ),
                )
                .child(
                    div().mt(px(16.)).child(
                        DesktopCheckbox::new("control-checkbox", "Keep this window on top")
                            .disabled(disabled)
                            .checked(self.flags[2])
                            .on_change(cx.listener(|this, value: &bool, _, cx| {
                                this.flags[2] = *value;
                                cx.notify();
                            })),
                    ),
                ),
        );
        body = body.child(
            div()
                .child(section("Tabs, badges & shortcuts", cx))
                .child(
                    DesktopTabs::new("Component tabs").children(
                        ["Chats", "Contacts", "Files"]
                            .into_iter()
                            .enumerate()
                            .map(|(i, label)| {
                                DesktopButton::new(("demo-tab", i), label)
                                    .role(Role::Tab)
                                    .selected(self.history_tab == i)
                                    .style(DesktopButtonStyle::Selection)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.history_tab = i;
                                        cx.notify();
                                    }))
                            }),
                    ),
                )
                .child(
                    div()
                        .mt(px(18.))
                        .flex()
                        .gap(px(12.))
                        .items_center()
                        .child(DesktopBadge::new(3))
                        .child(DesktopBadge::new(128))
                        .child(DesktopKeycap::new("⌘"))
                        .child(DesktopKeycap::new("Enter")),
                ),
        );
        body = body.child(
            div().child(section("Icons", cx)).child(
                div().flex().flex_wrap().gap(px(20.)).children(
                    [
                        DesktopIconKind::Chat,
                        DesktopIconKind::Contacts,
                        DesktopIconKind::Favorites,
                        DesktopIconKind::Discover,
                        DesktopIconKind::Phone,
                        DesktopIconKind::Video,
                        DesktopIconKind::Emoji,
                        DesktopIconKind::Folder,
                        DesktopIconKind::Scissors,
                        DesktopIconKind::Microphone,
                        DesktopIconKind::History,
                        DesktopIconKind::Settings,
                        DesktopIconKind::Bell,
                        DesktopIconKind::Keyboard,
                        DesktopIconKind::File,
                        DesktopIconKind::Link,
                        DesktopIconKind::Photo,
                        DesktopIconKind::Calendar,
                    ]
                    .map(|icon| DesktopIcon::new(icon).color(p.secondary)),
                ),
            ),
        );
        self.scroll("control-scroll", body, window, cx)
    }
    fn contacts(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let query = self.search.read(cx).text().to_lowercase();
        let mut list = div().p(px(4.));
        for (i, (name, _, _)) in PEOPLE.iter().enumerate() {
            if name.to_lowercase().contains(&query) {
                list = list.child(
                    ContactRow::new(("contact", i), *name, Self::avatar(i))
                        .selected(self.contact == i)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.contact = i;
                            cx.notify();
                        })),
                );
            }
        }
        let list = div()
            .size_full()
            .flex()
            .flex_col()
            .bg(p.list)
            .child(
                div()
                    .p(px(12.))
                    .child(DesktopField::new("contact-search", self.search.clone()).search()),
            )
            .child(div().flex_1().min_h_0().child(self.scroll(
                "contacts-scroll",
                list,
                window,
                cx,
            )));
        let (name, _, _) = PEOPLE[self.contact];
        let profile = ContactProfile::new(name, Self::avatar(self.contact))
            .subtitle("Studio · Contact")
            .child(DesktopSettingRow::new("Alias").child(name))
            .child(DesktopSettingRow::new("Region").child("San Francisco"))
            .child(DesktopSettingRow::new("Tags").child("Design studio"))
            .action(
                DesktopButton::new("profile-message", "Message")
                    .icon(DesktopIconKind::Chat)
                    .style(DesktopButtonStyle::Primary)
                    .on_click(|_, w, cx| crate::go("desktop-chat", w, cx)),
            )
            .action(
                DesktopButton::new("profile-more", "More")
                    .icon(DesktopIconKind::More)
                    .on_click(cx.listener(|this, _, w, cx| this.show(Popup::Message, w, cx))),
            );
        let main = div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(profile);
        div()
            .size_full()
            .flex()
            .when(window.viewport_size().width >= px(760.), |el| {
                el.child(self.rail(cx))
            })
            .child(
                div().flex_1().min_w_0().child(
                    DesktopSplit::new("contacts-divider", self.sidebar, list, main)
                        .limits(200., 320.)
                        .on_resize(cx.listener(|this, w: &f32, _, cx| {
                            this.sidebar = *w;
                            cx.notify();
                        })),
                ),
            )
            .into_any_element()
    }
    fn settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let mut settings =
            DesktopSettings::new(["General", "Shortcuts", "Notifications"][self.settings_tab]);
        for (i, (label, icon)) in [
            ("General", DesktopIconKind::Settings),
            ("Shortcuts", DesktopIconKind::Keyboard),
            ("Notifications", DesktopIconKind::Bell),
        ]
        .into_iter()
        .enumerate()
        {
            settings = settings.navigation(
                DesktopButton::new(("settings-tab", i), label)
                    .icon(icon)
                    .style(DesktopButtonStyle::Selection)
                    .selected(self.settings_tab == i)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.settings_tab = i;
                        cx.notify();
                    })),
            );
        }
        if self.settings_tab == 0 {
            settings = settings
                .child(
                    DesktopSettingRow::new("Appearance").child(
                        DesktopSelect::new(
                            "appearance",
                            "Appearance",
                            if Theme::get(cx).is_dark {
                                "Dark Mode"
                            } else {
                                "Light Mode"
                            },
                        )
                        .expanded(self.popup == Popup::Appearance && self.overlay.is_open())
                        .on_open(cx.listener(|this, _, w, cx| this.show(Popup::Appearance, w, cx))),
                    ),
                )
                .child(
                    DesktopSettingRow::new("Language").child(
                        DesktopSelect::new(
                            "language",
                            "Language",
                            ["Automatic", "English", "简体中文"][self.language],
                        )
                        .on_open(cx.listener(|this, _, w, cx| this.show(Popup::Language, w, cx))),
                    ),
                )
                .child(DesktopSettingRow::new("Translate to").child("English"));
            for (i, label) in [
                (0, "Show WeChat in menu bar"),
                (1, "Confirm before sending files"),
                (2, "Open chat files in read-only mode"),
            ] {
                settings = settings.child(
                    DesktopSettingRow::new(label).child(
                        DesktopSwitch::new(("general-switch", i), label)
                            .checked(self.flags[i])
                            .on_change(cx.listener(move |this, value: &bool, _, cx| {
                                this.flags[i] = *value;
                                cx.notify();
                            })),
                    ),
                );
            }
            settings = settings.child(
                div()
                    .pt(px(28.))
                    .text_size(px(12.))
                    .text_color(DesktopPalette::get(cx).secondary)
                    .child("Settings in this gallery apply to the local preview."),
            );
        } else if self.settings_tab == 1 {
            for (label, keys) in [
                ("Send message", vec!["⌘", "Enter"]),
                ("New line", vec!["Enter"]),
                ("Search", vec!["⌘", "F"]),
                ("Close overlay", vec!["Esc"]),
                ("Move between controls", vec!["Tab"]),
            ] {
                settings = settings.child(
                    DesktopSettingRow::new(label)
                        .children(keys.into_iter().map(DesktopKeycap::new)),
                );
            }
        } else {
            for (i, label) in [
                (0, "New message notifications"),
                (1, "Notification sounds"),
                (3, "Show message preview"),
                (4, "Mute group notifications"),
            ] {
                settings = settings.child(
                    DesktopSettingRow::new(label).child(
                        DesktopSwitch::new(("notify-switch", i), label)
                            .checked(self.flags[i])
                            .on_change(cx.listener(move |this, value: &bool, _, cx| {
                                this.flags[i] = *value;
                                cx.notify();
                            })),
                    ),
                );
            }
        }
        settings.into_any_element()
    }
    fn history(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let tabs = ["All", "Files", "Media", "Links"];
        let mut filters = DesktopTabs::new("History type");
        for (i, label) in tabs.into_iter().enumerate() {
            filters = filters.child(
                DesktopButton::new(("history-type", i), label)
                    .role(Role::Tab)
                    .selected(self.history_tab == i)
                    .style(DesktopButtonStyle::Selection)
                    .compact()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.history_tab = i;
                        cx.notify();
                    })),
            );
        }
        let query = self.search.read(cx).text().to_lowercase();
        let mut items = div().p(px(24.)).flex().flex_col().gap(px(20.));
        let mut count = 0;
        if self.state != "empty" {
            for (i, (kind, title, detail)) in [
                (
                    DesktopAttachmentKind::File,
                    "Desktop components.pdf",
                    "Morgan Chen · Today 09:38 · 2.4 MB",
                ),
                (
                    DesktopAttachmentKind::Image,
                    "Studio reference",
                    "Noah Brooks · Today 09:20",
                ),
                (
                    DesktopAttachmentKind::Link,
                    "A field guide to good interfaces",
                    "Hana Mori · Yesterday 16:20",
                ),
                (
                    DesktopAttachmentKind::File,
                    "Meeting notes.md",
                    "Lena Park · Yesterday 14:05 · 12 KB",
                ),
                (
                    DesktopAttachmentKind::Video,
                    "Prototype walkthrough.mov",
                    "Alex Rivera · Tuesday 11:32 · 18 MB",
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let matches = match self.history_tab {
                    1 => matches!(kind, DesktopAttachmentKind::File),
                    2 => matches!(
                        kind,
                        DesktopAttachmentKind::Image | DesktopAttachmentKind::Video
                    ),
                    3 => matches!(kind, DesktopAttachmentKind::Link),
                    _ => true,
                };
                if !matches || !format!("{title} {detail}").to_lowercase().contains(&query) {
                    continue;
                }
                count += 1;
                items = items.child(
                    div()
                        .pb(px(18.))
                        .border_b_1()
                        .border_color(p.border)
                        .flex()
                        .items_center()
                        .gap(px(16.))
                        .child(Self::avatar(i).size(34.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(MessageAttachment::new(kind, title, detail)),
                        )
                        .child(
                            DesktopButton::new(("history-favorite", i), "Save to Favorites")
                                .icon(DesktopIconKind::Favorites)
                                .icon_only()
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.saved = true;
                                    this.notice = Some("Saved to Favorites".into());
                                    cx.notify();
                                })),
                        ),
                );
            }
        }
        if self.history_tab == 2 && self.state != "empty" && query.is_empty() {
            let mut grid =
                DesktopMediaGrid::new().columns(if window.viewport_size().width < px(600.) {
                    2
                } else {
                    3
                });
            for (i, (label, image)) in [
                ("Studio reference", "pic_article.png"),
                ("Component study", "pic_160.png"),
                ("Prototype walkthrough", "pic_article.png"),
                ("Materials", "pic_160.png"),
            ]
            .into_iter()
            .enumerate()
            {
                grid = grid.child(
                    DesktopMediaTile::new(("media-tile", i), label, image)
                        .when(i == 2, |tile| tile.duration("0:24"))
                        .on_click(cx.listener(|this, _, w, cx| this.show(Popup::Media, w, cx))),
                );
            }
            items = div().p(px(18.)).child(grid);
        }
        if count == 0 {
            items = items.child(DesktopEmptyState::new(
                DesktopIconKind::Search,
                "No matching history",
                "Try another search or choose a different type.",
            ));
        }
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(ChatHeader::new("Design studio · Chat history"))
            .child(
                div()
                    .px(px(24.))
                    .py(px(16.))
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(DesktopField::new("history-search", self.search.clone()).search())
                    .child(filters),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .child(self.scroll("history-scroll", items, window, cx)),
            )
            .into_any_element()
    }
    fn overlays_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        div().size_full().p(px(40.)).child(section("Menus & dialogs",cx))
            .child(div().mt(px(12.)).flex().flex_wrap().gap(px(12.)).children([(Popup::Message,"Context menu"),(Popup::Confirm,"Confirmation"),(Popup::Emoji,"Emoji picker"),(Popup::More,"App menu")].into_iter().enumerate().map(|(i,(popup,label))|DesktopButton::new(("overlay-trigger",i),label).style(DesktopButtonStyle::Filled).on_click(cx.listener(move |this,_,w,cx|this.show(popup,w,cx))))))
            .child(div().mt(px(36.)).text_color(p.secondary).text_size(px(13.)).child("Use Tab and the arrow keys to navigate. Escape dismisses a layer and returns focus."))
            .child(div().mt(px(28.)).child(DesktopButton::new("show-toast","Show notice").on_click(cx.listener(|this,_,_,cx|{this.notice=Some("Saved to Favorites".into());cx.notify();})))).into_any_element()
    }
    fn favorites(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let p = DesktopPalette::get(cx);
        let body = div().p(px(32.)).child(section("Favorites", cx)).child(
            div()
                .mt(px(24.))
                .flex()
                .flex_col()
                .gap(px(24.))
                .child(MessageAttachment::new(
                    DesktopAttachmentKind::Link,
                    "A field guide to good interfaces",
                    "Saved link · Design",
                ))
                .when(self.saved, |el| {
                    el.child(MessageAttachment::new(
                        DesktopAttachmentKind::File,
                        "Desktop components.pdf",
                        "Saved file · 2.4 MB",
                    ))
                })
                .child(
                    DesktopButton::new("favorite-add", "Save sample file")
                        .icon(DesktopIconKind::Add)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.saved = true;
                            cx.notify();
                        })),
                ),
        );
        div()
            .size_full()
            .flex()
            .bg(p.canvas)
            .child(self.rail(cx))
            .child(div().flex_1().min_w_0().child(self.scroll(
                "favorites-scroll",
                body,
                window,
                cx,
            )))
            .into_any_element()
    }
    fn overlay_element(&self, cx: &mut Context<Self>) -> AnyElement {
        let weak = cx.entity().downgrade();
        let dismiss = move |w: &mut Window, cx: &mut App| {
            weak.update(cx, |this, cx| this.hide(w, cx)).ok();
        };
        if self.popup == Popup::Media {
            return wegpui::components::gallery::Gallery::new(
                "desktop-image-viewer",
                &self.overlay,
            )
            .src("pic_article.png")
            .label("Studio reference · WeUI sample image")
            .on_dismiss(dismiss)
            .into_any_element();
        }
        if self.popup == Popup::Confirm {
            return Modal::new("desktop-confirm",&self.overlay).label("Clear local messages").on_dismiss(dismiss)
                .child(DesktopDialog::new("Clear local messages?","This removes messages you sent in this gallery window. Your WeChat account and conversations are unaffected.")
                    .action(DesktopButton::new("cancel-clear","Cancel").style(DesktopButtonStyle::Filled).on_click(cx.listener(|this,_,w,cx|this.hide(w,cx))))
                    .action(DesktopButton::new("confirm-clear","Clear messages").style(DesktopButtonStyle::Danger).on_click(cx.listener(|this,_,w,cx|{this.sent.clear();this.notice=Some("Local messages cleared".into());this.hide(w,cx);}))))
                .into_any_element();
        }
        let width = if self.popup == Popup::Emoji {
            360.
        } else {
            240.
        };
        let mut popover = DesktopPopover::new("desktop-popup", &self.overlay, "Desktop menu")
            .position(self.popup_position)
            .width(width)
            .on_dismiss(dismiss);
        if self.popup == Popup::Emoji {
            popover = popover.child(DesktopEmojiPicker::new(EMOJI.iter().copied()).on_pick(
                cx.listener(|this, emoji: &SharedString, w, cx| {
                    let current = this.draft.read(cx).text(cx).to_string();
                    this.draft.update(cx, |area, cx| {
                        area.set_text(format!("{current}{emoji}"), cx)
                    });
                    this.hide(w, cx);
                }),
            ));
        } else {
            let mut menu = DesktopMenu::new().width(width);
            match self.popup {
                Popup::Navigation => {
                    for (i, (route, label)) in EXAMPLES.iter().enumerate() {
                        let route = *route;
                        menu = menu.child(
                            DesktopMenuItem::new(("example-menu", i), *label)
                                .checked(route == self.route)
                                .on_click(cx.listener(move |this, _, w, cx| {
                                    this.hide(w, cx);
                                    crate::go(route, w, cx);
                                })),
                        );
                    }
                }
                Popup::Appearance => {
                    for (i, (label, mode)) in [
                        ("Light Mode", ThemeMode::Light),
                        ("Dark Mode", ThemeMode::Dark),
                        ("Follow System", ThemeMode::System),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        menu = menu.child(
                            DesktopMenuItem::new(("appearance-option", i), label)
                                .checked(Theme::get(cx).mode == mode)
                                .on_click(cx.listener(move |this, _, w, cx| {
                                    Theme::set_mode(mode, cx);
                                    this.hide(w, cx);
                                })),
                        );
                    }
                }
                Popup::Language => {
                    for (i, label) in ["Automatic", "English", "简体中文"].into_iter().enumerate()
                    {
                        menu = menu.child(
                            DesktopMenuItem::new(("language-option", i), label)
                                .checked(self.language == i)
                                .on_click(cx.listener(move |this, _, w, cx| {
                                    this.language = i;
                                    this.hide(w, cx);
                                })),
                        );
                    }
                }
                Popup::More => {
                    for (i, (route, label, icon)) in [
                        ("desktop-history", "Chat files", DesktopIconKind::Folder),
                        ("desktop-history", "Chat history", DesktopIconKind::History),
                        ("desktop-contacts", "Contacts", DesktopIconKind::Contacts),
                        ("desktop-settings", "Settings", DesktopIconKind::Settings),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        menu = menu.child(
                            DesktopMenuItem::new(("more-item", i), label)
                                .icon(icon)
                                .on_click(cx.listener(move |this, _, w, cx| {
                                    this.hide(w, cx);
                                    crate::go(route, w, cx);
                                })),
                        );
                    }
                }
                _ => {
                    menu=menu.child(DesktopMenuItem::new("copy-message","Copy").icon(DesktopIconKind::File).shortcut("⌘ C").on_click(cx.listener(|this,_,w,cx|{cx.write_to_clipboard(ClipboardItem::new_string("The desktop components are ready for a look.".into()));this.hide(w,cx);})))
                        .child(DesktopMenuItem::new("quote-message","Quote").icon(DesktopIconKind::Reply).on_click(cx.listener(|this,_,w,cx|{this.draft.update(cx,|area,cx|area.set_text("Morgan: The desktop components are ready for a look.\n",cx));this.hide(w,cx);})))
                        .child(DesktopMenuItem::new("favorite-message","Save to Favorites").icon(DesktopIconKind::Favorites).on_click(cx.listener(|this,_,w,cx|{this.saved=true;this.notice=Some("Saved to Favorites".into());this.hide(w,cx);})))
                        .child(desktop_separator(cx)).child(DesktopMenuItem::new("forward-message","Forward").disabled(true))
                        .child(DesktopMenuItem::new("delete-message","Clear local messages").danger().on_click(cx.listener(|this,_,w,cx|{this.hide(w,cx);this.popup=Popup::Confirm;this.overlay.show(w,cx);cx.notify();})));
                }
            }
            popover = popover.child(menu);
        }
        popover.into_any_element()
    }
}

fn section(label: &'static str, cx: &App) -> Div {
    div()
        .mb(px(14.))
        .text_size(px(18.))
        .font_weight(FontWeight::MEDIUM)
        .text_color(DesktopPalette::get(cx).text)
        .child(label)
}

impl Render for MacGallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let content = match self.route {
            "desktop-controls" => self.controls(window, cx),
            "desktop-messages" => self.transcript(true, window, cx),
            "desktop-contacts" => self.contacts(window, cx),
            "desktop-settings" => self.settings(cx),
            "desktop-overlays" => self.overlays_page(cx),
            "desktop-history" => self.history(window, cx),
            "desktop-favorites" => self.favorites(window, cx),
            _ => self.chat(window, cx),
        };
        div()
            .id("mac-desktop-gallery")
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(p.canvas)
            .text_color(p.text)
            .text_size(px(14.))
            .line_height(relative(1.4))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, w, cx| {
                if event.keystroke.key == "enter"
                    && event.keystroke.modifiers.platform
                    && this.draft.read(cx).is_focused(w, cx)
                {
                    this.send(w, cx);
                    w.prevent_default();
                    cx.stop_propagation();
                }
            }))
            .child(self.toolbar(cx))
            .child(div().flex_1().min_h_0().child(content))
            .when_some(self.notice.clone(), |el, notice| {
                el.child(
                    div()
                        .absolute()
                        .bottom(px(18.))
                        .right(px(18.))
                        .max_w_full()
                        .flex()
                        .gap(px(4.))
                        .items_center()
                        .child(DesktopNotice::new(notice))
                        .child(
                            DesktopButton::new("dismiss-notice", "Dismiss notice")
                                .icon(DesktopIconKind::Close)
                                .icon_only()
                                .compact()
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.notice = None;
                                    cx.notify();
                                })),
                        ),
                )
            })
            .child(self.overlay_element(cx))
    }
}
