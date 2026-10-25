//! WeUI swiped cell (`weui-cell/weui-cell_swiped.less`): a list row that slides left to reveal
//! action buttons (`weui-swiped-btn`).
//!
//! [`SwipedCell`] keeps its open state in window element state keyed by its id; [`SwipedAction`]s
//! are its buttons. Closed cells hide their actions from accessibility and the Tab order, and the
//! row supports the Expand and Collapse accessible actions. Optional controlled state and change
//! requests let applications coordinate open rows. Known limitation: the upstream list
//! example is static, so the drag thresholds and 200 ms snap are this crate's own choices.

use super::{ClickHandler, modal::ease};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

const SNAP: Duration = Duration::from_millis(200);

/// Background colour of a [`SwipedAction`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SwipedActionKind {
    /// Neutral background (`weui-swiped-btn_default`).
    Default,
    /// Warning red background (`weui-swiped-btn_warn`); the default.
    #[default]
    Warn,
}

/// Button revealed behind a [`SwipedCell`]. While the cell is closed it is hidden from
/// accessibility and left out of the Tab order.
#[derive(IntoElement)]
pub struct SwipedAction {
    id: ElementId,
    label: SharedString,
    kind: SwipedActionKind,
    tab_stop: bool,
    focus: Option<FocusHandle>,
    on_click: Option<ClickHandler>,
}

impl SwipedAction {
    /// Creates a warn-coloured action showing `label`.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: SwipedActionKind::Warn,
            tab_stop: true,
            focus: None,
            on_click: None,
        }
    }
    /// Uses [`SwipedActionKind::Warn`], the default.
    pub fn warn(mut self) -> Self {
        self.kind = SwipedActionKind::Warn;
        self
    }
    /// Uses [`SwipedActionKind::Default`].
    pub fn default_kind(mut self) -> Self {
        self.kind = SwipedActionKind::Default;
        self
    }
    /// Called with the click event when the action is clicked or activated with Enter or Space.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for SwipedAction {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let bg = match self.kind {
                SwipedActionKind::Default => t.bg_0,
                SwipedActionKind::Warn => t.red,
            };
            // The ring sits on the button edge, next to the row background: a white ring on a
            // red button in a white list read as a 2px narrower button, not as focus.
            let preferred = match self.kind {
                SwipedActionKind::Default => t.brand,
                SwipedActionKind::Warn => t.fg_0,
            };
            let ring = t.focus_color_with(preferred, bg);
            let mask = t.btn_active_mask;
            let exposed = self.tab_stop;
            let el = div()
                .id(self.id)
                .role(Role::Button)
                .aria_label(self.label.clone())
                .flex()
                .items_center()
                .py(px(14.))
                .px(px(15.))
                .border_2()
                .border_color(transparent_black())
                .bg(bg)
                .text_size(px(17.))
                .line_height(px(24.))
                .active(move |s| s.bg(bg.blend(mask)))
                .when(self.tab_stop, |el| {
                    el.when_some(self.focus, |el, focus| el.track_focus(&focus))
                        .focusable()
                        .tab_stop(true)
                        .focus_visible(move |s| s.border_color(ring))
                })
                .when_some(self.on_click, |el, h| el.on_click(h))
                .child(self.label);
            Semantics::new(el).hidden(!exposed)
        })
    }
}

struct SwipeState {
    offset: f32,
    width: f32,
    open: bool,
    drag_from: Option<(f32, f32)>,
    moved: bool,
    anim: Option<(f32, f32, Instant)>,
    focus: FocusHandle,
    scope_focus: FocusHandle,
}

impl SwipeState {
    fn snap_to(&mut self, open: bool, reduce: bool) {
        self.open = open;
        let to = if open { -self.width } else { 0. };
        if reduce || self.offset == to {
            self.offset = to;
            self.anim = None;
        } else {
            self.anim = Some((self.offset, to, Instant::now()));
        }
    }
}

type OpenHandler = std::rc::Rc<dyn Fn(&bool, &mut Window, &mut App)>;

#[derive(Clone)]
struct SwipeControl {
    state: Entity<SwipeState>,
    controlled: Option<bool>,
    on_change: Option<OpenHandler>,
}

impl SwipeControl {
    fn request(&self, open: bool, window: &mut Window, cx: &mut App) {
        let restore = {
            let state = self.state.read(cx);
            !self.controlled.unwrap_or(open)
                && state.open
                && state.scope_focus.contains_focused(window, cx)
        };
        let changed = self.state.update(cx, |state, cx| {
            let changed = state.open != open;
            state.drag_from = None;
            state.snap_to(self.controlled.unwrap_or(open), cx.reduce_motion());
            cx.notify();
            changed
        });
        if restore {
            window.focus(&self.state.read(cx).focus.clone(), cx);
        }
        if changed && let Some(handler) = self.on_change.as_ref() {
            handler(&open, window, cx);
        }
    }

    fn release(&self, window: &mut Window, cx: &mut App) {
        let requested = self.state.read(cx).drag_from.map(|(_, base)| {
            let state = self.state.read(cx);
            if !state.moved {
                if base < 0. { false } else { state.open }
            } else {
                state.offset < -state.width / 2.
            }
        });
        if let Some(open) = requested {
            self.request(open, window, cx);
        }
    }
}

#[derive(Clone)]
struct SwipeDrag;

impl Render for SwipeDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// List row whose content slides left to expose its actions, with expanded state and a ListItem
/// role. Without actions it is a static row, without an expanded state or extra Tab stop.
///
/// Dragging left past half the actions' width and releasing requests opening; clicking an open
/// row requests closing. Left opens, Right or Escape closes (including from a focused action),
/// and Enter/Space on the row toggles. Tab from an open row reaches its first action; Shift+Tab
/// returns to the row. Closing with Right/Escape returns focus to the row.
/// The accessibility default Click action toggles too; explicit Expand/Collapse request their
/// corresponding state. Snaps take 200 ms with CSS `ease`, or are instant under reduced motion.
#[derive(IntoElement)]
pub struct SwipedCell {
    id: ElementId,
    content: Option<AnyElement>,
    actions: Vec<SwipedAction>,
    default_open: bool,
    controlled: Option<bool>,
    on_change: Option<OpenHandler>,
    label: Option<SharedString>,
}

impl SwipedCell {
    /// Creates a closed cell with no content or actions.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: None,
            actions: vec![],
            default_open: false,
            controlled: None,
            on_change: None,
            label: None,
        }
    }
    /// Sets the row content that slides over the actions.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    /// Appends an action; actions sit left to right at the row's right edge.
    pub fn action(mut self, action: SwipedAction) -> Self {
        self.actions.push(action);
        self
    }
    /// Starts an uncontrolled cell open when its state is first created; later changes are
    /// ignored. Default false. [`Self::open`] takes precedence when supplied.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    /// Sets the controlled open state on every render. Input requests go to
    /// [`Self::on_open_change`]; the caller must update this value to accept them. Without this
    /// setter, the cell manages its own state and still reports changes through that callback.
    pub fn open(mut self, open: bool) -> Self {
        self.controlled = Some(open);
        self
    }
    /// Reports a requested open/closed change from pointer, keyboard or accessibility actions.
    /// Repeated requests matching the current state do not call it. In controlled mode, ignored
    /// requests leave the supplied state unchanged. No callback occurs merely from rendering.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
    /// Sets the row's accessible name (default none).
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for SwipedCell {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.actions.is_empty() {
            return crate::scrolling::FocusTarget::new(
                div()
                    .id(self.id)
                    .role(Role::ListItem)
                    .bg(Theme::get(cx).bg_2)
                    .when_some(self.label, |el, label| el.aria_label(label))
                    .children(self.content),
            )
            .into_any_element();
        }
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let (bg, ring) = (t.bg_2, t.focus_color(t.bg_2));
            let initially_open = self.controlled.unwrap_or(self.default_open);
            let state = window.use_keyed_state(
                SharedString::from(format!("swiped-{:?}", self.id)),
                cx,
                |_, cx| SwipeState {
                    offset: 0.,
                    width: 0.,
                    open: initially_open,
                    drag_from: None,
                    moved: false,
                    anim: None,
                    focus: cx.focus_handle().tab_stop(true),
                    scope_focus: cx.focus_handle().tab_stop(false),
                },
            );
            let restore = self.controlled == Some(false)
                && state.read(cx).open
                && state.read(cx).scope_focus.contains_focused(window, cx);
            if restore {
                window.focus(&state.read(cx).focus.clone(), cx);
            }
            let reduce = cx.reduce_motion();
            let offset = state.update(cx, |s, _| {
                if let Some(open) = self.controlled
                    && s.open != open
                {
                    s.snap_to(open, reduce);
                }
                if let Some((from, to, at)) = s.anim {
                    let p = (at.elapsed().as_secs_f32() / SNAP.as_secs_f32()).min(1.);
                    s.offset = from + (to - from) * ease(p);
                    if p >= 1. {
                        s.anim = None;
                    }
                }
                s.offset
            });
            if state.read(cx).anim.is_some() {
                window.request_animation_frame();
            }
            let focus = state.read(cx).focus.clone();
            let show_ring = focus.is_focused(window) && window.last_input_was_keyboard();
            let open = state.read(cx).open;
            let scope_focus = state.read(cx).scope_focus.clone();
            let scope_id =
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "scope".into());
            let control = SwipeControl {
                state: state.clone(),
                controlled: self.controlled,
                on_change: self.on_change,
            };
            let width_state = state.clone();
            let action_focus: Vec<_> = self
                .actions
                .iter()
                .map(|action| {
                    window
                        .use_keyed_state(
                            SharedString::from(format!(
                                "swiped-action-focus-{:?}-{:?}",
                                self.id, action.id
                            )),
                            cx,
                            |_, cx| cx.focus_handle(),
                        )
                        .read(cx)
                        .clone()
                })
                .collect();
            let first_action = action_focus.first().cloned();
            let last_action = action_focus.last().cloned();
            let ft = div()
                .absolute()
                .top_0()
                .bottom_0()
                .right_0()
                .flex()
                .text_color(white())
                .on_children_prepainted(move |bounds, _, cx| {
                    let width: f32 = bounds.iter().map(|b| f32::from(b.size.width)).sum();
                    width_state.update(cx, |s, cx| {
                        if (s.width - width).abs() > 0.5 {
                            s.width = width;
                            if s.drag_from.is_none() {
                                let to = if s.open { -width } else { 0. };
                                if s.anim.is_some() {
                                    s.anim = Some((s.offset, to, Instant::now()));
                                } else {
                                    s.offset = to;
                                }
                            }
                            cx.notify();
                        }
                    });
                })
                .children(self.actions.into_iter().zip(action_focus).map(
                    move |(mut action, focus)| {
                        action.tab_stop = open;
                        action.focus = Some(focus);
                        action
                    },
                ));
            let down = state.clone();
            let movement = state.clone();
            let up = control.clone();
            let outside = control.clone();
            let click = control.clone();
            let accessible_click = control.clone();
            let expand = control.clone();
            let collapse = control.clone();
            let collapse_focus = focus.clone();
            let key_focus = focus.clone();
            let bd = div()
                .id(self.id.clone())
                .role(Role::ListItem)
                .when_some(self.label, |el, label| el.aria_label(label))
                .aria_expanded(open)
                .aria_description(if open {
                    "操作已展开，按右方向键或 Esc 隐藏"
                } else {
                    "操作已收起，按左方向键或回车显示操作"
                })
                .relative()
                .left(px(offset))
                .bg(bg)
                .track_focus(&focus)
                .tab_stop(true)
                .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                    let x = f32::from(event.position.x);
                    down.update(cx, |s, _| {
                        s.anim = None;
                        s.drag_from = Some((x, s.offset));
                        s.moved = false;
                    });
                })
                .on_drag(SwipeDrag, |_, _, _, cx| cx.new(|_| SwipeDrag))
                .on_drag_move::<SwipeDrag>(move |event, _, cx| {
                    let x = f32::from(event.event.position.x);
                    movement.update(cx, |s, cx| {
                        if let Some((start, base)) = s.drag_from {
                            let dx = x - start;
                            if dx.abs() > 3. {
                                s.moved = true;
                            }
                            let raw = base + dx;
                            s.offset = if raw > 0. {
                                0.
                            } else if raw < -s.width {
                                -s.width + (raw + s.width) * 0.3
                            } else {
                                raw
                            };
                            cx.notify();
                        }
                    });
                })
                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                    up.release(window, cx)
                })
                .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                    outside.release(window, cx)
                })
                .on_click(move |event, window, cx| {
                    if event.is_keyboard() {
                        click.request(!click.state.read(cx).open, window, cx);
                    }
                })
                .on_a11y_action(AccessibleAction::Expand, move |_, window, cx| {
                    expand.request(true, window, cx);
                })
                .on_a11y_action(AccessibleAction::Click, move |_, window, cx| {
                    let open = !accessible_click.state.read(cx).open;
                    accessible_click.request(open, window, cx);
                    window.focus(&accessible_click.state.read(cx).focus.clone(), cx);
                })
                .on_a11y_action(AccessibleAction::Collapse, move |_, window, cx| {
                    collapse.request(false, window, cx);
                    window.focus(&collapse_focus, cx);
                })
                .children(self.content)
                .when(show_ring, |el| {
                    el.child(div().absolute().inset_0().border_2().border_color(ring))
                });
            div()
                .id(scope_id)
                .track_focus(&scope_focus)
                .tab_stop(false)
                .relative()
                .overflow_hidden()
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.key == "tab" && control.state.read(cx).open {
                        if let (Some(first), Some(last)) =
                            (first_action.as_ref(), last_action.as_ref())
                        {
                            let back = event.keystroke.modifiers.shift;
                            if key_focus.is_focused(window) {
                                window.focus(first, cx);
                                if !back {
                                    cx.stop_propagation();
                                }
                            } else if back && first.is_focused(window) {
                                window.focus(&key_focus, cx);
                                cx.stop_propagation();
                            } else if !back && last.is_focused(window) {
                                window.focus(&key_focus, cx);
                            }
                        }
                        return;
                    }
                    let open = match event.keystroke.key.as_str() {
                        "left" => true,
                        "right" | "escape" => false,
                        _ => return,
                    };
                    cx.stop_propagation();
                    control.request(open, window, cx);
                    if !open {
                        window.focus(&key_focus, cx);
                    }
                })
                .child(ft)
                .child(bd)
        })
        .into_any_element()
    }
}
