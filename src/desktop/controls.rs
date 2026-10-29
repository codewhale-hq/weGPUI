use super::{DesktopIcon, DesktopIconKind};
use crate::{
    Theme,
    accessibility::Semantics,
    components::{ChangeHandler, ClickHandler},
    input::TextInput,
    scrolling::RevealFocus,
};
use gpui::{prelude::*, *};

#[derive(Clone, Copy)]
pub struct DesktopPalette {
    pub canvas: Hsla,
    pub list: Hsla,
    pub rail: Hsla,
    pub panel: Hsla,
    pub bubble: Hsla,
    pub outgoing: Hsla,
    pub outgoing_text: Hsla,
    pub selected: Hsla,
    pub on_selected: Hsla,
    pub pinned: Hsla,
    pub text: Hsla,
    pub secondary: Hsla,
    pub placeholder: Hsla,
    pub border: Hsla,
    pub hover: Hsla,
    pub field: Hsla,
    pub accent: Hsla,
    pub on_accent: Hsla,
    pub danger: Hsla,
    pub link: Hsla,
}

impl DesktopPalette {
    pub fn get(cx: &App) -> Self {
        let theme = Theme::get(cx);
        let on_accent = if crate::theme::contrast_ratio(black(), theme.brand) >= 4.5 {
            black()
        } else {
            white()
        };
        let color = |light, dark| rgb(if theme.is_dark { dark } else { light }).into();
        let outgoing = theme
            .brand_override
            .map(|brand| theme.surface_color(brand.opacity(0.22), theme.bg_1))
            .unwrap_or(color(0x95ec69, 0x3eb575));
        Self {
            canvas: color(0xfafafa, 0x1e1e1f),
            list: color(0xeeeef0, 0x2f2f30),
            rail: color(0xe3e4e6, 0x2a2c2d),
            panel: color(0xfafafa, 0x29292a),
            bubble: color(0xeeeef0, 0x2f2f30),
            outgoing,
            outgoing_text: if crate::theme::contrast_ratio(black(), outgoing) >= 4.5 {
                black()
            } else {
                white()
            },
            selected: theme.brand_override.unwrap_or(color(0x1cac72, 0x0da869)),
            on_selected: if theme.care { black() } else { white() },
            pinned: color(0xe6e6e8, 0x3b3b3d),
            text: color(0x1a1a1c, 0xe0e0e4),
            secondary: color(0x626266, 0xaaaab0),
            placeholder: color(0x626266, 0xb7b7bd),
            border: color(0xededed, 0x29292a),
            hover: theme.bg_color_active,
            field: color(0xe6e6e8, 0x454547),
            accent: theme.brand,
            on_accent,
            danger: theme.red_80,
            link: theme.link,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum DesktopButtonStyle {
    #[default]
    Quiet,
    Filled,
    Primary,
    Danger,
    Selection,
}

#[derive(IntoElement)]
pub struct DesktopButton {
    id: ElementId,
    label: SharedString,
    icon: Option<DesktopIconKind>,
    style: DesktopButtonStyle,
    disabled: bool,
    selected: Option<bool>,
    expanded: Option<bool>,
    loading: bool,
    compact: bool,
    icon_only: bool,
    role: Role,
    body: Vec<AnyElement>,
    on_click: Option<ClickHandler>,
}

impl DesktopButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            style: DesktopButtonStyle::Quiet,
            disabled: false,
            selected: None,
            expanded: None,
            loading: false,
            compact: false,
            icon_only: false,
            role: Role::Button,
            body: vec![],
            on_click: None,
        }
    }
    pub fn icon(mut self, icon: DesktopIconKind) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }
    pub fn style(mut self, style: DesktopButtonStyle) -> Self {
        self.style = style;
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
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

impl ParentElement for DesktopButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for DesktopButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let selected = self.selected == Some(true);
        let (bg, fg) = match self.style {
            DesktopButtonStyle::Primary => (p.accent, p.on_accent),
            DesktopButtonStyle::Danger => (p.field, p.danger),
            DesktopButtonStyle::Selection if selected => (p.selected, p.on_selected),
            DesktopButtonStyle::Filled => (p.field, p.text),
            _ => (
                transparent_black(),
                if selected { p.accent } else { p.text },
            ),
        };
        let mut el = div()
            .id(self.id)
            .role(self.role)
            .aria_label(self.label.clone())
            .flex()
            .items_center()
            .justify_center()
            .gap(px(7.))
            .min_h(px(if self.compact { 28. } else { 34. }))
            .px(px(if self.icon_only { 6. } else { 12. }))
            .rounded(px(5.))
            .bg(bg)
            .text_color(fg)
            .text_size(px(13.))
            .line_height(relative(1.3))
            .when(self.icon_only, |el| {
                el.w(px(if self.compact { 28. } else { 34. }))
            })
            .when_some(self.selected, |el, selected| el.aria_selected(selected))
            .when_some(self.expanded, |el, expanded| el.aria_expanded(expanded));
        if self.body.is_empty() {
            el = el
                .when(self.loading, |el| {
                    el.child(
                        crate::components::loading::Loading::new()
                            .size(14.)
                            .color(fg),
                    )
                })
                .when_some(self.icon, |el, icon| {
                    el.child(DesktopIcon::new(icon).size(20.).color(fg))
                })
                .when(!self.icon_only, |el| el.child(self.label));
        } else {
            el = el.children(self.body);
        }
        if self.disabled || self.loading {
            el = el.opacity(0.4);
        } else {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(|s| s.shadow(crate::components::form::dual_focus_ring()))
                .hover(move |s| {
                    if selected || bg == p.accent {
                        s.opacity(0.88)
                    } else {
                        s.bg(p.hover)
                    }
                })
                .active(|s| s.opacity(0.65));
            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        }
        Semantics::new(el)
            .disabled(self.disabled || self.loading)
            .busy(self.loading)
            .reveal_focus()
    }
}

#[derive(IntoElement)]
pub struct DesktopSwitch {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}

impl DesktopSwitch {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for DesktopSwitch {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let mut el = div()
            .id(self.id)
            .role(Role::Switch)
            .aria_label(self.label)
            .aria_toggled(if self.checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .w(px(36.))
            .h(px(21.))
            .flex_shrink_0()
            .rounded(px(12.))
            .relative()
            .bg(if self.checked { p.accent } else { p.secondary })
            .child(
                div()
                    .absolute()
                    .top(px(2.))
                    .left(px(if self.checked { 17. } else { 2. }))
                    .size(px(17.))
                    .rounded_full()
                    .bg(white()),
            )
            .when(self.disabled, |el| el.opacity(0.4));
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(|s| s.shadow(crate::components::form::dual_outer_focus_ring()));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| {
                    cx.stop_propagation();
                    handler(&!self.checked, w, cx);
                });
            }
        }
        Semantics::new(el).disabled(self.disabled).reveal_focus()
    }
}

#[derive(IntoElement)]
pub struct DesktopField {
    id: ElementId,
    input: Entity<TextInput>,
    search: bool,
    error: Option<SharedString>,
    clear: bool,
}

impl DesktopField {
    pub fn new(id: impl Into<ElementId>, input: Entity<TextInput>) -> Self {
        Self {
            id: id.into(),
            input,
            search: false,
            error: None,
            clear: true,
        }
    }
    pub fn search(mut self) -> Self {
        self.search = true;
        self
    }
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn clearable(mut self, clear: bool) -> Self {
        self.clear = clear;
        self
    }
}

impl RenderOnce for DesktopField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let focus = self.input.read(cx).focus_handle(cx);
        self.input.update(cx, |input, cx| {
            input.set_placeholder_color(Some(p.placeholder), cx);
            input.set_invalid(self.error.is_some(), cx);
        });
        let has_content = !self.input.read(cx).is_empty();
        let editable = !self.input.read(cx).is_disabled() && !self.input.read(cx).is_readonly();
        let clear = self.input.clone();
        div()
            .id(self.id)
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(5.))
            .child(
                div()
                    .id("field-frame")
                    .flex()
                    .items_center()
                    .gap(px(7.))
                    .h(px(30.))
                    .px(px(9.))
                    .rounded(px(5.))
                    .bg(p.field)
                    .border_1()
                    .border_color(if self.error.is_some() {
                        p.danger
                    } else {
                        transparent_black()
                    })
                    .when(self.input.read(cx).is_focused(window), |s| {
                        s.border_color(p.accent)
                    })
                    .when(self.search, |el| {
                        el.child(
                            DesktopIcon::new(DesktopIconKind::Search)
                                .size(15.)
                                .color(p.secondary),
                        )
                    })
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(13.))
                            .line_height(relative(1.2))
                            .text_color(p.text)
                            .child(self.input),
                    )
                    .when(self.clear && has_content && editable, |el| {
                        el.child(
                            DesktopButton::new("clear", "Clear text")
                                .icon(DesktopIconKind::Close)
                                .icon_only()
                                .compact()
                                .on_click(move |_, window, cx| {
                                    clear.update(cx, |input, cx| input.clear(cx));
                                    window.focus(&focus, cx);
                                }),
                        )
                    }),
            )
            .when_some(self.error, |el, error| {
                el.child(
                    div()
                        .id("field-error")
                        .role(Role::Alert)
                        .text_size(px(12.))
                        .text_color(p.danger)
                        .child(error),
                )
            })
    }
}

#[derive(IntoElement)]
pub struct DesktopBadge {
    id: ElementId,
    count: usize,
}
impl DesktopBadge {
    #[track_caller]
    pub fn new(count: usize) -> Self {
        Self {
            id: ElementId::from(std::panic::Location::caller()),
            count,
        }
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
}
impl RenderOnce for DesktopBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = if self.count > 99 {
            "99+".to_string()
        } else {
            self.count.to_string()
        };
        div()
            .id(self.id)
            .role(Role::Status)
            .aria_label(format!("{} unread", self.count))
            .min_w(px(17.))
            .h(px(17.))
            .px(px(4.))
            .rounded_full()
            .bg(Theme::get(cx).red)
            .text_color(
                if crate::theme::contrast_ratio(black(), Theme::get(cx).red) >= 4.5 {
                    black()
                } else {
                    white()
                },
            )
            .text_size(px(11.))
            .line_height(px(17.))
            .text_center()
            .child(label)
    }
}

#[derive(IntoElement)]
pub struct DesktopKeycap {
    text: SharedString,
}
impl DesktopKeycap {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}
impl RenderOnce for DesktopKeycap {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        div()
            .px(px(8.))
            .py(px(3.))
            .min_w(px(28.))
            .rounded(px(4.))
            .border_1()
            .border_color(p.border)
            .bg(p.canvas)
            .text_size(px(12.))
            .text_center()
            .child(self.text)
    }
}

#[derive(IntoElement)]
pub struct DesktopCheckbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<bool>>,
}
impl DesktopCheckbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for DesktopCheckbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let mut el = div()
            .id(self.id)
            .role(Role::CheckBox)
            .aria_label(self.label.clone())
            .aria_toggled(if self.checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .flex()
            .items_center()
            .gap(px(8.))
            .min_h(px(30.))
            .rounded(px(4.))
            .text_size(px(13.))
            .child(
                div()
                    .size(px(15.))
                    .rounded(px(3.))
                    .border_1()
                    .border_color(if self.checked { p.accent } else { p.secondary })
                    .bg(if self.checked {
                        p.accent
                    } else {
                        transparent_black()
                    })
                    .when(self.checked, |el| {
                        el.child(
                            DesktopIcon::new(DesktopIconKind::Check)
                                .size(13.)
                                .color(p.on_accent),
                        )
                    }),
            )
            .child(self.label)
            .when(self.disabled, |el| el.opacity(0.4));
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(|s| s.shadow(crate::components::form::dual_focus_ring()));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| handler(&!self.checked, w, cx));
            }
        }
        Semantics::new(el).disabled(self.disabled).reveal_focus()
    }
}

#[derive(IntoElement)]
pub struct DesktopSelect {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    expanded: bool,
    disabled: bool,
    on_open: Option<ClickHandler>,
}
impl DesktopSelect {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: value.into(),
            expanded: false,
            disabled: false,
            on_open: None,
        }
    }
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_open(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Box::new(handler));
        self
    }
}
impl RenderOnce for DesktopSelect {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let mut button = DesktopButton::new(self.id, format!("{}: {}", self.label, self.value))
            .role(Role::ComboBox)
            .expanded(self.expanded)
            .disabled(self.disabled)
            .style(DesktopButtonStyle::Filled)
            .compact()
            .child(
                div()
                    .min_w(px(96.))
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .child(div().flex_1().child(self.value))
                    .child(
                        DesktopIcon::new(DesktopIconKind::ChevronDown)
                            .size(12.)
                            .color(p.secondary),
                    ),
            );
        if let Some(handler) = self.on_open {
            button = button.on_click(handler);
        }
        button
    }
}

#[derive(IntoElement)]
pub struct DesktopTabs {
    label: SharedString,
    items: Vec<AnyElement>,
}
impl DesktopTabs {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            items: vec![],
        }
    }
}
impl ParentElement for DesktopTabs {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}
impl RenderOnce for DesktopTabs {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.label.clone())
            .role(Role::TabList)
            .aria_label(self.label)
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(4.))
            .p(px(3.))
            .rounded(px(6.))
            .bg(DesktopPalette::get(cx).panel)
            .children(self.items)
    }
}
