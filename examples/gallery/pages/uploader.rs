use crate::{
    chrome::{PageFrame, page_hd},
    pages::Args,
};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{
        Cell, Cells, Presence,
        gallery::{Gallery, gallery_presence},
        msg::LinkText,
        uploader::{Uploader, UploaderFile},
    },
};

const PIC: &str = "pic_160.png";

pub struct UploaderPage {
    route: &'static str,
    plain: Vec<UploaderFile>,
    deletable: Vec<UploaderFile>,
    viewer: Presence,
    preview: Presence,
    current: Option<usize>,
    preview_src: Option<SharedString>,
}

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    let page = cx.new(|cx| UploaderPage {
        route,
        plain: vec![
            UploaderFile::new(PIC),
            UploaderFile::new(PIC),
            UploaderFile::new(PIC),
            UploaderFile::new(PIC).error(),
            UploaderFile::new(PIC).progress("50%"),
        ],
        deletable: vec![
            UploaderFile::new(PIC),
            UploaderFile::new(PIC),
            UploaderFile::new(PIC).error(),
            UploaderFile::new(PIC).progress("50%"),
        ],
        viewer: gallery_presence(cx),
        preview: gallery_presence(cx),
        current: None,
        preview_src: None,
    });
    if matches!(state.as_deref(), Some("viewer" | "preview")) {
        let weak = page.downgrade();
        window.defer(cx, move |window, cx| {
            weak.update(cx, |this, cx| {
                if state.as_deref() == Some("viewer") {
                    this.current = Some(0);
                    this.viewer.show_instantly(window, cx);
                } else {
                    this.preview_src = Some(PIC.into());
                    this.preview.show_instantly(window, cx);
                }
                cx.notify();
            })
            .ok();
        });
    }
    page.into()
}

impl Render for UploaderPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let header = page_hd("Uploader", None, cx).child(
            div()
                .mt(px(4.))
                .text_size(px(14.))
                .line_height(relative(1.6))
                .text_color(t.fg_1)
                .child(
                    LinkText::new("uploader-desc")
                        .text("上传组件，一般配合")
                        .link("组件Gallery", |window, cx| {
                            crate::go("gallery", window, cx)
                        })
                        .text("来使用。"),
                ),
        );
        let first = Uploader::new("uploader-plain", "图片上传，缩略图不带删除")
            .info("3/10")
            .files(self.plain.clone())
            .on_preview(cx.listener(|this, ix: &usize, window, cx| {
                this.current = Some(*ix);
                this.viewer.show(window, cx);
                cx.notify();
            }))
            .on_add(cx.listener(|this, _, _, cx| {
                this.plain.push(UploaderFile::new(PIC));
                cx.notify();
            }));
        let second = Uploader::new("uploader-deletable", "图片上传，缩略图带删除")
            .info("3/10")
            .deletable(true)
            .files(self.deletable.clone())
            .on_preview(cx.listener(|this, ix: &usize, window, cx| {
                this.preview_src = this.deletable.get(*ix).map(|f| f.src.clone());
                this.preview.show(window, cx);
                cx.notify();
            }))
            .on_delete(cx.listener(|this, ix: &usize, _, cx| {
                if *ix < this.deletable.len() {
                    this.deletable.remove(*ix);
                    cx.notify();
                }
            }))
            .on_add(cx.listener(|this, _, _, cx| {
                this.deletable.push(UploaderFile::new(PIC));
                cx.notify();
            }));
        let cell = |id: &'static str, uploader: Uploader| {
            Cells::new().child(
                Cell::new(id)
                    .padding(Edges {
                        top: px(16.),
                        bottom: px(24.),
                        left: px(16.),
                        right: px(16.),
                    })
                    .bd(uploader),
            )
        };
        let viewer_page = cx.entity().downgrade();
        let preview_page = viewer_page.clone();
        let viewer_src = self
            .current
            .and_then(|ix| self.plain.get(ix))
            .map(|f| f.src.clone());
        PageFrame::new(self.route)
            .header(header)
            .child(cell("uploader-cell-1", first))
            .child(cell("uploader-cell-2", second))
            .child(
                Gallery::new("gallery-opr", &self.viewer)
                    .label("大图")
                    .when_some(viewer_src, |g, src| g.src(src))
                    .on_delete(cx.listener(|this, _, window, cx| {
                        if let Some(ix) = this.current.take()
                            && ix < this.plain.len()
                        {
                            this.plain.remove(ix);
                        }
                        this.viewer.hide(window, cx);
                        cx.notify();
                    }))
                    .on_dismiss(move |window, cx| {
                        viewer_page
                            .update(cx, |this, cx| {
                                this.viewer.hide(window, cx);
                                cx.notify();
                            })
                            .ok();
                    }),
            )
            .child(
                Gallery::new("gallery-plain", &self.preview)
                    .label("大图")
                    .when_some(self.preview_src.clone(), |g, src| g.src(src))
                    .on_dismiss(move |window, cx| {
                        preview_page
                            .update(cx, |this, cx| {
                                this.preview.hide(window, cx);
                                cx.notify();
                            })
                            .ok();
                    }),
            )
    }
}
