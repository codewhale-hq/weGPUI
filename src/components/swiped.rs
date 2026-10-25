//! WeUI swiped cell (`weui-cell/weui-cell_swiped.less`): a list row that slides left to reveal
//! action buttons (`weui-swiped-btn`).
//!
//! [`SwipedCell`] keeps its open state in window element state keyed by its id; [`SwipedAction`]s
//! are its buttons. Closed cells hide their actions from accessibility and the Tab order, and the
//! row supports the Expand and Collapse accessible actions. Known limitation: the upstream list
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
            let ring = match self.kind {
                SwipedActionKind::Default => t.brand,
                SwipedActionKind::Warn => t.fg_0,
            };
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
                    el.focusable()
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
    pending_open: bool,
    drag_from: Option<(f32, f32)>,
    moved: bool,
    anim: Option<(f32, f32, Instant)>,
    focus: FocusHandle,
}

impl SwipeState {
    fn open(&self) -> bool {
        self.pending_open || self.anim.map_or(self.offset, |(_, to, _)| to) < 0.
    }
    fn snap_to(&mut self, to: f32, reduce: bool) {
        self.pending_open = false;
        if reduce || self.offset == to {
            self.offset = to;
            self.anim = None;
        } else {
            self.anim = Some((self.offset, to, Instant::now()));
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

/// List row (`ListItem` role, with expanded state) whose content slides left to expose its actions.
///
/// Dragging left past half the actions' width and releasing opens it; a click without a drag on an
/// open row closes it. While focused, Left opens, Right or Escape closes, and Enter or Space
/// toggles. Snaps take 200 ms with CSS `ease`, or are instant under reduced motion.
#[derive(IntoElement)]
pub struct SwipedCell {
    id: ElementId,
    content: Option<AnyElement>,
    actions: Vec<SwipedAction>,
    open: bool,
    label: Option<SharedString>,
}

impl SwipedCell {
    /// Creates a closed cell with no content or actions.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: None,
            actions: vec![],
            open: false,
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
    /// Starts the cell open when its state is first created (default `false`); later changes are
    /// ignored.
    pub fn default_open(mut self, open: bool) -> Self {
        self.open = open;
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
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let (bg, ring) = (t.bg_2, t.brand);
            let initially_open = self.open;
            let state = window.use_keyed_state(
                SharedString::from(format!("swiped-{:?}", self.id)),
                cx,
                |_, cx| SwipeState {
                    offset: 0.,
                    width: 0.,
                    pending_open: initially_open,
                    drag_from: None,
                    moved: false,
                    anim: None,
                    focus: cx.focus_handle().tab_stop(true),
                },
            );
            let offset = state.update(cx, |s, _| {
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
            let open = state.read(cx).open();
            let width_state = state.clone();
            let actions_open = open;
            let ft = div()
                .absolute()
                .top_0()
                .bottom_0()
                .right_0()
                .flex()
                .text_color(white())
                .on_children_prepainted(move |bounds, _, cx| {
                    let w: f32 = bounds.iter().map(|b| f32::from(b.size.width)).sum();
                    width_state.update(cx, |s, cx| {
                        if (s.width - w).abs() > 0.5 {
                            s.width = w;
                            if s.pending_open
                                || s.offset < 0. && s.anim.is_none() && s.drag_from.is_none()
                            {
                                s.pending_open = false;
                                s.offset = -w;
                            }
                            cx.notify();
                        }
                    });
                })
                .children(self.actions.into_iter().map(move |mut a| {
                    a.tab_stop = actions_open;
                    a
                }));
            let s_down = state.clone();
            let s_move = state.clone();
            let s_up = state.clone();
            let s_up_out = state.clone();
            let s_click = state.clone();
            let s_key = state.clone();
            let release = move |s: &Entity<SwipeState>, cx: &mut App| {
                s.update(cx, |s, cx| {
                    if let Some((_, base)) = s.drag_from.take() {
                        let to = if !s.moved {
                            if base < 0. { 0. } else { s.offset }
                        } else if s.offset < -s.width / 2. {
                            -s.width
                        } else {
                            0.
                        };
                        s.snap_to(to, cx.reduce_motion());
                        cx.notify();
                    }
                });
            };
            let release_out = release;
            let bd = div()
                .id(self.id.clone())
                .role(Role::ListItem)
                .when_some(self.label, |el, l| el.aria_label(l))
                .aria_expanded(open)
                .aria_description("按左方向键或回车显示操作，右方向键或 Esc 隐藏")
                .relative()
                .left(px(offset))
                .bg(bg)
                .track_focus(&focus)
                .tab_stop(true)
                .on_mouse_down(MouseButton::Left, move |e, _, cx| {
                    let x = f32::from(e.position.x);
                    s_down.update(cx, |s, _| {
                        s.anim = None;
                        s.pending_open = false;
                        s.drag_from = Some((x, s.offset));
                        s.moved = false;
                    });
                })
                .on_drag(SwipeDrag, |_, _, _, cx| cx.new(|_| SwipeDrag))
                .on_drag_move::<SwipeDrag>(move |e, _, cx| {
                    let x = f32::from(e.event.position.x);
                    s_move.update(cx, |s, cx| {
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
                .on_mouse_up(MouseButton::Left, move |_, _, cx| release(&s_up, cx))
                .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                    release_out(&s_up_out, cx)
                })
                .on_click(move |e, _, cx| {
                    if e.is_keyboard() {
                        s_click.update(cx, |s, cx| {
                            let to = if s.open() { 0. } else { -s.width };
                            s.snap_to(to, cx.reduce_motion());
                            cx.notify();
                        });
                    }
                })
                .on_a11y_action(AccessibleAction::Expand, {
                    let state = state.clone();
                    move |_, _, cx| {
                        state.update(cx, |s, cx| {
                            let to = -s.width;
                            s.snap_to(to, cx.reduce_motion());
                            cx.notify();
                        })
                    }
                })
                .on_a11y_action(AccessibleAction::Collapse, {
                    let state = state.clone();
                    move |_, _, cx| {
                        state.update(cx, |s, cx| {
                            s.snap_to(0., cx.reduce_motion());
                            cx.notify();
                        })
                    }
                })
                .on_key_down(move |e, _, cx| {
                    let key = e.keystroke.key.as_str();
                    s_key.update(cx, |s, cx| {
                        let to = match key {
                            "left" => -s.width,
                            "right" | "escape" => 0.,
                            _ => return,
                        };
                        cx.stop_propagation();
                        s.snap_to(to, cx.reduce_motion());
                        cx.notify();
                    });
                })
                .children(self.content)
                .when(show_ring, |el| {
                    el.child(div().absolute().inset_0().border_2().border_color(ring))
                });
            div().relative().overflow_hidden().child(ft).child(bd)
        })
    }
}
