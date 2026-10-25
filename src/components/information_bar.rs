//! WeUI information bar (`weui-tips/weui-information-bar.less`) in its five colour variants, with
//! optional icon, link and close button, plus the bulleted list tips
//! (`weui-tips/weui-list-tips.less`).
//!
//! [`InformationBar`] and [`ListTips`] are stateless; removing a closed bar is up to the caller.

use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

/// Colour variant of an [`InformationBar`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum InformationBarKind {
    /// White text on red, the base `weui-information-bar` colours; the default.
    #[default]
    WarnStrong,
    /// Secondary text and a red icon on a pale red background (`weui-information-bar_warn-weak`).
    WarnWeak,
    /// White text on the grey `FG-2` background (`weui-information-bar_warn-no-color`).
    WarnNoColor,
    /// White text on orange (`weui-information-bar_tips-strong`).
    TipsStrong,
    /// Secondary text on the `BG-1` background (`weui-information-bar_tips-weak`).
    TipsWeak,
}

/// Rounded notice bar exposed as an `Alert` named by its text. The text is centred only when there
/// is no icon, link or close button.
#[derive(IntoElement)]
pub struct InformationBar {
    id: ElementId,
    text: SharedString,
    kind: InformationBarKind,
    icon: Option<SharedString>,
    link: Option<(SharedString, Option<ClickHandler>)>,
    close_label: Option<SharedString>,
    on_close: Option<ClickHandler>,
    floating: bool,
    opacity: f32,
}

impl InformationBar {
    /// Creates an [`InformationBarKind::WarnStrong`] bar showing `text` with the `outlined-warn`
    /// icon.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            kind: InformationBarKind::WarnStrong,
            icon: Some("outlined-warn".into()),
            link: None,
            close_label: None,
            on_close: None,
            floating: false,
            opacity: 1.,
        }
    }
    /// Sets the colour variant (default [`InformationBarKind::WarnStrong`]).
    pub fn kind(mut self, kind: InformationBarKind) -> Self {
        self.kind = kind;
        self
    }
    /// Sets the leading 24 px icon by asset name; `None` removes it (default `outlined-warn`).
    pub fn icon(mut self, name: Option<SharedString>) -> Self {
        self.icon = name;
        self
    }
    /// Adds a trailing focusable link with the `Link` role; the handler receives the click event.
    pub fn link(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.link = Some((label.into(), Some(Box::new(handler))));
        self
    }
    /// Adds a close button with this accessible label; it does nothing until
    /// [`InformationBar::on_close`] is set.
    pub fn closable(mut self, label: impl Into<SharedString>) -> Self {
        self.close_label = Some(label.into());
        self
    }
    /// Sets the close button's handler, adding the button with label "关闭" if `closable` was not
    /// called. The caller removes the bar.
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        if self.close_label.is_none() {
            self.close_label = Some("关闭".into());
        }
        self.on_close = Some(Box::new(handler));
        self
    }
    /// Positions the bar absolutely, 8 px from the top, left and right of its parent (default
    /// `false`).
    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = floating;
        self
    }
    /// Sets the bar's opacity, for example during a caller-driven fade (default `1.0`).
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

impl RenderOnce for InformationBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx).clone();
            let (fg, bg, icon, link) = match self.kind {
                InformationBarKind::WarnStrong => (
                    t.information_bar_fg,
                    t.information_bar_bg,
                    t.information_bar_icon,
                    t.information_bar_link,
                ),
                InformationBarKind::WarnWeak => (
                    t.information_bar_warn_weak_fg,
                    t.information_bar_warn_weak_bg,
                    t.information_bar_warn_weak_icon,
                    t.information_bar_warn_weak_link,
                ),
                InformationBarKind::WarnNoColor => (
                    t.information_bar_fg,
                    t.information_bar_no_color_bg,
                    t.information_bar_icon,
                    t.information_bar_link,
                ),
                InformationBarKind::TipsStrong => (
                    t.information_bar_fg,
                    t.information_bar_tips_strong_bg,
                    t.information_bar_icon,
                    t.information_bar_link,
                ),
                InformationBarKind::TipsWeak => (
                    t.information_bar_tips_weak_fg,
                    t.information_bar_tips_weak_bg,
                    t.information_bar_tips_weak_icon,
                    t.information_bar_tips_weak_link,
                ),
            };
            let has_hd = self.icon.is_some();
            let focus = fg;
            let has_ft = self.link.is_some() || self.close_label.is_some();
            let ft = has_ft.then(|| {
                div()
                    .flex()
                    .items_center()
                    .when_some(self.link, |el, (label, handler)| {
                        el.child(
                            div()
                                .id("information-bar-link")
                                .role(Role::Link)
                                .aria_label(label.clone())
                                .border_b_2()
                                .border_color(transparent_black())
                                .mb(px(-2.))
                                .text_color(link)
                                .cursor_pointer()
                                .focusable()
                                .tab_stop(true)
                                .active(|s| s.opacity(0.5))
                                .focus_visible(move |s| s.border_color(focus))
                                .when_some(handler, |el, h| el.on_click(h))
                                .child(label),
                        )
                    })
                    .when_some(self.close_label, |el, label| {
                        el.child(
                            div()
                                .id("information-bar-close")
                                .role(Role::Button)
                                .aria_label(label)
                                .ml(px(6.))
                                .my(px(-2.))
                                .mr(px(-2.))
                                .rounded(px(4.))
                                .border_2()
                                .border_color(transparent_black())
                                .cursor_pointer()
                                .focusable()
                                .tab_stop(true)
                                .active(|s| s.opacity(0.5))
                                .focus_visible(move |s| s.border_color(focus))
                                .when_some(self.on_close, |el, h| el.on_click(h))
                                .child(Icon::new("close-thin").size(24.).color(fg)),
                        )
                    })
            });
            div()
                .id(self.id)
                .role(Role::Alert)
                .aria_label(self.text.clone())
                .flex()
                .items_center()
                .py(px(12.))
                .px(px(16.))
                .rounded(px(8.))
                .text_size(px(14.))
                .line_height(relative(1.6))
                .text_color(fg)
                .bg(bg)
                .opacity(self.opacity)
                .when(self.floating, |el| {
                    el.absolute().top(px(8.)).left(px(8.)).right(px(8.))
                })
                .when_some(self.icon, |el, name| {
                    el.child(
                        div().flex().items_center().child(
                            div()
                                .mr(px(8.))
                                .child(Icon::new(name).size(24.).color(icon)),
                        ),
                    )
                })
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .when(!has_hd && !has_ft, |el| el.text_center())
                        .child(self.text),
                )
                .children(ft)
        })
    }
}

/// Bulleted tips list (`weui-list-tips`): 14 px secondary text under a hairline, one `ListItem` per
/// tip.
#[derive(IntoElement)]
pub struct ListTips {
    id: ElementId,
    items: Vec<SharedString>,
    last: bool,
}

impl Default for ListTips {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

impl ListTips {
    /// Creates an empty list styled as the last block, with no bottom padding.
    #[track_caller]
    pub fn new() -> Self {
        Self {
            id: ElementId::from(std::panic::Location::caller()),
            items: vec![],
            last: true,
        }
    }
    /// Sets a stable list id; use distinct ids for lists created in the same loop.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    /// Appends a bulleted tip.
    pub fn item(mut self, text: impl Into<SharedString>) -> Self {
        self.items.push(text.into());
        self
    }
    /// Whether this is the last block (default `true`); `false` keeps the 24 px bottom padding that
    /// upstream removes from the last child.
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }
}

impl RenderOnce for ListTips {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let count = self.items.len();
        div()
            .id(self.id)
            .role(Role::List)
            .relative()
            .pt(px(24.))
            .when(!self.last, |el| el.pb(px(24.)))
            .flex()
            .flex_col()
            .text_size(px(14.))
            .line_height(relative(1.4))
            .text_color(t.fg_1)
            .children(self.items.into_iter().enumerate().map(move |(i, text)| {
                div()
                    .id(ElementId::NamedInteger("list-tips-item".into(), i as u64))
                    .role(Role::ListItem)
                    .aria_position_in_set(i)
                    .aria_size_of_set(count)
                    .aria_label(text.clone())
                    .relative()
                    .pl(px(15.))
                    .when(i > 0, |el| el.mt(px(16.)))
                    .child(div().absolute().left_0().top(px(-1.4)).child("\u{2022}"))
                    .child(text)
            }))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(0.5))
                    .bg(t.fg_3),
            )
    }
}
