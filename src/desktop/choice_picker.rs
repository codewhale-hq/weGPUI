use super::{DesktopDropdownColors, DesktopIcon, DesktopIconKind};
use crate::{
    accessibility::Semantics,
    input::{InputEvent, TextInput},
    scrolling::{FocusScroll, RevealFocus},
};
use gpui::{prelude::*, *};
use std::{cell::Cell, rc::Rc, sync::Arc};

actions!(
    weui_desktop_choice,
    [
        DismissChoice,
        PreviousChoice,
        NextChoice,
        FirstChoice,
        LastChoice,
        ConfirmChoice,
        NextChoiceFocus,
        PreviousChoiceFocus
    ]
);

pub(super) fn init(cx: &mut App) {
    let context = Some("WeuiDesktopChoice");
    cx.bind_keys([
        KeyBinding::new("escape", DismissChoice, context),
        KeyBinding::new("up", PreviousChoice, context),
        KeyBinding::new("down", NextChoice, context),
        KeyBinding::new("home", FirstChoice, context),
        KeyBinding::new("end", LastChoice, context),
        KeyBinding::new("enter", ConfirmChoice, context),
        KeyBinding::new("tab", NextChoiceFocus, context),
        KeyBinding::new("shift-tab", PreviousChoiceFocus, context),
    ]);
}

pub type DesktopChoiceColors = DesktopDropdownColors;
type ChoiceIcon = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;
type ChoiceHandler<T> = Rc<dyn Fn(T, &mut Window, &mut App)>;

#[derive(Clone)]
pub struct DesktopChoice<T> {
    value: T,
    label: SharedString,
    detail: Option<SharedString>,
    search_terms: Vec<SharedString>,
    icon: Option<ChoiceIcon>,
    disabled: bool,
}

impl<T> DesktopChoice<T> {
    pub fn new(value: T, label: impl Into<SharedString>) -> Self {
        Self {
            value,
            label: label.into(),
            detail: None,
            search_terms: Vec::new(),
            icon: None,
            disabled: false,
        }
    }

    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn search_term(mut self, term: impl Into<SharedString>) -> Self {
        self.search_terms.push(term.into());
        self
    }

    pub fn icon(mut self, icon: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.icon = Some(Rc::new(icon));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn matches(&self, query: &str) -> bool {
        query.is_empty()
            || self.label.to_lowercase().contains(query)
            || self
                .detail
                .as_ref()
                .is_some_and(|value| value.to_lowercase().contains(query))
            || self
                .search_terms
                .iter()
                .any(|value| value.to_lowercase().contains(query))
    }
}

#[derive(Clone)]
pub struct DesktopChoiceGroup<T> {
    label: Option<SharedString>,
    choices: Vec<DesktopChoice<T>>,
}

impl<T> DesktopChoiceGroup<T> {
    pub fn new(choices: Vec<DesktopChoice<T>>) -> Self {
        Self {
            label: None,
            choices,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

#[derive(IntoElement)]
pub struct DesktopChoicePicker<T: Clone + PartialEq + 'static> {
    id: ElementId,
    label: SharedString,
    display: SharedString,
    selected: Option<T>,
    groups: Vec<DesktopChoiceGroup<T>>,
    colors: DesktopChoiceColors,
    on_select: ChoiceHandler<T>,
    search_label: SharedString,
    no_results: SharedString,
    loading_label: Option<SharedString>,
    searchable: bool,
    emit_reselect: bool,
    disabled: bool,
    loading: bool,
    width: Pixels,
    full_width: bool,
    menu_width: Pixels,
    menu_match_trigger: bool,
    max_height: Pixels,
    top_inset: Pixels,
    text_size: AbsoluteLength,
    context_key: Option<SharedString>,
    action_context: Option<FocusHandle>,
    trigger_focus: Option<FocusHandle>,
}

type ChoiceSignature<T> = Vec<(
    Option<SharedString>,
    T,
    SharedString,
    Option<SharedString>,
    Vec<SharedString>,
    bool,
)>;

struct ChoiceState<T: Clone + PartialEq + 'static> {
    open: bool,
    generation: u64,
    context_key: Option<SharedString>,
    signature: ChoiceSignature<T>,
    selected: Option<T>,
    disabled: bool,
    loading: bool,
    searchable: bool,
    trigger_focus: FocusHandle,
    scope_focus: FocusHandle,
    return_focus: Option<FocusHandle>,
    search: Entity<TextInput>,
    query: String,
    query_revision: u64,
    handles: Vec<Option<FocusHandle>>,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    scroll: ScrollHandle,
    _search_subscription: Subscription,
    _focus_subscription: Subscription,
    _activation_subscription: Subscription,
}

impl<T: Clone + PartialEq + 'static> DesktopChoicePicker<T> {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        display: impl Into<SharedString>,
        selected: Option<T>,
        groups: Vec<DesktopChoiceGroup<T>>,
        colors: DesktopChoiceColors,
        on_select: impl Fn(T, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            display: display.into(),
            selected,
            groups,
            colors,
            on_select: Rc::new(on_select),
            search_label: "Search".into(),
            no_results: "No results".into(),
            loading_label: None,
            searchable: false,
            emit_reselect: false,
            disabled: false,
            loading: false,
            width: px(220.),
            full_width: false,
            menu_width: px(240.),
            menu_match_trigger: false,
            max_height: px(240.),
            top_inset: px(8.),
            text_size: px(13.).into(),
            context_key: None,
            action_context: None,
            trigger_focus: None,
        }
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }
    pub fn emit_reselect(mut self, enabled: bool) -> Self {
        self.emit_reselect = enabled;
        self
    }
    pub fn search_label(mut self, label: impl Into<SharedString>) -> Self {
        self.search_label = label.into();
        self
    }
    pub fn no_results(mut self, label: impl Into<SharedString>) -> Self {
        self.no_results = label.into();
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn loading_label(mut self, label: impl Into<SharedString>) -> Self {
        self.loading_label = Some(label.into());
        self
    }
    pub fn action_context(mut self, focus: FocusHandle) -> Self {
        self.action_context = Some(focus);
        self
    }
    pub fn trigger_focus(mut self, focus: FocusHandle) -> Self {
        self.trigger_focus = Some(focus);
        self
    }
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width.max(px(0.));
        self.full_width = false;
        self
    }
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
    pub fn menu_width(mut self, width: Pixels) -> Self {
        self.menu_width = width.max(px(0.));
        self
    }
    pub fn menu_match_trigger(mut self, enabled: bool) -> Self {
        self.menu_match_trigger = enabled;
        self
    }
    pub fn max_height(mut self, height: Pixels) -> Self {
        self.max_height = height.max(px(0.));
        self
    }
    pub fn top_inset(mut self, inset: Pixels) -> Self {
        self.top_inset = inset.max(px(0.));
        self
    }
    pub fn text_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.text_size = size.into();
        self
    }
    pub fn context_key(mut self, key: impl Into<SharedString>) -> Self {
        self.context_key = Some(key.into());
        self
    }
}

fn dismiss<T: Clone + PartialEq + 'static>(
    state: &Entity<ChoiceState<T>>,
    window: &mut Window,
    cx: &mut App,
) {
    state.update(cx, |state, cx| {
        state.open = false;
        state.handles.clear();
        if let Some(focus) = state.return_focus.take() {
            window.focus(&focus, cx);
        }
        cx.notify();
    });
}

fn move_focus<T: Clone + PartialEq + 'static>(
    state: &Entity<ChoiceState<T>>,
    visible: &[(usize, DesktopChoice<T>)],
    delta: i32,
    edge: Option<bool>,
    searchable: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let state = state.read(cx);
    if state.loading {
        return;
    }
    let enabled: Vec<(FocusHandle, bool)> = visible
        .iter()
        .filter_map(|(index, choice)| {
            (!choice.disabled)
                .then(|| state.handles.get(*index).and_then(Option::as_ref))
                .flatten()
                .cloned()
                .map(|handle| (handle, state.selected.as_ref() == Some(&choice.value)))
        })
        .collect();
    if enabled.is_empty() {
        return;
    }
    let current = enabled
        .iter()
        .position(|(handle, _)| handle.is_focused(window));
    if searchable
        && edge.is_none()
        && ((delta < 0 && current == Some(0)) || (delta > 0 && current == Some(enabled.len() - 1)))
    {
        let search = state.search.clone();
        search.update(cx, |input, cx| input.focus(window, cx));
        return;
    }
    let next = match edge {
        Some(false) => 0,
        Some(true) => enabled.len() - 1,
        None if delta < 0 => current.map_or(enabled.len() - 1, |i| {
            (i + enabled.len() - 1) % enabled.len()
        }),
        None => current.map_or_else(
            || {
                enabled
                    .iter()
                    .position(|(_, selected)| *selected)
                    .unwrap_or(0)
            },
            |i| (i + 1) % enabled.len(),
        ),
    };
    let next = enabled[next].0.clone();
    window.focus(&next, cx);
}

fn move_tab<T: Clone + PartialEq + 'static>(
    state: &Entity<ChoiceState<T>>,
    generation: u64,
    backwards: bool,
    searchable: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let search = state.read(cx).search.clone();
    if search.read(cx).is_composing() {
        return;
    }
    if backwards && searchable && !search.read(cx).is_focused(window) {
        search.update(cx, |input, cx| input.focus(window, cx));
        return;
    }
    let trigger = state.read(cx).trigger_focus.clone();
    state.update(cx, |state, cx| {
        state.open = false;
        state.handles.clear();
        state.return_focus = None;
        window.focus(&state.trigger_focus, cx);
        cx.notify();
    });
    let state = state.clone();
    window.on_next_frame(move |window, cx| {
        if !state.read(cx).open
            && state.read(cx).generation == generation
            && trigger.is_focused(window)
        {
            if backwards {
                window.focus_prev(cx);
            } else {
                window.focus_next(cx);
            }
        }
    });
}

impl<T: Clone + PartialEq + 'static> RenderOnce for DesktopChoicePicker<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_id = ElementId::NamedChild(Arc::new(self.id.clone()), "choice-state".into());
        let scroll_id = ElementId::NamedChild(Arc::new(self.id.clone()), "choice-scroll".into());
        let search_label = self.search_label.clone();
        let external_focus = self.trigger_focus.clone();
        let state = window.use_keyed_state(state_id, cx, |window, cx| {
            let scope_focus = cx.focus_handle();
            let search = cx.new(|cx| {
                let mut input = TextInput::new(search_label.clone(), cx);
                input.set_aria_label(search_label.clone(), cx);
                input.set_accessible_role(Some(Role::SearchInput), cx);
                input.set_choice_navigation(true);
                input
            });
            let search_subscription = cx.subscribe(
                &search,
                |state: &mut ChoiceState<T>, input, event: &InputEvent, cx| {
                    if *event == InputEvent::Changed {
                        let query = input.read(cx).text().trim().to_lowercase();
                        if state.query != query {
                            state.query = query;
                            state.query_revision = state.query_revision.wrapping_add(1);
                        }
                        cx.notify();
                    }
                },
            );
            let focus_subscription = cx.on_focus_out(
                &scope_focus,
                window,
                |state: &mut ChoiceState<T>, _, window, cx| {
                    if state.open && !state.trigger_focus.is_focused(window) {
                        state.open = false;
                        state.handles.clear();
                        cx.notify();
                    }
                },
            );
            let activation_subscription =
                cx.observe_window_activation(window, |state: &mut ChoiceState<T>, window, cx| {
                    if state.open && !window.is_window_active() {
                        state.open = false;
                        state.handles.clear();
                        if let Some(focus) = state.return_focus.take() {
                            window.focus(&focus, cx);
                        }
                        cx.notify();
                    }
                });
            ChoiceState {
                open: false,
                generation: 0,
                context_key: None,
                signature: Vec::new(),
                selected: None,
                disabled: false,
                loading: false,
                searchable: false,
                trigger_focus: external_focus.unwrap_or_else(|| cx.focus_handle().tab_stop(true)),
                scope_focus,
                return_focus: None,
                search,
                query: String::new(),
                query_revision: 0,
                handles: Vec::new(),
                bounds: Rc::new(Cell::new(None)),
                scroll: ScrollHandle::new(),
                _search_subscription: search_subscription,
                _focus_subscription: focus_subscription,
                _activation_subscription: activation_subscription,
            }
        });
        let signature: ChoiceSignature<T> = self
            .groups
            .iter()
            .flat_map(|group| {
                group.choices.iter().map(|choice| {
                    (
                        group.label.clone(),
                        choice.value.clone(),
                        choice.label.clone(),
                        choice.detail.clone(),
                        choice.search_terms.clone(),
                        choice.disabled,
                    )
                })
            })
            .collect();
        let old = state.read(cx);
        let context_changed = old.context_key != self.context_key
            || self
                .trigger_focus
                .as_ref()
                .is_some_and(|focus| focus != &old.trigger_focus);
        let choices_changed = old.signature != signature;
        let changed = context_changed
            || choices_changed
            || old.selected != self.selected
            || old.disabled != self.disabled
            || old.loading != self.loading
            || (old.open && self.searchable && !old.searchable);
        let was_open = old.open;
        let focused =
            old.scope_focus.is_focused(window) || old.scope_focus.contains_focused(window, cx);
        let scope = old.scope_focus.clone();
        let trigger = old.trigger_focus.clone();
        let trigger_was_focused = trigger.is_focused(window);
        let fallback = self
            .action_context
            .clone()
            .or_else(|| old.return_focus.clone().filter(|focus| focus != &trigger));
        let old_focused = old.handles.iter().enumerate().find_map(|(index, handle)| {
            handle
                .as_ref()
                .filter(|handle| handle.is_focused(window))
                .map(|handle| (index, handle.clone()))
        });
        if changed || (was_open && self.disabled) {
            let context_key = self.context_key.clone();
            let trigger_focus = self.trigger_focus.clone();
            let selected = self.selected.clone();
            let loading = self.loading;
            let disabled = self.disabled;
            let searchable = self.searchable;
            state.update(cx, |state, cx| {
                state.context_key = context_key;
                if let Some(trigger_focus) = trigger_focus {
                    state.trigger_focus = trigger_focus;
                }
                state.selected = selected;
                state.loading = loading;
                state.disabled = disabled;
                if state.open && (context_changed || self.disabled) {
                    state.open = false;
                    state.generation = state.generation.wrapping_add(1);
                    state.handles.clear();
                    state.return_focus = None;
                } else if state.open && changed {
                    state.generation = state.generation.wrapping_add(1);
                    state.searchable |= searchable;
                    if choices_changed {
                        let previous_signature = std::mem::take(&mut state.signature);
                        let previous_handles = std::mem::take(&mut state.handles);
                        let mut used = vec![false; previous_signature.len()];
                        state.handles = signature
                            .iter()
                            .map(|choice| {
                                if choice.5 {
                                    return None;
                                }
                                let prior = previous_signature
                                    .iter()
                                    .enumerate()
                                    .find(|(index, item)| !used[*index] && item.1 == choice.1)
                                    .map(|(index, _)| index);
                                prior
                                    .and_then(|index| {
                                        used[index] = true;
                                        previous_handles.get(index).and_then(Clone::clone)
                                    })
                                    .or_else(|| Some(cx.focus_handle().tab_stop(true)))
                            })
                            .collect();
                    }
                }
                state.signature = signature;
                cx.notify();
            });
            if (was_open && (context_changed || self.disabled) && focused)
                || (self.disabled && trigger_was_focused)
            {
                if self.disabled {
                    if let Some(fallback) = fallback {
                        window.focus(&fallback, cx);
                    } else {
                        for _ in 0..256 {
                            window.focus_next(cx);
                            if !scope.is_focused(window)
                                && !scope.contains_focused(window, cx)
                                && !trigger.is_focused(window)
                            {
                                break;
                            }
                        }
                    }
                } else {
                    let trigger = state.read(cx).trigger_focus.clone();
                    window.focus(&trigger, cx);
                }
            } else if was_open
                && choices_changed
                && let Some((_, old_focus)) = old_focused
            {
                if !state
                    .read(cx)
                    .handles
                    .iter()
                    .flatten()
                    .any(|focus| focus == &old_focus)
                {
                    let scope = state.read(cx).scope_focus.clone();
                    window.focus(&scope, cx);
                    let state = state.clone();
                    let generation = state.read(cx).generation;
                    window.on_next_frame(move |window, cx| {
                        if state.read(cx).open
                            && state.read(cx).generation == generation
                            && scope.is_focused(window)
                        {
                            if state.read(cx).searchable {
                                let search = state.read(cx).search.clone();
                                search.update(cx, |input, cx| input.focus(window, cx));
                            }
                        }
                    });
                }
            }
        }
        let trigger_focus = state.read(cx).trigger_focus.clone();
        let open = state.read(cx).open;
        let bounds = state.read(cx).bounds.clone();
        let groups = Rc::new(self.groups);
        let toggle_state = state.clone();
        let toggle_groups = groups.clone();
        let searchable = self.searchable || (open && state.read(cx).searchable);
        let toggle = Rc::new(move |window: &mut Window, cx: &mut App| {
            if toggle_state.read(cx).disabled {
                return;
            }
            if toggle_state.read(cx).open {
                dismiss(&toggle_state, window, cx);
                return;
            }
            if toggle_state.read(cx).loading {
                return;
            }
            let search = toggle_state.read(cx).search.clone();
            search.update(cx, |input, cx| input.set_text("", cx));
            let previous = window.focused(cx);
            toggle_state.update(cx, |state, cx| {
                state.generation = state.generation.wrapping_add(1);
                state.return_focus = previous.or_else(|| Some(state.trigger_focus.clone()));
                state.query.clear();
                state.searchable = searchable;
                state.scroll.set_offset(point(px(0.), px(0.)));
                state.handles = toggle_groups
                    .iter()
                    .flat_map(|group| {
                        group.choices.iter().map(|choice| {
                            if choice.disabled {
                                None
                            } else {
                                Some(cx.focus_handle().tab_stop(true))
                            }
                        })
                    })
                    .collect();
                state.open = true;
                window.focus(&state.scope_focus, cx);
                cx.notify();
            });
            let state = toggle_state.clone();
            let generation = state.read(cx).generation;
            window.on_next_frame(move |window, cx| {
                if !state.read(cx).open || state.read(cx).generation != generation {
                    return;
                }
                if searchable {
                    let search = state.read(cx).search.clone();
                    search.update(cx, |input, cx| input.focus(window, cx));
                } else {
                    let selected = state.read(cx).selected.clone();
                    let first = state
                        .read(cx)
                        .signature
                        .iter()
                        .position(|choice| selected.as_ref() == Some(&choice.1))
                        .and_then(|index| state.read(cx).handles.get(index).and_then(Clone::clone));
                    if let Some(first) = first {
                        window.focus(&first, cx);
                    }
                }
            });
        });
        let colors = self.colors;
        let mut trigger = div()
            .id("choice-trigger")
            .role(Role::ComboBox)
            .aria_label(self.label.clone())
            .aria_value(self.display.clone())
            .aria_expanded(open)
            .when(self.full_width, |trigger| trigger.w_full())
            .when(!self.full_width, |trigger| trigger.w(self.width))
            .min_w_0()
            .h(px(28.))
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(9.))
            .rounded(px(5.))
            .border_1()
            .border_color(colors.border)
            .bg(colors.background)
            .text_color(colors.foreground)
            .text_size(self.text_size)
            .child(div().flex_1().min_w_0().truncate().child(self.display))
            .when(self.loading, |trigger| {
                let label = self
                    .loading_label
                    .clone()
                    .unwrap_or_else(|| crate::i18n::messages(cx).loading.clone());
                trigger.child(
                    div()
                        .id("choice-loading")
                        .role(Role::Status)
                        .aria_label(label.clone())
                        .text_color(colors.secondary)
                        .max_w(px(80.))
                        .truncate()
                        .child(label),
                )
            })
            .child(
                DesktopIcon::new(DesktopIconKind::ChevronDown)
                    .size(12.)
                    .color(colors.secondary),
            );
        if self.disabled {
            trigger = trigger.opacity(0.65);
        } else {
            let click = toggle.clone();
            let a11y = toggle.clone();
            trigger = trigger
                .track_focus(&trigger_focus)
                .tab_stop(true)
                .focus_visible(|style| style.shadow(crate::components::form::dual_focus_ring()));
            if !self.loading || open {
                trigger = trigger
                    .cursor_pointer()
                    .hover(move |style| style.bg(colors.hover))
                    .on_click(move |_, window, cx| {
                        cx.stop_propagation();
                        click(window, cx)
                    })
                    .on_a11y_action(gpui::accesskit::Action::Click, move |_, window, cx| {
                        a11y(window, cx)
                    });
            }
        }
        let mut root = div()
            .id(self.id)
            .relative()
            .when(self.full_width, |root| root.w_full())
            .child(
                Semantics::new(trigger)
                    .disabled(self.disabled)
                    .busy(self.loading),
            )
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
        let query = state.read(cx).query.clone();
        let mut visible = Vec::new();
        let mut index = 0;
        for group in groups.iter() {
            for choice in &group.choices {
                if choice.matches(&query) {
                    visible.push((index, choice.clone()));
                }
                index += 1;
            }
        }
        let visible = Rc::new(visible);
        let generation = state.read(cx).generation;
        let query_revision = state.read(cx).query_revision;
        let scope = state.read(cx).scope_focus.clone();
        let search = state.read(cx).search.clone();
        search.update(cx, |input, cx| {
            if input.placeholder() != self.search_label.as_ref() {
                input.set_placeholder(self.search_label.clone(), cx);
            }
            input.set_aria_label(self.search_label.clone(), cx);
            input.set_placeholder_color(Some(colors.secondary), cx);
        });
        let viewport = window.viewport_size();
        let estimated = px((visible.len() as f32 * 36.) + if searchable { 54. } else { 10. });
        let (anchor, position, available) = super::panels::popup_geometry(
            trigger_bounds,
            viewport,
            self.top_inset,
            estimated,
            Some(self.max_height),
        );
        let scroll = state.read(cx).scroll.clone();
        let on_select = self.on_select.clone();
        let emit_reselect = self.emit_reselect;
        let select = Rc::new({
            let state = state.clone();
            let on_select = on_select.clone();
            let rendered_query = query.clone();
            move |value: T, window: &mut Window, cx: &mut App| {
                if !state.read(cx).open
                    || state.read(cx).generation != generation
                    || state.read(cx).query != rendered_query
                    || state.read(cx).disabled
                    || state.read(cx).loading
                    || state.read(cx).search.read(cx).is_composing()
                {
                    return;
                }
                let target = state.read(cx).return_focus.clone();
                let scope = state.read(cx).scope_focus.clone();
                let unchanged = state.read(cx).selected.as_ref() == Some(&value);
                state.update(cx, |state, cx| {
                    state.open = false;
                    state.handles.clear();
                    cx.notify();
                });
                if !unchanged || emit_reselect {
                    on_select(value, window, cx);
                }
                if (scope.is_focused(window) || scope.contains_focused(window, cx))
                    && let Some(target) = target
                {
                    window.focus(&target, cx);
                }
            }
        });
        let is_current = move |state: &Entity<ChoiceState<T>>, cx: &App| {
            state.read(cx).open
                && state.read(cx).generation == generation
                && state.read(cx).query_revision == query_revision
        };
        let mut menu = div()
            .id("choice-popup")
            .key_context("WeuiDesktopChoice")
            .track_focus(&scope)
            .tab_group()
            .w((if self.menu_match_trigger {
                trigger_bounds.size.width
            } else {
                self.menu_width
            })
            .min((viewport.width - px(16.)).max(px(0.))))
            .max_h(available.min((viewport.height - px(16.)).max(px(0.))))
            .overflow_y_scroll()
            .track_scroll(&scroll)
            .occlude()
            .p(px(5.))
            .rounded(px(7.))
            .border_1()
            .border_color(colors.border)
            .bg(colors.background)
            .text_color(colors.foreground)
            .shadow(vec![BoxShadow {
                color: black().opacity(0.2),
                offset: point(px(0.), px(6.)),
                blur_radius: px(24.),
                spread_radius: px(0.),
                inset: false,
            }])
            .on_action({
                let state = state.clone();
                move |_: &DismissChoice, window, cx| {
                    if !state.read(cx).open {
                        cx.propagate();
                        return;
                    }
                    if !state.read(cx).search.read(cx).is_composing() {
                        dismiss(&state, window, cx);
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &NextChoice, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, 1, None, searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &PreviousChoice, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, -1, None, searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &FirstChoice, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, 1, Some(false), searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &LastChoice, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, 1, Some(true), searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &crate::input::Down, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, 1, None, searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                move |_: &crate::input::Up, window, cx| {
                    if is_current(&state, cx) {
                        move_focus(&state, &visible, -1, None, searchable, window, cx);
                    } else if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                let visible = visible.clone();
                let select = select.clone();
                move |_: &ConfirmChoice, window, cx| {
                    if !is_current(&state, cx) {
                        if !state.read(cx).open {
                            cx.propagate();
                        }
                        return;
                    }
                    let value = visible
                        .iter()
                        .find(|(index, choice)| {
                            !choice.disabled
                                && state
                                    .read(cx)
                                    .handles
                                    .get(*index)
                                    .and_then(Option::as_ref)
                                    .is_some_and(|focus| focus.is_focused(window))
                        })
                        .map(|(_, choice)| choice.value.clone());
                    if let Some(value) = value {
                        select(value, window, cx);
                    }
                }
            })
            .on_action({
                let state = state.clone();
                move |_: &crate::input::Enter, _, cx| {
                    if !state.read(cx).open {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                move |_: &NextChoiceFocus, window, cx| {
                    if state.read(cx).open {
                        let generation = state.read(cx).generation;
                        let searchable = state.read(cx).searchable;
                        move_tab(&state, generation, false, searchable, window, cx);
                    } else {
                        cx.propagate();
                    }
                }
            })
            .on_action({
                let state = state.clone();
                move |_: &PreviousChoiceFocus, window, cx| {
                    if state.read(cx).open {
                        let generation = state.read(cx).generation;
                        let searchable = state.read(cx).searchable;
                        move_tab(&state, generation, true, searchable, window, cx);
                    } else {
                        cx.propagate();
                    }
                }
            })
            .on_key_down({
                let state = state.clone();
                let visible = visible.clone();
                let select = select.clone();
                move |event, window, cx| {
                    if !is_current(&state, cx) {
                        return;
                    }
                    let key = event.keystroke.key.as_str();
                    if key == "space" && !event.keystroke.modifiers.modified() {
                        let value = visible
                            .iter()
                            .find(|(index, choice)| {
                                !choice.disabled
                                    && state
                                        .read(cx)
                                        .handles
                                        .get(*index)
                                        .and_then(Option::as_ref)
                                        .is_some_and(|focus| focus.is_focused(window))
                            })
                            .map(|(_, choice)| choice.value.clone());
                        if let Some(value) = value {
                            select(value, window, cx);
                            window.prevent_default();
                            cx.stop_propagation();
                        }
                    }
                }
            })
            .on_mouse_down_out({
                let state = state.clone();
                let bounds = bounds.clone();
                let scope = scope.clone();
                move |event, window, cx| {
                    if !state.read(cx).open
                        || bounds
                            .get()
                            .is_some_and(|bounds| bounds.contains(&event.position))
                    {
                        return;
                    }
                    let generation = state.read(cx).generation;
                    let target = state
                        .read(cx)
                        .return_focus
                        .clone()
                        .unwrap_or_else(|| state.read(cx).trigger_focus.clone());
                    let handles = state.read(cx).handles.clone();
                    state.update(cx, |state, cx| {
                        state.open = false;
                        state.handles.clear();
                        cx.notify();
                    });
                    let state = state.clone();
                    let scope = scope.clone();
                    window.on_next_frame(move |window, cx| {
                        if !state.read(cx).open
                            && state.read(cx).generation == generation
                            && (scope.is_focused(window)
                                || scope.contains_focused(window, cx)
                                || handles
                                    .iter()
                                    .flatten()
                                    .any(|focus| focus.is_focused(window)))
                        {
                            window.focus(&target, cx);
                        }
                    });
                }
            });
        if searchable {
            menu = menu.child(
                div()
                    .id("choice-search-wrap")
                    .px(px(8.))
                    .py(px(7.))
                    .text_size(self.text_size)
                    .text_color(colors.foreground)
                    .child(search),
            );
        }
        let mut list = div()
            .id("choice-list")
            .role(Role::ListBox)
            .aria_label(self.label)
            .flex()
            .flex_col();
        if visible.is_empty() {
            let notice = if self.loading {
                self.loading_label
                    .clone()
                    .unwrap_or_else(|| crate::i18n::messages(cx).loading.clone())
            } else {
                self.no_results.clone()
            };
            list = list.child(
                Semantics::new(
                    div()
                        .id("choice-empty")
                        .role(Role::Status)
                        .aria_label(notice.clone())
                        .px(px(10.))
                        .py(px(8.))
                        .text_color(colors.secondary)
                        .child(notice),
                )
                .read_only(true),
            );
        }
        let mut flat = 0;
        let mut option_index = 0;
        for group in groups.iter() {
            let count = group
                .choices
                .iter()
                .filter(|choice| choice.matches(&query))
                .count();
            let mut group_rows = div().id(("choice-group", flat)).flex().flex_col();
            if count > 0
                && let Some(label) = &group.label
            {
                group_rows = group_rows
                    .role(Role::Group)
                    .aria_label(label.clone())
                    .child(
                        Semantics::new(
                            div()
                                .id(("choice-group-label", flat))
                                .role(Role::Label)
                                .aria_label(label.clone())
                                .px(px(10.))
                                .py(px(6.))
                                .text_color(colors.secondary)
                                .child(label.clone()),
                        )
                        .read_only(true),
                    );
            }
            for choice in &group.choices {
                let row_index = flat;
                flat += 1;
                if !choice.matches(&query) {
                    continue;
                }
                let option_position = option_index;
                option_index += 1;
                let selected = self
                    .selected
                    .as_ref()
                    .is_some_and(|selected| selected == &choice.value);
                let icon = choice.icon.as_ref().map(|icon| icon(window, cx));
                let name = match choice.detail.as_ref() {
                    Some(detail) if !detail.is_empty() => {
                        SharedString::from(format!("{}. {}", choice.label, detail))
                    }
                    _ => choice.label.clone(),
                };
                let mut row = div()
                    .id(("choice-option", row_index))
                    .role(Role::ListBoxOption)
                    .aria_label(name)
                    .aria_selected(selected)
                    .aria_position_in_set(option_position)
                    .aria_size_of_set(visible.len())
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .px(px(10.))
                    .py(px(8.))
                    .rounded(px(5.))
                    .text_size(self.text_size)
                    .child(div().w(px(20.)).flex_shrink_0().children(icon))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(div().truncate().child(choice.label.clone()))
                            .when_some(choice.detail.clone(), |row, detail| {
                                row.child(
                                    div().text_color(colors.secondary).truncate().child(detail),
                                )
                            }),
                    )
                    .when(selected, |row| {
                        row.child(DesktopIcon::new(DesktopIconKind::Check).size(15.))
                    });
                if let Some(handle) = state.read(cx).handles.get(row_index).and_then(Clone::clone) {
                    let value = choice.value.clone();
                    let click = select.clone();
                    let a11y = select.clone();
                    row = row
                        .track_focus(&handle)
                        .tab_stop(true)
                        .focus_visible(|style| {
                            style.shadow(crate::components::form::dual_focus_ring())
                        });
                    if !self.loading {
                        row = row
                            .cursor_pointer()
                            .hover(move |style| style.bg(colors.hover))
                            .on_click({
                                let value = value.clone();
                                move |_, window, cx| {
                                    cx.stop_propagation();
                                    click(value.clone(), window, cx)
                                }
                            })
                            .on_a11y_action(
                                gpui::accesskit::Action::Click,
                                move |_, window, cx| a11y(value.clone(), window, cx),
                            );
                    } else {
                        row = row.opacity(0.65);
                    }
                } else {
                    row = row.opacity(0.65);
                }
                group_rows = group_rows.child(
                    Semantics::new(row)
                        .disabled(choice.disabled || self.loading)
                        .reveal_focus(),
                );
            }
            if count > 0 {
                list = list.child(group_rows);
            }
        }
        menu = menu.child(Semantics::new(list).busy(self.loading));
        root = root.child(
            deferred(
                anchored()
                    .anchor(anchor)
                    .position(position)
                    .snap_to_window_with_margin(px(8.))
                    .child(FocusScroll::new(scroll_id, &scroll, menu)),
            )
            .with_priority(1),
        );
        root
    }
}
