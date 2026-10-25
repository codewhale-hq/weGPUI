//! Overlay foundation for WeUI popups: the mask (`weui-tips/weui-mask.less`) and the fade and slide
//! transitions (`weui-animate/weui-animate.less`).
//!
//! [`Presence`] is caller-owned open/closed state with focus restoration and transition timing.
//! [`Modal`] renders it as a deferred, window-sized layer with mask, placement, motion, Tab
//! containment and Escape/mask dismissal, and lets [`crate::accessibility::ModalScope`] hide the
//! background and covered modal layers from assistive technology. Known limitations: a reversal
//! restarts the easing curve from the current value, and a slide travels the window height until
//! the panel is first measured.

use crate::accessibility::Semantics;
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

/// Default open and close transition length, matching WeUI's 0.3 s animations.
pub const TRANSITION: Duration = Duration::from_millis(300);

/// Caller-owned visibility state for one overlay.
///
/// Holds the open/closed target, a focus handle, the opener focus to restore, a generation counter
/// and the transition timing. Keep one value in your view state, change it with [`Presence::show`]
/// and [`Presence::hide`], and pass `&presence` to the overlay on every render. Components clone
/// it, so changes must be made to your copy; clones share the focus handle. Under reduced motion
/// (`App::reduce_motion`) every transition completes instantly.
#[derive(Clone)]
pub struct Presence {
    visible: bool,
    changed: Option<Instant>,
    from: f32,
    span: Duration,
    opener: Option<FocusHandle>,
    focus: FocusHandle,
    generation: usize,
    duration: Duration,
    passive: bool,
}

impl Presence {
    /// Creates a closed presence with the default [`TRANSITION`] and a fresh focus handle.
    pub fn new(cx: &mut App) -> Self {
        Self {
            visible: false,
            changed: None,
            from: 0.,
            span: TRANSITION,
            opener: None,
            focus: cx.focus_handle(),
            generation: 0,
            duration: TRANSITION,
            passive: false,
        }
    }
    /// Sets the length of a full open or close transition (default [`TRANSITION`]); zero disables
    /// the animation.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self.span = duration;
        self
    }
    /// Makes the overlay non-modal, as WeUI toasts are: `show` and `hide` leave keyboard focus
    /// where it is, and [`Modal`] neither hides the page from assistive technology nor traps Tab.
    /// The content should carry its own role (a toast is an alert). A mask still blocks pointer
    /// input.
    pub fn passive(mut self) -> Self {
        self.passive = true;
        self
    }
    /// Returns `true` after `show` and `false` as soon as `hide` is called, even while the closing
    /// transition is still drawing; use [`Presence::progress`] for what is on screen.
    pub fn is_open(&self) -> bool {
        self.visible
    }
    /// Counter incremented each time a closed presence is shown. Delayed tasks compare it to ignore
    /// a stale timer, as [`Presence::hide_after`] does.
    pub fn generation(&self) -> usize {
        self.generation
    }
    /// Focus handle that `show` focuses and that [`Modal`] tracks as its layer root.
    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }
    fn linear(&self, cx: &App) -> f32 {
        match self.changed {
            Some(at) if !cx.reduce_motion() && !self.span.is_zero() => {
                (at.elapsed().as_secs_f32() / self.span.as_secs_f32()).min(1.)
            }
            _ => 1.,
        }
    }
    fn value(&self, cx: &App) -> f32 {
        let to = if self.visible { 1. } else { 0. };
        self.from + (to - self.from) * ease(self.linear(cx))
    }
    fn retarget(&mut self, visible: bool, cx: &App) {
        let current = self.value(cx);
        let to = if visible { 1. } else { 0. };
        self.from = current;
        self.span = self.duration.mul_f32((to - current).abs());
        self.visible = visible;
        self.changed = Some(Instant::now());
    }
    /// Opens the overlay and moves focus into it.
    ///
    /// When closed, records the currently focused element as the opener (restored by `hide`),
    /// starts the opening transition from the current value and bumps the generation. Unless the
    /// presence is [passive](Presence::passive), always focuses [`Presence::focus_handle`].
    /// Refreshes the window.
    pub fn show(&mut self, window: &mut Window, cx: &mut App) {
        if !self.visible {
            if !self.passive {
                self.opener = window.focused(cx).filter(|f| f != &self.focus);
            }
            self.retarget(true, cx);
            self.generation += 1;
        }
        if !self.passive {
            window.focus(&self.focus, cx);
        }
        window.refresh();
    }
    /// Like [`Presence::show`] but jumps straight to fully visible, skipping the transition.
    pub fn show_instantly(&mut self, window: &mut Window, cx: &mut App) {
        self.show(window, cx);
        self.changed = None;
        self.from = 1.;
    }
    /// Starts the closing transition; does nothing when already closed.
    ///
    /// Restores focus to the recorded opener or, without one, blurs the window if focus is inside
    /// the overlay. An interrupted opening reverses from its current value and takes the matching
    /// share of the duration.
    pub fn hide(&mut self, window: &mut Window, cx: &mut App) {
        if !self.visible {
            return;
        }
        self.retarget(false, cx);
        if !self.passive {
            if let Some(opener) = self.opener.take() {
                window.focus(&opener, cx);
            } else if self.focus.contains_focused(window, cx) {
                window.blur(cx);
            }
        }
        window.refresh();
    }
    /// Calls [`Presence::hide`] when open, otherwise [`Presence::show`].
    pub fn toggle(&mut self, window: &mut Window, cx: &mut App) {
        if self.visible {
            self.hide(window, cx)
        } else {
            self.show(window, cx)
        }
    }
    /// Hides after `delay` unless the presence was hidden or shown again in the meantime.
    ///
    /// Spawns a task on the entity `T` that owns the presence; `get` returns the presence field of
    /// that entity, which is notified when it hides. Call it after `show`: the current generation
    /// is captured at call time.
    pub fn hide_after<T: 'static>(
        &self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<T>,
        get: fn(&mut T) -> &mut Presence,
    ) {
        let generation = self.generation;
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update_in(cx, |this, window, cx| {
                let presence = get(this);
                if presence.visible && presence.generation == generation {
                    presence.hide(window, cx);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
    /// Returns the eased visibility in `0.0..=1.0`, or `None` once fully hidden.
    ///
    /// Requests another animation frame while a transition is running, so call it during render.
    pub fn progress(&self, window: &Window, cx: &App) -> Option<f32> {
        let linear = self.linear(cx);
        if linear < 1. {
            window.request_animation_frame();
        }
        if !self.visible && linear >= 1. {
            return None;
        }
        Some(self.value(cx))
    }
}

/// CSS `ease` timing function, `cubic-bezier(0.25, 0.1, 0.25, 1)`.
///
/// Maps linear progress `t` in `0.0..=1.0` to eased progress by bisecting the curve's x coordinate
/// (24 steps); inputs outside the range give the end values.
pub fn ease(t: f32) -> f32 {
    let (x1, y1, x2, y2) = (0.25f32, 0.1f32, 0.25f32, 1.0f32);
    let mut lo = 0f32;
    let mut hi = 1f32;
    let mut u = t;
    for _ in 0..24 {
        u = (lo + hi) / 2.;
        let x = 3. * (1. - u).powi(2) * u * x1 + 3. * (1. - u) * u * u * x2 + u.powi(3);
        if x < t { lo = u } else { hi = u }
    }
    3. * (1. - u).powi(2) * u * y1 + 3. * (1. - u) * u * u * y2 + u.powi(3)
}

/// Backdrop drawn behind a [`Modal`]'s content.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MaskKind {
    /// Black at 50 % opacity (`weui-mask`); the default.
    #[default]
    Dark,
    /// Invisible, but still blocks pointer input to the window (`weui-mask_transparent`).
    Transparent,
    /// No backdrop. The layer still blocks pointer input while open, but not during the closing
    /// transition.
    None,
}

/// Where a [`Modal`] places its content inside the window.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Placement {
    /// Centred horizontally and vertically at the content's own size; the default.
    #[default]
    Center,
    /// Full width, anchored to the bottom edge, as for sheets and half-screen dialogs.
    Bottom,
    /// Full width, anchored to the top edge.
    Top,
    /// Fills the whole window, inside any [`Modal::padding`].
    Fill,
}

/// Entrance and exit animation of a [`Modal`]'s content, driven by [`Presence::progress`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Motion {
    /// Fades the content's opacity; the default.
    #[default]
    Fade,
    /// Slides up from below by the measured panel height, like CSS `translateY(100%)`.
    SlideUp,
    /// Slides down from above by the measured panel height.
    SlideDown,
    /// Shows and hides the content without animating it; the mask still fades unless
    /// `mask_motion(false)`.
    None,
}

type DismissHandler = Box<dyn Fn(&mut Window, &mut App)>;

/// Window-sized overlay layer driven by a [`Presence`]; children go in its content panel.
///
/// Renders nothing once the presence is fully hidden. Otherwise draws a deferred layer at the
/// window origin with the mask, places and animates the content per [`Placement`] and [`Motion`],
/// and is exposed as a modal `Dialog`. While open it keeps Tab and Shift-Tab inside itself, calls
/// `on_dismiss` on Escape or a mask click, and marks the window so
/// [`crate::accessibility::ModalScope`] hides the background. With a
/// [passive](Presence::passive) presence it is an unnamed group instead, whose content supplies
/// its own role (a toast is an alert): no focus move, no Tab trap and no background hiding. It
/// never hides itself.
#[derive(IntoElement)]
pub struct Modal {
    id: ElementId,
    presence: Presence,
    mask: MaskKind,
    placement: Placement,
    motion: Motion,
    mask_motion: bool,
    label: SharedString,
    dismiss_on_mask: bool,
    on_dismiss: Option<DismissHandler>,
    padding: Edges<Pixels>,
    children: Vec<AnyElement>,
}

impl Modal {
    /// Creates a modal for `presence` (cloned) with a dark mask, centred placement, fade motion and
    /// mask-click dismissal enabled.
    pub fn new(id: impl Into<ElementId>, presence: &Presence) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            mask: MaskKind::Dark,
            placement: Placement::Center,
            motion: Motion::Fade,
            mask_motion: true,
            label: SharedString::default(),
            dismiss_on_mask: true,
            on_dismiss: None,
            padding: Edges::default(),
            children: vec![],
        }
    }
    /// Sets the backdrop (default [`MaskKind::Dark`]).
    pub fn mask(mut self, mask: MaskKind) -> Self {
        self.mask = mask;
        self
    }
    /// Sets where the content sits (default [`Placement::Center`]).
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
    /// Sets the content animation (default [`Motion::Fade`]).
    pub fn motion(mut self, motion: Motion) -> Self {
        self.motion = motion;
        self
    }
    /// Whether the mask fades with the transition (default `true`); `false` keeps it at full
    /// strength whenever the modal is drawn.
    pub fn mask_motion(mut self, animate: bool) -> Self {
        self.mask_motion = animate;
        self
    }
    /// Sets the accessible name of the dialog layer (default empty). Unused for passive presences.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Whether clicking the mask outside the content calls `on_dismiss` (default `true`).
    pub fn dismiss_on_mask(mut self, value: bool) -> Self {
        self.dismiss_on_mask = value;
        self
    }
    /// Insets the content area from the window edges (default zero); the mask still covers the
    /// whole window.
    pub fn padding(mut self, padding: Edges<Pixels>) -> Self {
        self.padding = padding;
        self
    }
    /// Called when the open modal is dismissed by Escape or, if enabled, a mask click. Hide the
    /// presence here; the modal does not close itself.
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Modal {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(p) = self.presence.progress(window, cx) else {
            return div().into_any_element();
        };
        let viewport = window.viewport_size();
        let focus = self.presence.focus.clone();
        let interactive = self.presence.visible;
        let passive = self.presence.passive;
        let hidden_flag = (interactive && !passive)
            .then(|| crate::accessibility::register_open_modal(window, cx));
        let panel_height = window
            .use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "panel-height".into()),
                cx,
                |_, _| std::rc::Rc::new(std::cell::Cell::new(0f32)),
            )
            .read(cx)
            .clone();
        let dismiss = std::rc::Rc::new(self.on_dismiss);
        let dismiss_key = dismiss.clone();
        let mask_alpha = if self.mask_motion { p } else { 1. };
        let mask_color = match self.mask {
            MaskKind::Dark => hsla(0., 0., 0., 0.5 * mask_alpha),
            _ => transparent_black(),
        };
        let mut content = div()
            .id("modal-content")
            .relative()
            .flex()
            .flex_col()
            .occlude()
            .on_click(|_, _, cx| cx.stop_propagation())
            .child({
                let panel_height = panel_height.clone();
                canvas(
                    move |bounds, _, _| panel_height.set(bounds.size.height.as_f32()),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .children(self.children);
        let travel = match panel_height.get() {
            h if h > 0. => h,
            _ => viewport.height.as_f32(),
        };
        let shift = (1. - p) * travel;
        content = match self.motion {
            Motion::Fade => content.opacity(p),
            Motion::SlideUp => content.top(px(shift)),
            Motion::SlideDown => content.top(px(-shift)),
            Motion::None => content,
        };
        content = match self.placement {
            Placement::Bottom | Placement::Top | Placement::Fill => content.w_full(),
            Placement::Center => content,
        };
        if self.placement == Placement::Fill {
            content = content.h_full();
        }
        let dismiss_on_mask = self.dismiss_on_mask;
        let layer = div()
            .id(self.id)
            .key_context("WeuiModal")
            .track_focus(&focus)
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .flex_col()
            .pt(self.padding.top)
            .pb(self.padding.bottom)
            .pl(self.padding.left)
            .pr(self.padding.right)
            .map(|el| match self.placement {
                Placement::Center => el.items_center().justify_center(),
                Placement::Bottom => el.justify_end(),
                Placement::Top => el.justify_start(),
                Placement::Fill => el,
            })
            .when(self.mask != MaskKind::None || interactive, |el| {
                el.occlude()
            })
            .bg(mask_color)
            // Bubble phase, not capture: when one modal is rendered inside another, the
            // innermost layer sees Tab first and stops it, so focus stays in the topmost modal.
            .on_key_down({
                let focus = focus.clone();
                move |event, window, cx| {
                    if event.keystroke.key != "tab" {
                        return;
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                    let back = event.keystroke.modifiers.shift;
                    for _ in 0..256 {
                        if back {
                            window.focus_prev(cx)
                        } else {
                            window.focus_next(cx)
                        }
                        if focus.contains_focused(window, cx) && !focus.is_focused(window) {
                            return;
                        }
                    }
                    window.focus(&focus, cx);
                }
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.key == "escape" && interactive {
                    if let Some(handler) = dismiss_key.as_ref() {
                        handler(window, cx);
                    }
                    cx.stop_propagation();
                }
            })
            .on_click(move |_, window, cx| {
                if dismiss_on_mask
                    && interactive
                    && let Some(handler) = dismiss.as_ref()
                {
                    handler(window, cx);
                }
            })
            .child(content);
        let semantics = if passive {
            Semantics::new(layer.role(Role::Group))
        } else {
            Semantics::new(layer.role(Role::Dialog).aria_label(self.label))
                .modal()
                .hidden(!interactive)
        };
        let semantics = if let Some(flag) = hidden_flag {
            semantics.hidden_flag(flag)
        } else {
            semantics
        };
        deferred(
            anchored()
                .position(point(px(0.), px(0.)))
                .child(semantics.modal_layer()),
        )
        .with_priority(1)
        .into_any_element()
    }
}
