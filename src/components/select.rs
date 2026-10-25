//! WeUI select cells (`src/style/widget/weui-cell/weui-form/weui-select.less`, examples
//! `src/example/form/form_select.html` and `form_select_primary.html`). [`SelectCell`] is the
//! `.weui-cell_select` trigger row and [`SelectMenu`] the option sheet it opens.
//! Known limitations: the native HTML `<select>` popup is replaced by a bottom-sheet overlay
//! that the caller opens and closes through a [`Presence`]; the cell never opens it by itself.
use crate::scrolling::RevealFocus;

use super::{
    ClickHandler, Icon,
    form::{LABEL_MAX_WIDTH, LABEL_WIDTH, form_label},
    modal::{MaskKind, Modal, Motion, Placement, Presence},
};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Arrangement of a [`SelectCell`], after the `weui-cell_select*` modifiers. Chosen with
/// [`SelectCell::before`] and [`SelectCell::after`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectLayout {
    /// `.weui-cell_select`: the whole row is the trigger and shows the value. The default.
    #[default]
    Plain,
    /// `.weui-cell_select-before`: a narrow trigger on the left, then caller content (e.g. a
    /// phone-number input) after a vertical hairline.
    Before,
    /// `.weui-cell_select-after`: a form label, then the value; the whole row is the trigger.
    After,
}

fn arrow(cx: &App) -> impl IntoElement {
    Icon::new("arrow").w(12.).h(24.).color(Theme::get(cx).fg_2)
}

/// A select trigger row: the current value on a 56px row with a trailing `fg_2` arrow, exposed
/// as a focusable `Role::ComboBox` carrying the value and expanded state. It only reports
/// activation; the caller opens a [`SelectMenu`] (or a picker) and reports it via `expanded`.
#[derive(IntoElement)]
pub struct SelectCell {
    id: ElementId,
    layout: SelectLayout,
    value: SharedString,
    label: Option<SharedString>,
    label_width: f32,
    body: Option<AnyElement>,
    expanded: bool,
    in_primary: bool,
    on_click: Option<ClickHandler>,
}

impl SelectCell {
    /// Creates a collapsed [`SelectLayout::Plain`] row showing `value`, which is also its
    /// accessible value and name, with a [`LABEL_WIDTH`] label width and no click handler.
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            layout: SelectLayout::Plain,
            value: value.into(),
            label: None,
            label_width: LABEL_WIDTH,
            body: None,
            expanded: false,
            in_primary: false,
            on_click: None,
        }
    }
    /// Switches to [`SelectLayout::Before`]: the trigger shrinks to the label width (capped at
    /// [`LABEL_MAX_WIDTH`]) on the left and `body` fills the rest of the row, outside the trigger.
    pub fn before(mut self, body: impl IntoElement) -> Self {
        self.layout = SelectLayout::Before;
        self.body = Some(body.into_any_element());
        self
    }
    /// Switches to [`SelectLayout::After`] with `label` as the visible form label; the accessible
    /// name becomes the label followed by the value.
    pub fn after(mut self, label: impl Into<SharedString>) -> Self {
        self.layout = SelectLayout::After;
        self.label = Some(label.into());
        self
    }
    /// Sets the label width in logical pixels: the label column for `after`, or the trigger's
    /// value width for `before`, both capped at [`LABEL_MAX_WIDTH`]. Default [`LABEL_WIDTH`].
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }
    /// Exposes whether the menu this cell opened is showing (`aria-expanded`). Default `false`.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
    /// Squares the pressed highlight's corners (0 instead of 8px) for rows inside a primary form
    /// group.
    pub fn in_primary_group(mut self) -> Self {
        self.in_primary = true;
        self
    }
    /// Calls `handler` when the trigger is clicked or activated with Enter/Space, typically to
    /// show a [`SelectMenu`]. Without it the trigger is still focusable but does nothing.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for SelectCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let pressed = t.fg_3;
        let brand = t.brand;
        let radius = if self.in_primary { 0. } else { 8. };
        let label_text = self
            .label
            .clone()
            .map(|l| SharedString::from(format!("{l} {}", self.value)))
            .unwrap_or_else(|| self.value.clone());
        let value = div()
            .min_h(px(52.))
            .line_height(px(52.))
            .whitespace_nowrap()
            .overflow_hidden()
            .text_ellipsis()
            .text_color(t.fg_0)
            .child(self.value.clone());
        let trigger = |el: Stateful<Div>, handler: Option<ClickHandler>| {
            let el = el
                .role(Role::ComboBox)
                .aria_label(label_text.clone())
                .aria_value(self.value.clone())
                .aria_expanded(self.expanded)
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .border_2()
                .border_color(transparent_black())
                .focus_visible(move |s| s.border_color(brand))
                .active(move |s| s.bg(pressed).rounded(px(radius)));
            match handler {
                Some(h) => el.on_click(h),
                None => el,
            }
        };
        match self.layout {
            SelectLayout::Plain => trigger(
                div()
                    .id(self.id)
                    .relative()
                    .flex()
                    .items_center()
                    .text_size(px(17.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(14.))
                            .pr(px(38.))
                            .child(value),
                    )
                    .child(
                        div()
                            .absolute()
                            .right(px(14.))
                            .top(px(14.))
                            .child(arrow(cx)),
                    ),
                self.on_click,
            )
            .into_any_element(),
            SelectLayout::After => trigger(
                div()
                    .id(self.id)
                    .relative()
                    .flex()
                    .items_center()
                    .text_size(px(17.))
                    .line_height(relative(24. / 17.))
                    .child(
                        div()
                            .pl(px(14.))
                            .pr(px(16.))
                            .text_color(t.fg_0)
                            .child(form_label(self.label.unwrap_or_default(), self.label_width)),
                    )
                    .child(div().flex_1().min_w_0().pr(px(38.)).child(value))
                    .child(
                        div()
                            .absolute()
                            .right(px(14.))
                            .top(px(14.))
                            .child(arrow(cx)),
                    ),
                self.on_click,
            )
            .into_any_element(),
            SelectLayout::Before => {
                let w = self.label_width.min(LABEL_MAX_WIDTH);
                div()
                    .id(self.id)
                    .relative()
                    .flex()
                    .items_center()
                    .text_size(px(17.))
                    .line_height(relative(24. / 17.))
                    .child(trigger(
                        div()
                            .id("select-before")
                            .relative()
                            .flex_shrink_0()
                            .child(div().w(px(w + 52.)).pl(px(14.)).pr(px(38.)).child(value))
                            .child(
                                div()
                                    .absolute()
                                    .right(px(14.))
                                    .top(px(14.))
                                    .child(arrow(cx)),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .top(px(-2.))
                                    .bottom(px(-2.))
                                    .right(px(-2.))
                                    .w(px(0.5))
                                    .bg(t.fg_3),
                            ),
                        self.on_click,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .pl(px(16.))
                            .pr(px(16.))
                            .children(self.body),
                    )
                    .into_any_element()
            }
        }
    }
}

type PickHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;
type DismissHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A bottom-sheet option list replacing the native select popup: an optional title, a
/// `Role::ListBox` of options (the selected one in brand colour with a tick), and a "取消"
/// button, in a dark-masked [`Modal`] that slides up.
///
/// Visibility is the caller's [`Presence`]; the menu never hides itself, so hide it from
/// `on_select` and `on_dismiss`. On opening, focus moves to the selected (or first enabled)
/// option; Up/Down/Home/End move between enabled options without wrapping.
#[derive(IntoElement)]
pub struct SelectMenu {
    id: ElementId,
    presence: Presence,
    title: Option<SharedString>,
    options: Vec<(SharedString, bool)>,
    selected: Option<usize>,
    on_select: Option<PickHandler>,
    on_dismiss: Option<DismissHandler>,
}

impl SelectMenu {
    /// Creates an empty menu bound to a clone of `presence`, so rebuild it each render to see
    /// the current state. `id` keys the modal and the option focus handles in window state.
    pub fn new(id: impl Into<ElementId>, presence: &Presence) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            title: None,
            options: vec![],
            selected: None,
            on_select: None,
            on_dismiss: None,
        }
    }
    /// Sets the title row above the options (12px, `fg_1`), also used as the modal's accessible
    /// label. Without it there is no title row and the label is "选择".
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Appends an enabled option; `label` is its text and accessible name.
    pub fn option(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push((label.into(), false));
        self
    }
    /// Appends a disabled option: `fg_2` text, flagged disabled, not focusable or selectable,
    /// and skipped by the arrow keys.
    pub fn disabled_option(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push((label.into(), true));
        self
    }
    /// Appends several enabled options in order.
    pub fn options<S: Into<SharedString>>(mut self, labels: impl IntoIterator<Item = S>) -> Self {
        self.options
            .extend(labels.into_iter().map(|l| (l.into(), false)));
        self
    }
    /// Marks the option at `index` as selected (brand colour, tick, `aria-selected`) and makes
    /// it the initial focus target if enabled. Default `None`.
    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }
    /// Calls `handler` with the index of an enabled option when it is clicked or activated with
    /// Enter/Space. The menu stays open until the caller hides its [`Presence`].
    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    /// Calls `handler` when the user cancels with the "取消" button, a mask click or Escape. The
    /// caller should hide the [`Presence`]; nothing closes otherwise.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for SelectMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let pressed = t.bg_color_active;
        let brand = t.brand;
        let n = self.options.len();
        let handles = window.use_keyed_state(self.id.clone(), cx, |_, _| Vec::<FocusHandle>::new());
        let handles = handles.update(cx, |v, cx| {
            while v.len() < n {
                v.push(cx.focus_handle().tab_stop(true));
            }
            v.clone()
        });
        if self.presence.is_open() && self.presence.focus_handle().is_focused(window) {
            let target = self
                .selected
                .filter(|i| self.options.get(*i).is_some_and(|(_, d)| !d))
                .or_else(|| self.options.iter().position(|(_, d)| !d));
            if let Some(h) = target.map(|i| handles[i].clone()) {
                window.defer(cx, move |window, cx| window.focus(&h, cx));
            }
        }
        let enabled: Rc<Vec<bool>> = Rc::new(self.options.iter().map(|(_, d)| !d).collect());
        let option_handles = Rc::new(handles.clone());
        let dismiss = self.on_dismiss.clone();
        let rows = self
            .options
            .into_iter()
            .enumerate()
            .map(|(i, (label, disabled))| {
                let selected = self.selected == Some(i);
                let on_select = self.on_select.clone();
                let el = div()
                    .id(ElementId::NamedInteger("select-option".into(), i as u64))
                    .role(Role::ListBoxOption)
                    .aria_label(label.clone())
                    .aria_selected(selected)
                    .aria_position_in_set(i + 1)
                    .aria_size_of_set(n)
                    .relative()
                    .h(px(56.))
                    .px(px(22.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_2()
                    .border_color(transparent_black())
                    .text_size(px(17.))
                    .text_color(if disabled {
                        t.fg_2
                    } else if selected {
                        t.brand
                    } else {
                        t.fg_0
                    })
                    .when(i > 0, |el| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(-2.))
                                .left_0()
                                .right_0()
                                .h(px(0.5))
                                .bg(t.fg_3),
                        )
                    })
                    .child(label)
                    .when(selected, |el| {
                        el.child(
                            div()
                                .absolute()
                                .right(px(14.))
                                .top(px(14.))
                                .child(Icon::new("checked").size(24.).color(t.brand)),
                        )
                    });
                let el = if disabled {
                    el
                } else {
                    el.track_focus(&handles[i])
                        .focusable()
                        .tab_stop(true)
                        .cursor_pointer()
                        .active(move |s| s.bg(pressed))
                        .focus_visible(move |s| s.border_color(brand))
                        .on_key_down({
                            let enabled = enabled.clone();
                            let option_handles = option_handles.clone();
                            move |event, window, cx| {
                                let next = match event.keystroke.key.as_str() {
                                    "down" => (i + 1..n).find(|j| enabled[*j]),
                                    "up" => (0..i).rev().find(|j| enabled[*j]),
                                    "home" => (0..n).find(|j| enabled[*j]),
                                    "end" => (0..n).rev().find(|j| enabled[*j]),
                                    _ => return,
                                };
                                cx.stop_propagation();
                                if let Some(j) = next {
                                    window.focus(&option_handles[j], cx);
                                }
                            }
                        })
                        .on_click(move |_, window, cx| {
                            if let Some(h) = on_select.as_ref() {
                                h(&i, window, cx);
                            }
                        })
                };
                Semantics::new(el).disabled(disabled).reveal_focus()
            });
        let cancel_dismiss = dismiss.clone();
        let mut modal = Modal::new(self.id, &self.presence)
            .mask(MaskKind::Dark)
            .placement(Placement::Bottom)
            .motion(Motion::SlideUp)
            .label(self.title.clone().unwrap_or_else(|| "选择".into()));
        if let Some(d) = dismiss {
            modal = modal.on_dismiss(move |w, cx| d(w, cx));
        }
        modal.child(
            div()
                .w_full()
                .rounded_t(px(12.))
                .overflow_hidden()
                .bg(t.bg_2)
                .line_height(relative(1.4))
                .when_some(self.title, |el, title| {
                    el.child(
                        div()
                            .h(px(56.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.))
                            .text_color(t.fg_1)
                            .border_b(px(0.5))
                            .border_color(t.fg_3)
                            .child(title),
                    )
                })
                .child(
                    div()
                        .id("select-listbox")
                        .role(Role::ListBox)
                        .flex()
                        .flex_col()
                        .children(rows),
                )
                .child(div().h(px(8.)).bg(t.bg_0))
                .child(
                    div()
                        .id("select-cancel")
                        .role(Role::Button)
                        .aria_label("取消")
                        .h(px(56.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_2()
                        .border_color(transparent_black())
                        .text_size(px(17.))
                        .text_color(t.fg_0)
                        .focusable()
                        .tab_stop(true)
                        .cursor_pointer()
                        .active(move |s| s.bg(pressed))
                        .focus_visible(move |s| s.border_color(brand))
                        .on_click(move |_, w, cx| {
                            if let Some(d) = cancel_dismiss.as_ref() {
                                d(w, cx);
                            }
                        })
                        .child("取消"),
                ),
        )
    }
}
