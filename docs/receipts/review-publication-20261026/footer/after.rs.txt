//! Page footers from WeUI's `src/style/widget/weui-footer/`: [`Footer`] (`weui-footer`) with
//! [`FooterLink`]s, 12px text lines and optional bottom placement.
//!
//! Fixed footers use window coordinates and are hidden from assistive technology when the
//! surrounding [`crate::accessibility::ModalScope`] has an active modal. Applications supply
//! bottom safe-area space through [`Footer::safe_area_bottom`]. Links have at least 44px targets.
//! Fixed footers paint the page surface so scrolled content does not show through their targets.

use super::ClickHandler;
use crate::{Theme, logical::Logical};
use gpui::{prelude::*, *};

/// A footer link (`weui-footer__link`): 14px text in the theme link colour, with a hairline
/// separator before every link except the first.
///
/// It exposes `Role::Link` named by its text, is a Tab stop and activates on click, Enter or Space;
/// pressing halves its opacity. Its invisible target is at least 44×44 logical pixels without
/// enlarging the visible text row.
#[derive(IntoElement)]
pub struct FooterLink {
    id: ElementId,
    text: SharedString,
    first: bool,
    on_click: Option<ClickHandler>,
}

impl FooterLink {
    /// Creates a link with element id `id` showing `text`.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            first: true,
            on_click: None,
        }
    }
    /// Sets the activation callback. It receives the click event on a pointer click, or on Enter or
    /// Space while focused. Default: none, though the link stays focusable.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for FooterLink {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let (link, line) = (t.link, t.fg_3);
        let dir = crate::i18n::direction(cx);
        let font = window.text_style().font();
        let width = self.text.split('\n').fold(px(0.), |width, text| {
            let text = SharedString::from(text.to_owned());
            let run = TextRun {
                len: text.len(),
                font: font.clone(),
                color: link,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            width.max(
                window
                    .text_system()
                    .shape_line(text, px(14.), &[run], None)
                    .width,
            )
        });
        let horizontal = (px(44.) - width).max(px(0.)) / 2.;
        let target = div()
            .id("link")
            .role(Role::Link)
            .aria_label(self.text.clone())
            .absolute()
            .left(-horizontal)
            .right(-horizontal)
            .top(px(-12.2))
            .bottom(px(-12.2))
            .min_w(px(44.))
            .min_h(px(44.))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(14.))
            .text_color(link)
            .rounded(px(2.))
            .focusable()
            .tab_stop(true)
            .cursor_pointer()
            .active(|s| s.opacity(0.5))
            .child(self.text.clone())
            .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
            .when_some(self.on_click, |el, h| el.on_click(h));
        div()
            .id(self.id)
            .relative()
            .max_w_full()
            .min_w_0()
            .mx(px(8.))
            .text_size(px(14.))
            .when(!self.first, |el| {
                el.child(
                    div()
                        .absolute()
                        .inset_s(dir, px(-8.))
                        .top(px(14. * 0.36))
                        .bottom(px(14. * 0.36))
                        .w(px(0.5))
                        .bg(line),
                )
            })
            .child(
                crate::accessibility::Semantics::new(
                    div()
                        .id("measure")
                        .role(Role::Paragraph)
                        .opacity(0.)
                        .child(self.text),
                )
                .hidden(true),
            )
            .child(crate::scrolling::FocusTarget::new(target))
    }
}

/// A centred footer with 14px text at 20% black (20% white in dark mode, see [`footer_color`]).
///
/// It shows one row of links, then the text lines, then any children added through `ParentElement`.
#[derive(IntoElement)]
pub struct Footer {
    id: ElementId,
    links: Vec<FooterLink>,
    texts: Vec<AnyElement>,
    extra: Vec<AnyElement>,
    fixed_bottom: bool,
    safe_area_bottom: Pixels,
}

impl Default for Footer {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

impl Footer {
    /// Creates an empty footer in normal flow.
    #[track_caller]
    pub fn new() -> Self {
        Self {
            id: ElementId::from(std::panic::Location::caller()),
            links: vec![],
            texts: vec![],
            extra: vec![],
            fixed_bottom: false,
            safe_area_bottom: px(0.),
        }
    }
    /// Sets a stable footer id. Defaults to the constructor's caller location; use distinct ids
    /// for sibling footers created in one helper or loop.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    /// Appends a link to the links row; only the first link has no separator.
    pub fn link(mut self, mut link: FooterLink) -> Self {
        link.first = self.links.is_empty();
        self.links.push(link);
        self
    }
    /// Appends a 12px content line with 16px side padding (`weui-footer__text`). Provide semantic
    /// children such as GPUI [`Text`] for accessibility, or use [`Footer::text_line`] for plain text.
    pub fn text(mut self, text: impl IntoElement) -> Self {
        self.texts.push(text.into_any_element());
        self
    }
    /// Appends an accessible plain-text line, with the same styling as [`Footer::text`].
    pub fn text_line(mut self, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        let index = self.texts.len();
        self.texts.push(
            div()
                .id(("footer-text", index))
                .role(Role::Paragraph)
                .aria_label(text.clone())
                .child(text)
                .into_any_element(),
        );
        self
    }
    /// Fixes the footer to the bottom of the window, full width, with 16px vertical padding
    /// (`weui-footer_fixed-bottom`), independently of parent padding, scrolling and clipping.
    /// Wrap the application root in [`crate::accessibility::ModalScope`] so covered fixed-footer
    /// links are hidden from assistive technology while a modal is open. Default: normal flow.
    pub fn fixed_bottom(mut self) -> Self {
        self.fixed_bottom = true;
        self
    }

    /// Adds application-supplied bottom safe-area padding in logical pixels. For a fixed footer
    /// this is added to its normal 16px bottom padding. Default zero; negative or non-finite
    /// values are treated as zero. The application obtains the inset from its host platform.
    pub fn safe_area_bottom(mut self, inset: Pixels) -> Self {
        self.safe_area_bottom = if inset.as_f32().is_finite() {
            inset.max(px(0.))
        } else {
            px(0.)
        };
        self
    }
}

impl ParentElement for Footer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.extra.extend(elements);
    }
}

/// Returns the footer text colour: black at 20% alpha, or white at 20% alpha in dark mode.
pub fn footer_color(cx: &App) -> Hsla {
    if Theme::get(cx).is_dark {
        rgba(0xffffff33).into()
    } else {
        rgba(0x00000033).into()
    }
}

impl RenderOnce for Footer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = footer_color(cx);
        let dir = crate::i18n::direction(cx);
        let fixed = self.fixed_bottom;
        let footer = div()
            .id(self.id)
            .role(Role::Group)
            .flex()
            .flex_col()
            .items_center()
            .text_center()
            .text_color(color)
            .text_size(px(14.))
            .line_height(relative(1.4))
            .when(fixed, |el| {
                el.w(window.viewport_size().width)
                    .py(px(16.))
                    .bg(Theme::get(cx).bg_0)
                    .occlude()
            })
            .pb(self.safe_area_bottom + if fixed { px(16.) } else { px(0.) })
            .when(!self.links.is_empty(), |el| {
                el.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_y(px(24.4))
                        .max_w_full()
                        .row(dir)
                        .justify_center()
                        .children(self.links),
                )
            })
            .children(
                self.texts
                    .into_iter()
                    .map(|t| div().px(px(16.)).text_size(px(12.)).child(t)),
            )
            .children(self.extra);
        if fixed {
            let hidden = crate::accessibility::background_hidden_flag(window, cx);
            deferred(
                anchored()
                    .anchor(Anchor::BottomLeft)
                    .position(point(px(0.), window.viewport_size().height))
                    .child(crate::accessibility::Semantics::new(footer).hidden_flag(hidden)),
            )
            .into_any_element()
        } else {
            footer.into_any_element()
        }
    }
}
