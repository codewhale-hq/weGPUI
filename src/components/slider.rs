use super::ChangeHandler;
use crate::Theme;
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
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max.max(min);
        self
    }
    pub fn step(mut self, step: f32) -> Self {
        self.step = step.max(f32::EPSILON);
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let snapped = min + ((value - min) / step + 1e-4).floor() * step;
    snapped.clamp(min, max)
}

fn format_value(value: f32) -> SharedString {
    if value.fract() == 0. {
        SharedString::from(format!("{}", value as i64))
    } else {
        SharedString::from(format!("{value:.1}"))
    }
}

impl RenderOnce for Slider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let (min, max, step) = (self.min, self.max, self.step);
        let value = self.value.clamp(min, max);
        let span = (max - min).max(f32::EPSILON);
        let fraction = (value - min) / span;
        let brand = t.brand;
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
            .left(relative(fraction))
            .top(px(-13.))
            .ml(px(-14.))
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
            handler = handler
                .focusable()
                .tab_stop(true)
                .cursor_grab()
                .focus_visible(move |s| s.border_color(brand))
                .on_key_down(move |event, window, cx| {
                    let big = (span / 10.).max(step);
                    let next = match event.keystroke.key.as_str() {
                        "left" | "down" => value - step,
                        "right" | "up" => value + step,
                        "pageup" => value + big,
                        "pagedown" => value - big,
                        "home" => min,
                        "end" => max,
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
                .on_drag(SliderDrag(drag_id.clone()), |_, _, _, cx| {
                    cx.new(|_| SliderGhost)
                });
        }
        let inner_id = self.id.clone();
        let move_emit = emit.clone();
        let inner = div()
            .relative()
            .h(px(2.))
            .bg(t.fg_3)
            .child(div().h_full().w(relative(fraction)).bg(brand))
            .when(editable, |el| {
                el.on_drag_move::<SliderDrag>(move |event, window, cx| {
                    if event.drag(cx).0 != inner_id {
                        return;
                    }
                    let b = event.bounds;
                    let x = (event.event.position.x - b.origin.x).as_f32();
                    let w = b.size.width.as_f32().max(1.);
                    move_emit(min + (x / w).clamp(0., 1.) * span, window, cx);
                })
            })
            .child(handler);
        let slider = div().flex_1().py(px(15.)).px(px(18.)).child(inner);
        let label_value = format_value(value);
        div()
            .id(self.id)
            .flex()
            .items_center()
            .child(slider)
            .when(self.show_value, |el| {
                el.child(
                    div()
                        .id("slider-value")
                        .role(Role::Status)
                        .aria_label(label_value.clone())
                        .ml(px(7.))
                        .min_w(px(24.))
                        .text_center()
                        .text_size(px(14.))
                        .text_color(t.fg_1)
                        .child(label_value),
                )
            })
    }
}
