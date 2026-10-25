//! WeUI agreement checkbox (`src/style/widget/weui-agree/weui-agree.less`, example
//! `src/example/form/form_vcode.html`). [`Agree`] is a 20px check, description text and an
//! optional terms link, with the `weui-agree_warn` and `weui-agree_animate` (shake) states.
//! With reduced motion the warn state shows without the shake.
use crate::scrolling::RevealFocus;

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

/// An inline consent control (`.weui-agree`, 14px text). The whole line is one `Role::CheckBox`
/// tab stop named by the text plus the link text; the optional link is a separate `Role::Link`
/// tab stop. Controlled; the caller stores the value.
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
    /// Creates an unchecked, enabled line showing `text` (e.g. "阅读并同意") in `fg_1`.
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
    /// Appends link-coloured `text` (e.g. "《相关条款》") after the description. Clicking or
    /// activating it calls `handler` and does not toggle the checkbox. No link by default.
    pub fn link(
        mut self,
        text: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.link = Some((text.into(), Box::new(handler)));
        self
    }
    /// Sets whether the check is drawn and exposed as checked. Default `false`.
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = value;
        self
    }
    /// Disables the checkbox: icon at 30% opacity, not focusable, `on_change` never called, and
    /// flagged disabled. The link stays active. Default `false`.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Shows the description in red (`weui-agree_warn`), e.g. after submitting without consent;
    /// the link keeps its colour. Default `false`.
    pub fn warn(mut self, value: bool) -> Self {
        self.warn = value;
        self
    }
    /// Plays the 300ms horizontal shake (`weui-agree_animate`) whenever `generation` changes to a
    /// new non-zero value; increment a counter to replay it. Default 0 (no shake).
    pub fn shake(mut self, generation: usize) -> Self {
        self.shake = generation;
        self
    }
    /// Calls `handler` with the requested new value (`!checked`) when the line outside the link
    /// is clicked or activated with Enter/Space. Re-render with [`Agree::checked`] to show it.
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
        let el = Semantics::new(el).disabled(self.disabled).reveal_focus();
        if self.shake > 0 && !cx.reduce_motion() {
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
