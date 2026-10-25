//! Load-more indicators from WeUI's `src/style/widget/weui-tips/weui-loadmore.less`: [`LoadMore`]
//! shows a spinner with text, text on a line ("no data"), or an end-of-list dot.
//!
//! Known limitations: it is presentation only and loads nothing; the owner switches the kind when
//! loading ends. The line and dot variants paint the label background in theme `bg_2`, so they
//! assume a `bg_2` surface.

use super::loading::Loading;
use crate::Theme;
use gpui::{prelude::*, *};

/// Visual state of a [`LoadMore`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadMoreKind {
    /// A spinner followed by the text, exposed as an alert so the text is announced.
    #[default]
    Loading,
    /// The text centred on a hairline (`weui-loadmore_line`), for example "暂无数据".
    Line,
    /// A 4px dot centred on a hairline (`weui-loadmore_dot`); the text is only the accessible name.
    Dot,
}

/// A load-more row: 65% of the container width, centred, with 14px theme `fg_1` text.
///
/// Its role is alert for [`LoadMoreKind::Loading`], paragraph for `Line` and list item for `Dot`;
/// the text is always the accessible name.
#[derive(IntoElement)]
pub struct LoadMore {
    id: ElementId,
    text: SharedString,
    kind: LoadMoreKind,
    margin: Option<(f32, f32)>,
}

impl LoadMore {
    /// Creates a [`LoadMoreKind::Loading`] row showing `text`. The element id defaults to
    /// `"loadmore-{text}"`.
    pub fn new(text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            id: ElementId::Name(format!("loadmore-{text}").into()),
            text,
            kind: LoadMoreKind::Loading,
            margin: None,
        }
    }
    /// Same as [`LoadMore::new`]: a spinner with `text`.
    pub fn loading(text: impl Into<SharedString>) -> Self {
        Self::new(text)
    }
    /// Creates a [`LoadMoreKind::Line`] row showing `text`.
    pub fn line(text: impl Into<SharedString>) -> Self {
        Self::new(text).kind(LoadMoreKind::Line)
    }
    /// Creates a [`LoadMoreKind::Dot`] row; `text` is announced but not drawn.
    pub fn dot(text: impl Into<SharedString>) -> Self {
        Self::new(text).kind(LoadMoreKind::Dot)
    }
    /// Sets the visual state. Default: [`LoadMoreKind::Loading`].
    pub fn kind(mut self, kind: LoadMoreKind) -> Self {
        self.kind = kind;
        self
    }
    /// Shorthand for `kind(LoadMoreKind::Line)`. The text is unchanged, so construct the row with
    /// the end-state text.
    pub fn finished(self) -> Self {
        self.kind(LoadMoreKind::Line)
    }
    /// Sets the element id. Default: `"loadmore-{text}"`.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    /// Sets the top and bottom margins in logical pixels. Default: 20px/20px for `Loading`,
    /// 32px/20px for `Line` and `Dot`.
    pub fn margin(mut self, top: f32, bottom: f32) -> Self {
        self.margin = Some((top, bottom));
        self
    }
}

impl RenderOnce for LoadMore {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let (fg, line, bg) = (t.fg_1, t.fg_3, t.bg_2);
        let (mt, mb) = self.margin.unwrap_or(match self.kind {
            LoadMoreKind::Loading => (20., 20.),
            _ => (32., 20.),
        });
        let lh = 14. * 1.6;
        let base = div()
            .id(self.id)
            .w(relative(0.65))
            .mx_auto()
            .mt(px(mt))
            .mb(px(mb))
            .flex()
            .justify_center()
            .text_size(px(14.))
            .line_height(px(lh))
            .text_color(fg);
        match self.kind {
            LoadMoreKind::Loading => crate::accessibility::Semantics::new(
                base.role(Role::Alert)
                    .aria_label(self.text.clone())
                    .items_center()
                    .h(px(lh))
                    .child(div().mr(px(8.)).child(Loading::primary()))
                    .child(self.text),
            )
            .live(accesskit::Live::Assertive)
            .into_any_element(),
            LoadMoreKind::Line => base
                .role(Role::Paragraph)
                .aria_label(self.text.clone())
                .h(px(lh + 1.))
                .pt(px(1.))
                .relative()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(1.))
                        .bg(line),
                )
                .child(
                    div()
                        .relative()
                        .top(px(-0.9 * 14.))
                        .px(px(8.))
                        .bg(bg)
                        .child(self.text),
                )
                .into_any_element(),
            LoadMoreKind::Dot => base
                .role(Role::ListItem)
                .aria_label(self.text)
                .h(px(lh + 1.))
                .pt(px(1.))
                .relative()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(1.))
                        .bg(line),
                )
                .child(
                    div()
                        .relative()
                        .top(px(-0.9 * 14.))
                        .h(px(lh))
                        .px(px(8.))
                        .bg(bg)
                        .child(
                            div()
                                .mt(px(lh * 0.5 - 1.))
                                .size(px(4.))
                                .rounded_full()
                                .bg(line),
                        ),
                )
                .into_any_element(),
        }
    }
}
