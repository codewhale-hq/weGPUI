//! WeUI switch (`src/style/widget/weui-cell/weui-switch.less`, example
//! `src/example/form/form_switch.html`). [`Switch`] is the 52×32px toggle with upstream's 0.35s
//! knob and 0.1s colour transitions, which become instant when reduced motion is set.
//! Place it in the footer of a `CellKind::Switch` form row for the `.weui-cell_switch` layout.
use crate::scrolling::RevealFocus;

use super::ChangeHandler;
use crate::{Theme, accessibility::Semantics, logical::Logical};
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

const KNOB: Duration = Duration::from_millis(350);
const COLOR: Duration = Duration::from_millis(100);

fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let bez = |u: f32, a: f32, b: f32| {
        3. * (1. - u).powi(2) * u * a + 3. * (1. - u) * u * u * b + u.powi(3)
    };
    let (mut lo, mut hi) = (0f32, 1f32);
    let mut u = t;
    for _ in 0..24 {
        u = (lo + hi) / 2.;
        if bez(u, x1, x2) < t { lo = u } else { hi = u }
    }
    bez(u, y1, y2)
}

fn transition_progress(elapsed: Duration, duration: Duration, curve: [f32; 4]) -> f32 {
    if duration.is_zero() || elapsed >= duration {
        1.
    } else if elapsed.is_zero() {
        0.
    } else {
        cubic_bezier(
            elapsed.as_secs_f32() / duration.as_secs_f32(),
            curve[0],
            curve[1],
            curve[2],
            curve[3],
        )
    }
}

pub(crate) struct SwitchMotion {
    checked: bool,
    changed: Option<Instant>,
    from_color: f32,
    from_knob: f32,
    color_duration: Duration,
    knob_duration: Duration,
    color_factor: f32,
    knob_factor: f32,
}

impl SwitchMotion {
    pub(crate) fn new(checked: bool) -> Self {
        let position = f32::from(u8::from(checked));
        Self {
            checked,
            changed: None,
            from_color: position,
            from_knob: position,
            color_duration: COLOR,
            knob_duration: KNOB,
            color_factor: 1.,
            knob_factor: 1.,
        }
    }

    fn progress(&self, now: Instant) -> (f32, f32) {
        let Some(changed) = self.changed else {
            return (1., 1.);
        };
        let elapsed = now.saturating_duration_since(changed);
        (
            transition_progress(elapsed, self.color_duration, [0.25, 0.1, 0.25, 1.]),
            transition_progress(elapsed, self.knob_duration, [0.4, 0.4, 0.25, 1.35]),
        )
    }

    fn position(&self, now: Instant) -> (f32, f32) {
        let target = f32::from(u8::from(self.checked));
        if self.changed.is_none() {
            return (target, target);
        }
        let (color, knob) = self.progress(now);
        (
            self.from_color + (target - self.from_color) * color,
            self.from_knob + (target - self.from_knob) * knob,
        )
    }

    pub(crate) fn update(
        &mut self,
        checked: bool,
        now: Instant,
        reduced: bool,
    ) -> (f32, f32, bool) {
        if reduced {
            *self = Self::new(checked);
            return (
                f32::from(u8::from(checked)),
                20. * f32::from(u8::from(checked)),
                false,
            );
        }
        if self.checked != checked {
            let (color_progress, knob_progress) = self.progress(now);
            let elapsed = self.changed.map(|at| now.saturating_duration_since(at));
            self.color_factor = if elapsed.is_some_and(|elapsed| elapsed < self.color_duration) {
                (color_progress * self.color_factor + (1. - self.color_factor))
                    .abs()
                    .clamp(0., 1.)
            } else {
                1.
            };
            self.knob_factor = if elapsed.is_some_and(|elapsed| elapsed < self.knob_duration) {
                (knob_progress * self.knob_factor + (1. - self.knob_factor))
                    .abs()
                    .clamp(0., 1.)
            } else {
                1.
            };
            (self.from_color, self.from_knob) = self.position(now);
            self.color_duration = COLOR.mul_f32(self.color_factor);
            self.knob_duration = KNOB.mul_f32(self.knob_factor);
            self.checked = checked;
            self.changed = Some(now);
        }
        let (color, knob) = self.position(now);
        let animating = self.changed.is_some_and(|at| {
            now.saturating_duration_since(at) < self.color_duration.max(self.knob_duration)
        });
        if !animating {
            self.changed = None;
        }
        (color, 20. * knob, animating)
    }
}

/// A toggle switch exposed as `Role::Switch` with a toggled state. Controlled: the caller stores
/// the value. The knob animates when the `checked` value changes between renders; the animation
/// state lives in window state keyed by the element id, so keep the id stable. An interrupted
/// transition continues from the currently drawn position and colour; reduced motion settles it.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    surface: Option<Hsla>,
    on_change: Option<ChangeHandler<bool>>,
}

impl Switch {
    /// Creates an off, enabled switch whose accessible name is the localized
    /// [`crate::Messages::switch`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            disabled: false,
            surface: None,
            on_change: None,
        }
    }
    /// Sets the accessible name, usually the row's visible title. It is not drawn. Default: the
    /// localized [`crate::Messages::switch`].
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the on/off state to draw and expose; a change between renders animates the knob.
    /// Default `false`.
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    /// Disables the switch: faded track (10% opacity) and knob, not focusable, `on_change` never
    /// called, and flagged disabled. Default `false`.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Sets the colour behind the switch, used to composite the translucent track. Defaults to
    /// the theme's `bg_2`; pass `bg_1` inside a primary (`bg_1`) form group.
    pub fn surface(mut self, color: Hsla) -> Self {
        self.surface = Some(color);
        self
    }
    /// Calls `handler` with the requested new value (`!checked`) on click or Enter/Space. The
    /// click does not propagate, so a row `on_tap` that also toggles does not fire as well.
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let dir = crate::i18n::direction(cx);
        let checked = self.checked;
        let motion = window.use_keyed_state(self.id.clone(), cx, |_, _| SwitchMotion::new(checked));
        let reduced = cx.reduce_motion();
        let (on, x, animating) =
            motion.update(cx, |m, _| m.update(checked, Instant::now(), reduced));
        if animating {
            window.request_animation_frame();
        }
        let track = t.fg_3.blend(t.brand.opacity(on));
        let disabled = self.disabled;
        let mut el = div()
            .id(self.id)
            .role(Role::Switch)
            .aria_label(
                self.label
                    .unwrap_or_else(|| crate::i18n::messages(cx).switch.clone()),
            )
            .aria_toggled(if checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .flex_shrink_0()
            .w(px(52.))
            .h(px(32.))
            .rounded(px(16.))
            .child(
                div()
                    .relative()
                    .w(px(52.))
                    .h(px(32.))
                    .rounded(px(16.))
                    .bg(if disabled {
                        track.opacity(0.1)
                    } else {
                        self.surface.unwrap_or(t.bg_2).blend(track)
                    })
                    .child(
                        div()
                            .absolute()
                            .top(px(2.))
                            .inset_s(dir, px(2. + x))
                            .size(px(28.))
                            .rounded(px(15.))
                            .bg(if disabled {
                                t.bg_2.blend(white().opacity(0.1))
                            } else {
                                white()
                            })
                            .shadow(vec![BoxShadow {
                                color: hsla(0., 0., 0., 0.06),
                                offset: point(px(0.), px(2.)),
                                blur_radius: px(3.),
                                spread_radius: px(0.),
                                inset: false,
                            }]),
                    ),
            );
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.shadow(super::form::dual_outer_focus_ring()));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| {
                    cx.stop_propagation();
                    handler(&!checked, w, cx)
                });
            }
        }
        Semantics::new(el).disabled(self.disabled).reveal_focus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn reversing_during_motion_preserves_the_current_position_and_colour() {
        let start = Instant::now();
        let mut motion = SwitchMotion::new(false);
        motion.update(true, start, false);
        let reverse = start + Duration::from_millis(80);
        let (before_color, before_knob) = motion.position(reverse);
        assert!(before_knob > 0. && before_knob < 1.);
        let (after_color, after_x, animating) = motion.update(false, reverse, false);
        assert!((after_color - before_color).abs() < 0.00001);
        assert!((after_x / 20. - before_knob).abs() < 0.00001);
        assert!(animating);
        assert_eq!(motion.update(false, reverse + KNOB, false), (0., 0., false));
    }

    #[test]
    fn enabling_reduced_motion_settles_both_channels_without_restarting() {
        let start = Instant::now();
        let mut motion = SwitchMotion::new(false);
        motion.update(true, start, false);
        let interrupted = start + Duration::from_millis(50);
        assert_eq!(motion.update(true, interrupted, true), (1., 20., false));
        assert_eq!(motion.update(true, interrupted, false), (1., 20., false));
        assert_eq!(motion.update(false, interrupted, true), (0., 0., false));
    }

    #[test]
    fn reversing_shortens_each_running_css_transition() {
        let start = Instant::now();
        let mut motion = SwitchMotion::new(false);
        motion.update(true, start, false);
        let reverse = start + Duration::from_millis(80);
        let (color_progress, knob_progress) = motion.progress(reverse);
        motion.update(false, reverse, false);
        assert_eq!(motion.color_duration, COLOR.mul_f32(color_progress));
        assert_eq!(motion.knob_duration, KNOB.mul_f32(knob_progress));
        assert!(motion.color_duration < COLOR);
        assert!(motion.knob_duration < KNOB);
        let duration = motion.color_duration.max(motion.knob_duration);
        assert_eq!(
            motion.update(false, reverse + duration, false),
            (0., 0., false)
        );
    }

    #[test]
    fn a_completed_color_transition_restarts_at_its_full_css_duration() {
        let start = Instant::now();
        let mut motion = SwitchMotion::new(false);
        motion.update(true, start, false);
        motion.update(false, start + COLOR, false);
        assert!(motion.color_duration.abs_diff(COLOR) <= Duration::from_nanos(1));
        assert!(motion.knob_duration < KNOB);
    }
}
