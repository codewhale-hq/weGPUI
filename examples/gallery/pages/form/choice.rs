use super::{primary_button, show_toast, toast, toast_presence};
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::components::{
    Cell,
    checkbox::CheckboxCell,
    form::{CellKind, Form, FormCell, FormGroup, form_link, form_tips},
    modal::Presence,
    radio::RadioGroup,
    switch::Switch,
};

pub struct Choice {
    route: &'static str,
    checks: [bool; 2],
    radio: usize,
    switches: [bool; 2],
    toast: Presence,
}

impl Choice {
    pub fn new(
        route: &'static str,
        state: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            route,
            checks: [true, false],
            radio: 1,
            switches: [false, true],
            toast: toast_presence(cx),
        };
        match state.as_deref() {
            Some("toast") => this.toast.show_instantly(window, cx),
            Some("checked") => {
                this.checks = [false, true];
                this.radio = 0;
                this.switches = [true, false];
            }
            _ => {}
        }
        this
    }
}

fn toggle(
    i: usize,
    current: bool,
    cx: &mut Context<Choice>,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    cx.listener(move |this: &mut Choice, _: &ClickEvent, _, cx| {
        this.switches[i] = !current;
        cx.notify();
    })
}

fn set(i: usize, cx: &mut Context<Choice>) -> impl Fn(&bool, &mut Window, &mut App) + 'static {
    cx.listener(move |this: &mut Choice, v: &bool, _, cx| {
        this.switches[i] = *v;
        cx.notify();
    })
}

impl Render for Choice {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let submit =
            cx.listener(|this, _, window, cx| show_toast(this, window, cx, |t| &mut t.toast));
        let add_more = |id: &'static str| {
            Cell::new(id)
                .title("添加更多")
                .link()
                .form()
                .on_click(|_, _, _| {})
        };
        let form = match self.route {
            "form_access" => Form::new("weui-form").title("跳转列表项").child(
                FormGroup::new("group-1")
                    .first()
                    .row(
                        Cell::new("access-1")
                            .title("cell standard")
                            .access()
                            .form()
                            .on_click(|_, _, _| {}),
                    )
                    .row(
                        Cell::new("access-2")
                            .title("cell standard")
                            .access()
                            .form()
                            .on_click(|_, _, _| {}),
                    ),
            ),
            "form_checkbox" => {
                let labels = ["standard is dealt for u.", "standard is dealicient for u."];
                let mut group = FormGroup::new("group-1").first().role(Role::Group);
                for (i, label) in labels.into_iter().enumerate() {
                    group = group.row_inset(
                        CheckboxCell::new(("check", i), label)
                            .checked(self.checks[i])
                            .on_change(cx.listener(move |this, v: &bool, _, cx| {
                                this.checks[i] = *v;
                                cx.notify();
                            })),
                        56.,
                    );
                }
                let group = group
                    .row_inset(
                        CheckboxCell::new("check-disabled", "standard is dealicient for u.")
                            .checked(true)
                            .disabled(true),
                        56.,
                    )
                    .row(add_more("add-more"));
                Form::new("weui-form")
                    .title("复选框样式展示")
                    .child(group)
                    .opr_area(primary_button("showTooltips", "下一步", false, submit))
                    .tips_area(form_tips(
                        div()
                            .flex()
                            .justify_center()
                            .child("点击下一步即表示")
                            .child(form_link(
                                "agreement",
                                "同意用户协议",
                                false,
                                cx,
                                |_, _, _| {},
                            )),
                        cx,
                    ))
            }
            "form_radio" => {
                let rows = RadioGroup::new("radio1")
                    .option("cell standard")
                    .option("cell standard")
                    .selected(Some(self.radio))
                    .on_change(cx.listener(|this, i: &usize, _, cx| {
                        this.radio = *i;
                        cx.notify();
                    }))
                    .rows(window, cx);
                Form::new("weui-form")
                    .title("单选框样式展示")
                    .child(
                        FormGroup::new("group-1")
                            .first()
                            .role(Role::RadioGroup)
                            .rows(rows, 16.)
                            .row(add_more("add-more")),
                    )
                    .opr_area(primary_button("showTooltips", "确定", false, submit))
            }
            _ => Form::new("weui-form")
                .title("开关样式展示")
                .child(
                    FormGroup::new("group-1")
                        .first()
                        .row(
                            FormCell::new("cell-switch")
                                .kind(CellKind::Switch)
                                .on_tap(toggle(0, self.switches[0], cx))
                                .child("标题文字")
                                .ft(Switch::new("cb")
                                    .label("标题文字")
                                    .checked(self.switches[0])
                                    .on_change(set(0, cx))),
                        )
                        .row(
                            FormCell::new("cell-switch-disabled")
                                .kind(CellKind::Switch)
                                .disabled(true)
                                .child("标题文字")
                                .ft(Switch::new("cb1")
                                    .label("标题文字")
                                    .checked(true)
                                    .disabled(true)),
                        )
                        .row(
                            FormCell::new("cell-switch-cp")
                                .kind(CellKind::Switch)
                                .on_tap(toggle(1, self.switches[1], cx))
                                .child("兼容IE Edge的版本")
                                .ft(Switch::new("switchCP")
                                    .label("兼容IE Edge的版本")
                                    .checked(self.switches[1])
                                    .on_change(set(1, cx))),
                        ),
                )
                .opr_area(primary_button("showTooltips", "确定", false, submit)),
        };
        PageFrame::new(self.route)
            .child(form)
            .fixed(toast(&self.toast))
    }
}
