//! WeUI slider (`weui-slider/weui-slider.less`) with the optional value readout of
//! `weui-slider-box`.
//!
//! [`Slider`] is controlled: the caller owns the value and stores what `on_change` reports. Known
//! limitations: only the thumb starts a drag (clicking the track does nothing, as in the upstream
//! example), and the readout shows at most one decimal place.

use super::ChangeHandler;
use crate::{Theme, logical::Logical};
use gpui::{prelude::*, *};
use std::rc::Rc;

#[derive(Clone)]
struct SliderDrag(ElementId);

struct SliderGhost;

impl Render for SliderGhost {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// Horizontal range slider with a 28 px thumb (`Slider` role) on a 2 px track.
///
/// Without `on_change`, or when disabled, it is read-only: not focusable or draggable. While
/// focused, Left/Down and Right/Up move one step, PageUp and PageDown a tenth of the range (at
/// least one step), and Home and End jump to the ends. Increment and Decrement accessible actions
/// and SetValue accessible actions are supported. Disabled sliders are exposed as disabled;
/// sliders without a callback are exposed as read-only.
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    label: SharedString,
    show_value: bool,
    disabled: bool,
    on_change: Option<ChangeHandler<f32>>,
}

impl Slider {
    /// Creates a read-only slider at 0 in `0..=100` with step 1.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            min: 0.,
            max: 100.,
            step: 1.,
            label: SharedString::default(),
            show_value: false,
            disabled: false,
            on_change: None,
        }
    }
    /// Sets the current value (default 0), clamped to the range when drawn. A non-finite value
    /// renders at the midpoint, as an HTML range input does by default.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
    /// Sets `min` and `max` (default `0..=100`); a `max` below `min` becomes `min`. Ignored when
    /// either bound or their difference is not finite.
    pub fn range(mut self, min: f32, max: f32) -> Self {
        if min.is_finite() && max.is_finite() && (max - min).is_finite() {
            self.min = min;
            self.max = max.max(min);
        }
        self
    }
    /// Sets the step (default 1); non-finite or non-positive values fall back to 1.
    pub fn step(mut self, step: f32) -> Self {
        self.step = if step.is_finite() && step > 0. {
            step
        } else {
            1.
        };
        self
    }
    /// Sets the accessible name of the thumb (default empty).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Shows the current value to the right (`weui-slider-box__value`). Default `false`.
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
    /// Makes the slider read-only without changing its appearance (default `false`).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Called with the new value when a drag, key or accessible action changes it. The value is
    /// snapped to the nearest step from `min` (as HTML range does) and clamped. Store it and
    /// re-render.
    pub fn on_change(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let (value, min, max, step) = (value as f64, min as f64, max as f64, step as f64);
    let snapped = min + ((value.clamp(min, max) - min) / step).round() * step;
    let snapped = if snapped as f32 > max as f32 {
        min + ((max - min) / step).floor() * step
    } else {
        snapped
    };
    snapped.clamp(min, max) as f32
}

fn requested_value(value: f64, min: f32, max: f32) -> Option<f32> {
    value
        .is_finite()
        .then(|| value.clamp(min as f64, max as f64) as f32)
}

fn format_value(value: f32) -> SharedString {
    SharedString::from(if value == 0. {
        "0".to_owned()
    } else {
        value.to_string()
    })
}

impl RenderOnce for Slider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx).clone();
            let dir = crate::i18n::direction(cx);
            let rtl = dir.is_rtl();
            let (min, max, step) = (self.min, self.max, self.step);
            let value = if self.value.is_finite() {
                self.value.clamp(min, max)
            } else {
                min + (max - min) / 2.
            };
            let span = (max - min).max(f32::EPSILON);
            let fraction = (value - min) / span;
            let brand = t.brand;
            let focus = t.focus_color(white());
            let editable = !self.disabled && self.on_change.is_some();
            let on_change: Option<Rc<ChangeHandler<f32>>> = self.on_change.map(Rc::new);
            let emit = {
                let on_change = on_change.clone();
                move |next: f32, window: &mut Window, cx: &mut App| {
                    let next = snap(next, min, max, step);
                    if next != value
                        && let Some(handler) = on_change.as_ref()
                    {
                        handler(&next, window, cx);
                    }
                }
            };
            let emit = Rc::new(emit);
            let drag_id = self.id.clone();
            let mut handler = div()
                .id("slider-handler")
                .role(Role::Slider)
                .aria_label(self.label.clone())
                .aria_numeric_value(value as f64)
                .aria_min_numeric_value(min as f64)
                .aria_max_numeric_value(max as f64)
                .aria_numeric_value_step(step as f64)
                .aria_value(format_value(value))
                .aria_orientation(Orientation::Horizontal)
                .absolute()
                .inset_s(dir, relative(fraction))
                .top(px(-13.))
                .ms(dir, px(-14.))
                .size(px(28.))
                .rounded_full()
                .bg(white())
                .border_2()
                .border_color(transparent_black())
                .shadow(vec![BoxShadow {
                    color: t.fg_3,
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(4.),
                    spread_radius: px(0.),
                    inset: false,
                }]);
            if editable {
                let key_emit = emit.clone();
                let inc_emit = emit.clone();
                let dec_emit = emit.clone();
                let value_emit = emit.clone();
                handler = handler
                    .focusable()
                    .tab_stop(true)
                    .cursor_grab()
                    .focus_visible(move |s| s.border_color(focus))
                    .on_key_down(move |event, window, cx| {
                        let big = (span / 10.).max(step);
                        let next = match (event.keystroke.key.as_str(), rtl) {
                            ("down", _) | ("left", false) | ("right", true) => value - step,
                            ("up", _) | ("right", false) | ("left", true) => value + step,
                            ("pageup", _) => value + big,
                            ("pagedown", _) => value - big,
                            ("home", _) => min,
                            ("end", _) => max,
                            _ => return,
                        };
                        cx.stop_propagation();
                        key_emit(next, window, cx);
                    })
                    .on_a11y_action(AccessibleAction::Increment, move |_, window, cx| {
                        inc_emit(value + step, window, cx)
                    })
                    .on_a11y_action(AccessibleAction::Decrement, move |_, window, cx| {
                        dec_emit(value - step, window, cx)
                    })
                    .on_a11y_action(AccessibleAction::SetValue, move |data, window, cx| {
                        if let Some(accesskit::ActionData::NumericValue(value)) = data
                            && let Some(value) = requested_value(*value, min, max)
                        {
                            value_emit(value, window, cx);
                        }
                    })
                    .on_drag(SliderDrag(drag_id.clone()), |_, _, _, cx| {
                        cx.new(|_| SliderGhost)
                    });
            }
            let inner_id = self.id.clone();
            let move_emit = emit.clone();
            let inner = div()
                .relative()
                .flex()
                .row(dir)
                .h(px(2.))
                .bg(t.fg_3)
                .child(div().h_full().w(relative(fraction)).bg(brand))
                .when(editable, |el| {
                    el.on_drag_move::<SliderDrag>(move |event, window, cx| {
                        if event.drag(cx).0 != inner_id {
                            return;
                        }
                        let b = event.bounds;
                        let x = if rtl {
                            (b.origin.x + b.size.width - event.event.position.x).as_f32()
                        } else {
                            (event.event.position.x - b.origin.x).as_f32()
                        };
                        let w = b.size.width.as_f32().max(1.);
                        move_emit(min + (x / w).clamp(0., 1.) * span, window, cx);
                    })
                })
                .child(
                    crate::accessibility::Semantics::new(handler)
                        .disabled(self.disabled)
                        .read_only(!editable && !self.disabled),
                );
            let slider = div().flex_1().py(px(15.)).px(px(18.)).child(inner);
            let label_value = format_value(value);
            div()
                .id(self.id)
                .flex()
                .row(dir)
                .items_center()
                .child(slider)
                .when(self.show_value, |el| {
                    el.child(
                        div()
                            .id("slider-value")
                            .role(Role::Status)
                            .aria_label(label_value.clone())
                            .ms(dir, px(7.))
                            .min_w(px(24.))
                            .text_center()
                            .text_size(px(14.))
                            .text_color(t.fg_1)
                            .child(label_value),
                    )
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn invalid_steps_preserve_a_usable_keyboard_increment() {
        for step in [0., -1., f32::NAN, f32::INFINITY] {
            let slider = Slider::new("test").step(step);
            assert_eq!(snap(41., slider.min, slider.max, slider.step), 41.);
        }
    }

    #[test]
    fn tiny_valid_steps_do_not_overflow_and_jump_to_the_maximum() {
        for step in [f32::MIN_POSITIVE, f32::from_bits(1)] {
            assert_eq!(snap(41., 0., 100., step), 41.);
            assert_eq!(snap(-41., -100., 100., step), -41.);
        }
    }

    #[test]
    fn accessible_value_requests_reject_non_finite_values_before_narrowing() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(requested_value(value, 0., 100.), None);
        }
        assert_eq!(requested_value(f64::MAX, 0., 100.), Some(100.));
        assert_eq!(requested_value(-f64::MAX, 0., 100.), Some(0.));
        assert_eq!(
            requested_value(14., 5., 25.).map(|v| snap(v, 5., 25., 5.)),
            Some(15.)
        );
    }

    #[test]
    fn snapping_is_relative_to_minimum_and_clamped() {
        assert_eq!(snap(14., 5., 25., 5.), 15.);
        assert_eq!(snap(-100., 5., 25., 5.), 5.);
        assert_eq!(snap(100., 5., 25., 5.), 25.);
        assert_eq!(snap(10., 5., 5., 1.), 5.);
        for (value, expected) in [
            (0., "0"),
            (-0., "0"),
            (0.125, "0.125"),
            (-0.125, "-0.125"),
            (50., "50"),
            (1e20, "100000000000000000000"),
        ] {
            assert_eq!(&*format_value(value), expected);
        }
    }

    #[test]
    fn upper_endpoint_keeps_a_valid_step_without_losing_float_endpoints() {
        assert_eq!(snap(10., 0., 10., 4.), 8.);
        assert_eq!(snap(24., 5., 24., 5.), 20.);
        assert_eq!(snap(8., 5., 8., 10.), 5.);
        assert_eq!(snap(1., 0., 1., 0.1), 1.);
        assert_eq!(snap(0.5, 0., 1., 0.125), 0.5);
    }

    #[test]
    fn invalid_ranges_cannot_replace_the_last_finite_range() {
        for (min, max) in [(f32::NAN, 5.), (0., f32::INFINITY), (-f32::MAX, f32::MAX)] {
            let slider = Slider::new("test").range(10., 20.).range(min, max);
            assert_eq!((slider.min, slider.max), (10., 20.));
        }
        let slider = Slider::new("test").range(20., 10.);
        assert_eq!((slider.min, slider.max), (20., 20.));
    }
}
