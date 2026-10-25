use crate::Theme;
use gpui::{prelude::*, *};

const FONT: f32 = 17.;
const LINE: f32 = 17. * 1.6;

#[derive(IntoElement)]
pub struct Article {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl Article {
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

pub fn section(depth: usize) -> Div {
    div().flex().flex_col().mb(px(match depth {
        0 => 48.,
        1 => 32.,
        _ => 24.,
    }))
}

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

pub fn h1(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(1, text)
}
pub fn h2(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(2, text)
}
pub fn h3(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(3, text)
}
pub fn h4(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(4, text)
}
pub fn h5(text: impl Into<SharedString>) -> Stateful<Div> {
    heading(5, text)
}

pub fn paragraph() -> Div {
    div().mb(px(24.))
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ListStyle {
    #[default]
    Disc,
    Decimal,
    Inside,
    None,
}

pub struct ListItem {
    text: SharedString,
    nested: Option<ArticleList>,
}

impl ListItem {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            nested: None,
        }
    }
    pub fn nested(mut self, list: ArticleList) -> Self {
        self.nested = Some(list);
        self
    }
}

#[derive(IntoElement)]
pub struct ArticleList {
    style: ListStyle,
    marker: ListStyle,
    nested: bool,
    items: Vec<ListItem>,
}

impl ArticleList {
    pub fn new(style: ListStyle) -> Self {
        Self {
            style,
            marker: style,
            nested: false,
            items: vec![],
        }
    }
    pub fn unordered() -> Self {
        Self::new(ListStyle::Disc)
    }
    pub fn ordered() -> Self {
        Self::new(ListStyle::Decimal)
    }
    pub fn inside() -> Self {
        let mut list = Self::new(ListStyle::Inside);
        list.marker = ListStyle::Decimal;
        list
    }
    pub fn none() -> Self {
        Self::new(ListStyle::None)
    }
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
                    .aria_position_in_set(i + 1)
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
