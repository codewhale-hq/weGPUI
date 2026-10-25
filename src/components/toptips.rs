//! WeUI top tips (`weui-tips/weui-toptips.less`): a full-width alert banner 8 px from the top of
//! the window, drawn above modals.
//!
//! [`ToptipsState`] is the caller-owned visibility; [`Toptips`] renders it. Unlike
//! [`Presence`](super::modal::Presence) it never moves focus. Showing or hiding mid-fade reverses
//! from the current opacity. Known limitation: the fade is linear.

use crate::Theme;
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

/// Fade-in and fade-out length of [`Toptips`].
pub const FADE: Duration = Duration::from_millis(100);

/// Caller-owned, cloneable visibility of a [`Toptips`]: open flag, transition start and a
/// generation counter. Keep one in your view state and pass `&state` when rendering. Under reduced
/// motion the fade is instant.
#[derive(Clone, Default)]
pub struct ToptipsState {
    visible: bool,
    changed: Option<Instant>,
    generation: usize,
}

impl ToptipsState {
    /// Creates a hidden state.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns `true` after `show` and `false` as soon as `hide` is called, even while fading out.
    pub fn is_open(&self) -> bool {
        self.visible
    }
    /// Fades the tips in if hidden, bumping the generation, and refreshes the window.
    pub fn show(&mut self, window: &mut Window) {
        if !self.visible {
            self.visible = true;
            self.generation += 1;
            self.reverse();
        }
        window.refresh();
    }
    /// Like [`ToptipsState::show`] but fully opaque at once.
    pub fn show_instantly(&mut self, window: &mut Window) {
        self.show(window);
        self.changed = None;
    }
    /// Starts the fade-out; does nothing when already hidden.
    pub fn hide(&mut self, window: &mut Window) {
        if self.visible {
            self.visible = false;
            self.reverse();
            window.refresh();
        }
    }
    fn reverse(&mut self) {
        let done = self.changed.map_or(1., |at| {
            (at.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.)
        });
        let now = Instant::now();
        self.changed = Some(now.checked_sub(FADE.mul_f32(1. - done)).unwrap_or(now));
    }
    /// Hides after `delay` unless the tips were hidden or shown again in the meantime. Spawns a
    /// task on the entity `T`; `get` returns this state's field from it. Call it after `show`.
    pub fn hide_after<T: 'static>(
        &self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<T>,
        get: fn(&mut T) -> &mut ToptipsState,
    ) {
        let generation = self.generation;
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update_in(cx, |this, window, cx| {
                let state = get(this);
                if state.visible && state.generation == generation {
                    state.hide(window);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
    fn opacity(&self, window: &Window, cx: &App) -> Option<f32> {
        let linear = match self.changed {
            Some(at) if !cx.reduce_motion() => {
                (at.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.)
            }
            _ => 1.,
        };
        if linear < 1. {
            window.request_animation_frame();
        }
        if self.visible {
            Some(linear)
        } else if linear < 1. {
            Some(1. - linear)
        } else {
            None
        }
    }
}

/// Top alert banner (`weui-toptips`): window width minus 16 px, 14 px white centred text, exposed
/// as an assertive live `Alert`. Renders nothing once its state has faded out.
#[derive(IntoElement)]
pub struct Toptips {
    id: ElementId,
    state: ToptipsState,
    text: SharedString,
    warn: bool,
}

impl Toptips {
    /// Creates a banner showing `text` while `state` (cloned) is visible.
    pub fn new(
        id: impl Into<ElementId>,
        state: &ToptipsState,
        text: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            text: text.into(),
            warn: false,
        }
    }
    /// Uses the warning red background (`weui-toptips_warn`). Without it there is no background, as
    /// in the upstream CSS.
    pub fn warn(mut self) -> Self {
        self.warn = true;
        self
    }
}

impl RenderOnce for Toptips {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(opacity) = self.state.opacity(window, cx) else {
            return div().into_any_element();
        };
        let t = Theme::get(cx);
        let width = window.viewport_size().width;
        let tips = div()
            .id(self.id)
            .role(Role::Alert)
            .aria_label(self.text.clone())
            .w(width - px(16.))
            .p(px(10.))
            .rounded(px(8.))
            .text_size(px(14.))
            .line_height(relative(1.4))
            .text_center()
            .text_color(white())
            .when(self.warn, |el| el.bg(t.red))
            .opacity(opacity)
            .child(self.text);
        let tips = crate::accessibility::Semantics::new(tips).live(accesskit::Live::Assertive);
        deferred(anchored().position(point(px(8.), px(8.))).child(tips))
            .with_priority(2)
            .into_any_element()
    }
}
