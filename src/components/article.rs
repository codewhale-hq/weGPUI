//! Article typography from WeUI's `src/style/widget/weui-page/weui-article.less`: the [`Article`]
//! container plus [`section`], heading ([`h1`] to [`h5`]), [`paragraph`] and [`ArticleList`]
//! builders.
//!
//! Known limitations: headings and list items hold plain text only, there is no `h6` shorthand, and
//! safe-area insets are not applied. The independent review notes that heading and list semantics
//! still need accessibility work.

use crate::Theme;
use gpui::{prelude::*, *};

const FONT: f32 = 17.;
const LINE: f32 = 17. * 1.6;

/// The article container (`weui-article`): 48px vertical and 24px horizontal padding, 17px theme
/// `fg_0` text at 1.6 line height, exposed as `Role::Article`. Add content with `child`.
#[derive(IntoElement)]
pub struct Article {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl Article {
    /// Creates an empty article with element id `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: vec![],
        }
    }
}

impl ParentElement for Article {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Article {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .role(Role::Article)
            .flex()
            .flex_col()
            .pt(px(48.))
            .pb(px(48.))
            .px(px(24.))
            .text_color(Theme::get(cx).fg_0)
            .text_size(px(FONT))
            .line_height(px(LINE))
            .children(self.children)
    }
}

/// A vertical section with the bottom margin of WeUI's nested `section` rules: 48px at `depth` 0,
/// 32px at depth 1 and 24px deeper.
pub fn section(depth: usize) -> Div {
    div().flex().flex_col().mb(px(match depth {
        0 => 48.,
        1 => 32.,
        _ => 24.,
    }))
}

/// A heading exposed as `Role::Heading` at `level`. Levels 1 to 4 follow WeUI's `h1`–`h4` styles;
/// any other level uses the 17px regular `h5` style with no bottom margin.
///
/// The element id is `h{level}-{text}`, so identical headings under one parent share an id.
pub fn heading(level: usize, text: impl Into<SharedString>) -> Stateful<Div> {
    let text: SharedString = text.into();
    let (size, weight, mb) = match level {
        1 => (22., FontWeight::SEMIBOLD, 48.),
        2 => (20., FontWeight::SEMIBOLD, 16.),
        3 => (17., FontWeight::SEMIBOLD, 8.),
        4 => (17., FontWeight::NORMAL, 4.),
        _ => (17., FontWeight::NORMAL, 0.),
    };
    div()
        .id(SharedString::from(format!("h{level}-{text}")))
        .role(Role::Heading)
        .aria_label(text.clone())
        .aria_level(level)
        .text_size(px(size))
        .font_weight(weight)
        .line_height(relative(1.4))
        .mb(px(mb))
        .when(level == 1, |el| el.text_center())
        .child(text)
}

/// Shorthand for `heading(1, text)`: 22px semibold, centred, 48px bottom margin.
pub fn h1(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(1, text)
}
/// Shorthand for `heading(2, text)`: 20px semibold, 16px bottom margin.
pub fn h2(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(2, text)
}
/// Shorthand for `heading(3, text)`: 17px semibold, 8px bottom margin.
pub fn h3(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(3, text)
}
/// Shorthand for `heading(4, text)`: 17px regular, 4px bottom margin.
pub fn h4(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(4, text)
}
/// Shorthand for `heading(5, text)`: 17px regular, no bottom margin.
pub fn h5(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(5, text)
}

/// A paragraph block with WeUI's 24px bottom margin; add text with `child`.
pub fn paragraph() -> Div {
    div().mb(px(24.))
}

/// Marker style of an [`ArticleList`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ListStyle {
    /// Bullets outside the text and a 1.2em indent, as `ul`.
    #[default]
    Disc,
    /// "1." numbers outside the text and a 1.2em indent, as `ol`.
    Decimal,
    /// Markers inside the text block and no indent at the top level (`weui-article__list_inside`).
    Inside,
    /// No markers and no indent at the top level (`weui-article__list_none`).
    None,
}

/// One list entry: plain text and an optional nested list.
pub struct ListItem {
    text: SharedString,
    nested: Option<ArticleList>,
}

impl ListItem {
    /// Creates an item showing `text`.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            nested: None,
        }
    }
    /// Nests `list` under this item with a 1.2em indent. A nested ordered or unordered list takes
    /// on an `Inside` or `None` parent's style.
    pub fn nested(mut self, list: ArticleList) -> Self {
        self.nested = Some(list);
        self
    }
}

/// A list in article typography, exposed as `Role::List`. Each item is a `Role::ListItem` named by
/// its text, with its position and the set size; items are 0.5em apart.
#[derive(IntoElement)]
pub struct ArticleList {
    style: ListStyle,
    marker: ListStyle,
    nested: bool,
    items: Vec<ListItem>,
}

impl ArticleList {
    /// Creates an empty list in `style`. [`ListStyle::Inside`] uses inline numbers,
    /// equivalent to [`ArticleList::inside`].
    pub fn new(style: ListStyle) -> Self {
        Self {
            style,
            marker: if style == ListStyle::Inside {
                ListStyle::Decimal
            } else {
                style
            },
            nested: false,
            items: vec![],
        }
    }
    /// Creates a bulleted list ([`ListStyle::Disc`]).
    pub fn unordered() -> Self {
        Self::new(ListStyle::Disc)
    }
    /// Creates a numbered list ([`ListStyle::Decimal`]).
    pub fn ordered() -> Self {
        Self::new(ListStyle::Decimal)
    }
    /// Creates a numbered list with inline "1. " prefixes and no indent ([`ListStyle::Inside`]).
    pub fn inside() -> Self {
        Self::new(ListStyle::Inside)
    }
    /// Creates a list without markers or indent ([`ListStyle::None`]).
    pub fn none() -> Self {
        Self::new(ListStyle::None)
    }
    /// Appends an item; a `&'static str` converts into a plain [`ListItem`].
    pub fn item(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
        self
    }
    fn inherit(mut self, parent: ListStyle) -> Self {
        self.nested = true;
        if self.style == ListStyle::Decimal || self.style == ListStyle::Disc {
            if parent == ListStyle::Inside {
                self.style = ListStyle::Inside;
            } else if parent == ListStyle::None {
                self.style = ListStyle::None;
            }
        }
        if parent == ListStyle::None {
            self.marker = ListStyle::None;
        }
        self
    }
}

impl From<&'static str> for ListItem {
    fn from(text: &'static str) -> Self {
        ListItem::new(text)
    }
}

impl RenderOnce for ArticleList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = Theme::get(cx).fg_0;
        let em = FONT;
        let indent = match self.style {
            ListStyle::Disc | ListStyle::Decimal => 1.2 * em,
            _ if self.nested => 1.2 * em,
            _ => 0.,
        };
        let style = self.style;
        let marker_kind = self.marker;
        let count = self.items.len();
        let nested_list = self.nested;
        let list_id = SharedString::from(format!(
            "article-list-{}-{}-{}",
            style as u8,
            count,
            self.items.first().map_or("", |i| i.text.as_ref())
        ));
        div()
            .id(list_id)
            .role(Role::List)
            .flex()
            .flex_col()
            .ml(px(indent))
            .when(self.nested, |el| el.mt(px(0.5 * em)))
            .when(!self.nested, |el| el.mb(px(24.)))
            .children(self.items.into_iter().enumerate().map(move |(i, item)| {
                let marker: Option<SharedString> = match marker_kind {
                    ListStyle::Disc => None,
                    ListStyle::Decimal => Some(format!("{}.", i + 1).into()),
                    _ => None,
                };
                let text = match (style, &marker) {
                    (ListStyle::Inside, Some(m)) => {
                        SharedString::from(format!("{m} {}", item.text))
                    }
                    _ => item.text.clone(),
                };
                let outside = matches!(style, ListStyle::Disc | ListStyle::Decimal);
                div()
                    .id(("article-list-item", i))
                    .role(Role::ListItem)
                    .aria_label(text.clone())
                    .aria_position_in_set(i)
                    .aria_size_of_set(count)
                    .relative()
                    .mt(px(if i == 0 && !nested_list { 0.5 * em } else { 0. }))
                    .mb(px(if i + 1 == count { 0. } else { 0.5 * em }))
                    .when(marker_kind == ListStyle::Disc, |el| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(LINE / 2. - 2.5))
                                .left(px(-19.))
                                .size(px(5.))
                                .rounded_full()
                                .bg(color),
                        )
                    })
                    .when(outside, |el| {
                        el.when_some(marker, |el, m| {
                            el.child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .right(relative(1.))
                                    .pr(px(if style == ListStyle::Disc { 10. } else { 4. }))
                                    .whitespace_nowrap()
                                    .child(m),
                            )
                        })
                    })
                    .child(text)
                    .when_some(item.nested, |el, nested| el.child(nested.inherit(style)))
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn public_constructors_agree_on_marker_and_placement() {
        for (style, shorthand, marker) in [
            (ListStyle::Disc, ArticleList::unordered(), ListStyle::Disc),
            (
                ListStyle::Decimal,
                ArticleList::ordered(),
                ListStyle::Decimal,
            ),
            (ListStyle::Inside, ArticleList::inside(), ListStyle::Decimal),
            (ListStyle::None, ArticleList::none(), ListStyle::None),
        ] {
            let explicit = ArticleList::new(style);
            assert!(explicit.style == shorthand.style);
            assert!(explicit.marker == marker);
            assert!(shorthand.marker == marker);
            assert!(!explicit.nested);
        }
    }

    #[test]
    fn nested_lists_inherit_placement_without_losing_marker_kind() {
        let ordered = ArticleList::ordered().inherit(ListStyle::Inside);
        assert!(ordered.nested);
        assert!(ordered.style == ListStyle::Inside);
        assert!(ordered.marker == ListStyle::Decimal);
        let unordered = ArticleList::unordered().inherit(ListStyle::Inside);
        assert!(unordered.style == ListStyle::Inside);
        assert!(unordered.marker == ListStyle::Disc);
        for list in [
            ArticleList::ordered(),
            ArticleList::unordered(),
            ArticleList::inside(),
        ] {
            let hidden = list.inherit(ListStyle::None);
            assert!(hidden.marker == ListStyle::None);
        }
    }
}
