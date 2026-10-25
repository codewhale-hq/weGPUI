use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum UploaderStatus {
    Done,
    Error,
    Progress(SharedString),
}

#[derive(Clone)]
pub struct UploaderFile {
    pub src: ImageSource,
    pub status: UploaderStatus,
    pub label: SharedString,
}

impl UploaderFile {
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Self {
            src: src.into(),
            status: UploaderStatus::Done,
            label: "图片标题".into(),
        }
    }
    pub fn error(mut self) -> Self {
        self.status = UploaderStatus::Error;
        self
    }
    pub fn progress(mut self, text: impl Into<SharedString>) -> Self {
        self.status = UploaderStatus::Progress(text.into());
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

type IndexHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Uploader {
    id: ElementId,
    title: SharedString,
    info: Option<SharedString>,
    files: Vec<UploaderFile>,
    deletable: bool,
    hint: SharedString,
    add_label: SharedString,
    on_preview: Option<IndexHandler>,
    on_delete: Option<IndexHandler>,
    on_add: Option<ClickHandler>,
}

impl Uploader {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            info: None,
            files: vec![],
            deletable: false,
            hint: "轻点两下查看大图".into(),
            add_label: "上传图片".into(),
            on_preview: None,
            on_delete: None,
            on_add: None,
        }
    }
    pub fn info(mut self, info: impl Into<SharedString>) -> Self {
        self.info = Some(info.into());
        self
    }
    pub fn files(mut self, files: impl IntoIterator<Item = UploaderFile>) -> Self {
        self.files.extend(files);
        self
    }
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = deletable;
        self
    }
    pub fn add_label(mut self, label: impl Into<SharedString>) -> Self {
        self.add_label = label.into();
        self
    }
    pub fn on_preview(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_preview = Some(Rc::new(handler));
        self
    }
    pub fn on_delete(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_delete = Some(Rc::new(handler));
        self
    }
    pub fn on_add(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_add = Some(Box::new(handler));
        self
    }
}

const SIZE: f32 = 96.;

impl RenderOnce for Uploader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let dark = Theme::get(cx).is_dark;
        let brand = t.brand;
        let white = t.white;
        let red = t.red;
        let header_label = SharedString::from(match &self.info {
            Some(info) => format!("{}，{}", self.title, info),
            None => self.title.to_string(),
        });
        let header = div()
            .id("uploader-hd")
            .role(Role::ListItem)
            .aria_label(header_label)
            .flex()
            .items_center()
            .pb(px(12.))
            .child(div().flex_1().child(self.title))
            .when_some(self.info, |el, info| {
                el.child(div().text_color(t.fg_2).child(info))
            });
        let deletable = self.deletable;
        let files = self.files.into_iter().enumerate().map(|(i, file)| {
            let thumb = img(file.src.clone())
                .size(px(SIZE))
                .object_fit(ObjectFit::Cover);
            let status = file.status.clone();
            let mut item = div()
                .id(ElementId::NamedInteger("uploader-file".into(), i as u64))
                .relative()
                .size(px(SIZE))
                .flex_shrink_0()
                .overflow_hidden()
                .child(thumb);
            match status {
                UploaderStatus::Done => {
                    let mut hit = div()
                        .id("uploader-file-thumb")
                        .role(Role::Image)
                        .aria_label(file.label.clone())
                        .aria_description(self.hint.clone())
                        .absolute()
                        .inset_0()
                        .border_2()
                        .border_color(transparent_black());
                    if let Some(preview) = self.on_preview.clone() {
                        hit = hit
                            .focusable()
                            .tab_stop(true)
                            .cursor_pointer()
                            .focus_visible(move |s| s.border_color(brand))
                            .on_click(move |_, window, cx| preview(&i, window, cx));
                    }
                    item = item.child(hit);
                    if deletable && let Some(delete) = self.on_delete.clone() {
                        item = item
                            .child(
                                div()
                                    .absolute()
                                    .top(px(-1.))
                                    .right(px(-1.))
                                    .size(px(1.))
                                    .rounded_full()
                                    .shadow(vec![BoxShadow {
                                        color: hsla(0., 0., 0., 0.3),
                                        offset: point(px(0.), px(0.)),
                                        blur_radius: px(32.),
                                        spread_radius: px(32.),
                                        inset: false,
                                    }]),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .top(px(4.))
                                    .right(px(4.))
                                    .size(px(16.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .child(Icon::new("close").size(16.).color(white))
                                    .child(
                                        div()
                                            .id("uploader-file-delete")
                                            .role(Role::Button)
                                            .aria_label("删除")
                                            .absolute()
                                            .top(px(-8.))
                                            .right(px(-8.))
                                            .size(px(32.))
                                            .rounded_full()
                                            .border_2()
                                            .border_color(transparent_black())
                                            .focusable()
                                            .tab_stop(true)
                                            .focus_visible(move |s| s.border_color(white))
                                            .on_click(move |_, window, cx| {
                                                cx.stop_propagation();
                                                delete(&i, window, cx)
                                            }),
                                    ),
                            );
                    }
                }
                UploaderStatus::Error | UploaderStatus::Progress(_) => {
                    let base = div().id("uploader-file-status").role(Role::Alert);
                    let content = match status {
                        UploaderStatus::Progress(text) => base.aria_label(text.clone()).child(text),
                        _ => base
                            .aria_label("错误")
                            .child(Icon::new("warn").size(24.).color(red)),
                    };
                    item = item
                        .child(div().absolute().inset_0().bg(hsla(0., 0., 0., 0.5)))
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(white)
                                .child(content),
                        );
                }
            }
            item
        });
        let (box_bg, plus) = if dark {
            (rgb(0x2e2e2e), rgb(0x6d6d6d))
        } else {
            (rgb(0xededed), rgb(0xa3a3a3))
        };
        let add = div()
            .id("uploader-input-box")
            .role(Role::Button)
            .aria_label(self.add_label)
            .relative()
            .size(px(SIZE))
            .flex_shrink_0()
            .bg(box_bg)
            .border_2()
            .border_color(transparent_black())
            .cursor_pointer()
            .focusable()
            .tab_stop(true)
            .focus_visible(move |s| s.border_color(brand))
            .child(
                div()
                    .id("uploader-plus")
                    .absolute()
                    .inset_0()
                    .active(|s| s.opacity(0.7))
                    .child(
                        div()
                            .absolute()
                            .left(px(45.))
                            .top(px(30.))
                            .w(px(2.))
                            .h(px(32.))
                            .bg(plus),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(30.))
                            .top(px(45.))
                            .w(px(32.))
                            .h(px(2.))
                            .bg(plus),
                    ),
            )
            .when_some(self.on_add, |el, add| el.on_click(add));
        div()
            .id(self.id)
            .flex_1()
            .flex()
            .flex_col()
            .child(header)
            .child(
                div()
                    .id("uploader-files")
                    .role(Role::List)
                    .flex()
                    .flex_wrap()
                    .gap(px(8.))
                    .children(files)
                    .child(add),
            )
    }
}
