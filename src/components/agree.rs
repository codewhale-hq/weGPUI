use super::{ChangeHandler, ClickHandler, checkbox::check_icon, form::focus_ring};
use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::time::Duration;

fn shake(delta: f32) -> f32 {
    const KEYS: [(f32, f32); 9] = [
        (0., 0.),
        (0.16, -8.),
        (0.28, -16.),
        (0.44, 0.),
        (0.59, -16.),
        (0.73, 0.),
        (0.82, 16.),
        (0.94, 8.),
        (1., 0.),
    ];
    for w in KEYS.windows(2) {
        let ((t0, x0), (t1, x1)) = (w[0], w[1]);
        if delta <= t1 {
            let p = ((delta - t0) / (t1 - t0)).clamp(0., 1.);
            let p = p * p * (3. - 2. * p);
            return x0 + (x1 - x0) * p;
        }
    }
    0.
}

#[derive(IntoElement)]
pub struct Agree {
    id: ElementId,
    text: SharedString,
    link: Option<(SharedString, ClickHandler)>,
    checked: bool,
    disabled: bool,
    warn: bool,
    shake: usize,
    on_change: Option<ChangeHandler<bool>>,
}

impl Agree {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            link: None,
            checked: false,
            disabled: false,
            warn: false,
            shake: 0,
            on_change: None,
        }
    }
    pub fn link(
        mut self,
        text: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.link = Some((text.into(), Box::new(handler)));
        self
    }
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn warn(mut self, value: bool) -> Self {
        self.warn = value;
        self
    }
    pub fn shake(mut self, generation: usize) -> Self {
        self.shake = generation;
        self
    }
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Agree {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let checked = self.checked;
        let brand = t.brand;
        let label = match &self.link {
            Some((link, _)) => SharedString::from(format!("{}{}", self.text, link)),
            None => self.text.clone(),
        };
        let link_hit = std::rc::Rc::new(std::cell::Cell::new(false));
        let link_flag = link_hit.clone();
        let link = self.link.map(|(text, handler)| {
            div()
                .id("agree-link")
                .role(Role::Link)
                .aria_label(text.clone())
                .text_color(t.link)
                .cursor_pointer()
                .focusable()
                .tab_stop(true)
                .focus_visible(move |s| s.shadow(focus_ring(brand)))
                .active(|s| s.opacity(0.5))
                .on_click(move |e, w, cx| {
                    cx.stop_propagation();
                    link_flag.set(true);
                    handler(e, w, cx)
                })
                .child(text)
        });
        let mut el = div()
            .id(self.id.clone())
            .role(Role::CheckBox)
            .aria_label(label)
            .aria_toggled(if checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .flex()
            .max_w_full()
            .min_h(px(19.6))
            .text_left()
            .text_size(px(14.))
            .line_height(relative(1.4))
            .child(
                div().h(px(19.6)).flex().items_center().child(
                    check_icon(checked, 20., cx)
                        .h(px(19.6))
                        .when(self.disabled, |el| el.opacity(0.3)),
                ),
            )
            .child(
                div()
                    .ml(px(4.))
                    .min_w_0()
                    .flex()
                    .flex_wrap()
                    .text_color(if self.warn { t.red } else { t.fg_1 })
                    .child(self.text)
                    .children(link),
            );
        if !self.disabled {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.shadow(focus_ring(brand)));
            if let Some(handler) = self.on_change {
                el = el.on_click(move |_, w, cx| {
                    if !link_hit.replace(false) {
                        handler(&!checked, w, cx)
                    }
                });
            }
        }
        let el = Semantics::new(el).disabled(self.disabled);
        if self.shake > 0 {
            div()
                .relative()
                .child(el)
                .with_animation(
                    ElementId::NamedInteger("agree-shake".into(), self.shake as u64),
                    Animation::new(Duration::from_millis(300)),
                    |el, delta| el.left(px(shake(delta))),
                )
                .into_any_element()
        } else {
            div().relative().child(el).into_any_element()
        }
    }
}
