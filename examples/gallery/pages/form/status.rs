use super::{focus_of, input, primary_button, show_toast, toast, toast_presence};
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::{
    components::{
        form::{ClearableInput, Form, FormCell, FormGroup},
        modal::Presence,
        toptips::{Toptips, ToptipsState},
    },
    input::{InputEvent, InputKind, TextInput},
};

const W: f32 = 17. * 314. / 100.;

pub struct Status {
    card: Entity<TextInput>,
    card_field: Entity<ClearableInput>,
    holder: Entity<TextInput>,
    amount: Entity<TextInput>,
    email: Entity<TextInput>,
    wechat: Entity<TextInput>,
    warn_text: Option<String>,
    tips: ToptipsState,
    toast: Presence,
    _subscriptions: Vec<Subscription>,
}

impl Status {
    pub fn new(state: Option<String>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let card = input("请输入16位数卡号", InputKind::Text, "", cx);
        card.update(cx, |i, cx| i.set_max_len(Some(16), cx));
        let card_field = cx.new(|cx| ClearableInput::new(card.clone(), window, cx));
        let holder = input("请输入持卡人姓名", InputKind::Text, "", cx);
        let amount = input("请输入金额", InputKind::Text, "", cx);
        let email = input("请输入EMail", InputKind::Text, "1234567", cx);
        email.update(cx, |i, cx| i.set_readonly(true, cx));
        let wechat = input("请输入微信号", InputKind::Text, "WeUI", cx);
        wechat.update(cx, |i, cx| i.set_disabled(true, cx));
        let subscriptions = vec![
            cx.subscribe(&card, |_, _, _: &InputEvent, cx| cx.notify()),
            cx.subscribe(&amount, |_, _, _: &InputEvent, cx| cx.notify()),
        ];
        let mut this = Self {
            card,
            card_field,
            holder,
            amount,
            email,
            wechat,
            warn_text: None,
            tips: ToptipsState::new(),
            toast: toast_presence(cx),
            _subscriptions: subscriptions,
        };
        match state.as_deref() {
            Some("error") => {
                this.card.update(cx, |i, cx| i.set_text("1234", cx));
                this.warn_text = Some("1234".into());
                this.tips.show_instantly(window);
            }
            Some("amount") => this.amount.update(cx, |i, cx| i.set_text("123456789", cx)),
            Some("toast") => {
                this.card
                    .update(cx, |i, cx| i.set_text("1234567890123456", cx));
                this.toast.show_instantly(window, cx);
            }
            _ => {}
        }
        this
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.tips.is_open() {
            return;
        }
        let text = self.card.read(cx).text().to_string();
        if text.chars().count() < 16 {
            self.warn_text = Some(text);
            self.tips.show(window);
            self.tips
                .hide_after(Duration::from_secs(2), window, cx, |t| &mut t.tips);
            cx.notify();
        } else {
            show_toast(self, window, cx, |t| &mut t.toast);
        }
    }
}

impl Render for Status {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let card_text = self.card.read(cx).text();
        let empty = card_text.is_empty();
        let warn = self.warn_text.as_deref() == Some(card_text);
        let amount_tips = !self.amount.read(cx).text().is_empty();
        let form = Form::new("weui-form")
            .title("输入框状态")
            .desc("可体验表单输入的反馈状态。显示报错信息有两种类型，一种是超过1个输入项的时候，用置顶tipsbar报错信息，告知错误原因，页面聚焦到对应的报错区域，内容标红色，另一种是在当前输入项位置报错，适用于只有1个输入项的时候，用户焦点主要是输入区域。")
            .child(
                FormGroup::new("group-1")
                    .first()
                    .title("表单报错：置顶tipsbar报错信息")
                    .row(
                        FormCell::new("js_cell")
                            .label("卡号")
                            .label_width(W)
                            .warn(warn)
                            .bd_flex()
                            .focus_target(focus_of(&self.card, cx))
                            .child(self.card_field.clone()),
                    )
                    .row(
                        FormCell::new("cell-holder")
                            .label("持卡人")
                            .label_width(W)
                            .bd_flex()
                            .focus_target(focus_of(&self.holder, cx))
                            .child(self.holder.clone()),
                    ),
            )
            .child(
                FormGroup::new("group-2")
                    .title("表单报错：当前项位置报错")
                    .row(
                        FormCell::new("cell-amount")
                            .label("金额")
                            .label_width(W)
                            .bd_flex()
                            .focus_target(focus_of(&self.amount, cx))
                            .child(self.amount.clone()),
                    )
                    .when(amount_tips, |g| {
                        g.tips(
                            div()
                                .id("js_current_tips")
                                .role(Role::Alert)
                                .aria_label("最多支持8位数")
                                .child("最多支持8位数"),
                        )
                        .tips_warn(true)
                    }),
            )
            .child(
                FormGroup::new("group-3")
                    .title("表单只读、置灰")
                    .row(
                        FormCell::new("cell-email")
                            .label("EMail")
                            .label_width(W)
                            .readonly(true)
                            .focus_target(focus_of(&self.email, cx))
                            .child(self.email.clone()),
                    )
                    .row(
                        FormCell::new("cell-wechat")
                            .label("微信号")
                            .label_width(W)
                            .disabled(true)
                            .child(self.wechat.clone()),
                    ),
            )
            .opr_area(primary_button(
                "showTooltips",
                "确定",
                empty,
                cx.listener(|this, _, window, cx| this.submit(window, cx)),
            ));
        PageFrame::new("form_input_status")
            .child(form)
            .fixed(Toptips::new("js_tooltips", &self.tips, "错误提示").warn())
            .fixed(toast(&self.toast))
    }
}
