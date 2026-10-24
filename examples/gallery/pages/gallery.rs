use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    Presence,
    gallery::{Gallery, gallery_presence},
};

pub struct GalleryPage {
    route: &'static str,
    viewer: Presence,
}

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let closed = args.state.as_deref() == Some("closed");
    let page = cx.new(|cx| GalleryPage {
        route,
        viewer: gallery_presence(cx),
    });
    if !closed {
        let weak = page.downgrade();
        window.defer(cx, move |window, cx| {
            weak.update(cx, |this, cx| {
                this.viewer.show(window, cx);
                cx.notify();
            })
            .ok();
        });
    }
    page.into()
}

impl Render for GalleryPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = cx.entity().downgrade();
        PageFrame::new(self.route)
            .title("Gallery")
            .desc("画廊，可实现上传图片的展示或幻灯片播放")
            .no_scroll()
            .child(
                Gallery::new("gallery", &self.viewer)
                    .src("pic_article.png")
                    .label("图片详情")
                    .on_delete(cx.listener(|this, _, window, cx| {
                        this.viewer.hide(window, cx);
                        cx.notify();
                    }))
                    .on_dismiss(move |window, cx| {
                        page.update(cx, |this, cx| {
                            this.viewer.hide(window, cx);
                            cx.notify();
                        })
                        .ok();
                    }),
            )
    }
}
