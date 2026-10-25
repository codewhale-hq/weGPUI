use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    button::Button,
    modal::Presence,
    toast::{TOAST_DURATION, Toast, ToastKind},
};

const TOASTS: [(&str, &str, &str, ToastKind); 5] = [
    ("success", "成功提示", "已完成", ToastKind::Success),
    ("fail", "失败提示", "获取链接失败", ToastKind::Warn),
    (
        "longtext",
        "长文案提示",
        "长文案提示详情，长文案提示详情，长文案提示详情",
        ToastKind::Warn,
    ),
    ("loading", "正在加载提示", "正在加载", ToastKind::Loading),
    ("text", "文字提示", "文字提示", ToastKind::Text),
];

pub struct ToastPage {
    toasts: Vec<Presence>,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| {
        if let Some(i) = state.and_then(|s| TOASTS.iter().position(|t| t.0 == s)) {
            let this = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                this.update(cx, |page: &mut ToastPage, cx| {
                    page.toasts[i].show_instantly(window, cx);
                    cx.notify();
                })
                .ok();
            });
        }
        ToastPage {
            toasts: (0..TOASTS.len()).map(|_| Toast::presence_for(cx)).collect(),
        }
    })
    .into()
}

impl ToastPage {
    fn show(&mut self, i: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.toasts[i].is_open() {
            return;
        }
        self.toasts[i].show(window, cx);
        let generation = self.toasts[i].generation();
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(TOAST_DURATION).await;
            this.update_in(cx, |this, window, cx| {
                let toast = &mut this.toasts[i];
                if toast.is_open() && toast.generation() == generation {
                    toast.hide(window, cx);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }
}

impl Render for ToastPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let buttons = TOASTS.iter().enumerate().map(|(i, (id, label, _, _))| {
            Button::new(tr(label))
                .id(SharedString::from(format!("show-toast-{id}")))
                .on_click(cx.listener(move |this, _, window, cx| this.show(i, window, cx)))
        });
        let toasts = TOASTS.iter().enumerate().map(|(i, (id, _, text, kind))| {
            Toast::new(tr(text))
                .id(SharedString::from(format!("toast-{id}")))
                .kind(*kind)
                .presence(&self.toasts[i])
        });
        PageFrame::new("toast")
            .title("Toast")
            .desc(tr("弹出式提示"))
            .spacing()
            .child(div().flex().flex_col().gap(px(16.)).children(buttons))
            .children(toasts)
    }
}
