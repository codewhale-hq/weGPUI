use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Direction, I18n, Theme,
    components::{
        agree::Agree,
        button::{BottomFixedOpr, Button},
        half_screen_dialog::{
            HalfScreenDialog, HdIconButton, action_group, attachment_area, hd_link,
        },
        modal::Presence,
    },
    logical::Logical,
};

const LOREM: &str = "Dolor adipisci quidem consequuntur similique consequuntur doloribus modi possimus sunt voluptas qui Aspernatur natus error quisquam quidem ipsa corrupti! Dignissimos quasi quis natus fugiat odio in? Mollitia molestias error earum.";

pub struct HalfScreenDialogPage {
    dialogs: Vec<Presence>,
    agree: [bool; 2],
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| {
        if let Some(i) = state
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|i| (1..=5).contains(i))
        {
            let this = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                this.update(cx, |page: &mut HalfScreenDialogPage, cx| {
                    page.dialogs[i - 1].show_instantly(window, cx);
                    cx.notify();
                })
                .ok();
            });
        }
        HalfScreenDialogPage {
            dialogs: (0..5).map(|_| Presence::new(cx)).collect(),
            agree: [false; 2],
        }
    })
    .into()
}

fn closer(
    weak: &WeakEntity<HalfScreenDialogPage>,
    i: usize,
) -> impl Fn(&mut Window, &mut App) + Clone + 'static {
    let weak = weak.clone();
    move |window, cx| {
        weak.update(cx, |page, cx| {
            page.dialogs[i].hide(window, cx);
            cx.notify();
        })
        .ok();
    }
}

fn click(
    f: impl Fn(&mut Window, &mut App) + 'static,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    move |_, window, cx| f(window, cx)
}

fn breaks(before: usize, text: &'static str, after: usize) -> Div {
    let line = || div().h(px(14. * 1.4));
    div()
        .children((0..before).map(|_| line()))
        .child(div().child(text))
        .children((0..after).map(|_| line()))
}

fn nickname(dir: Direction) -> Div {
    div()
        .flex()
        .row(dir)
        .items_center()
        .text_size(px(14.))
        .child(
            img("pic_placeholder_46.png")
                .size(px(24.))
                .me(dir, px(8.))
                .rounded_full(),
        )
        .child(tr("昵称"))
}

impl Render for HalfScreenDialogPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dir = I18n::get(cx).direction();
        let t = Theme::get(cx).clone();
        let weak = cx.entity().downgrade();
        let triggers = [
            tr("样式一"),
            tr("样式二"),
            tr("样式三"),
            tr("样式四"),
            tr("样式五"),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, label)| {
            Button::new(label)
                .id(SharedString::from(format!("showDialog{}", i + 1)))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.dialogs[i].show(window, cx);
                    cx.notify();
                }))
        });
        let noop = |_: &ClickEvent, _: &mut Window, _: &mut App| {};

        let d1 = HalfScreenDialog::new("js_dialog_1")
            .title(tr("标题"))
            .subtitle(tr("副标题"))
            .left(
                HdIconButton::new("d1-close", "close-thin", tr("关闭"))
                    .on_click(click(closer(&weak, 0))),
            )
            .right(action_group([
                HdIconButton::new("d1-search", "search", tr("搜索")).into_any_element(),
                HdIconButton::new("d1-more", "more", tr("更多")).into_any_element(),
                hd_link("d1-done", tr("完成"), noop, cx).into_any_element(),
            ]))
            .child(breaks(2, tr("可放自定义内容"), 3))
            .presence(&self.dialogs[0])
            .on_dismiss(closer(&weak, 0));

        let d2 = HalfScreenDialog::new("js_dialog_2")
            .bottom_fixed(true)
            .title(tr("标题"))
            .left(
                HdIconButton::new("d2-close", "slide-down", tr("关闭"))
                    .on_click(click(closer(&weak, 1))),
            )
            .right(HdIconButton::new("d2-more", "more", tr("更多")))
            .child(
                div()
                    .text_size(px(17.))
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(relative(1.4))
                    .child(tr("辅助描述内容，可根据实际需要安排")),
            )
            .child(
                div()
                    .id("d2-tips")
                    .role(Role::ListBoxOption)
                    .pt(px(16.))
                    .text_size(px(14.))
                    .line_height(relative(1.4))
                    .text_color(t.fg_2)
                    .child(format!(
                        "{} {LOREM} {LOREM} {LOREM} {LOREM} {LOREM}",
                        tr("辅助提示内容，可根据实际需要安排")
                    )),
            )
            .footer(
                BottomFixedOpr::new()
                    .button(
                        Button::new(tr("次要操作"))
                            .id("d2-secondary")
                            .on_click(click(closer(&weak, 1))),
                    )
                    .button(
                        Button::new(tr("同意下一步"))
                            .id("d2-primary")
                            .primary()
                            .on_click(click(closer(&weak, 1))),
                    ),
            )
            .presence(&self.dialogs[1])
            .on_dismiss(closer(&weak, 1));

        let d3 = HalfScreenDialog::new("js_dialog_3")
            .large(true)
            .label(tr("弹窗标题"))
            .main(nickname(dir))
            .child(breaks(1, tr("可放自定义内容"), 0))
            .footer(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        BottomFixedOpr::new()
                            .button(
                                Button::new(tr("次要操作"))
                                    .id("d3-secondary")
                                    .on_click(click(closer(&weak, 2))),
                            )
                            .button(
                                Button::new(tr("主要操作"))
                                    .id("d3-primary")
                                    .primary()
                                    .on_click(click(closer(&weak, 2))),
                            ),
                    )
                    .child(attachment_area(
                        hd_link("d3-attach", tr("附加操作"), noop, cx).text_size(px(17.)),
                    )),
            )
            .presence(&self.dialogs[2])
            .on_dismiss(closer(&weak, 2));

        let agree = self.agree;
        let d4 = HalfScreenDialog::new("js_dialog_4")
            .label(tr("弹窗标题"))
            .main(nickname(dir))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        Agree::new("weuiAgree1", tr("阅读并同意"))
                            .link(tr("《相关条款》"), noop)
                            .checked(agree[0])
                            .on_change(cx.listener(|this, v: &bool, _, cx| {
                                this.agree[0] = *v;
                                cx.notify();
                            })),
                    )
                    .child(
                        Agree::new("weuiAgree0", tr("以上信息用于核对账号绑定的身份"))
                            .checked(agree[1])
                            .on_change(cx.listener(|this, v: &bool, _, cx| {
                                this.agree[1] = *v;
                                cx.notify();
                            })),
                    ),
            )
            .footer(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        BottomFixedOpr::new()
                            .button(
                                Button::new(tr("非主要操作"))
                                    .id("d4-secondary")
                                    .on_click(click(closer(&weak, 3))),
                            )
                            .button(
                                Button::new(tr("主要操作"))
                                    .id("d4-primary")
                                    .primary()
                                    .on_click(click(closer(&weak, 3))),
                            ),
                    )
                    .child(attachment_area(
                        hd_link("d4-attach", tr("附加操作"), noop, cx).text_size(px(17.)),
                    )),
            )
            .presence(&self.dialogs[3])
            .on_dismiss(closer(&weak, 3));

        let d5 = HalfScreenDialog::new("js_dialog_5")
            .grab(true)
            .title(tr("标题"))
            .left(
                HdIconButton::new("d5-close", "slide-down", tr("关闭"))
                    .on_click(click(closer(&weak, 4))),
            )
            .right(action_group([
                HdIconButton::new("d5-search", "search", tr("搜索")).into_any_element(),
                HdIconButton::new("d5-more", "more", tr("更多")).into_any_element(),
                Button::new(tr("完成"))
                    .id("d5-done")
                    .primary()
                    .xmini()
                    .into_any_element(),
            ]))
            .child(breaks(2, tr("可放自定义内容"), 3))
            .presence(&self.dialogs[4])
            .on_dismiss(closer(&weak, 4));

        PageFrame::new("half-screen-dialog")
            .title("Half-screen Dialog")
            .desc(tr("半屏组件"))
            .spacing()
            .child(div().flex().flex_col().gap(px(16.)).children(triggers))
            .child(d1)
            .child(d2)
            .child(d3)
            .child(d4)
            .child(d5)
    }
}
