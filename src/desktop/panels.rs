use super::{
    DesktopAvatar, DesktopButton, DesktopButtonStyle, DesktopIcon, DesktopIconKind, DesktopPalette,
};

type DismissHandler = Box<dyn Fn(&mut Window, &mut App)>;
use crate::{
    components::{ClickHandler, MaskKind, Modal, Motion, Placement, Presence},
    scrolling::{FocusScroll, RevealFocus},
};
use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct DesktopSettingRow {
    label: SharedString,
    detail: Option<SharedString>,
    controls: Vec<AnyElement>,
}
impl DesktopSettingRow {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            detail: None,
            controls: vec![],
        }
    }
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}
impl ParentElement for DesktopSettingRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.controls.extend(elements);
    }
}
impl RenderOnce for DesktopSettingRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .id(self.label.clone())
            .role(Role::Group)
            .aria_label(self.label.clone())
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(20.))
            .py(px(14.))
            .border_b_1()
            .border_color(p.border)
            .child(
                div()
                    .flex_1()
                    .min_w(px(96.))
                    .min_w_0()
                    .text_size(px(14.))
                    .child(self.label)
                    .when_some(self.detail, |el, detail| {
                        el.child(
                            div()
                                .mt(px(5.))
                                .text_size(px(12.))
                                .text_color(p.secondary)
                                .child(detail),
                        )
                    }),
            )
            .child(
                div()
                    .min_w_0()
                    .max_w_full()
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .children(self.controls),
            )
    }
}

#[derive(IntoElement)]
pub struct DesktopSettings {
    sidebar: Vec<AnyElement>,
    title: SharedString,
    body: Vec<AnyElement>,
}
impl DesktopSettings {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            sidebar: vec![],
            title: title.into(),
            body: vec![],
        }
    }
    pub fn navigation(mut self, item: impl IntoElement) -> Self {
        self.sidebar.push(item.into_any_element());
        self
    }
}
impl ParentElement for DesktopSettings {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}
impl RenderOnce for DesktopSettings {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let compact = window.viewport_size().width < px(600.);
        let scroll = window
            .use_keyed_state("desktop-settings-scroll", cx, |_, _| ScrollHandle::new())
            .read(cx)
            .clone();
        let body = div()
            .id("settings-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .px(px(if compact { 18. } else { 40. }))
            .pb(px(30.))
            .children(self.body);
        div()
            .size_full()
            .flex()
            .bg(p.canvas)
            .text_color(p.text)
            .text_size(px(14.))
            .child(
                div()
                    .w(px(if compact { 132. } else { 160. }))
                    .h_full()
                    .flex_shrink_0()
                    .bg(p.panel)
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .p(px(10.))
                    .pt(px(24.))
                    .children(self.sidebar),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(68.))
                            .px(px(if compact { 18. } else { 40. }))
                            .flex()
                            .items_center()
                            .text_size(px(18.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(self.title),
                    )
                    .child(FocusScroll::new("settings-focus", &scroll, body)),
            )
    }
}

#[derive(IntoElement)]
pub struct ContactRow {
    id: ElementId,
    name: SharedString,
    avatar: DesktopAvatar,
    detail: Option<SharedString>,
    selected: bool,
    on_click: Option<ClickHandler>,
}
impl ContactRow {
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        avatar: DesktopAvatar,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            avatar,
            detail: None,
            selected: false,
            on_click: None,
        }
    }
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
impl RenderOnce for ContactRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut button = DesktopButton::new(self.id, self.name.clone())
            .selected(self.selected)
            .style(DesktopButtonStyle::Selection)
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .py(px(9.))
                    .child(self.avatar)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(div().text_size(px(14.)).truncate().child(self.name))
                            .when_some(self.detail, |el, detail| {
                                el.child(
                                    div()
                                        .mt(px(4.))
                                        .text_size(px(12.))
                                        .text_color(if self.selected {
                                            DesktopPalette::get(cx).on_selected
                                        } else {
                                            DesktopPalette::get(cx).secondary
                                        })
                                        .truncate()
                                        .child(detail),
                                )
                            }),
                    ),
            );
        if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }
        button
    }
}

#[derive(IntoElement)]
pub struct ContactProfile {
    name: SharedString,
    subtitle: SharedString,
    avatar: DesktopAvatar,
    fields: Vec<AnyElement>,
    actions: Vec<AnyElement>,
}
impl ContactProfile {
    pub fn new(name: impl Into<SharedString>, avatar: DesktopAvatar) -> Self {
        Self {
            name: name.into(),
            avatar,
            subtitle: "".into(),
            fields: vec![],
            actions: vec![],
        }
    }
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = subtitle.into();
        self
    }
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}
impl ParentElement for ContactProfile {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.fields.extend(elements);
    }
}
impl RenderOnce for ContactProfile {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .w(px(440.))
            .max_w_full()
            .p(px(32.))
            .flex()
            .flex_col()
            .gap(px(24.))
            .child(
                div()
                    .flex()
                    .gap(px(20.))
                    .items_center()
                    .child(self.avatar.size(70.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(div().text_size(px(23.)).child(self.name))
                            .child(
                                div()
                                    .mt(px(7.))
                                    .text_size(px(13.))
                                    .text_color(p.secondary)
                                    .child(self.subtitle),
                            ),
                    ),
            )
            .child(
                div()
                    .border_t_1()
                    .border_color(p.border)
                    .pt(px(12.))
                    .children(self.fields),
            )
            .child(
                div()
                    .flex()
                    .justify_center()
                    .gap(px(18.))
                    .children(self.actions),
            )
    }
}

#[derive(IntoElement)]
pub struct DesktopMenu {
    items: Vec<AnyElement>,
    width: f32,
}
impl DesktopMenu {
    pub fn new() -> Self {
        Self {
            items: vec![],
            width: 230.,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
impl Default for DesktopMenu {
    fn default() -> Self {
        Self::new()
    }
}
impl ParentElement for DesktopMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}
impl RenderOnce for DesktopMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let state = window
            .use_keyed_state("desktop-menu-state", cx, |_, cx| {
                (ScrollHandle::new(), cx.focus_handle())
            })
            .read(cx)
            .clone();
        let (scroll, scope) = state;
        let body = div()
            .id("menu-items")
            .track_focus(&scope)
            .role(Role::Menu)
            .aria_label("Actions")
            .w(px(self.width))
            .max_h(px(440.))
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .p(px(5.))
            .rounded(px(7.))
            .bg(p.panel)
            .shadow(vec![BoxShadow {
                color: black().opacity(0.2),
                offset: point(px(0.), px(6.)),
                blur_radius: px(24.),
                spread_radius: px(0.),
                inset: false,
            }])
            .on_key_down(move |event, window, cx| {
                let backward = match event.keystroke.key.as_str() {
                    "up" => true,
                    "down" => false,
                    _ => return,
                };
                for _ in 0..256 {
                    if backward {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    if scope.contains_focused(window, cx) {
                        break;
                    }
                }
                window.prevent_default();
                cx.stop_propagation();
            })
            .children(self.items);
        FocusScroll::new("menu-focus", &scroll, body)
    }
}

#[derive(IntoElement)]
pub struct DesktopMenuItem {
    id: ElementId,
    label: SharedString,
    icon: Option<DesktopIconKind>,
    shortcut: Option<SharedString>,
    checked: bool,
    disabled: bool,
    danger: bool,
    on_click: Option<ClickHandler>,
}
impl DesktopMenuItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            checked: false,
            disabled: false,
            danger: false,
            on_click: None,
        }
    }
    pub fn icon(mut self, icon: DesktopIconKind) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn danger(mut self) -> Self {
        self.danger = true;
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
impl RenderOnce for DesktopMenuItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let label = self.label.clone();
        let mut button = DesktopButton::new(self.id, label)
            .role(Role::MenuItem)
            .selected(self.checked)
            .disabled(self.disabled)
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .text_color(if self.danger { p.danger } else { p.text })
                    .child(div().w(px(20.)).children(self.icon.map(DesktopIcon::new)))
                    .child(div().flex_1().text_size(px(13.)).child(self.label))
                    .when(self.checked, |el| {
                        el.child(DesktopIcon::new(DesktopIconKind::Check).size(15.))
                    })
                    .when_some(self.shortcut, |el, shortcut| {
                        el.child(
                            div()
                                .text_color(p.secondary)
                                .text_size(px(11.))
                                .child(shortcut),
                        )
                    }),
            );
        if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }
        button
    }
}

pub fn desktop_separator(cx: &App) -> Div {
    div()
        .h(px(1.))
        .my(px(5.))
        .mx(px(8.))
        .bg(DesktopPalette::get(cx).border)
}

#[derive(IntoElement)]
pub struct DesktopDialog {
    title: SharedString,
    description: SharedString,
    body: Vec<AnyElement>,
    actions: Vec<AnyElement>,
}
impl DesktopDialog {
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            body: vec![],
            actions: vec![],
        }
    }
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}
impl ParentElement for DesktopDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}
impl RenderOnce for DesktopDialog {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .w(px(380.))
            .max_w_full()
            .rounded(px(10.))
            .bg(p.canvas)
            .p(px(28.))
            .text_color(p.text)
            .child(
                div()
                    .text_size(px(17.))
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.title),
            )
            .child(
                div()
                    .mt(px(12.))
                    .text_size(px(13.))
                    .line_height(relative(1.6))
                    .child(self.description),
            )
            .child(div().mt(px(16.)).children(self.body))
            .child(
                div()
                    .mt(px(24.))
                    .flex()
                    .justify_end()
                    .gap(px(10.))
                    .children(self.actions),
            )
    }
}

#[derive(IntoElement)]
pub struct DesktopPopover {
    id: ElementId,
    presence: Presence,
    position: Point<Pixels>,
    label: SharedString,
    width: f32,
    body: Vec<AnyElement>,
    dismiss: Option<DismissHandler>,
}
impl DesktopPopover {
    pub fn new(
        id: impl Into<ElementId>,
        presence: &Presence,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            position: point(px(12.), px(52.)),
            label: label.into(),
            width: 240.,
            body: vec![],
            dismiss: None,
        }
    }
    pub fn position(mut self, position: Point<Pixels>) -> Self {
        self.position = position;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.dismiss = Some(Box::new(handler));
        self
    }
}
impl ParentElement for DesktopPopover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}
impl RenderOnce for DesktopPopover {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let viewport = window.viewport_size();
        let x = self
            .position
            .x
            .max(px(8.))
            .min((viewport.width - px(self.width + 8.)).max(px(8.)));
        let y = self
            .position
            .y
            .max(px(8.))
            .min((viewport.height - px(460.)).max(px(8.)));
        let mut modal = Modal::new(self.id, &self.presence)
            .label(self.label)
            .mask(MaskKind::Transparent)
            .placement(Placement::Top)
            .motion(Motion::None)
            .padding(Edges {
                top: y,
                left: x,
                right: (viewport.width - x - px(self.width)).max(px(8.)),
                bottom: px(8.),
            })
            .child(div().w(px(self.width)).max_w_full().children(self.body));
        if let Some(handler) = self.dismiss {
            modal = modal.on_dismiss(handler);
        }
        modal
    }
}

#[derive(IntoElement)]
pub struct DesktopNotice {
    text: SharedString,
    error: bool,
}
impl DesktopNotice {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            error: false,
        }
    }
    pub fn error(mut self) -> Self {
        self.error = true;
        self
    }
}
impl RenderOnce for DesktopNotice {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .id("desktop-notice")
            .role(if self.error {
                Role::Alert
            } else {
                Role::Status
            })
            .px(px(16.))
            .py(px(10.))
            .rounded(px(6.))
            .bg(p.panel)
            .text_color(if self.error { p.danger } else { p.text })
            .text_size(px(13.))
            .flex()
            .gap(px(9.))
            .items_center()
            .child(
                DesktopIcon::new(if self.error {
                    DesktopIconKind::Info
                } else {
                    DesktopIconKind::Check
                })
                .size(17.),
            )
            .child(self.text)
    }
}

#[derive(IntoElement)]
pub struct DesktopEmptyState {
    title: SharedString,
    detail: SharedString,
    icon: DesktopIconKind,
    actions: Vec<AnyElement>,
}
impl DesktopEmptyState {
    pub fn new(
        icon: DesktopIconKind,
        title: impl Into<SharedString>,
        detail: impl Into<SharedString>,
    ) -> Self {
        Self {
            title: title.into(),
            detail: detail.into(),
            icon,
            actions: vec![],
        }
    }
}
impl ParentElement for DesktopEmptyState {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements);
    }
}
impl RenderOnce for DesktopEmptyState {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(14.))
            .p(px(32.))
            .text_center()
            .text_color(p.secondary)
            .child(DesktopIcon::new(self.icon).size(42.).color(p.secondary))
            .child(
                div()
                    .text_size(px(16.))
                    .text_color(p.text)
                    .child(self.title),
            )
            .child(
                div()
                    .max_w(px(320.))
                    .text_size(px(13.))
                    .line_height(relative(1.5))
                    .child(self.detail),
            )
            .children(self.actions)
    }
}

#[derive(IntoElement)]
pub struct DesktopMemberGrid {
    members: Vec<(SharedString, DesktopAvatar)>,
    columns: usize,
}
impl DesktopMemberGrid {
    pub fn new() -> Self {
        Self {
            members: vec![],
            columns: 4,
        }
    }
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(1, 8);
        self
    }
    pub fn member(mut self, name: impl Into<SharedString>, avatar: DesktopAvatar) -> Self {
        self.members.push((name.into(), avatar));
        self
    }
}
impl Default for DesktopMemberGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl RenderOnce for DesktopMemberGrid {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .flex()
            .flex_wrap()
            .gap_y(px(16.))
            .children(self.members.into_iter().map(|(name, avatar)| {
                div()
                    .w(relative(1. / self.columns as f32))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(6.))
                    .child(avatar)
                    .child(
                        div()
                            .max_w(px(52.))
                            .truncate()
                            .text_size(px(11.))
                            .text_color(p.secondary)
                            .child(name),
                    )
            }))
    }
}

#[derive(IntoElement)]
pub struct DesktopEmojiPicker {
    emoji: Vec<SharedString>,
    on_pick: Option<crate::components::ChangeHandler<SharedString>>,
}
impl DesktopEmojiPicker {
    pub fn new(emoji: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        Self {
            emoji: emoji.into_iter().map(Into::into).collect(),
            on_pick: None,
        }
    }
    pub fn on_pick(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_pick = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for DesktopEmojiPicker {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let handler = std::rc::Rc::new(self.on_pick);
        div()
            .w(px(360.))
            .max_w_full()
            .rounded(px(8.))
            .bg(p.panel)
            .p(px(16.))
            .text_color(p.text)
            .child(
                div()
                    .mb(px(12.))
                    .text_size(px(12.))
                    .text_color(p.secondary)
                    .child("Emoji"),
            )
            .child(div().flex().flex_wrap().gap(px(3.)).children(
                self.emoji.into_iter().enumerate().map(|(i, emoji)| {
                    let pick = handler.clone();
                    let value = emoji.clone();
                    DesktopButton::new(("emoji", i), emoji.clone())
                        .icon_only()
                        .child(div().text_size(px(25.)).child(emoji))
                        .on_click(move |_, w, cx| {
                            if let Some(handler) = pick.as_ref() {
                                handler(&value, w, cx);
                            }
                        })
                }),
            ))
    }
}

struct DesktopSplitDrag;
impl Render for DesktopSplitDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct DesktopSplit {
    id: ElementId,
    width: f32,
    min: f32,
    max: f32,
    sidebar: AnyElement,
    main: AnyElement,
    on_resize: Option<crate::components::ChangeHandler<f32>>,
}
impl DesktopSplit {
    pub fn new(
        id: impl Into<ElementId>,
        width: f32,
        sidebar: impl IntoElement,
        main: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            width,
            min: 220.,
            max: 420.,
            sidebar: sidebar.into_any_element(),
            main: main.into_any_element(),
            on_resize: None,
        }
    }
    pub fn limits(mut self, min: f32, max: f32) -> Self {
        self.min = min.max(0.);
        self.max = max.max(self.min);
        self
    }
    pub fn on_resize(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_resize = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for DesktopSplit {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let width = self.width.clamp(self.min, self.max);
        let emit = std::rc::Rc::new(self.on_resize);
        let key_emit = emit.clone();
        let drag_emit = emit.clone();
        let divider = div()
            .id(self.id)
            .role(Role::Splitter)
            .aria_label("Resize sidebar")
            .w(px(5.))
            .h_full()
            .flex_shrink_0()
            .bg(p.border)
            .cursor_col_resize()
            .focusable()
            .tab_stop(true)
            .focus_visible(|s| s.shadow(crate::components::form::dual_focus_ring()))
            .on_key_down(move |event, window, cx| {
                let next = match event.keystroke.key.as_str() {
                    "left" => width - 10.,
                    "right" => width + 10.,
                    "home" => self.min,
                    "end" => self.max,
                    _ => return,
                };
                if let Some(emit) = key_emit.as_ref() {
                    emit(&next.clamp(self.min, self.max), window, cx);
                }
                cx.stop_propagation();
                window.prevent_default();
            })
            .on_drag(DesktopSplitDrag, |_, _, _, cx| cx.new(|_| DesktopSplitDrag))
            .on_drag_move::<DesktopSplitDrag>(move |event, window, cx| {
                if let Some(emit) = drag_emit.as_ref() {
                    let next = width + (event.event.position.x - event.bounds.origin.x).as_f32();
                    emit(&next.clamp(self.min, self.max), window, cx);
                }
            })
            .reveal_focus();
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .w(px(width))
                    .h_full()
                    .flex_shrink_0()
                    .child(self.sidebar),
            )
            .child(divider)
            .child(div().flex_1().min_w_0().h_full().child(self.main))
    }
}

#[derive(IntoElement)]
pub struct DesktopMediaGrid {
    columns: usize,
    items: Vec<AnyElement>,
}
impl DesktopMediaGrid {
    pub fn new() -> Self {
        Self {
            columns: 3,
            items: vec![],
        }
    }
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(1, 8);
        self
    }
}
impl Default for DesktopMediaGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl ParentElement for DesktopMediaGrid {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}
impl RenderOnce for DesktopMediaGrid {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .children(self.items.into_iter().map(|item| {
                div()
                    .w(relative(1. / self.columns as f32))
                    .p(px(6.))
                    .child(item)
            }))
    }
}

#[derive(IntoElement)]
pub struct DesktopMediaTile {
    id: ElementId,
    label: SharedString,
    image: SharedString,
    duration: Option<SharedString>,
    on_click: Option<ClickHandler>,
}
impl DesktopMediaTile {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        image: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            image: image.into(),
            duration: None,
            on_click: None,
        }
    }
    pub fn duration(mut self, duration: impl Into<SharedString>) -> Self {
        self.duration = Some(duration.into());
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
impl RenderOnce for DesktopMediaTile {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let mut button = DesktopButton::new(self.id, self.label.clone()).child(
            div()
                .w_full()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    div()
                        .w_full()
                        .h(px(144.))
                        .relative()
                        .rounded(px(4.))
                        .overflow_hidden()
                        .bg(p.field)
                        .child(img(self.image).size_full().object_fit(ObjectFit::Cover))
                        .when_some(self.duration, |el, duration| {
                            el.child(
                                div()
                                    .absolute()
                                    .right(px(6.))
                                    .bottom(px(6.))
                                    .px(px(6.))
                                    .py(px(2.))
                                    .rounded(px(3.))
                                    .bg(black().opacity(0.65))
                                    .text_color(white())
                                    .text_size(px(11.))
                                    .child(duration),
                            )
                        }),
                )
                .child(div().truncate().text_size(px(12.)).child(self.label)),
        );
        if let Some(handler) = self.on_click {
            button = button.on_click(handler);
        }
        button
    }
}
