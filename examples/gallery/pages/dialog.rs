use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    button::Button,
    dialog::{Dialog, DialogButton},
    modal::Presence,
};

const BODY: &str = "弹窗内容，告知当前状态、信息和解决方法，描述文字尽量控制在三行内";
const STATES: [&str; 5] = ["ios1", "ios2", "ios3", "android1", "android2"];

pub struct DialogPage {
    dialogs: Vec<Presence>,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| {
        let dialogs = (0..STATES.len())
            .map(|_| Dialog::presence_for(cx))
            .collect();
        if let Some(i) = state.and_then(|s| STATES.iter().position(|x| *x == s)) {
            let this = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                this.update(cx, |page: &mut DialogPage, cx| {
                    page.dialogs[i].show_instantly(window, cx);
                    cx.notify();
                })
                .ok();
            });
        }
        DialogPage { dialogs }
    })
    .into()
}

impl DialogPage {
    fn show(&mut self, i: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.dialogs[i].show(window, cx);
        cx.notify();
    }
}

fn hide(entity: &WeakEntity<DialogPage>, i: usize) -> impl Fn(&mut Window, &mut App) + 'static {
    let entity = entity.clone();
    move |window, cx| {
        entity
            .update(cx, |page, cx| {
                page.dialogs[i].hide(window, cx);
                cx.notify();
            })
            .ok();
    }
}

impl Render for DialogPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.entity().downgrade();
        let trigger = |label: &'static str, i: usize, cx: &mut Context<Self>| {
            Button::new(label)
                .id(SharedString::from(format!("show-dialog-{i}")))
                .on_click(cx.listener(move |this, _, window, cx| this.show(i, window, cx)))
        };
        let d = &self.dialogs;
        PageFrame::new("dialog")
            .title("Dialog")
            .desc("对话框")
            .spacing()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(trigger("样式一", 0, cx))
                    .child(trigger("样式二", 1, cx))
                    .child(trigger("样式三", 2, cx)),
            )
            .child(
                Dialog::new("iosDialog1")
                    .title("弹窗标题")
                    .body(BODY)
                    .button(
                        DialogButton::new("辅助操作")
                            .default_style()
                            .on_click(hide(&weak, 0)),
                    )
                    .button(
                        DialogButton::new("主操作")
                            .primary()
                            .on_click(hide(&weak, 0)),
                    )
                    .presence(&d[0])
                    .on_dismiss(hide(&weak, 0)),
            )
            .child(
                Dialog::new("iosDialog2")
                    .body(BODY)
                    .button(
                        DialogButton::new("辅助操作")
                            .default_style()
                            .on_click(hide(&weak, 1)),
                    )
                    .button(
                        DialogButton::new("负向操作")
                            .warn()
                            .on_click(hide(&weak, 1)),
                    )
                    .presence(&d[1])
                    .on_dismiss(hide(&weak, 1)),
            )
            .child(
                Dialog::new("iosDialog5")
                    .title("弹窗标题")
                    .body(BODY)
                    .btn_wrap(true)
                    .button(
                        DialogButton::new("主操作")
                            .primary()
                            .on_click(hide(&weak, 2)),
                    )
                    .button(
                        DialogButton::new("辅助操作")
                            .default_style()
                            .on_click(hide(&weak, 2)),
                    )
                    .presence(&d[2])
                    .on_dismiss(hide(&weak, 2)),
            )
            .child(
                Dialog::new("androidDialog1")
                    .android(true)
                    .title("弹窗标题")
                    .body(BODY)
                    .button(
                        DialogButton::new("辅助操作")
                            .default_style()
                            .on_click(hide(&weak, 3)),
                    )
                    .button(
                        DialogButton::new("主操作")
                            .primary()
                            .on_click(hide(&weak, 3)),
                    )
                    .presence(&d[3])
                    .on_dismiss(hide(&weak, 3)),
            )
            .child(
                Dialog::new("androidDialog2")
                    .android(true)
                    .body(BODY)
                    .button(
                        DialogButton::new("辅助操作")
                            .default_style()
                            .on_click(hide(&weak, 4)),
                    )
                    .button(
                        DialogButton::new("主操作")
                            .primary()
                            .on_click(hide(&weak, 4)),
                    )
                    .presence(&d[4])
                    .on_dismiss(hide(&weak, 4)),
            )
    }
}
