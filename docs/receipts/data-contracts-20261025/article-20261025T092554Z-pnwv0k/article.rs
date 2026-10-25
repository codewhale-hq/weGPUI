use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::article::{
        Article, ArticleList, ListItem, h1, h2, h3, h4, h5, paragraph, paragraph_text, section,
    },
    scrolling::RevealFocus,
};

pub struct ArticlePage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| ArticlePage { route }).into()
}

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
const COMBO: &str =
    "Ipsum cum soluta laudantium aperiam rem? Accusamus nostrum itaque sint? read more";
const ITEM: &str = "Elit quo illum ut voluptate possimus. Maxime";
const LONG: &str = "Elit quo illum ut voluptateElit quo illum ut voluptate possimus. Maxime";
const ADI: &str = "Adipisicing adipisci labore deleniti";
const ADI_LONG: &str = "Adipisicing adipisci labore delenitAdipisicing adipisci labore delenitiiAdipisicing adipisci labore deleniti";

fn ordered_nested(
    list: ArticleList,
    first: &'static str,
    nested: [&'static str; 3],
    last: &'static str,
) -> ArticleList {
    let mut inner = ArticleList::ordered();
    for n in nested {
        inner = inner.item(n);
    }
    list.item(first)
        .item(ListItem::new(ITEM).nested(inner))
        .item(last)
}

impl Render for ArticlePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let link = t.link;
        let brand = t.brand;
        let lists = section(2)
            .mb_0()
            .child(h4("列表示例"))
            .child(
                section(3).mb_0().child(h5("原生无序列表")).child(
                    ArticleList::unordered()
                        .item(ITEM)
                        .item(ListItem::new(ITEM).nested(
                            ArticleList::unordered()
                                .item("Elit facilis nam nam")
                                .item("Elit facilis nam nam")
                                .item("Elit facilis nam nam"),
                        ))
                        .item(ITEM),
                ),
            )
            .child(section(3).mb_0().child(h5("原生有序列表")).child(ordered_nested(
                ArticleList::ordered(),
                ITEM,
                [ADI, ADI, ADI],
                ITEM,
            )))
            .child(
                section(3)
                    .mb_0()
                    .child(h5("原生段落列表（需要自行添加class）"))
                    .child(ordered_nested(ArticleList::inside(), LONG, [ADI_LONG, ADI, ADI], ITEM)),
            )
            .child(
                section(3).mb_0().child(h5("自定义序号列表（需要自行添加class）")).child(
                    ArticleList::none()
                        .item(ListItem::new("1 Elit quo illum ut voluptateElit quo illum ut voluptate possimus. Maxime"))
                        .item(
                            ListItem::new("2 Elit quo illum ut voluptate possimus. Maxime").nested(
                                ArticleList::ordered()
                                    .item("2.1 Adipisicing adipisci labore delenitAdipisicing adipisci labore delenitiiAdipisicing adipisci labore deleniti")
                                    .item("2.2 Adipisicing adipisci labore deleniti")
                                    .item("2.3 Adipisicing adipisci labore deleniti"),
                            ),
                        )
                        .item(ListItem::new("3 Elit quo illum ut voluptate possimus. Maxime")),
                ),
            );
        let combo = paragraph().mb_0().child(
            div()
                .id("a11y-combo")
                .role(Role::Link)
                .aria_label(COMBO)
                .tab_stop(true)
                .focusable()
                .focus_visible(move |s| s.shadow(wegpui::components::form::focus_ring(brand)))
                .child(StyledText::new(COMBO).with_highlights([(
                    COMBO.len() - "read more".len()..COMBO.len(),
                    HighlightStyle {
                        color: Some(link),
                        ..Default::default()
                    },
                )]))
                .reveal_focus(),
        );
        PageFrame::new(self.route).child(
            Article::new("article")
                .child(h1("文章页大标题"))
                .child(
                    section(0).mb(px(24.)).child(h2("二级标题")).child(
                        section(1)
                            .mb_0()
                            .child(h3("三级标题"))
                            .child(
                                section(2)
                                    .child(h4("四级标题"))
                                    .child(paragraph_text("article-body", LOREM))
                                    .child(
                                        paragraph()
                                            .mb_0()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .id("article-image-1")
                                                    .role(Role::Image)
                                                    .aria_label("配图")
                                                    .flex()
                                                    .flex_col()
                                                    .w_full()
                                                    .h(px(327. * 389. / 690.))
                                                    .flex_shrink_0()
                                                    .child(
                                                        img("pic_article.png")
                                                            .w_full()
                                                            .h(px(327. * 389. / 690.)),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .id("article-image-2")
                                                    .role(Role::Image)
                                                    .aria_label("配图")
                                                    .flex()
                                                    .flex_col()
                                                    .w_full()
                                                    .h(px(327. * 389. / 690.))
                                                    .flex_shrink_0()
                                                    .child(
                                                        img("pic_article.png")
                                                            .w_full()
                                                            .h(px(327. * 389. / 690.)),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(lists),
                    ),
                )
                .child(
                    section(0)
                        .child(h2("无障碍焦点整合示例"))
                        .child(section(1).mb_0().child(combo)),
                ),
        )
    }
}
