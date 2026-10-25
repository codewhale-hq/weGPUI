//! Page footers from WeUI's `src/style/widget/weui-footer/`: [`Footer`] (`weui-footer`) with
//! [`FooterLink`]s, 12px text lines and optional bottom placement.
//!
//! Fixed footers use window coordinates and are hidden from assistive technology when the
//! surrounding [`crate::accessibility::ModalScope`] has an active modal. Safe-area insets are not
//! applied, and links keep a text-sized hit area (no 44px `weui-wa-hotarea`).

use super::ClickHandler;
use crate::{Theme, logical::Logical};
use gpui::{prelude::*, *};

/// A footer link (`weui-footer__link`): 14px text in the theme link colour, with a hairline
/// separator before every link except the first.
///
/// It exposes `Role::Link` named by its text, is a Tab stop and activates on click, Enter or Space;
/// pressing halves its opacity.
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx);
            let (link, line) = (t.link, t.fg_3);
            let dir = crate::i18n::direction(cx);
            div()
                .relative()
                .mx(px(8.))
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
                    div()
                        .id(self.id)
                        .role(Role::Link)
                        .aria_label(self.text.clone())
                        .relative()
                        .text_size(px(14.))
                        .text_color(link)
                        .rounded(px(2.))
                        .focusable()
                        .tab_stop(true)
                        .active(|s| s.opacity(0.5))
                        .child(self.text)
                        .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
                        .when_some(self.on_click, |el, h| el.on_click(h)),
                )
        })
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
                el.w(window.viewport_size().width).py(px(16.)).occlude()
            })
            .when(!self.links.is_empty(), |el| {
                el.child(div().flex().row(dir).justify_center().children(self.links))
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
