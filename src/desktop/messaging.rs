use super::{
    DesktopBadge, DesktopButton, DesktopButtonStyle, DesktopIcon, DesktopIconKind, DesktopPalette,
};
use crate::{components::ClickHandler, components::textarea::TextArea};
use gpui::{prelude::*, *};

#[derive(IntoElement, Clone)]
pub struct DesktopAvatar {
    label: SharedString,
    color: Hsla,
    size: f32,
    image: Option<SharedString>,
    members: Vec<(SharedString, Hsla)>,
}

impl DesktopAvatar {
    pub fn new(label: impl Into<SharedString>, color: impl Into<Hsla>) -> Self {
        Self {
            label: label.into(),
            color: color.into(),
            size: 38.,
            image: None,
            members: vec![],
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn image(mut self, path: impl Into<SharedString>) -> Self {
        self.image = Some(path.into());
        self
    }
    pub fn members(mut self, members: impl IntoIterator<Item = (SharedString, Hsla)>) -> Self {
        self.members = members.into_iter().take(9).collect();
        self
    }
}

impl RenderOnce for DesktopAvatar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let initials: String = self
            .label
            .split_whitespace()
            .take(2)
            .filter_map(|s| s.chars().next())
            .collect();
        let mut avatar = div()
            .id(self.label.clone())
            .role(Role::Image)
            .aria_label(self.label)
            .size(px(self.size))
            .flex_shrink_0()
            .rounded(px(5.))
            .overflow_hidden()
            .bg(self.color)
            .text_color(
                if crate::theme::contrast_ratio(black(), self.color) >= 4.5 {
                    black()
                } else {
                    white()
                },
            )
            .text_size(px(self.size * 0.32))
            .font_weight(FontWeight::MEDIUM)
            .flex()
            .items_center()
            .justify_center();
        if let Some(path) = self.image {
            avatar = avatar.child(img(path).size_full().object_fit(ObjectFit::Cover));
        } else if self.members.is_empty() {
            avatar = avatar.child(initials);
        } else {
            let columns = if self.members.len() > 4 { 3. } else { 2. };
            let cell = (self.size - 4. - (columns - 1.) * 2.) / columns;
            avatar = avatar.flex_wrap().gap(px(2.)).p(px(2.));
            for (name, color) in self.members {
                avatar = avatar.child(
                    div()
                        .size(px(cell))
                        .bg(color)
                        .text_color(if crate::theme::contrast_ratio(black(), color) >= 4.5 {
                            black()
                        } else {
                            white()
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(cell * 0.53))
                        .child(
                            name.chars()
                                .next()
                                .map(|c| c.to_string())
                                .unwrap_or_default(),
                        ),
                );
            }
        }
        avatar
    }
}

#[derive(IntoElement)]
pub struct ConversationRow {
    id: ElementId,
    title: SharedString,
    preview: SharedString,
    time: SharedString,
    avatar: DesktopAvatar,
    selected: bool,
    pinned: bool,
    muted: bool,
    unread: usize,
    on_click: Option<ClickHandler>,
}

impl ConversationRow {
    pub fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        avatar: DesktopAvatar,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            avatar,
            preview: "".into(),
            time: "".into(),
            selected: false,
            pinned: false,
            muted: false,
            unread: 0,
            on_click: None,
        }
    }
    pub fn preview(mut self, preview: impl Into<SharedString>) -> Self {
        self.preview = preview.into();
        self
    }
    pub fn time(mut self, time: impl Into<SharedString>) -> Self {
        self.time = time.into();
        self
    }
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }
    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }
    pub fn muted(mut self, value: bool) -> Self {
        self.muted = value;
        self
    }
    pub fn unread(mut self, count: usize) -> Self {
        self.unread = count;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ConversationRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let secondary = if self.selected {
            p.on_selected
                .opacity(if crate::Theme::get(cx).care { 1. } else { 0.7 })
        } else {
            p.secondary
        };
        let accessible = format!(
            "{}, {}, {}, {} unread{}{}",
            self.title,
            self.preview,
            self.time,
            self.unread,
            if self.pinned { ", pinned" } else { "" },
            if self.muted { ", muted" } else { "" }
        );
        let pinned = self.pinned && !self.selected;
        let mut button = DesktopButton::new(self.id, accessible)
            .style(DesktopButtonStyle::Selection)
            .selected(self.selected)
            .child(
                div()
                    .flex()
                    .w_full()
                    .min_w_0()
                    .items_center()
                    .gap(px(10.))
                    .py(px(10.))
                    .child(
                        div()
                            .relative()
                            .child(self.avatar)
                            .when(self.unread > 0, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .top(px(-5.))
                                        .right(px(-5.))
                                        .child(DesktopBadge::new(self.unread)),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(5.))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.))
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w_0()
                                            .truncate()
                                            .text_size(px(14.))
                                            .child(self.title),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.))
                                            .text_color(secondary)
                                            .child(self.time),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.))
                                    .text_color(secondary)
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w_0()
                                            .truncate()
                                            .text_size(px(12.))
                                            .child(self.preview),
                                    )
                                    .when(self.muted, |el| {
                                        el.child(
                                            DesktopIcon::new(DesktopIconKind::Mute)
                                                .size(13.)
                                                .color(secondary),
                                        )
                                    })
                                    .when(self.pinned, |el| {
                                        el.child(
                                            DesktopIcon::new(DesktopIconKind::Pin)
                                                .size(11.)
                                                .color(secondary),
                                        )
                                    }),
                            ),
                    ),
            );
        if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }
        div().when(pinned, |el| el.bg(p.pinned)).child(button)
    }
}

#[derive(IntoElement)]
pub struct DesktopRail {
    top: Vec<AnyElement>,
    bottom: Vec<AnyElement>,
}
impl DesktopRail {
    pub fn new() -> Self {
        Self {
            top: vec![],
            bottom: vec![],
        }
    }
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.bottom.push(footer.into_any_element());
        self
    }
}
impl Default for DesktopRail {
    fn default() -> Self {
        Self::new()
    }
}
impl ParentElement for DesktopRail {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.top.extend(elements);
    }
}
impl RenderOnce for DesktopRail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("navigation-rail")
            .role(Role::Navigation)
            .aria_label("Desktop navigation")
            .w(px(60.))
            .h_full()
            .flex_shrink_0()
            .bg(DesktopPalette::get(cx).rail)
            .flex()
            .flex_col()
            .items_center()
            .pt(px(18.))
            .pb(px(14.))
            .gap(px(18.))
            .children(self.top)
            .child(div().flex_1())
            .children(self.bottom)
    }
}

#[derive(IntoElement)]
pub struct ChatHeader {
    title: SharedString,
    detail: Option<SharedString>,
    actions: Vec<AnyElement>,
}
impl ChatHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            detail: None,
            actions: vec![],
        }
    }
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}
impl ParentElement for ChatHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements);
    }
}
impl RenderOnce for ChatHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .h(px(52.))
            .flex_shrink_0()
            .px(px(18.))
            .flex()
            .items_center()
            .gap(px(6.))
            .border_b_1()
            .border_color(p.border)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(15.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(self.title),
                    )
                    .when_some(self.detail, |el, detail| {
                        el.child(
                            div()
                                .text_size(px(11.))
                                .text_color(p.secondary)
                                .truncate()
                                .child(detail),
                        )
                    }),
            )
            .children(self.actions)
    }
}

#[derive(IntoElement)]
pub struct MessageBubble {
    id: ElementId,
    sender: Option<SharedString>,
    avatar: Option<DesktopAvatar>,
    outgoing: bool,
    body: Vec<AnyElement>,
    status: Option<SharedString>,
}
impl MessageBubble {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            sender: None,
            avatar: None,
            outgoing: false,
            body: vec![],
            status: None,
        }
    }
    pub fn outgoing(mut self) -> Self {
        self.outgoing = true;
        self
    }
    pub fn sender(mut self, sender: impl Into<SharedString>) -> Self {
        self.sender = Some(sender.into());
        self
    }
    pub fn avatar(mut self, avatar: DesktopAvatar) -> Self {
        self.avatar = Some(avatar);
        self
    }
    pub fn status(mut self, status: impl Into<SharedString>) -> Self {
        self.status = Some(status.into());
        self
    }
}
impl ParentElement for MessageBubble {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}
impl RenderOnce for MessageBubble {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let avatar = self.avatar;
        let text = if self.outgoing {
            p.outgoing_text
        } else {
            p.text
        };
        div()
            .id(self.id)
            .w_full()
            .flex()
            .gap(px(12.))
            .items_start()
            .when(self.outgoing, |el| el.flex_row_reverse())
            .children(avatar)
            .child(
                div()
                    .max_w(relative(0.78))
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(5.))
                    .when(self.outgoing, |el| el.items_end())
                    .when_some(self.sender, |el, name| {
                        el.child(div().text_size(px(11.)).text_color(p.secondary).child(name))
                    })
                    .child(
                        div()
                            .relative()
                            .px(px(12.))
                            .py(px(8.))
                            .rounded(px(5.))
                            .bg(if self.outgoing { p.outgoing } else { p.bubble })
                            .text_color(text)
                            .text_size(px(14.))
                            .line_height(relative(1.5))
                            .child(svg().data(if self.outgoing {
                                b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 8 12'><path d='M0 0 7 6 0 12Z'/></svg>"
                            } else {
                                b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 8 12'><path d='M8 0 1 6 8 12Z'/></svg>"
                            }).absolute().top(px(9.)).w(px(8.)).h(px(12.)).text_color(if self.outgoing { p.outgoing } else { p.bubble })
                                .when(self.outgoing, |tail| tail.right(px(-6.)))
                                .when(!self.outgoing, |tail| tail.left(px(-6.))))
                            .children(self.body),
                    )
                    .when_some(self.status, |el, status| {
                        el.child(
                            div()
                                .id("message-status")
                                .role(Role::Status)
                                .text_size(px(11.))
                                .text_color(p.secondary)
                                .child(status),
                        )
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct MessageQuote {
    author: SharedString,
    text: SharedString,
}
impl MessageQuote {
    pub fn new(author: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self {
            author: author.into(),
            text: text.into(),
        }
    }
}
impl RenderOnce for MessageQuote {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .border_l_1()
            .border_color(p.secondary)
            .pl(px(9.))
            .py(px(2.))
            .text_size(px(12.))
            .line_height(relative(1.45))
            .text_color(p.secondary)
            .child(format!("{}: {}", self.author, self.text))
    }
}

#[derive(IntoElement)]
pub struct MessageTranslation {
    text: SharedString,
}
impl MessageTranslation {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}
impl RenderOnce for MessageTranslation {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .mt(px(8.))
            .pt(px(8.))
            .border_t_1()
            .border_color(p.border)
            .child(self.text)
            .child(
                div()
                    .mt(px(5.))
                    .text_size(px(11.))
                    .text_color(p.secondary)
                    .child("Translated text"),
            )
    }
}

#[derive(Clone, Copy)]
pub enum DesktopAttachmentKind {
    File,
    Link,
    Image,
    Video,
    Voice,
}

#[derive(IntoElement)]
pub struct MessageAttachment {
    title: SharedString,
    detail: SharedString,
    kind: DesktopAttachmentKind,
    image: Option<SharedString>,
}
impl MessageAttachment {
    pub fn new(
        kind: DesktopAttachmentKind,
        title: impl Into<SharedString>,
        detail: impl Into<SharedString>,
    ) -> Self {
        Self {
            kind,
            title: title.into(),
            detail: detail.into(),
            image: None,
        }
    }
    pub fn image(mut self, image: impl Into<SharedString>) -> Self {
        self.image = Some(image.into());
        self
    }
}
impl RenderOnce for MessageAttachment {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let icon = match self.kind {
            DesktopAttachmentKind::File => DesktopIconKind::File,
            DesktopAttachmentKind::Link => DesktopIconKind::Link,
            DesktopAttachmentKind::Image => DesktopIconKind::Photo,
            DesktopAttachmentKind::Video => DesktopIconKind::Video,
            DesktopAttachmentKind::Voice => DesktopIconKind::Microphone,
        };
        div()
            .w(px(236.))
            .max_w_full()
            .flex()
            .gap(px(12.))
            .items_center()
            .child(if let Some(image) = self.image {
                img(image)
                    .size(px(48.))
                    .rounded(px(4.))
                    .object_fit(ObjectFit::Cover)
                    .into_any_element()
            } else {
                div()
                    .size(px(42.))
                    .rounded(px(5.))
                    .bg(p.canvas)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(DesktopIcon::new(icon).size(24.).color(p.link))
                    .into_any_element()
            })
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(div().text_size(px(13.)).child(self.title))
                    .child(
                        div()
                            .mt(px(5.))
                            .text_size(px(11.))
                            .text_color(p.secondary)
                            .child(self.detail),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct DesktopComposer {
    input: Entity<TextArea>,
    toolbar: Vec<AnyElement>,
    on_send: Option<ClickHandler>,
    disabled: bool,
    hint: SharedString,
}
impl DesktopComposer {
    pub fn new(input: Entity<TextArea>) -> Self {
        Self {
            input,
            toolbar: vec![],
            on_send: None,
            disabled: false,
            hint: "⌘ Enter to send".into(),
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = hint.into();
        self
    }
    pub fn on_send(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_send = Some(Box::new(handler));
        self
    }
}
impl ParentElement for DesktopComposer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.toolbar.extend(elements);
    }
}
impl RenderOnce for DesktopComposer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let disabled = self.disabled || self.input.read(cx).text(cx).trim().is_empty();
        self.input.update(cx, |input, cx| {
            input.set_placeholder_color(Some(p.secondary), cx)
        });
        let mut send = DesktopButton::new("send", "Send")
            .style(DesktopButtonStyle::Primary)
            .compact()
            .disabled(disabled);
        if let Some(handler) = self.on_send {
            send = send.on_click(handler);
        }
        div()
            .h(px(170.))
            .min_h(px(130.))
            .flex_shrink_0()
            .p(px(8.))
            .child(
                div()
                    .id("composer-frame")
                    .size_full()
                    .flex()
                    .flex_col()
                    .rounded(px(8.))
                    .border_1()
                    .border_color(p.border)
                    .when(self.input.read(cx).is_focused(window, cx), |s| {
                        s.border_color(p.accent)
                    })
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .overflow_hidden()
                            .p(px(12.))
                            .text_size(px(14.))
                            .text_color(p.text)
                            .child(self.input),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.))
                            .px(px(8.))
                            .pb(px(8.))
                            .children(self.toolbar)
                            .child(div().flex_1())
                            .child(
                                div()
                                    .text_size(px(10.))
                                    .text_color(p.secondary)
                                    .child(self.hint),
                            )
                            .child(send),
                    ),
            )
    }
}
