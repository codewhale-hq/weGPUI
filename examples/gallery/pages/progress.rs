use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::components::{
    button::{Button, ButtonArea},
    progress::Progress,
};

pub struct ProgressPage {
    route: &'static str,
    values: [f32; 3],
    uploading: bool,
    generation: usize,
    announcement: SharedString,
}

pub fn build(route: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let uploading = args.state.as_deref() == Some("uploading");
    cx.new(|_| ProgressPage {
        route,
        values: if uploading { [40.; 3] } else { [0., 50., 80.] },
        uploading,
        generation: 0,
        announcement: SharedString::default(),
    })
    .into()
}

impl ProgressPage {
    fn start(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.uploading {
            return;
        }
        self.uploading = true;
        self.generation += 1;
        let generation = self.generation;
        cx.spawn_in(window, async move |this, cx| {
            let mut progress = 0u32;
            let mut announced = std::time::Instant::now();
            loop {
                let keep = this
                    .update(cx, |this, cx| {
                        if !this.uploading || this.generation != generation {
                            return false;
                        }
                        if progress > 10 && announced.elapsed() > Duration::from_secs(3) {
                            this.announcement = format!("已上传百分之{progress}").into();
                            announced = std::time::Instant::now();
                        }
                        if progress == 100 {
                            this.announcement = format!("已上传百分之{progress}").into();
                            this.uploading = false;
                            cx.notify();
                            return false;
                        }
                        this.values = [progress as f32; 3];
                        cx.notify();
                        true
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
                progress += 1;
                cx.background_executor()
                    .timer(Duration::from_millis(20))
                    .await;
            }
        })
        .detach();
    }

    fn cancel(&mut self, cx: &mut Context<Self>) {
        if self.uploading {
            self.uploading = false;
            self.announcement = "已取消上传".into();
            cx.notify();
        }
    }
}

impl Render for ProgressPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bars = self.values.iter().enumerate().map(|(i, v)| {
            Progress::new(ElementId::NamedInteger("progress".into(), i as u64))
                .label(format!("上传进度{}", i + 1))
                .value(*v)
                .on_cancel(cx.listener(|this, _, _, cx| this.cancel(cx)))
        });
        let mut body = div().flex().flex_col();
        for (i, bar) in bars.enumerate() {
            if i > 0 {
                body = body.child(div().h(px(25.6)));
            }
            body = body.child(bar);
        }
        PageFrame::new(self.route)
            .title("Progress")
            .desc("进度条")
            .spacing()
            .child(body)
            .child(
                div()
                    .id("progress-announcement")
                    .role(Role::Status)
                    .aria_label(self.announcement.clone())
                    .absolute()
                    .size(px(1.))
                    .opacity(0.),
            )
            .child(
                ButtonArea::new().button(
                    Button::new("上传")
                        .id("btnUpload")
                        .primary()
                        .disabled(self.uploading)
                        .on_click(cx.listener(|this, _, window, cx| this.start(window, cx))),
                ),
            )
    }
}
