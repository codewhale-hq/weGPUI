//! Panels from WeUI's `src/style/widget/weui-panel/` and media boxes from
//! `src/style/widget/weui-media-box/`: a [`Panel`] groups [`MediaBox`] rows under an optional title
//! and footer.
//!
//! Descriptions use two shaped lines with ellipsis and common CJK punctuation guards.
//! Complete Unicode line breaking and cross-renderer font parity remain unqualified.
//! Panel rows of any kind get the separators drawn for media boxes.

use super::ClickHandler;
use crate::{Theme, logical::Logical};
use gpui::{prelude::*, *};

/// A panel (`weui-panel`) on theme `bg_2` with full-width top and bottom hairlines.
///
/// Children added through `ParentElement` form the body (`weui-panel__bd`), separated by hairlines
/// inset 16px from the left. The panel is exposed as `Role::Group`, named by its title when set.
#[derive(IntoElement)]
pub struct Panel {
    id: ElementId,
    title: Option<SharedString>,
    footer: Option<AnyElement>,
    margin_top: f32,
    children: Vec<AnyElement>,
}

impl Panel {
    /// Creates an empty panel with element id `id` and a 10px top margin.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            footer: None,
            margin_top: 10.,
            children: vec![],
        }
    }
    /// Sets the header (`weui-panel__hd`): 15px medium text above an inset hairline. It also
    /// names the group for assistive technology. Default: none.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Sets the footer slot (`weui-panel__ft`) below the body, after an inset hairline; typically a
    /// "View more" link cell. Default: none.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    /// Removes the top margin, as WeUI's `:first-child` rule does for the first panel on a page.
    pub fn first(mut self) -> Self {
        self.margin_top = 0.;
        self
    }
    /// Sets the top margin in logical pixels. Default: 10px.
    pub fn margin_top(mut self, margin: f32) -> Self {
        self.margin_top = margin;
        self
    }
}

impl ParentElement for Panel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

impl RenderOnce for Panel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx).clone();
            let dir = crate::i18n::direction(cx);
            let line = t.fg_3;
            let rows = self.children.into_iter().enumerate().map(|(i, child)| {
                div().relative().child(child).when(i > 0, |el| {
                    el.child(
                        hairline(line)
                            .top_0()
                            .inset_s(dir, px(16.))
                            .inset_e(dir, px(0.)),
                    )
                })
            });
            div()
                .id(self.id)
                .role(Role::Group)
                .when_some(self.title.clone(), |el, title| el.aria_label(title))
                .relative()
                .mt(px(self.margin_top))
                .flex()
                .flex_col()
                .overflow_hidden()
                .bg(t.bg_2)
                .when_some(self.title, |el, title| {
                    el.child(
                        div()
                            .relative()
                            .pt(px(16.))
                            .px(px(16.))
                            .pb(px(13.))
                            .text_size(px(15.))
                            .line_height(px(24.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(t.fg_0)
                            .child(title)
                            .child(
                                hairline(line)
                                    .bottom_0()
                                    .inset_s(dir, px(15.))
                                    .inset_e(dir, px(0.)),
                            ),
                    )
                })
                .child(div().flex().flex_col().children(rows))
                .when_some(self.footer, |el, footer| {
                    el.child(
                        div().relative().child(footer).child(
                            hairline(line)
                                .top_0()
                                .inset_s(dir, px(16.))
                                .inset_e(dir, px(0.)),
                        ),
                    )
                })
                .child(hairline(line).top_0().left_0().right_0())
                .child(hairline(line).bottom_0().left_0().right_0())
        })
    }
}

/// Layout of a [`MediaBox`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MediaBoxKind {
    /// `weui-media-box_text`: title, description and meta row stacked.
    #[default]
    Text,
    /// `weui-media-box_appmsg`: a 60px thumbnail to the left of the text.
    Appmsg,
    /// `weui-media-box_small-appmsg`: no padding or text; children such as cells fill the box.
    SmallAppmsg,
}

/// One entry of a media box's meta row (`weui-media-box__info__meta`). It is created only by
/// [`MediaBox::meta`] and [`MediaBox::meta_extra`].
pub struct MediaMeta {
    text: SharedString,
    extra: bool,
}

/// A media box row (`weui-media-box`): a one-line 17px title, a two-line 14px description and an
/// optional 13px meta row, padded 16px including a 2px focus-ring border.
///
/// The accessible name defaults to the title and description joined by the locale's list
/// separator ([`crate::Messages::list_separator`]). With
/// [`MediaBox::on_click`] the box is a `Role::Link` Tab stop; otherwise it is a `Role::ListItem`.
/// Children added through `ParentElement` follow the text.
#[derive(IntoElement)]
pub struct MediaBox {
    id: ElementId,
    kind: MediaBoxKind,
    title: Option<SharedString>,
    desc: Option<SharedString>,
    thumb: Option<SharedString>,
    meta: Vec<MediaMeta>,
    label: Option<SharedString>,
    description: Option<SharedString>,
    role: Option<Role>,
    on_click: Option<ClickHandler>,
    children: Vec<AnyElement>,
}

impl MediaBox {
    /// Creates an empty [`MediaBoxKind::Text`] box with element id `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            kind: MediaBoxKind::Text,
            title: None,
            desc: None,
            thumb: None,
            meta: vec![],
            label: None,
            description: None,
            role: None,
            on_click: None,
            children: vec![],
        }
    }
    /// Same as [`MediaBox::new`].
    pub fn text(id: impl Into<ElementId>) -> Self {
        Self::new(id)
    }
    /// Creates a [`MediaBoxKind::Appmsg`] box with the thumbnail `thumb`, a URL or the name of an
    /// asset served by the app's `AssetSource`, fitted inside 60px.
    pub fn appmsg(id: impl Into<ElementId>, thumb: impl Into<SharedString>) -> Self {
        let mut this = Self::new(id);
        this.kind = MediaBoxKind::Appmsg;
        this.thumb = Some(thumb.into());
        this
    }
    /// Creates a [`MediaBoxKind::SmallAppmsg`] box; add its content with `child`.
    pub fn small_appmsg(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id);
        this.kind = MediaBoxKind::SmallAppmsg;
        this
    }
    /// Sets the title (`weui-media-box__title`), truncated to one line. Default: none.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Sets the description (`weui-media-box__desc`), clamped to two lines. Default: none.
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    /// Appends a meta entry (`weui-media-box__info__meta`), such as a source or a time.
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta.push(MediaMeta {
            text: meta.into(),
            extra: false,
        });
        self
    }
    /// Appends a meta entry preceded by a 1px separator line (`weui-media-box__info__meta_extra`).
    pub fn meta_extra(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta.push(MediaMeta {
            text: meta.into(),
            extra: true,
        });
        self
    }
    /// Overrides the accessible name. Default: the title and description joined by the locale's
    /// list separator ([`crate::Messages::list_separator`]).
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the accessible description, for example the meta texts. Default: none.
    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Overrides the accessibility role. Default: `Role::Link` with a click handler, otherwise
    /// `Role::ListItem`.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    /// Makes the box interactive: a Tab stop with a pressed background and two contrasting inset focus bands. The
    /// callback receives the click event on a pointer click, or on Enter or Space while focused.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for MediaBox {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MediaBox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx).clone();
            let dir = crate::i18n::direction(cx);
            let label = self.label.clone().or_else(|| {
                let parts: Vec<&str> = [self.title.as_deref(), self.desc.as_deref()]
                    .into_iter()
                    .flatten()
                    .collect();
                (!parts.is_empty())
                    .then(|| SharedString::from(crate::i18n::messages(cx).join(&parts)))
            });
            let interactive = self.on_click.is_some();
            let role = self.role.unwrap_or(if interactive {
                Role::Link
            } else {
                Role::ListItem
            });
            let title = self.title.map(|title| {
                div()
                    .w_full()
                    .text_size(px(17.))
                    .line_height(relative(1.4))
                    .text_color(t.fg_0)
                    .truncate()
                    .child(title)
            });
            let desc = self.desc.map(|desc| {
                div()
                    .pt(px(4.))
                    .text_size(px(14.))
                    .line_height(relative(1.4))
                    .text_color(t.fg_2)
                    .overflow_hidden()
                    .text_ellipsis()
                    .line_clamp(2)
                    .child(desc)
            });
            let meta_color = t.fg_2;
            let info = (!self.meta.is_empty()).then(|| {
                div()
                    .mt(px(16.))
                    .pb(px(4.))
                    .flex()
                    .row(dir)
                    .text_size(px(13.))
                    .line_height(px(13.))
                    .text_color(meta_color)
                    .children(self.meta.into_iter().map(move |m| {
                        div()
                            .pe(dir, px(13.))
                            .when(m.extra, |el| {
                                let el = el.ps(dir, px(13.));
                                let el = if dir.is_rtl() {
                                    el.border_r_1()
                                } else {
                                    el.border_l_1()
                                };
                                el.border_color(meta_color)
                            })
                            .child(m.text)
                    }))
            });
            let body = div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .text_start(dir)
                .children(title)
                .children(desc)
                .children(info);
            let mut el = div()
                .id(self.id)
                .role(role)
                .when_some(label, |el, label| el.aria_label(label))
                .when_some(self.description, |el, d| el.aria_description(d))
                .relative()
                .border_2()
                .border_color(transparent_black());
            el = match self.kind {
                MediaBoxKind::Text => el.p(px(14.)).child(body),
                MediaBoxKind::Appmsg => el
                    .p(px(14.))
                    .flex()
                    .row(dir)
                    .items_center()
                    .when_some(self.thumb, |el, thumb| {
                        el.child(
                            div()
                                .me(dir, px(16.))
                                .size(px(60.))
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(img(thumb).size(px(60.)).object_fit(ObjectFit::Contain)),
                        )
                    })
                    .child(body),
                MediaBoxKind::SmallAppmsg => el.border_0(),
            };
            el = el.children(self.children);
            if let Some(handler) = self.on_click {
                let pressed = t.bg_color_active;
                el = el
                    .cursor_pointer()
                    .focusable()
                    .tab_stop(true)
                    .active(move |s| s.bg(pressed))
                    .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
                    .on_click(handler);
            }
            el
        })
    }
}
