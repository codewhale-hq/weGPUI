use super::{dismiss, footer, input, primary_button, show_toast, toast, toast_presence};
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{
        Cell,
        form::{CellKind, Form, FormCell, FormGroup, cell_control, cell_control_flex, form_tips},
        modal::Presence,
        select::{SelectCell, SelectMenu},
        switch::Switch,
        textarea::TextArea,
    },
    input::{InputEvent, InputKind, TextInput},
};

const ACCOUNTS: [&str; 3] = ["微信号", "QQ号", "Email"];
const CODES: [&str; 4] = ["+86", "+80", "+84", "+87"];
const COUNTRIES: [&str; 3] = ["中国", "美国", "英国"];

pub struct Structure {
    route: &'static str,
    wechat: Entity<TextInput>,
    nickname: Entity<TextInput>,
    phone: Entity<TextInput>,
    address: Entity<TextArea>,
    number: Entity<TextInput>,
    mobile: Entity<TextInput>,
    switch_on: bool,
    account: usize,
    code: usize,
    country: usize,
    menu: Option<usize>,
    menu_presence: Presence,
    toast: Presence,
    _subscriptions: Vec<Subscription>,
}

impl Structure {
    pub fn new(
        route: &'static str,
        state: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let wechat = input("填写本人微信号", InputKind::Text, "", cx);
        let nickname = input("填写本人微信号的昵称", InputKind::Text, "", cx);
        let phone = input("填写绑定的电话号码", InputKind::Number, "", cx);
        let number = input("请输入号码", InputKind::Number, "12345678907", cx);
        let mobile = input("请输入号码", InputKind::Number, "12345678907", cx);
        let address = cx.new(|cx| {
            let mut area = TextArea::new("input your address", cx);
            area.set_rows(3, cx);
            area
        });
        let subscriptions = vec![cx.subscribe(&wechat, |_, _, _: &InputEvent, cx| cx.notify())];
        let mut this = Self {
            route,
            wechat,
            nickname,
            phone,
            address,
            number,
            mobile,
            switch_on: false,
            account: 0,
            code: 0,
            country: 0,
            menu: None,
            menu_presence: Presence::new(cx),
            toast: toast_presence(cx),
            _subscriptions: subscriptions,
        };
        match state.as_deref() {
            Some("filled") | Some("toast") => {
                this.wechat.update(cx, |i, cx| i.set_text("weui", cx));
                if state.as_deref() == Some("toast") {
                    this.toast.show_instantly(window, cx);
                }
            }
            Some("switch") => this.switch_on = true,
            Some("select") if route == "form_primary" => {
                this.menu = Some(0);
                this.menu_presence.show_instantly(window, cx);
            }
            _ => {}
        }
        this
    }

    fn open_menu(&mut self, which: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.menu = Some(which);
        self.menu_presence.show(window, cx);
        cx.notify();
    }

    fn input_cell(
        &self,
        id: &'static str,
        label: &'static str,
        input: &Entity<TextInput>,
        width: f32,
        primary: bool,
        _cx: &App,
    ) -> FormCell {
        FormCell::new(id)
            .label(label)
            .label_width(width)
            .label_for(input)
            .when(primary, |el| el.in_primary_group())
            .child(input.clone())
    }
}

impl Render for Structure {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let primary = self.route == "form_primary";
        let width = match self.route {
            "form_page" => 17. * 4.2,
            "form_primary" => 17. * 314. / 100.,
            _ => 105.,
        };
        let empty = self.wechat.read(cx).text().is_empty();
        let mut form = Form::new("weui-form")
            .title("表单结构")
            .desc("展示表单页面的信息结构样式, 分别由头部区域/控件区域/提示区域/操作区域和底部信息区域组成。");
        let group = |id: &'static str| {
            let g = FormGroup::new(id);
            if primary { g.primary() } else { g }
        };
        let mut g1 = group("group-1")
            .first()
            .title("表单组标题")
            .row(self.input_cell("cell-wechat", "微信号", &self.wechat, width, primary, cx))
            .row(self.input_cell("cell-nickname", "昵称", &self.nickname, width, primary, cx));
        if !primary {
            g1 = g1.row(
                self.input_cell("cell-phone", "联系电话", &self.phone, width, false, cx)
                    .label_desc("副标题"),
            );
            if self.route == "form_page" {
                g1 = g1.row(
                    FormCell::new("cell-address")
                        .kind(CellKind::Primary)
                        .label("verification address")
                        .label_width(width)
                        .label_for(self.address.read(cx).input())
                        .child(div().min_h(px(80.)).child(self.address.clone())),
                );
            }
        }
        form = form.child(g1);
        if primary {
            let switch_on = self.switch_on;
            let bg1 = Theme::get(cx).bg_1;
            form = form
                .child(
                    group("group-2")
                        .title("表单组标题")
                        .row(
                            SelectCell::new("select-account", ACCOUNTS[self.account])
                                .in_primary_group()
                                .expanded(self.menu == Some(0) && self.menu_presence.is_open())
                                .on_click(cx.listener(|this, _, w, cx| this.open_menu(0, w, cx))),
                        )
                        .row(
                            SelectCell::new("select-code", CODES[self.code])
                                .label_width(width)
                                .in_primary_group()
                                .expanded(self.menu == Some(1) && self.menu_presence.is_open())
                                .on_click(cx.listener(|this, _, w, cx| this.open_menu(1, w, cx)))
                                .before(self.number.clone()),
                        )
                        .row(
                            SelectCell::new("select-country", COUNTRIES[self.country])
                                .after("国家")
                                .label_width(width)
                                .in_primary_group()
                                .expanded(self.menu == Some(2) && self.menu_presence.is_open())
                                .on_click(cx.listener(|this, _, w, cx| this.open_menu(2, w, cx))),
                        )
                        .row(
                            FormCell::new("cell-mobile")
                                .kind(CellKind::Wrap)
                                .label("手机号")
                                .label_width(width)
                                .in_primary_group()
                                .label_for(&self.mobile)
                                .child(cell_control("+86"))
                                .child(cell_control_flex(self.mobile.clone())),
                        ),
                )
                .child(
                    group("group-3").row(
                        FormCell::new("cell-switch")
                            .kind(CellKind::Switch)
                            .on_tap(cx.listener(move |this, _, _, cx| {
                                this.switch_on = !switch_on;
                                cx.notify();
                            }))
                            .child("标题文字")
                            .ft(Switch::new("switch")
                                .label("标题文字")
                                .checked(switch_on)
                                .surface(bg1)
                                .on_change(cx.listener(|this, v: &bool, _, cx| {
                                    this.switch_on = *v;
                                    cx.notify();
                                }))),
                    ),
                )
                .child(
                    group("group-4").row(
                        Cell::new("cell-access")
                            .title("cell standard")
                            .access()
                            .form()
                            .on_click(|_, _, _| {}),
                    ),
                );
        }
        form = form
            .tips_area(form_tips("表单页提示，居中对齐", cx))
            .opr_area(primary_button(
                "showTooltips",
                "确定",
                empty,
                cx.listener(|this, _, window, cx| show_toast(this, window, cx, |t| &mut t.toast)),
            ))
            .tips_area(form_tips("表单页提示，居中对齐", cx))
            .extra_area(footer(cx));
        let menu = self.menu.map(|which| {
            let (title, options, selected): (&str, &[&str], usize) = match which {
                0 => ("账号类型", &ACCOUNTS, self.account),
                1 => ("区号", &CODES, self.code),
                _ => ("国家", &COUNTRIES, self.country),
            };
            SelectMenu::new("form-select-menu", &self.menu_presence)
                .title(title)
                .options(options.iter().copied())
                .selected(Some(selected))
                .on_select(cx.listener(move |this, i: &usize, window, cx| {
                    match which {
                        0 => this.account = *i,
                        1 => this.code = *i,
                        _ => this.country = *i,
                    }
                    this.menu_presence.hide(window, cx);
                    cx.notify();
                }))
                .on_dismiss(dismiss(cx, |this, window, cx| {
                    this.menu_presence.hide(window, cx);
                    cx.notify();
                }))
        });
        PageFrame::new(self.route)
            .child(form)
            .fixed(toast(&self.toast))
            .children(menu.map(|m| div().child(m)))
    }
}
