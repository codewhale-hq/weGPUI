//! WeUI uploader (`weui-cell/weui-uploader.less`): a titled, wrapping grid of 96 px thumbnails with
//! progress, error and delete states, followed by an add box (`weui-uploader__input-box`).
//!
//! [`Uploader`] only renders [`UploaderFile`]s in their [`UploaderStatus`]. Choosing and
//! transporting files is application-owned: open a chooser in `on_add` and update the statuses.
//! [`Uploader::max_files`] can hide the add box at capacity without discarding supplied files.
//! Transport, retry policy and status updates remain application-owned.

use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Display state of an [`UploaderFile`].
#[derive(Clone, PartialEq)]
pub enum UploaderStatus {
    /// Uploaded: shows the thumbnail and allows preview and delete.
    Done,
    /// Failed: dims the thumbnail and shows a red warning icon, exposed as an `Alert` named "错误".
    Error,
    /// In progress: dims the thumbnail and shows the given text, such as `"50%"`, as an `Alert`.
    Progress(SharedString),
}

/// One file shown by an [`Uploader`].
#[derive(Clone)]
pub struct UploaderFile {
    /// Thumbnail as a GPUI `ImageSource` (asset name, URI or filesystem path), cropped to cover the
    /// 96 px square.
    pub src: ImageSource,
    /// Display state (default [`UploaderStatus::Done`]).
    pub status: UploaderStatus,
    /// Accessible name of the thumbnail (default "图片标题").
    pub label: SharedString,
}

impl UploaderFile {
    /// Creates a done file showing `src` with the default label.
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Self {
            src: src.into(),
            status: UploaderStatus::Done,
            label: "图片标题".into(),
        }
    }
    /// Sets the status to [`UploaderStatus::Error`].
    pub fn error(mut self) -> Self {
        self.status = UploaderStatus::Error;
        self
    }
    /// Sets the presentation status. The uploader does not initiate transport or mutate it.
    pub fn status(mut self, status: UploaderStatus) -> Self {
        self.status = status;
        self
    }
    /// Sets the status to [`UploaderStatus::Progress`] with `text`.
    pub fn progress(mut self, text: impl Into<SharedString>) -> Self {
        self.status = UploaderStatus::Progress(text.into());
        self
    }
    /// Sets the thumbnail's accessible name.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

type IndexHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

/// Uploader body: a header with title and info, then the files in a wrapping row and the add box.
#[derive(IntoElement)]
pub struct Uploader {
    id: ElementId,
    title: SharedString,
    info: Option<SharedString>,
    files: Vec<UploaderFile>,
    deletable: bool,
    hint: SharedString,
    add_label: SharedString,
    max_files: Option<usize>,
    add_focus: Option<FocusHandle>,
    on_preview: Option<IndexHandler>,
    on_delete: Option<IndexHandler>,
    on_add: Option<ClickHandler>,
}

impl Uploader {
    /// Creates an uploader titled `title` with no files, deletion off and add label "上传图片".
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            info: None,
            files: vec![],
            deletable: false,
            hint: "轻点两下查看大图".into(),
            add_label: "上传图片".into(),
            max_files: None,
            add_focus: None,
            on_preview: None,
            on_delete: None,
            on_add: None,
        }
    }
    /// Sets the right-aligned header text (`weui-uploader__info`), such as a `"0/2"` count; it is
    /// read together with the title.
    pub fn info(mut self, info: impl Into<SharedString>) -> Self {
        self.info = Some(info.into());
        self
    }
    /// Appends files in display order; callbacks receive indices into this list.
    pub fn files(mut self, files: impl IntoIterator<Item = UploaderFile>) -> Self {
        self.files.extend(files);
        self
    }
    /// Shows a delete button on done files when `on_delete` is also set (default `false`).
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = deletable;
        self
    }
    /// Sets the add box's accessible label (default "上传图片").
    pub fn add_label(mut self, label: impl Into<SharedString>) -> Self {
        self.add_label = label.into();
        self
    }
    /// Sets the accessible description of a done thumbnail (default "轻点两下查看大图").
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = hint.into();
        self
    }
    /// Hides the add box when the supplied file count reaches `maximum`. All statuses count;
    /// zero hides it even when empty. No files are removed and no chooser/transport limit is
    /// enforced: the application must also bound files returned by its chooser. If a focused add
    /// box disappears, focus advances to the next surviving stop (or blurs when none exists).
    /// Unset by default.
    pub fn max_files(mut self, maximum: usize) -> Self {
        self.max_files = Some(maximum);
        self
    }
    /// Tracks the add box with `focus` when it is shown and has `on_add`. A caller removing the
    /// focused thumbnail can focus this handle after updating its files. Hidden add boxes are
    /// not focus targets, so choose another surviving control while at capacity.
    pub fn track_add_focus(mut self, focus: &FocusHandle) -> Self {
        self.add_focus = Some(focus.clone());
        self
    }
    /// Makes done thumbnails focusable and calls the handler with the file index on click or
    /// Enter/Space.
    pub fn on_preview(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_preview = Some(Rc::new(handler));
        self
    }
    /// Called with the file index when a done file's delete button is activated; requires
    /// `deletable(true)`.
    pub fn on_delete(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_delete = Some(Rc::new(handler));
        self
    }
    /// Called with the click event when the add box is clicked or activated with Enter or Space.
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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let can_add = self
                .max_files
                .is_none_or(|maximum| self.files.len() < maximum);
            let add_focus = self.add_focus.unwrap_or_else(|| {
                window
                    .use_keyed_state(
                        ElementId::NamedChild(
                            std::sync::Arc::new(self.id.clone()),
                            "add-focus".into(),
                        ),
                        cx,
                        |_, cx| cx.focus_handle(),
                    )
                    .read(cx)
                    .clone()
            });
            if !can_add && add_focus.is_focused(window) {
                window.focus_next(cx);
                if add_focus.is_focused(window) {
                    window.blur(cx);
                }
            }
            let t = Theme::get(cx).clone();
            let dark = Theme::get(cx).is_dark;
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
                                .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
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
                                                .focus_visible(move |s| {
                                                    s.shadow(super::form::dual_focus_ring())
                                                })
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
                            UploaderStatus::Progress(text) => {
                                base.aria_label(text.clone()).child(text)
                            }
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
            let add_focus_color = t.focus_color(box_bg.into());
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
                .when(self.on_add.is_some(), |el| {
                    el.track_focus(&add_focus)
                        .focusable()
                        .tab_stop(true)
                        .cursor_pointer()
                        .focus_visible(move |s| s.border_color(add_focus_color))
                })
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
                        .when(can_add, |el| el.child(add)),
                )
        })
    }
}
