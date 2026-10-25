use super::{
    dismiss, half_screen_dialog, info_button, input, primary_button, show_toast, toast,
    toast_presence,
};
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::{
    components::{
        agree::Agree,
        button::{BottomFixedOpr, Button},
        form::{
            CellKind, ClearableInput, Form, FormCell, FormGroup, TipsGroup, cell_control_flex,
            form_link,
        },
        modal::Presence,
        vcode::VcodeButton,
    },
    input::{InputEvent, InputKind, TextInput},
    scrolling::RevealFocus,
};

const W: f32 = 17. * 314. / 100.;

pub struct Vcode {
    route: &'static str,
    phones: Vec<Entity<TextInput>>,
    code: Entity<TextInput>,
    code_field: Entity<ClearableInput>,
    vcode: Entity<VcodeButton>,
    agree: [bool; 2],
    warn: bool,
    shake: usize,
    dialog: Presence,
    toast: Presence,
    scroll: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl Vcode {
    pub fn new(
        route: &'static str,
        state: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let rows = if route == "form_bottom_fixed" { 6 } else { 1 };
        let phones = (0..rows)
            .map(|_| input("请输入手机号", InputKind::Number, "12345678907", cx))
            .collect();
        let code = input("输入验证码", InputKind::Text, "", cx);
        code.update(cx, |i, cx| i.set_max_len(Some(6), cx));
        let code_field = cx.new(|cx| ClearableInput::new(code.clone(), window, cx));
        let vcode = cx.new(VcodeButton::new);
        let subscriptions = vec![cx.subscribe(&code, |_, _, _: &InputEvent, cx| cx.notify())];
        let mut this = Self {
            route,
            phones,
            code,
            code_field,
            vcode,
            agree: [false, false],
            warn: false,
            shake: 0,
            dialog: Presence::new(cx),
            toast: toast_presence(cx),
            scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        };
        match state.as_deref() {
            Some("dialog") => this.dialog.show_instantly(window, cx),
            Some("toast") => {
                this.agree = [true, true];
                this.toast.show_instantly(window, cx);
            }
            Some("checked") => this.agree = [true, true],
            Some("warn") => this.warn = true,
            Some("countdown") => this.vcode.update(cx, |v, cx| v.start(window, cx)),
            _ => {}
        }
        this
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.agree[0] {
            self.warn = false;
            show_toast(self, window, cx, |t| &mut t.toast);
        } else if self.route == "form_bottom_fixed" {
            self.shake += 1;
            cx.notify();
        } else {
            self.warn = true;
            cx.notify();
        }
    }

    fn open_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.dialog.show(window, cx);
        cx.notify();
    }

    fn phone_cell(&self, i: usize, cx: &mut Context<Self>) -> FormCell {
        let phone = &self.phones[i];
        let width = if self.route == "form_vcode" { W } else { 105. };
        FormCell::new(("cell-phone", i))
            .label("手机号")
            .label_width(width)
            .label_for(phone)
            .child(phone.clone())
            .ft(info_button(
                ("showIOSDialog1", i),
                cx,
                cx.listener(|this, _, window, cx| this.open_dialog(window, cx)),
            ))
    }
}

impl Render for Vcode {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fixed = self.route == "form_bottom_fixed";
        let tips_link = form_link("no-code", "收不到验证码", true, cx, |_, _, _| {}).reveal_focus();
        let mut group = FormGroup::new("group-1").first();
        for i in 0..self.phones.len() {
            group = group.row(self.phone_cell(i, cx));
        }
        if !fixed {
            group = group.row(
                FormCell::new("js_cell_vcode")
                    .kind(CellKind::Vcode)
                    .label("验证码")
                    .label_width(W)
                    .label_for(&self.code)
                    .child(cell_control_flex(
                        div()
                            .text_size(px(17.))
                            .min_h(px(32.))
                            .flex()
                            .items_center()
                            .child(self.code_field.clone()),
                    ))
                    .child(div().my(px(8.)).ml(px(16.)).child(self.vcode.clone())),
            );
        }
        let group = group.tips(tips_link);
        let agree = |id: &'static str,
                     i: usize,
                     text: &'static str,
                     link: bool,
                     warn: bool,
                     shake: usize,
                     cx: &mut Context<Self>| {
            let a = Agree::new(id, text)
                .checked(self.agree[i])
                .warn(warn)
                .shake(shake)
                .on_change(cx.listener(move |this, v: &bool, _, cx| {
                    this.agree[i] = *v;
                    if i == 0 && *v {
                        this.warn = false;
                    }
                    cx.notify();
                }));
            if link {
                a.link("《相关条款》", |_, _, _| {})
            } else {
                a
            }
        };
        let submit = cx.listener(|this, _, window, cx| this.submit(window, cx));
        let mut form = Form::new("weui-form").title(if fixed {
            "底部悬浮表单"
        } else {
            "验证码"
        });
        form = form
            .desc(if fixed {
                "底部悬浮表单样式"
            } else {
                "验证手机号样式"
            })
            .child(group);
        if fixed {
            let shake = self.shake;
            form = form
                .bottom_fixed(&self.scroll)
                .tips_area(agree("weuiAgree", 0, "阅读并同意", true, false, shake, cx))
                .opr_area(
                    BottomFixedOpr::new().button(
                        Button::new("确定")
                            .id("showTooltips")
                            .primary()
                            .on_click(submit),
                    ),
                );
        } else {
            let warn = self.warn;
            form = form
                .tips_area(
                    TipsGroup::new()
                        .child(agree("weuiAgree1", 0, "阅读并同意", true, warn, 0, cx))
                        .child(agree(
                            "weuiAgree0",
                            1,
                            "以上信息用于核对账号绑定的身份",
                            false,
                            false,
                            0,
                            cx,
                        )),
                )
                .opr_area(primary_button("showTooltips", "确定", false, submit));
        }
        let close = dismiss(cx, |this, window, cx| {
            this.dialog.hide(window, cx);
            cx.notify();
        });
        let frame = PageFrame::new(self.route);
        let frame = if fixed {
            frame
                .no_scroll()
                .fixed(div().absolute().inset_0().child(form))
        } else {
            frame.child(form)
        };
        frame
            .fixed(toast(&self.toast))
            .fixed(half_screen_dialog(&self.dialog, close))
    }
}
