//! WeUI switch (`src/style/widget/weui-cell/weui-switch.less`, example
//! `src/example/form/form_switch.html`). [`Switch`] is the 52×32px toggle with upstream's 0.35s
//! knob and 0.1s colour transitions, which become instant when reduced motion is set.
//! Place it in the footer of a `CellKind::Switch` form row for the `.weui-cell_switch` layout.
use crate::scrolling::RevealFocus;

use super::ChangeHandler;
use crate::{Theme, accessibility::Semantics};
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

struct SwitchMotion {
    checked: bool,
    changed: Option<Instant>,
}

/// A toggle switch exposed as `Role::Switch` with a toggled state. Controlled: the caller stores
/// the value. The knob animates when the `checked` value changes between renders; the animation
/// state lives in window state keyed by the element id, so keep the id stable.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    surface: Option<Hsla>,
    on_change: Option<ChangeHandler<bool>>,
}

impl Switch {
    /// Creates an off, enabled switch whose accessible name is "开关".
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "开关".into(),
            checked: false,
            disabled: false,
            surface: None,
            on_change: None,
        }
    }
    /// Sets the accessible name, usually the row's visible title. It is not drawn. Default "开关".
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
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
        let checked = self.checked;
        let motion = window.use_keyed_state(self.id.clone(), cx, |_, _| SwitchMotion {
            checked,
            changed: None,
        });
        let changed = motion.update(cx, |m, _| {
            if m.checked != checked {
                m.checked = checked;
                m.changed = Some(Instant::now());
            }
            m.changed
        });
        let elapsed = match changed {
            Some(at) if !cx.reduce_motion() => at.elapsed(),
            _ => KNOB,
        };
        if elapsed < KNOB {
            window.request_animation_frame();
        }
        let knob_p = cubic_bezier(
            (elapsed.as_secs_f32() / KNOB.as_secs_f32()).min(1.),
            0.4,
            0.4,
            0.25,
            1.35,
        );
        let color_p = (elapsed.as_secs_f32() / COLOR.as_secs_f32()).min(1.);
        let (on, x) = if checked {
            (color_p, 20. * knob_p)
        } else {
            (1. - color_p, 20. * (1. - knob_p))
        };
        let track = t.fg_3.blend(t.brand.opacity(on));
        let disabled = self.disabled;
        let ring = if checked { t.fg_0 } else { t.brand };
        let mut el = div()
            .id(self.id)
            .role(Role::Switch)
            .aria_label(self.label)
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
                            .left(px(2. + x))
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
                .focus_visible(move |s| {
                    s.shadow(vec![BoxShadow {
                        color: ring,
                        offset: point(px(0.), px(0.)),
                        blur_radius: px(0.),
                        spread_radius: px(2.),
                        inset: false,
                    }])
                });
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
