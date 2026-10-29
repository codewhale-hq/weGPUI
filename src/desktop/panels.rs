use super::{
    DesktopAvatar, DesktopButton, DesktopButtonStyle, DesktopIcon, DesktopIconKind, DesktopPalette,
};

type DismissHandler = Box<dyn Fn(&mut Window, &mut App)>;
use crate::{
    components::{ClickHandler, MaskKind, Modal, Motion, Placement, Presence},
    scrolling::{FocusScroll, RevealFocus},
};
use gpui::{prelude::*, *};

actions!(weui_desktop_dropdown, [DismissDesktopDropdown]);

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "escape",
        DismissDesktopDropdown,
        Some("WeuiPopupMenu"),
    )]);
    cx.bind_keys(
        [
            "up", "down", "home", "end", "left", "right", "enter", "space",
        ]
        .into_iter()
        .map(|key| KeyBinding::new(key, NoAction, Some("WeuiPopupMenu"))),
    );
}

pub(super) fn popup_geometry(
    trigger: Bounds<Pixels>,
    viewport: Size<Pixels>,
    top_inset: Pixels,
    estimated: Pixels,
    max_height: Option<Pixels>,
) -> (Anchor, Point<Pixels>, Pixels) {
    let top_inset = top_inset.min((viewport.height - px(8.)).max(px(0.)));
    let below_y = (trigger.bottom() + px(4.)).max(top_inset);
    let below = (viewport.height - below_y - px(8.)).max(px(0.));
    let above = (trigger.top() - top_inset - px(4.)).max(px(0.));
    let estimated = max_height.map_or(estimated, |max| estimated.min(max));
    let opens_above = below < estimated && above > below;
    let available = if opens_above { above } else { below };
    let available = max_height.map_or(available, |max| available.min(max));
    if opens_above {
        (
            Anchor::BottomLeft,
            trigger.origin - point(px(0.), px(4.)),
            available,
        )
    } else {
        (Anchor::TopLeft, point(trigger.left(), below_y), available)
    }
}

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

#[derive(Clone, Copy)]
pub struct DesktopDropdownColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub secondary: Hsla,
    pub border: Hsla,
    pub hover: Hsla,
}

pub type DesktopDropdownAction = std::rc::Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;
pub type DesktopContextMenuAction = std::rc::Rc<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>;
type DropdownAction = DesktopDropdownAction;
type DropdownIcon = std::rc::Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
type DropdownBuilder = std::rc::Rc<dyn Fn(&mut Window, &mut App) -> Vec<DesktopDropdownEntry>>;
type DropdownSubmenuBuilder = std::rc::Rc<
    dyn Fn(DesktopDropdownSubmenuHandle, &mut Window, &mut App) -> Vec<DesktopDropdownEntry>,
>;
type DropdownTrigger = Box<
    dyn FnOnce(
        bool,
        FocusHandle,
        DropdownAction,
        DesktopContextMenuAction,
        DesktopDropdownAnchor,
    ) -> AnyElement,
>;

#[derive(Clone)]
pub struct DesktopDropdownAnchor {
    bounds: std::rc::Rc<std::cell::Cell<Option<Bounds<Pixels>>>>,
}

#[derive(IntoElement)]
pub struct DesktopDropdownTrigger {
    id: ElementId,
    label: SharedString,
    focus: FocusHandle,
    expanded: bool,
    disabled: bool,
    toggle: DesktopDropdownAction,
    hover: Option<Hsla>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    debug_selector: Option<String>,
    root_wrapper: Option<Box<dyn FnOnce(Stateful<Div>, bool) -> AnyElement>>,
    anchor: Option<DesktopDropdownAnchor>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DesktopDropdownTrigger {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        focus: FocusHandle,
        expanded: bool,
        toggle: DesktopDropdownAction,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            focus,
            expanded,
            disabled: false,
            toggle,
            hover: None,
            tooltip: None,
            debug_selector: None,
            root_wrapper: None,
            anchor: None,
            style: StyleRefinement::default().flex().items_center(),
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn hover_color(mut self, color: impl Into<Hsla>) -> Self {
        self.hover = Some(color.into());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn debug_selector(mut self, selector: impl Into<String>) -> Self {
        self.debug_selector = Some(selector.into());
        self
    }

    pub fn anchor(mut self, anchor: DesktopDropdownAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn wrap_root<E: IntoElement>(
        mut self,
        wrapper: impl FnOnce(Stateful<Div>) -> E + 'static,
    ) -> Self {
        self.root_wrapper = Some(Box::new(move |root, disabled| {
            crate::accessibility::Semantics::new(wrapper(root))
                .disabled(disabled)
                .into_any_element()
        }));
        self
    }
}

impl Styled for DesktopDropdownTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DesktopDropdownTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DesktopDropdownTrigger {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut root = div()
            .id(self.id)
            .role(Role::Button)
            .aria_label(self.label)
            .aria_expanded(self.expanded);
        *root.style() = self.style;
        let mut root = root
            .children(self.children)
            .when_some(self.anchor, |root, anchor| {
                root.relative().child(
                    canvas(
                        move |value, window, _| {
                            if anchor.bounds.get() != Some(value) {
                                anchor.bounds.set(Some(value));
                                window.refresh();
                            }
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .inset_0()
                    .size_full(),
                )
            })
            .when_some(self.debug_selector, |root, selector| {
                root.debug_selector(|| selector)
            })
            .when_some(self.tooltip, |root, tooltip| {
                root.tooltip(move |window, cx| tooltip(window, cx))
            });
        if self.disabled {
            root = root.opacity(0.5);
        } else {
            let action = self.toggle.clone();
            root = root
                .track_focus(&self.focus)
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(|style| style.shadow(crate::components::form::dual_focus_ring()))
                .when_some(self.hover, |root, color| {
                    root.hover(move |style| style.bg(color))
                })
                .on_click(move |event, window, cx| action(event, window, cx))
                .on_a11y_action(gpui::accesskit::Action::Click, move |_, window, cx| {
                    (self.toggle)(&ClickEvent::default(), window, cx)
                });
        }
        let root = match self.root_wrapper {
            Some(wrapper) => wrapper(root, self.disabled),
            None => crate::accessibility::Semantics::new(root)
                .disabled(self.disabled)
                .into_any_element(),
        };
        root
    }
}

#[derive(Clone)]
pub struct DesktopDropdownItem {
    id: ElementId,
    title: SharedString,
    detail: Option<SharedString>,
    icon: Option<DropdownIcon>,
    checked: Option<(bool, bool)>,
    disabled: bool,
    title_size: Option<AbsoluteLength>,
    detail_size: Option<AbsoluteLength>,
    foreground: Option<Hsla>,
    action: Option<DropdownAction>,
    dispatch: Option<std::rc::Rc<dyn Action>>,
    submenu: Option<DropdownSubmenuBuilder>,
}

impl DesktopDropdownItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            detail: None,
            icon: None,
            checked: None,
            disabled: false,
            title_size: None,
            detail_size: None,
            foreground: None,
            action: None,
            dispatch: None,
            submenu: None,
        }
    }

    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn icon<E: IntoElement>(
        mut self,
        icon: impl Fn(&mut Window, &mut App) -> E + 'static,
    ) -> Self {
        self.icon = Some(std::rc::Rc::new(move |window, cx| {
            icon(window, cx).into_any_element()
        }));
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some((checked, false));
        self
    }

    pub fn radio(mut self, checked: bool) -> Self {
        self.checked = Some((checked, true));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn title_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.title_size = Some(size.into());
        self
    }

    pub fn detail_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.detail_size = Some(size.into());
        self
    }

    pub fn foreground(mut self, color: impl Into<Hsla>) -> Self {
        self.foreground = Some(color.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.dispatch = Some(std::rc::Rc::from(action));
        self
    }

    pub fn submenu(
        mut self,
        builder: impl Fn(
            DesktopDropdownSubmenuHandle,
            &mut Window,
            &mut App,
        ) -> Vec<DesktopDropdownEntry>
        + 'static,
    ) -> Self {
        self.submenu = Some(std::rc::Rc::new(builder));
        self
    }
}

#[derive(Clone)]
pub enum DesktopDropdownEntry {
    Item(DesktopDropdownItem),
    Label(SharedString),
    Separator,
}

impl DesktopDropdownEntry {
    pub fn item(item: DesktopDropdownItem) -> Self {
        Self::Item(item)
    }
    pub fn label(label: impl Into<SharedString>) -> Self {
        Self::Label(label.into())
    }
    pub fn separator() -> Self {
        Self::Separator
    }
}

#[derive(IntoElement)]
pub struct DesktopDropdown {
    id: ElementId,
    label: SharedString,
    colors: DesktopDropdownColors,
    width: f32,
    max_height: Option<Pixels>,
    trigger: DropdownTrigger,
    builder: DropdownBuilder,
    action_context: Option<FocusHandle>,
    disabled: bool,
    context_key: Option<SharedString>,
    top_inset: Pixels,
    top_inset_rems: Option<f32>,
    anchored_trigger: bool,
}

struct DesktopDropdownState {
    open: bool,
    generation: u64,
    disabled: bool,
    context_key: Option<SharedString>,
    trigger_focus: FocusHandle,
    menu_focus: FocusHandle,
    return_focus: Option<FocusHandle>,
    action_context: Option<FocusHandle>,
    bounds: std::rc::Rc<std::cell::Cell<Option<Bounds<Pixels>>>>,
    trigger_bounds: DesktopDropdownAnchor,
    scroll: ScrollHandle,
    entries: Vec<DesktopDropdownEntry>,
    handles: Vec<Option<FocusHandle>>,
    context_position: Option<Point<Pixels>>,
    child_serial: u64,
    child: Option<DesktopDropdownChild>,
    _focus_out: Subscription,
    _activation: Subscription,
}

#[derive(Clone)]
struct DesktopDropdownChild {
    index: usize,
    serial: u64,
    revision: u64,
    request: u64,
    label: SharedString,
    focus: FocusHandle,
    focus_requested: bool,
    return_focus: FocusHandle,
    bounds: std::rc::Rc<std::cell::Cell<Option<Bounds<Pixels>>>>,
    row_bounds: std::rc::Rc<std::cell::Cell<Option<Bounds<Pixels>>>>,
    scroll: ScrollHandle,
    entries: Vec<DesktopDropdownEntry>,
    handles: Vec<Option<FocusHandle>>,
}

#[derive(Clone)]
pub struct DesktopDropdownSubmenuHandle {
    state: WeakEntity<DesktopDropdownState>,
    generation: u64,
    serial: u64,
}

#[derive(Clone)]
pub struct DesktopDropdownSubmenuRequest {
    handle: DesktopDropdownSubmenuHandle,
    request: u64,
}

impl DesktopDropdownSubmenuRequest {
    pub fn replace(
        &self,
        entries: Vec<DesktopDropdownEntry>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.handle.replace(entries, Some(self.request), window, cx)
    }
}

impl DesktopDropdownSubmenuHandle {
    pub fn request(&self, cx: &mut App) -> Option<DesktopDropdownSubmenuRequest> {
        let request = self
            .state
            .update(cx, |state, _| {
                if !state.open || state.generation != self.generation {
                    return None;
                }
                let child = state
                    .child
                    .as_mut()
                    .filter(|child| child.serial == self.serial)?;
                child.request = child.request.wrapping_add(1);
                Some(child.request)
            })
            .ok()
            .flatten()?;
        Some(DesktopDropdownSubmenuRequest {
            handle: self.clone(),
            request,
        })
    }

    fn replace(
        &self,
        entries: Vec<DesktopDropdownEntry>,
        request: Option<u64>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if entries.iter().any(
            |entry| matches!(entry, DesktopDropdownEntry::Item(item) if item.submenu.is_some()),
        ) {
            return false;
        }
        let result = self
            .state
            .update(cx, |state, cx| {
                if !state.open || state.generation != self.generation {
                    return None;
                }
                let Some(child) = state
                    .child
                    .as_mut()
                    .filter(|child| child.serial == self.serial)
                else {
                    return None;
                };
                if request.is_some_and(|request| request != child.request) {
                    return None;
                }
                let focused_id =
                    child
                        .entries
                        .iter()
                        .zip(&child.handles)
                        .find_map(|(entry, handle)| {
                            let DesktopDropdownEntry::Item(item) = entry else {
                                return None;
                            };
                            handle
                                .as_ref()
                                .filter(|handle| handle.is_focused(window))
                                .map(|_| item.id.clone())
                        });
                let old = child
                    .entries
                    .iter()
                    .zip(&child.handles)
                    .filter_map(|(entry, handle)| {
                        let DesktopDropdownEntry::Item(item) = entry else {
                            return None;
                        };
                        handle
                            .as_ref()
                            .map(|handle| (item.id.clone(), handle.clone()))
                    })
                    .collect::<Vec<_>>();
                child.handles = entries
                    .iter()
                    .map(|entry| match entry {
                        DesktopDropdownEntry::Item(item)
                            if !item.disabled
                                && (item.action.is_some() || item.dispatch.is_some()) =>
                        {
                            Some(
                                old.iter()
                                    .find(|(id, _)| id == &item.id)
                                    .map(|(_, handle)| handle.clone())
                                    .unwrap_or_else(|| cx.focus_handle().tab_stop(true)),
                            )
                        }
                        _ => None,
                    })
                    .collect();
                child.entries = entries;
                child.revision = child.revision.wrapping_add(1);
                let restore = focused_id
                    .is_some_and(|id| {
                        !child
                            .entries
                            .iter()
                            .zip(&child.handles)
                            .any(|(entry, handle)| {
                                matches!(entry, DesktopDropdownEntry::Item(item) if item.id == id)
                                    && handle.is_some()
                            })
                    })
                    .then(|| child.focus.clone());
                cx.notify();
                Some(restore)
            })
            .ok()
            .flatten();
        if let Some(focus) = result.clone().flatten() {
            window.focus(&focus, cx);
        }
        result.is_some()
    }
}

fn dismiss_dropdown(state: &Entity<DesktopDropdownState>, window: &mut Window, cx: &mut App) {
    state.update(cx, |state, cx| {
        state.open = false;
        state.entries.clear();
        state.handles.clear();
        state.child = None;
        state.context_position = None;
        if let Some(focus) = state.return_focus.take() {
            window.focus(&focus, cx);
        }
        cx.notify();
    });
}

fn open_dropdown(
    state: &Entity<DesktopDropdownState>,
    builder: &DropdownBuilder,
    action_context: &Option<FocusHandle>,
    position: Option<Point<Pixels>>,
    window: &mut Window,
    cx: &mut App,
) {
    if state.read(cx).disabled {
        return;
    }
    let previous = if state.read(cx).open {
        state.read(cx).return_focus.clone()
    } else {
        window.focused(cx)
    };
    let entries = builder(window, cx);
    state.update(cx, |state, cx| {
        state.generation = state.generation.wrapping_add(1);
        state.return_focus = previous
            .or_else(|| action_context.clone())
            .or_else(|| Some(state.trigger_focus.clone()));
        state.action_context = action_context.clone();
        state.context_position = position;
        state.child = None;
        state.handles = entries
            .iter()
            .map(|entry| match entry {
                DesktopDropdownEntry::Item(item)
                    if !item.disabled
                        && (item.action.is_some()
                            || item.dispatch.is_some()
                            || item.submenu.is_some()) =>
                {
                    Some(cx.focus_handle().tab_stop(true))
                }
                _ => None,
            })
            .collect();
        state.entries = entries;
        state.open = true;
        window.focus(&state.menu_focus, cx);
        cx.notify();
    });
}

fn open_dropdown_child(
    state: &Entity<DesktopDropdownState>,
    index: usize,
    focus_child: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let current = state.read(cx);
    if !current.open || current.disabled {
        return;
    }
    if let Some(child) = current.child.as_ref().filter(|child| child.index == index) {
        if focus_child {
            let serial = child.serial;
            let focus = child.return_focus.clone();
            window.focus(&focus, cx);
            state.update(cx, |state, cx| {
                if let Some(child) = state.child.as_mut().filter(|child| child.serial == serial) {
                    child.focus_requested = true;
                    cx.notify();
                }
            });
        }
        return;
    }
    let Some(DesktopDropdownEntry::Item(item)) = current.entries.get(index) else {
        return;
    };
    let Some(builder) = item.submenu.clone().filter(|_| !item.disabled) else {
        return;
    };
    let label = item.title.clone();
    let Some(return_focus) = current.handles.get(index).and_then(Clone::clone) else {
        return;
    };
    let generation = current.generation;
    let prior_focus = current.child.as_ref().and_then(|child| {
        (child.focus.is_focused(window) || child.focus.contains_focused(window, cx))
            .then(|| child.return_focus.clone())
    });
    if let Some(prior_focus) = prior_focus {
        window.focus(&prior_focus, cx);
    }
    if focus_child {
        window.focus(&return_focus, cx);
    }
    let serial = state.update(cx, |state, cx| {
        state.child_serial = state.child_serial.wrapping_add(1);
        let serial = state.child_serial;
        state.child = Some(DesktopDropdownChild {
            index,
            serial,
            revision: 0,
            request: 0,
            label,
            focus: cx.focus_handle(),
            focus_requested: focus_child,
            return_focus,
            bounds: std::rc::Rc::new(std::cell::Cell::new(None)),
            row_bounds: std::rc::Rc::new(std::cell::Cell::new(None)),
            scroll: ScrollHandle::new(),
            entries: Vec::new(),
            handles: Vec::new(),
        });
        cx.notify();
        serial
    });
    let handle = DesktopDropdownSubmenuHandle {
        state: state.downgrade(),
        generation,
        serial,
    };
    let entries = builder(handle.clone(), window, cx);
    let initial = state.read(cx).child.as_ref().is_some_and(|child| {
        state.read(cx).open
            && state.read(cx).generation == generation
            && child.serial == serial
            && child.revision == 0
    });
    if initial && !handle.replace(entries, None, window, cx) {
        return;
    }
}

fn close_dropdown_child(
    state: &Entity<DesktopDropdownState>,
    generation: u64,
    serial: u64,
    restore_focus: bool,
    window: &mut Window,
    cx: &mut App,
) {
    if !state.read(cx).open || state.read(cx).generation != generation {
        return;
    }
    let Some(child) = state
        .read(cx)
        .child
        .as_ref()
        .filter(|child| child.serial == serial)
    else {
        return;
    };
    let return_focus = child.return_focus.clone();
    state.update(cx, |state, cx| {
        state.child = None;
        cx.notify();
    });
    if restore_focus {
        window.focus(&return_focus, cx);
    }
}

fn render_dropdown_child(
    state: Entity<DesktopDropdownState>,
    child: DesktopDropdownChild,
    generation: u64,
    colors: DesktopDropdownColors,
    width: f32,
    max_height: Option<Pixels>,
    top_inset: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    let row_bounds = child.row_bounds.get()?;
    let viewport = window.viewport_size();
    let width = px(width).min((viewport.width - px(16.)).max(px(0.)));
    let top_inset = top_inset.min((viewport.height - px(8.)).max(px(0.)));
    let available = (viewport.height - top_inset - px(8.)).max(px(0.));
    let open_left = viewport.width - row_bounds.right() < width + px(8.)
        && row_bounds.left() > viewport.width - row_bounds.right();
    let (anchor, position) = if open_left {
        (
            Anchor::TopRight,
            point(row_bounds.left() - px(4.), row_bounds.top().max(top_inset)),
        )
    } else {
        (
            Anchor::TopLeft,
            point(row_bounds.right() + px(4.), row_bounds.top().max(top_inset)),
        )
    };
    let serial = child.serial;
    let revision = child.revision;
    if child.focus_requested {
        let state = state.clone();
        window.on_next_frame(move |window, cx| {
            if !state.read(cx).open || state.read(cx).generation != generation {
                return;
            }
            let focus = state.update(cx, |state, cx| {
                let child = state
                    .child
                    .as_mut()
                    .filter(|child| child.serial == serial && child.focus_requested)?;
                child.focus_requested = false;
                let focus = child
                    .return_focus
                    .is_focused(window)
                    .then(|| child.focus.clone());
                cx.notify();
                focus
            });
            if let Some(focus) = focus {
                window.focus(&focus, cx);
            }
        });
    }
    if child.focus.is_focused(window) {
        if let Some(first) = child.handles.iter().flatten().next().cloned() {
            let state = state.clone();
            let focus = child.focus.clone();
            window.on_next_frame(move |window, cx| {
                if state.read(cx).open
                    && state.read(cx).generation == generation
                    && state
                        .read(cx)
                        .child
                        .as_ref()
                        .is_some_and(|child| child.serial == serial && child.revision == revision)
                    && focus.is_focused(window)
                {
                    window.focus(&first, cx);
                }
            });
        }
    }
    let child_focus = child.focus.clone();
    let handles = child.handles.clone();
    let mut menu =
        div()
            .id("desktop-dropdown-submenu")
            .key_context("WeuiPopupMenu")
            .role(Role::Menu)
            .aria_label(child.label.clone())
            .track_focus(&child_focus)
            .tab_group()
            .w(width)
            .max_h(max_height.unwrap_or(available).min(available))
            .overflow_y_scroll()
            .track_scroll(&child.scroll)
            .occlude()
            .p(px(5.))
            .rounded(px(7.))
            .border_1()
            .border_color(colors.border)
            .bg(colors.background)
            .shadow(vec![BoxShadow {
                color: black().opacity(0.2),
                offset: point(px(0.), px(6.)),
                blur_radius: px(24.),
                spread_radius: px(0.),
                inset: false,
            }])
            .on_action({
                let state = state.clone();
                move |_: &DismissDesktopDropdown, window, cx| {
                    close_dropdown_child(&state, generation, serial, true, window, cx);
                    cx.stop_propagation();
                }
            })
            .on_key_down({
                let state = state.clone();
                let handles = handles.clone();
                move |event, window, cx| {
                    if !state.read(cx).open
                        || state.read(cx).generation != generation
                        || !state.read(cx).child.as_ref().is_some_and(|child| {
                            child.serial == serial && child.revision == revision
                        })
                        || event.keystroke.modifiers.modified()
                    {
                        return;
                    }
                    match event.keystroke.key.as_str() {
                        "left" | "escape" => {
                            close_dropdown_child(&state, generation, serial, true, window, cx)
                        }
                        "up" | "down" | "home" | "end" => {
                            let enabled: Vec<_> = handles.iter().flatten().collect();
                            if !enabled.is_empty() {
                                let current =
                                    enabled.iter().position(|handle| handle.is_focused(window));
                                let next = match event.keystroke.key.as_str() {
                                    "home" => 0,
                                    "end" => enabled.len() - 1,
                                    "up" => current
                                        .map(|i| (i + enabled.len() - 1) % enabled.len())
                                        .unwrap_or(enabled.len() - 1),
                                    _ => current.map(|i| (i + 1) % enabled.len()).unwrap_or(0),
                                };
                                window.focus(enabled[next], cx);
                            }
                        }
                        _ => return,
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            });
    for (index, entry) in child.entries.iter().cloned().enumerate() {
        match entry {
            DesktopDropdownEntry::Separator => {
                menu = menu.child(div().h(px(1.)).my(px(5.)).mx(px(8.)).bg(colors.border));
            }
            DesktopDropdownEntry::Label(label) => {
                menu = menu.child(
                    crate::accessibility::Semantics::new(
                        div()
                            .id(("desktop-submenu-label", index))
                            .role(Role::Label)
                            .aria_label(label.clone())
                            .px(px(10.))
                            .py(px(6.))
                            .text_size(px(11.))
                            .text_color(colors.secondary)
                            .child(label),
                    )
                    .read_only(true),
                );
            }
            DesktopDropdownEntry::Item(item) => {
                let disabled = item.disabled || (item.action.is_none() && item.dispatch.is_none());
                let name = match &item.detail {
                    Some(detail) if !detail.is_empty() => {
                        SharedString::from(format!("{}. {}", item.title, detail))
                    }
                    _ => item.title.clone(),
                };
                let icon = item.icon.as_ref().map(|icon| icon(window, cx));
                let mut row = div()
                    .id(item.id)
                    .role(match item.checked {
                        Some((_, true)) => Role::MenuItemRadio,
                        Some((_, false)) => Role::MenuItemCheckBox,
                        None => Role::MenuItem,
                    })
                    .aria_label(name)
                    .when_some(item.checked, |row, (checked, _)| {
                        row.aria_toggled(if checked {
                            gpui::accesskit::Toggled::True
                        } else {
                            gpui::accesskit::Toggled::False
                        })
                    })
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .px(px(10.))
                    .py(px(8.))
                    .rounded(px(5.))
                    .text_color(item.foreground.unwrap_or(colors.foreground))
                    .child(div().w(px(20.)).flex_shrink_0().children(icon))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(item.title_size.unwrap_or(px(13.).into()))
                                    .truncate()
                                    .child(item.title),
                            )
                            .when_some(item.detail, |row, detail| {
                                row.child(
                                    div()
                                        .text_size(item.detail_size.unwrap_or(px(11.).into()))
                                        .text_color(colors.secondary)
                                        .whitespace_normal()
                                        .child(detail),
                                )
                            }),
                    )
                    .when(item.checked.is_some_and(|(checked, _)| checked), |row| {
                        row.child(DesktopIcon::new(DesktopIconKind::Check).size(15.))
                    });
                if let Some(handle) = child.handles[index].clone() {
                    let action = item.action;
                    let dispatch = item.dispatch;
                    let invoke: DropdownAction = std::rc::Rc::new({
                        let state = state.clone();
                        let scope = child_focus.clone();
                        let parent_scope = state.read(cx).menu_focus.clone();
                        move |event, window, cx| {
                            if !state.read(cx).open
                                || state.read(cx).generation != generation
                                || !state.read(cx).child.as_ref().is_some_and(|child| {
                                    child.serial == serial && child.revision == revision
                                })
                            {
                                return;
                            }
                            let target = state.read(cx).return_focus.clone();
                            let context = state.read(cx).action_context.clone();
                            state.update(cx, |state, cx| {
                                state.open = false;
                                state.entries.clear();
                                state.handles.clear();
                                state.child = None;
                                state.context_position = None;
                                cx.notify();
                            });
                            if let Some(action) = &action {
                                action(event, window, cx);
                            } else if let Some(dispatch) = &dispatch {
                                if let Some(context) = context {
                                    window.focus(&context, cx);
                                }
                                window.dispatch_action(dispatch.boxed_clone(), cx);
                            }
                            if (scope.is_focused(window)
                                || scope.contains_focused(window, cx)
                                || parent_scope.is_focused(window)
                                || parent_scope.contains_focused(window, cx))
                                && let Some(target) = target
                            {
                                window.focus(&target, cx);
                            }
                        }
                    });
                    row = row
                        .track_focus(&handle)
                        .tab_stop(true)
                        .cursor_pointer()
                        .focus_visible(|style| {
                            style.shadow(crate::components::form::dual_focus_ring())
                        })
                        .hover(move |style| style.bg(colors.hover))
                        .on_click({
                            let invoke = invoke.clone();
                            move |event, window, cx| invoke(event, window, cx)
                        })
                        .on_a11y_action(gpui::accesskit::Action::Click, move |_, window, cx| {
                            invoke(&ClickEvent::default(), window, cx)
                        });
                } else {
                    row = row.opacity(0.65);
                }
                menu = menu.child(
                    crate::accessibility::Semantics::new(row)
                        .disabled(disabled)
                        .reveal_focus(),
                );
            }
        }
    }
    let bounds = child.bounds.clone();
    let body = div().relative().child(menu).child(
        canvas(
            move |value, window, _| {
                if bounds.get() != Some(value) {
                    bounds.set(Some(value));
                    window.refresh();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .inset_0()
        .size_full(),
    );
    Some(
        deferred(
            anchored()
                .anchor(anchor)
                .position(position)
                .snap_to_window_with_margin(px(8.))
                .child(FocusScroll::new(
                    "desktop-dropdown-submenu-scroll",
                    &child.scroll,
                    body,
                )),
        )
        .with_priority(2)
        .into_any_element(),
    )
}

impl DesktopDropdown {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        colors: DesktopDropdownColors,
        trigger: impl FnOnce(bool, FocusHandle, DesktopDropdownAction) -> AnyElement + 'static,
        builder: impl Fn(&mut Window, &mut App) -> Vec<DesktopDropdownEntry> + 'static,
    ) -> Self {
        let mut dropdown = Self::contextual(
            id,
            label,
            colors,
            move |open, focus, toggle, _, _| trigger(open, focus, toggle),
            builder,
        );
        dropdown.anchored_trigger = false;
        dropdown
    }

    pub fn contextual(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        colors: DesktopDropdownColors,
        trigger: impl FnOnce(
            bool,
            FocusHandle,
            DesktopDropdownAction,
            DesktopContextMenuAction,
            DesktopDropdownAnchor,
        ) -> AnyElement
        + 'static,
        builder: impl Fn(&mut Window, &mut App) -> Vec<DesktopDropdownEntry> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            colors,
            width: 300.,
            max_height: None,
            trigger: Box::new(trigger),
            builder: std::rc::Rc::new(builder),
            action_context: None,
            disabled: false,
            context_key: None,
            top_inset: px(8.),
            top_inset_rems: None,
            anchored_trigger: true,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn max_height(mut self, height: Pixels) -> Self {
        self.max_height = Some(height.max(px(0.)));
        self
    }

    pub fn action_context(mut self, focus: FocusHandle) -> Self {
        self.action_context = Some(focus);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn context_key(mut self, context_key: impl Into<SharedString>) -> Self {
        self.context_key = Some(context_key.into());
        self
    }

    pub fn top_inset(mut self, inset: Pixels) -> Self {
        self.top_inset = inset.max(px(0.));
        self.top_inset_rems = None;
        self
    }

    pub fn top_inset_rems(mut self, inset: f32) -> Self {
        self.top_inset_rems = Some(inset.max(0.));
        self
    }
}

impl RenderOnce for DesktopDropdown {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_id = ElementId::NamedChild(
            std::sync::Arc::new(self.id.clone()),
            "dropdown-state".into(),
        );
        let scroll_id = ElementId::NamedChild(
            std::sync::Arc::new(self.id.clone()),
            "dropdown-scroll".into(),
        );
        let state = window.use_keyed_state(state_id, cx, |window, cx| {
            let menu_focus = cx.focus_handle();
            let focus_out = cx.on_focus_out(
                &menu_focus,
                window,
                |state: &mut DesktopDropdownState, _, window, cx| {
                    if state.open && !state.trigger_focus.is_focused(window) {
                        state.open = false;
                        state.entries.clear();
                        state.handles.clear();
                        state.child = None;
                        state.context_position = None;
                        cx.notify();
                    }
                },
            );
            let activation = cx.observe_window_activation(
                window,
                |state: &mut DesktopDropdownState, window, cx| {
                    if state.open && !window.is_window_active() {
                        state.open = false;
                        state.entries.clear();
                        state.handles.clear();
                        state.child = None;
                        state.context_position = None;
                        if let Some(focus) = state.return_focus.take() {
                            window.focus(&focus, cx);
                        }
                        cx.notify();
                    }
                },
            );
            DesktopDropdownState {
                open: false,
                generation: 0,
                disabled: false,
                context_key: None,
                trigger_focus: cx.focus_handle().tab_stop(true),
                menu_focus,
                return_focus: None,
                action_context: None,
                bounds: std::rc::Rc::new(std::cell::Cell::new(None)),
                trigger_bounds: DesktopDropdownAnchor {
                    bounds: std::rc::Rc::new(std::cell::Cell::new(None)),
                },
                scroll: ScrollHandle::new(),
                entries: Vec::new(),
                handles: Vec::new(),
                context_position: None,
                child_serial: 0,
                child: None,
                _focus_out: focus_out,
                _activation: activation,
            }
        });
        if state.read(cx).disabled != self.disabled
            || state.read(cx).context_key != self.context_key
        {
            let old = state.read(cx);
            let was_open = old.open;
            let focus_was_in_menu =
                old.menu_focus.is_focused(window) || old.menu_focus.contains_focused(window, cx);
            let trigger_focus = old.trigger_focus.clone();
            let fallback = self.action_context.clone();
            state.update(cx, |state, _| {
                state.disabled = self.disabled;
                state.context_key = self.context_key.clone();
                if was_open {
                    state.open = false;
                    state.generation = state.generation.wrapping_add(1);
                    state.entries.clear();
                    state.handles.clear();
                    state.child = None;
                    state.context_position = None;
                    state.return_focus = None;
                }
            });
            if was_open && focus_was_in_menu {
                if !self.disabled && !self.anchored_trigger {
                    window.focus(&trigger_focus, cx);
                } else if let Some(focus) = fallback.filter(|focus| focus != &trigger_focus) {
                    window.focus(&focus, cx);
                } else {
                    window.blur(cx);
                }
            }
        }
        let open = state.read(cx).open;
        let trigger_focus = state.read(cx).trigger_focus.clone();
        let menu_focus = state.read(cx).menu_focus.clone();
        let bounds = state.read(cx).bounds.clone();
        let trigger_anchor = state.read(cx).trigger_bounds.clone();
        let scroll = state.read(cx).scroll.clone();
        let builder = self.builder.clone();
        let action_context = self.action_context.clone();
        let context_builder = builder.clone();
        let context_action = action_context.clone();
        let toggle: DropdownAction = std::rc::Rc::new({
            let state = state.clone();
            move |_, window, cx| {
                if state.read(cx).disabled {
                    return;
                }
                if state.read(cx).open {
                    state.update(cx, |state, cx| {
                        state.open = false;
                        state.entries.clear();
                        state.handles.clear();
                        state.child = None;
                        state.context_position = None;
                        if let Some(focus) = state.return_focus.take() {
                            window.focus(&focus, cx);
                        }
                        cx.notify();
                    });
                } else {
                    open_dropdown(&state, &builder, &action_context, None, window, cx);
                }
            }
        });
        let context: DesktopContextMenuAction = std::rc::Rc::new({
            let state = state.clone();
            move |event, window, cx| {
                if event.button != MouseButton::Right || state.read(cx).disabled {
                    return;
                }
                open_dropdown(
                    &state,
                    &context_builder,
                    &context_action,
                    Some(event.position),
                    window,
                    cx,
                );
                cx.stop_propagation();
            }
        });
        let mut root = div()
            .id(self.id)
            .relative()
            .child((self.trigger)(
                open,
                trigger_focus.clone(),
                toggle,
                context,
                trigger_anchor.clone(),
            ))
            .child(
                canvas(
                    {
                        let bounds = bounds.clone();
                        move |value, window, _| {
                            if bounds.get() != Some(value) {
                                bounds.set(Some(value));
                                window.refresh();
                            }
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .inset_0()
                .size_full(),
            );
        if !open {
            return root;
        }
        let Some(trigger_bounds) = bounds.get() else {
            return root;
        };
        let trigger_bounds = if let Some(position) = state.read(cx).context_position {
            Bounds {
                origin: position,
                size: size(px(0.), px(0.)),
            }
        } else if self.anchored_trigger {
            trigger_anchor.bounds.get().unwrap_or(trigger_bounds)
        } else {
            trigger_bounds
        };
        let entries = state.read(cx).entries.clone();
        let handles = state.read(cx).handles.clone();
        let child = state.read(cx).child.clone();
        let generation = state.read(cx).generation;
        if menu_focus.is_focused(window) {
            let selected = entries
                .iter()
                .zip(&handles)
                .find_map(|(entry, handle)| match entry {
                    DesktopDropdownEntry::Item(item)
                        if item.checked.is_some_and(|(checked, _)| checked) =>
                    {
                        handle.clone()
                    }
                    _ => None,
                });
            if let Some(first) = selected.or_else(|| handles.iter().flatten().next().cloned()) {
                let scope = menu_focus.clone();
                let state = state.clone();
                window.on_next_frame(move |window, cx| {
                    if state.read(cx).open
                        && state.read(cx).generation == generation
                        && scope.is_focused(window)
                    {
                        window.focus(&first, cx);
                    }
                });
            }
        }
        let viewport = window.viewport_size();
        let top_inset = self
            .top_inset_rems
            .map(|inset| window.rem_size() * inset)
            .unwrap_or(self.top_inset);
        let estimated = px(entries
            .iter()
            .map(|entry| match entry {
                DesktopDropdownEntry::Item(item) if item.detail.is_some() => 64.,
                DesktopDropdownEntry::Item(_) => 36.,
                DesktopDropdownEntry::Label(_) => 28.,
                DesktopDropdownEntry::Separator => 11.,
            })
            .sum::<f32>()
            + 10.);
        let (anchor, position, available) = popup_geometry(
            trigger_bounds,
            viewport,
            top_inset,
            estimated,
            self.max_height,
        );
        let colors = self.colors;
        let mut menu = div()
            .id("desktop-dropdown-menu")
            .key_context("WeuiPopupMenu")
            .role(Role::Menu)
            .aria_label(self.label)
            .track_focus(&menu_focus)
            .tab_group()
            .w(px(self.width).min((viewport.width - px(16.)).max(px(0.))))
            .max_h(available)
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .occlude()
            .p(px(5.))
            .rounded(px(7.))
            .border_1()
            .border_color(colors.border)
            .bg(colors.background)
            .shadow(vec![BoxShadow {
                color: black().opacity(0.2),
                offset: point(px(0.), px(6.)),
                blur_radius: px(24.),
                spread_radius: px(0.),
                inset: false,
            }])
            .on_mouse_down_out({
                let state = state.clone();
                let bounds = bounds.clone();
                let trigger_anchor = trigger_anchor.clone();
                let anchored_trigger = self.anchored_trigger;
                let handles = handles.clone();
                let scope = menu_focus.clone();
                move |event, window, cx| {
                    if state.read(cx).disabled
                        || !state.read(cx).open
                        || state.read(cx).generation != generation
                    {
                        return;
                    }
                    let trigger_bounds = if anchored_trigger {
                        trigger_anchor.bounds.get()
                    } else {
                        bounds.get()
                    };
                    if trigger_bounds.is_some_and(|bounds| bounds.contains(&event.position))
                        || state.read(cx).child.as_ref().is_some_and(|child| {
                            child
                                .bounds
                                .get()
                                .is_some_and(|bounds| bounds.contains(&event.position))
                        })
                    {
                        return;
                    }
                    let target = state
                        .read(cx)
                        .return_focus
                        .clone()
                        .unwrap_or_else(|| state.read(cx).trigger_focus.clone());
                    let child = state.read(cx).child.clone();
                    state.update(cx, |state, cx| {
                        state.open = false;
                        state.entries.clear();
                        state.handles.clear();
                        state.child = None;
                        state.context_position = None;
                        cx.notify();
                    });
                    let handles = handles.clone();
                    let scope = scope.clone();
                    let state = state.clone();
                    window.on_next_frame(move |window, cx| {
                        if !state.read(cx).open
                            && state.read(cx).generation == generation
                            && (scope.is_focused(window)
                                || handles
                                    .iter()
                                    .flatten()
                                    .any(|handle| handle.is_focused(window))
                                || child.as_ref().is_some_and(|child| {
                                    child.focus.is_focused(window)
                                        || child
                                            .handles
                                            .iter()
                                            .flatten()
                                            .any(|handle| handle.is_focused(window))
                                }))
                        {
                            window.focus(&target, cx);
                        }
                    });
                }
            })
            .on_action({
                let state = state.clone();
                move |_: &DismissDesktopDropdown, window, cx| {
                    if state.read(cx).disabled
                        || !state.read(cx).open
                        || state.read(cx).generation != generation
                    {
                        cx.propagate();
                        return;
                    }
                    dismiss_dropdown(&state, window, cx);
                }
            })
            .on_key_down({
                let state = state.clone();
                let handles = handles.clone();
                move |event, window, cx| {
                    if state.read(cx).disabled
                        || !state.read(cx).open
                        || state.read(cx).generation != generation
                    {
                        return;
                    }
                    if event.keystroke.modifiers.modified()
                        && !(event.keystroke.key == "tab"
                            && event.keystroke.modifiers.shift
                            && !event.keystroke.modifiers.control
                            && !event.keystroke.modifiers.alt
                            && !event.keystroke.modifiers.platform
                            && !event.keystroke.modifiers.function)
                    {
                        return;
                    }
                    match event.keystroke.key.as_str() {
                        "escape" | "tab" => {
                            let tab = event.keystroke.key == "tab";
                            let backward = event.keystroke.modifiers.shift;
                            let trigger = state.read(cx).trigger_focus.clone();
                            if tab {
                                state.update(cx, |state, cx| {
                                    state.open = false;
                                    state.entries.clear();
                                    state.handles.clear();
                                    state.child = None;
                                    state.context_position = None;
                                    window.focus(&state.trigger_focus, cx);
                                    cx.notify();
                                });
                                let state = state.clone();
                                window.on_next_frame(move |window, cx| {
                                    if !state.read(cx).open
                                        && state.read(cx).generation == generation
                                        && trigger.is_focused(window)
                                    {
                                        if backward {
                                            window.focus_prev(cx);
                                        } else {
                                            window.focus_next(cx);
                                        }
                                    }
                                });
                            } else {
                                dismiss_dropdown(&state, window, cx);
                            }
                        }
                        "up" | "down" | "home" | "end" => {
                            let enabled: Vec<_> = handles.iter().flatten().collect();
                            if !enabled.is_empty() {
                                let current =
                                    enabled.iter().position(|handle| handle.is_focused(window));
                                let next = match event.keystroke.key.as_str() {
                                    "home" => 0,
                                    "end" => enabled.len() - 1,
                                    "up" => current
                                        .map(|i| (i + enabled.len() - 1) % enabled.len())
                                        .unwrap_or(enabled.len() - 1),
                                    _ => current.map(|i| (i + 1) % enabled.len()).unwrap_or(0),
                                };
                                let child = state.read(cx).child.as_ref().and_then(|child| {
                                    (enabled[next] != &child.return_focus).then_some(child.serial)
                                });
                                if let Some(serial) = child {
                                    close_dropdown_child(
                                        &state, generation, serial, false, window, cx,
                                    );
                                }
                                window.focus(enabled[next], cx);
                            }
                        }
                        "right" => {
                            if let Some(index) = handles.iter().position(|handle| {
                                handle
                                    .as_ref()
                                    .is_some_and(|handle| handle.is_focused(window))
                            }) {
                                open_dropdown_child(&state, index, true, window, cx);
                            }
                        }
                        _ => return,
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            });
        for (index, entry) in entries.into_iter().enumerate() {
            match entry {
                DesktopDropdownEntry::Separator => {
                    menu = menu.child(div().h(px(1.)).my(px(5.)).mx(px(8.)).bg(colors.border))
                }
                DesktopDropdownEntry::Label(label) => {
                    menu = menu.child(
                        crate::accessibility::Semantics::new(
                            div()
                                .id(("desktop-dropdown-label", index))
                                .role(Role::Label)
                                .aria_label(label.clone())
                                .px(px(10.))
                                .py(px(6.))
                                .text_size(px(11.))
                                .text_color(colors.secondary)
                                .child(label),
                        )
                        .read_only(true),
                    )
                }
                DesktopDropdownEntry::Item(item) => {
                    let has_submenu = item.submenu.is_some();
                    let child_open = child.as_ref().is_some_and(|child| child.index == index);
                    let disabled = item.disabled
                        || (item.action.is_none() && item.dispatch.is_none() && !has_submenu);
                    let name = match &item.detail {
                        Some(detail) if !detail.is_empty() => {
                            SharedString::from(format!("{}. {}", item.title, detail))
                        }
                        _ => item.title.clone(),
                    };
                    let icon = item.icon.as_ref().map(|icon| icon(window, cx));
                    let mut row = div()
                        .id(item.id)
                        .role(match item.checked {
                            Some((_, true)) => Role::MenuItemRadio,
                            Some((_, false)) => Role::MenuItemCheckBox,
                            None => Role::MenuItem,
                        })
                        .aria_label(name)
                        .when(has_submenu, |row| row.aria_expanded(child_open))
                        .when_some(item.checked, |row, (checked, _)| {
                            row.aria_toggled(if checked {
                                gpui::accesskit::Toggled::True
                            } else {
                                gpui::accesskit::Toggled::False
                            })
                        })
                        .relative()
                        .flex()
                        .items_center()
                        .gap(px(10.))
                        .px(px(10.))
                        .py(px(8.))
                        .rounded(px(5.))
                        .text_color(item.foreground.unwrap_or(colors.foreground))
                        .child(div().w(px(20.)).flex_shrink_0().children(icon))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(item.title_size.unwrap_or(px(13.).into()))
                                        .truncate()
                                        .child(item.title),
                                )
                                .when_some(item.detail, |row, detail| {
                                    row.child(
                                        div()
                                            .text_size(item.detail_size.unwrap_or(px(11.).into()))
                                            .text_color(colors.secondary)
                                            .whitespace_normal()
                                            .child(detail),
                                    )
                                }),
                        )
                        .when(item.checked.is_some_and(|(checked, _)| checked), |row| {
                            row.child(DesktopIcon::new(DesktopIconKind::Check).size(15.))
                        })
                        .when(has_submenu, |row| {
                            row.child(DesktopIcon::new(DesktopIconKind::ChevronRight).size(15.))
                        });
                    if let Some(handle) = handles[index].clone() {
                        let action = item.action;
                        let dispatch = item.dispatch;
                        let invoke: DropdownAction = std::rc::Rc::new({
                            let state = state.clone();
                            let scope = menu_focus.clone();
                            move |event, window, cx| {
                                if state.read(cx).disabled
                                    || !state.read(cx).open
                                    || state.read(cx).generation != generation
                                {
                                    return;
                                }
                                if has_submenu {
                                    open_dropdown_child(&state, index, true, window, cx);
                                    return;
                                }
                                let target = state.read(cx).return_focus.clone();
                                let context = state.read(cx).action_context.clone();
                                state.update(cx, |state, cx| {
                                    state.open = false;
                                    state.entries.clear();
                                    state.handles.clear();
                                    state.child = None;
                                    state.context_position = None;
                                    cx.notify();
                                });
                                if let Some(action) = &action {
                                    action(event, window, cx);
                                } else if let Some(dispatch) = &dispatch {
                                    if let Some(context) = context {
                                        window.focus(&context, cx);
                                    }
                                    window.dispatch_action(dispatch.boxed_clone(), cx);
                                }
                                if (scope.is_focused(window) || scope.contains_focused(window, cx))
                                    && let Some(target) = target
                                {
                                    window.focus(&target, cx);
                                }
                            }
                        });
                        row = row
                            .track_focus(&handle)
                            .tab_stop(true)
                            .cursor_pointer()
                            .focus_visible(|style| {
                                style.shadow(crate::components::form::dual_focus_ring())
                            })
                            .hover(move |style| style.bg(colors.hover))
                            .when(has_submenu, |row| {
                                let state = state.clone();
                                row.on_hover(move |hovered, window, cx| {
                                    if *hovered && state.read(cx).generation == generation {
                                        open_dropdown_child(&state, index, false, window, cx);
                                    }
                                })
                            })
                            .when(!has_submenu, |row| {
                                let state = state.clone();
                                row.on_hover(move |hovered, window, cx| {
                                    if !*hovered || state.read(cx).generation != generation {
                                        return;
                                    }
                                    let child = state.read(cx).child.as_ref().and_then(|child| {
                                        (!(child.focus.is_focused(window)
                                            || child.focus.contains_focused(window, cx)))
                                        .then_some(child.serial)
                                    });
                                    if let Some(serial) = child {
                                        close_dropdown_child(
                                            &state, generation, serial, false, window, cx,
                                        );
                                    }
                                })
                            })
                            .on_click({
                                let invoke = invoke.clone();
                                move |event, window, cx| invoke(event, window, cx)
                            })
                            .on_a11y_action(
                                gpui::accesskit::Action::Click,
                                move |_, window, cx| invoke(&ClickEvent::default(), window, cx),
                            );
                    } else {
                        row = row.opacity(0.65);
                    }
                    if let Some(active) = child.as_ref().filter(|child| child.index == index) {
                        let row_bounds = active.row_bounds.clone();
                        row = row.child(
                            canvas(
                                move |value, window, _| {
                                    if row_bounds.get() != Some(value) {
                                        row_bounds.set(Some(value));
                                        window.refresh();
                                    }
                                },
                                |_, _, _, _| {},
                            )
                            .absolute()
                            .inset_0()
                            .size_full(),
                        );
                        if let Some(submenu) = render_dropdown_child(
                            state.clone(),
                            active.clone(),
                            generation,
                            colors,
                            self.width,
                            self.max_height,
                            top_inset,
                            window,
                            cx,
                        ) {
                            row = row.child(submenu);
                        }
                    }
                    let semantics = crate::accessibility::Semantics::new(row).disabled(disabled);
                    menu = if has_submenu {
                        menu.child(
                            semantics
                                .has_popup(gpui::accesskit::HasPopup::Menu)
                                .reveal_focus(),
                        )
                    } else {
                        menu.child(semantics.reveal_focus())
                    };
                }
            }
        }
        root = root.child(
            deferred(
                anchored()
                    .anchor(anchor)
                    .position(position)
                    .child(FocusScroll::new(scroll_id, &scroll, menu)),
            )
            .with_priority(1),
        );
        root
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
