use crate::{Theme, accessibility::Semantics};
use gpui::{prelude::*, *};
use std::time::Duration;

pub enum VcodeEvent {
    Send,
}

pub struct VcodeButton {
    label: SharedString,
    seconds: u32,
    remaining: u32,
    plain: bool,
    focus: FocusHandle,
    timer: Option<Task<()>>,
}

impl EventEmitter<VcodeEvent> for VcodeButton {}

impl VcodeButton {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            label: "获取验证码".into(),
            seconds: 60,
            remaining: 0,
            plain: false,
            focus: cx.focus_handle().tab_stop(true),
            timer: None,
        }
    }
    pub fn set_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.label = label.into();
        cx.notify();
    }
    pub fn set_seconds(&mut self, seconds: u32) {
        self.seconds = seconds.max(1);
    }
    pub fn set_plain(&mut self, plain: bool, cx: &mut Context<Self>) {
        self.plain = plain;
        cx.notify();
    }
    pub fn is_counting(&self) -> bool {
        self.remaining > 0
    }
    pub fn remaining(&self) -> u32 {
        self.remaining
    }
    pub fn start(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.remaining = self.seconds - 1;
        cx.emit(VcodeEvent::Send);
        cx.notify();
        self.timer = Some(cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let done = this
                    .update(cx, |this, cx| {
                        this.remaining = this.remaining.saturating_sub(1);
                        cx.notify();
                        this.remaining == 0
                    })
                    .unwrap_or(true);
                if done {
                    break;
                }
            }
        }));
    }
    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.remaining = 0;
        self.timer = None;
        cx.notify();
    }
}

impl Focusable for VcodeButton {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for VcodeButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let counting = self.remaining > 0;
        let text: SharedString = if counting {
            format!("重新发送({}s)", self.remaining).into()
        } else {
            self.label.clone()
        };
        let brand = t.brand;
        let mask = t.btn_active_mask;
        let base = div()
            .id("vcode-btn")
            .role(Role::Button)
            .aria_label(text.clone())
            .flex_shrink_0()
            .whitespace_nowrap()
            .cursor_pointer();
        let el = if self.plain {
            let link_active = t.link.opacity(0.5);
            base.relative()
                .h(px(56.))
                .ml(px(5.))
                .pl(px(17. * 0.7))
                .pr(px(17. * 0.6))
                .flex()
                .items_center()
                .text_size(px(17.))
                .text_color(if counting { t.fg_2 } else { t.link })
                .border_b_2()
                .border_color(transparent_black())
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .top_0()
                        .bottom_0()
                        .w(px(0.5))
                        .bg(t.fg_3),
                )
                .when(!counting, |el| {
                    el.active(move |s| s.text_color(link_active))
                })
        } else {
            let bg = t.fg_5;
            base.h(px(32.))
                .px(px(10.))
                .flex()
                .items_center()
                .rounded(px(6.))
                .border_2()
                .border_color(transparent_black())
                .bg(bg)
                .text_size(px(16.))
                .line_height(px(28.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(if counting { t.fg_4 } else { t.fg_0 })
                .when(!counting, |el| el.active(move |s| s.bg(bg.blend(mask))))
        };
        let el = el
            .track_focus(&self.focus)
            .focus_visible(move |s| s.border_color(brand))
            .on_click(cx.listener(|this, _, window, cx| {
                cx.stop_propagation();
                if this.remaining == 0 {
                    this.start(window, cx);
                }
            }))
            .child(text);
        Semantics::new(el).disabled(counting)
    }
}

#[derive(IntoElement)]
pub struct VcodeImage {
    src: SharedString,
    label: SharedString,
}

impl VcodeImage {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            label: "验证码".into(),
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl RenderOnce for VcodeImage {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("vcode-img")
            .role(Role::Image)
            .aria_label(self.label)
            .ml(px(5.))
            .flex_shrink_0()
            .child(img(self.src).h(px(56.)))
    }
}
