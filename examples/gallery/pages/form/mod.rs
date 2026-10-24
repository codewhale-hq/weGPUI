use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::{
    Theme,
    components::{
        button::Button,
        form::focus_ring,
        half_screen_dialog::{HalfScreenDialog, HdIconButton},
        modal::Presence,
        toast::Toast,
    },
    input::{InputKind, TextInput},
};

mod choice;
mod select;
mod status;
mod structure;
mod text;
mod vcode;

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    match route {
        "form_page" | "form_page_old" | "form_primary" => cx
            .new(|cx| structure::Structure::new(route, state, window, cx))
            .into(),
        "form_input_status" => cx.new(|cx| status::Status::new(state, window, cx)).into(),
        "form_vcode" | "form_bottom_fixed" => cx
            .new(|cx| vcode::Vcode::new(route, state, window, cx))
            .into(),
        "form_access" | "form_checkbox" | "form_radio" | "form_switch" => cx
            .new(|cx| choice::Choice::new(route, state, window, cx))
            .into(),
        "form_select" | "form_select_primary" => cx
            .new(|cx| select::Select::new(route, state, window, cx))
            .into(),
        "form_textarea" | "form_vertical" => cx.new(|cx| text::Text::new(route, window, cx)).into(),
        _ => cx.new(|_| Index).into(),
    }
}

struct Index;

const LINKS: &[(&str, &str)] = &[
    ("form_page", "表单结构"),
    ("form_input_status", "输入框状态"),
    ("form_vcode", "验证码"),
    ("form_bottom_fixed", "底部悬浮"),
    ("form_checkbox", "复选框"),
    ("form_access", "跳转列表项"),
    ("form_radio", "单选框"),
    ("form_switch", "开关"),
    ("form_select", "原生选择框"),
    ("form_select_primary", "模拟选择框"),
    ("form_textarea", "文本域"),
    ("form_vertical", "上下结构"),
];

impl Render for Index {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PageFrame::new("form")
            .title("Form")
            .desc("表单页")
            .spacing()
            .child(
                div()
                    .pb(px(30.))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.))
                    .children(LINKS.iter().map(|(route, label)| {
                        Button::new(*label)
                            .id(*route)
                            .on_click(crate::chrome::go(route))
                    })),
            )
    }
}

pub(super) fn input(
    placeholder: &str,
    kind: InputKind,
    value: &str,
    cx: &mut App,
) -> Entity<TextInput> {
    let placeholder = SharedString::from(placeholder.to_string());
    let value = SharedString::from(value.to_string());
    cx.new(|cx| {
        let mut input = TextInput::new(placeholder, cx);
        input.set_kind(kind, cx);
        if !value.is_empty() {
            input.set_text(value, cx);
        }
        input
    })
}

pub(super) fn focus_of(input: &Entity<TextInput>, cx: &App) -> FocusHandle {
    input.read(cx).focus_handle(cx)
}

pub(super) fn primary_button(
    id: &'static str,
    label: &'static str,
    disabled: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div().flex().flex_col().child(
        Button::new(label)
            .id(id)
            .primary()
            .disabled(disabled)
            .on_click(on_click),
    )
}

pub(super) fn toast(presence: &Presence) -> impl IntoElement {
    Toast::new("已完成")
        .id("form-toast")
        .standalone()
        .presence(presence)
}

pub(super) fn toast_presence(cx: &mut App) -> Presence {
    Toast::presence_for(cx)
}

pub(super) fn show_toast<T: 'static>(
    this: &mut T,
    window: &mut Window,
    cx: &mut Context<T>,
    get: fn(&mut T) -> &mut Presence,
) {
    let presence = get(this);
    presence.show(window, cx);
    presence.hide_after(Duration::from_secs(2), window, cx, get);
    cx.notify();
}

pub(super) fn dismiss<T: 'static>(
    cx: &Context<T>,
    f: impl Fn(&mut T, &mut Window, &mut Context<T>) + 'static,
) -> impl Fn(&mut Window, &mut App) + Clone + 'static {
    let this = cx.entity().downgrade();
    let f = std::rc::Rc::new(f);
    move |window, cx| {
        let f = f.clone();
        this.update(cx, |this, cx| f(this, window, cx)).ok();
    }
}

pub(super) fn footer(cx: &App) -> impl IntoElement {
    let t = Theme::get(cx);
    let brand = t.brand;
    let meta: Hsla = if t.is_dark {
        hsla(0., 0., 1., 0.2)
    } else {
        hsla(0., 0., 0., 0.2)
    };
    div()
        .flex()
        .flex_col()
        .items_center()
        .text_center()
        .line_height(relative(1.4))
        .child(
            div()
                .id("form-footer-link")
                .role(Role::Link)
                .aria_label("底部链接文本")
                .mx(px(8.))
                .text_size(px(14.))
                .text_color(t.link)
                .focusable()
                .tab_stop(true)
                .focus_visible(move |s| s.shadow(focus_ring(brand)))
                .active(|s| s.opacity(0.5))
                .on_click(|_, _, _| {})
                .child("底部链接文本"),
        )
        .child(
            div()
                .px(px(16.))
                .text_size(px(12.))
                .text_color(meta)
                .child("Copyright © 2008-2019 weui.io"),
        )
}

pub(super) fn info_button(
    id: impl Into<ElementId>,
    cx: &App,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let t = Theme::get(cx);
    let brand = t.brand;
    let (rest, pressed) = (t.fg_2, t.fg_1);
    div()
        .id(id)
        .role(Role::Button)
        .aria_label("帮助")
        .group("info-btn")
        .flex()
        .rounded_full()
        .focusable()
        .tab_stop(true)
        .cursor_pointer()
        .focus_visible(move |s| s.shadow(focus_ring(brand)))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |e, w, cx| {
            cx.stop_propagation();
            on_click(e, w, cx)
        })
        .child(
            svg()
                .path("icon-info-circle.svg")
                .id("info-icon")
                .size(px(24.))
                .text_color(rest)
                .group_active("info-btn", move |s| s.text_color(pressed)),
        )
}

pub(super) fn half_screen_dialog(
    presence: &Presence,
    on_close: impl Fn(&mut Window, &mut App) + Clone + 'static,
) -> impl IntoElement {
    let close = on_close.clone();
    let line = || div().h(px(14. * 1.4));
    HalfScreenDialog::new("iosDialog1")
        .title("标题")
        .left(
            HdIconButton::new("dialogClose", "close-thin", "关闭")
                .on_click(move |_, w, cx| close(w, cx)),
        )
        .child(
            div()
                .child(line())
                .child(line())
                .child("可放自定义内容")
                .children((0..3).map(|_| line())),
        )
        .presence(presence)
        .on_dismiss(on_close)
}
