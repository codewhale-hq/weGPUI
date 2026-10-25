use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};
use wegpui::components::information_bar::{InformationBar, InformationBarKind};

const FADE: Duration = Duration::from_millis(200);

const BARS: [(InformationBarKind, &str); 5] = [
    (InformationBarKind::WarnStrong, "信息提示 warn strong"),
    (InformationBarKind::WarnWeak, "信息提示 warn weak"),
    (InformationBarKind::WarnNoColor, "信息提示 warn no color"),
    (InformationBarKind::TipsStrong, "信息提示 tips strong"),
    (InformationBarKind::TipsWeak, "信息提示 tips weak"),
];

pub struct InformationBarPage {
    route: &'static str,
    closing: [Option<Instant>; 5],
}

pub fn build(route: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let mut closing = [None; 5];
    if args.state.as_deref() == Some("closed") {
        closing[0] = Some(Instant::now() - FADE);
    }
    cx.new(|_| InformationBarPage { route, closing }).into()
}

impl Render for InformationBarPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let reduce = cx.reduce_motion();
        let mut bars = vec![];
        for (i, (kind, text)) in BARS
            .into_iter()
            .map(|(kind, text)| (kind, tr(text)))
            .enumerate()
        {
            let opacity = match self.closing[i] {
                None => 1.,
                Some(_) if reduce => 0.,
                Some(at) => 1. - (at.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.),
            };
            if opacity <= 0. {
                continue;
            }
            if opacity < 1. {
                window.request_animation_frame();
            }
            bars.push(
                div().mb(px(20.)).child(
                    InformationBar::new(
                        ElementId::NamedInteger("information-bar".into(), i as u64),
                        text,
                    )
                    .kind(kind)
                    .opacity(opacity)
                    .link(tr("详情"), |_, _, _| {})
                    .on_close(cx.listener(move |this, _, _, cx| {
                        if this.closing[i].is_none() {
                            this.closing[i] = Some(Instant::now());
                            cx.notify();
                        }
                    })),
                ),
            );
        }
        PageFrame::new(self.route)
            .title("Information Bar")
            .desc(tr("信息提示条"))
            .spacing()
            .children(bars)
    }
}
