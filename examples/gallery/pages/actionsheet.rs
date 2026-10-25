use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    action_sheet::{ActionSheet, ActionSheetItem},
    button::Button,
    modal::Presence,
};

pub struct ActionSheetPage {
    ios: Presence,
    android: Presence,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| {
        if let Some(state) = state.filter(|s| s == "ios" || s == "android") {
            let this = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                this.update(cx, |page: &mut ActionSheetPage, cx| {
                    if state == "ios" {
                        page.ios.show_instantly(window, cx);
                    } else {
                        page.android.show_instantly(window, cx);
                    }
                    cx.notify();
                })
                .ok();
            });
        }
        ActionSheetPage {
            ios: Presence::new(cx),
            android: Presence::new(cx).duration(std::time::Duration::from_millis(200)),
        }
    })
    .into()
}

impl Render for ActionSheetPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.entity().downgrade();
        let hide_ios = {
            let weak = weak.clone();
            move |window: &mut Window, cx: &mut App| {
                weak.update(cx, |p, cx| {
                    p.ios.hide(window, cx);
                    cx.notify();
                })
                .ok();
            }
        };
        let hide_android = move |window: &mut Window, cx: &mut App| {
            weak.update(cx, |p, cx| {
                p.android.hide(window, cx);
                cx.notify();
            })
            .ok();
        };
        PageFrame::new("actionsheet")
            .title("ActionSheet")
            .desc(tr("弹出式菜单"))
            .spacing()
            .child(
                Button::new("ActionSheet")
                    .id("showIOSActionSheet")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.ios.show(window, cx);
                        cx.notify();
                    })),
            )
            .child(
                ActionSheet::new("iosActionsheet")
                    .title(tr("这是一个标题，可以为一行或者两行。"))
                    .item(ActionSheetItem::new(tr("示例菜单")))
                    .item(ActionSheetItem::new(tr("示例菜单")).tips(tr("副标题")))
                    .item(ActionSheetItem::new(tr("负向菜单")).warn())
                    .action(ActionSheetItem::new(tr("取消")).on_click(hide_ios.clone()))
                    .presence(&self.ios)
                    .on_dismiss(hide_ios),
            )
            .child(
                ActionSheet::new("androidActionsheet")
                    .android(true)
                    .label(tr("示例菜单"))
                    .item(ActionSheetItem::new(tr("示例菜单")))
                    .item(ActionSheetItem::new(tr("示例菜单")))
                    .item(ActionSheetItem::new(tr("示例菜单")))
                    .presence(&self.android)
                    .on_dismiss(hide_android),
            )
    }
}
