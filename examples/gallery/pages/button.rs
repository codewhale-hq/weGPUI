use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{
        agree::Agree,
        button::{BottomFixedOpr, BottomFixedOprPage, Button, ButtonKind, Size},
    },
};

const DEMO: &str = "Dolor tempora dolore aperiam fuga necessitatibus? Odio atque tempora deleniti voluptates voluptatem. Dolorem earum voluptas blanditiis labore quisquam? Quibusdam nesciunt consequatur aliquam ea corrupti animi, itaque consequatur neque? At porroSit nemo aliquid quas error doloremque Reiciendis ratione repellendus quae sit commodi amet architecto? Aut officiis aliquam fugit nulla at necessitatibus Optio totam quibusdam laboriosam aperiam libero! Officiis reiciendis Elit quaerat sed vero perferendis architecto consequatur. Consequuntur ad illum dolore ut accusamus. Cum possimus odit sequi quaerat beatae, eveniet. Rerum dolore ipsam quia consectetur iste Veniam mollitia dolores tempore? ";

pub struct ButtonPage {
    route: &'static str,
    agreed: bool,
}

pub fn build(route: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let agreed = args.state.as_deref() == Some("agreed");
    cx.new(|_| ButtonPage { route, agreed }).into()
}

fn sp_area(desktop: bool) -> Div {
    div()
        .my(px(15.))
        .p(px(15.))
        .flex()
        .gap(px(16.))
        .when(desktop, |area| area.flex_wrap().items_start())
        .when(!desktop, |area| area.flex_col())
}

const KINDS: [(ButtonKind, &str, &str); 3] = [
    (ButtonKind::Primary, "primary", "主要操作"),
    (ButtonKind::Default, "default", "次要操作"),
    (ButtonKind::Warn, "warn", "警示操作"),
];

impl ButtonPage {
    fn index(&self) -> impl IntoElement {
        PageFrame::new(self.route)
            .title("Button")
            .desc(tr("按钮"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        Button::new(tr("普通型"))
                            .id("btn-nav-default")
                            .on_click(crate::chrome::go("button_default")),
                    )
                    .child(
                        Button::new(tr("底部悬浮型"))
                            .id("btn-nav-bottom-fixed")
                            .on_click(crate::chrome::go("button_bottom_fixed")),
                    ),
            )
    }

    fn default_page(&self, desktop: bool, cx: &App) -> impl IntoElement {
        let brand = Theme::get(cx).brand;
        let mut first = sp_area(desktop);
        for (kind, key, label) in KINDS.map(|(kind, key, label)| (kind, key, tr(label))) {
            first = first
                .child(
                    Button::new(label)
                        .id(key)
                        .kind(kind)
                        .when(desktop, |button| button.mini().inline().margin_reset()),
                )
                .child(
                    Button::new("")
                        .id(format!("{key}-loading-only"))
                        .title(tr("正在操作"))
                        .kind(kind)
                        .when(desktop, |button| button.mini().inline().margin_reset())
                        .loading(true),
                )
                .child(
                    Button::new(label)
                        .id(format!("{key}-loading"))
                        .title(tr("正在操作"))
                        .kind(kind)
                        .when(desktop, |button| button.mini().inline().margin_reset())
                        .loading(true),
                )
                .child(
                    Button::new(label)
                        .id(format!("{key}-disabled"))
                        .kind(kind)
                        .when(desktop, |button| button.mini().inline().margin_reset())
                        .disabled(true),
                );
        }
        let widths = sp_area(desktop)
            .child(Button::new(tr("按钮宽度")).id("w1").primary())
            .child(Button::new(tr("按钮宽度自适应内容")).id("w2").primary())
            .child(
                Button::new(tr(
                    "按钮宽度自适应内容，达到最大值后折行，最大行数两行的示例",
                ))
                .id("w3")
                .primary(),
            )
            .child(
                Button::new(tr("按钮宽度自适应内容"))
                    .id("w4")
                    .title(tr("正在操作"))
                    .primary()
                    .loading(true),
            )
            .child(
                Button::new(tr(
                    "按钮宽度自适应内容，达到最大值后折行，最大行数两行的示例",
                ))
                .id("w5")
                .title(tr("正在操作"))
                .primary()
                .loading(true),
            );
        let mut medium = sp_area(desktop);
        let mut overlay = sp_area(desktop).px_0().bg(brand);
        let mut overlay_disabled = sp_area(desktop).px_0().bg(brand);
        let mut mini = div()
            .my(px(15.))
            .p(px(15.))
            .flex()
            .justify_center()
            .items_center()
            .min_h(px(32.));
        let mut margin = div().my(px(15.)).p(px(15.)).flex().justify_between();
        for (i, (kind, key, _)) in KINDS.into_iter().enumerate() {
            medium = medium.child(
                Button::new(tr("medium 按钮"))
                    .id(format!("medium-{key}"))
                    .kind(kind)
                    .size(Size::Medium),
            );
            overlay = overlay.child(
                Button::new(tr("overlay 按钮"))
                    .id(format!("overlay-{key}"))
                    .kind(kind)
                    .overlay(),
            );
            overlay_disabled = overlay_disabled.child(
                Button::new(tr("overlay 按钮"))
                    .id(format!("overlay-disabled-{key}"))
                    .kind(kind)
                    .overlay()
                    .disabled(true),
            );
            mini = mini.child(
                div()
                    .mx(px(12.))
                    .when(i > 0, |el| el.ml(px(12. + 4.4)))
                    .child(
                        Button::new(tr("按钮"))
                            .id(format!("mini-{key}"))
                            .kind(kind)
                            .size(Size::Mini),
                    ),
            );
        }
        for (kind, key) in [
            (ButtonKind::Default, "default"),
            (ButtonKind::Primary, "primary"),
            (ButtonKind::Warn, "warn"),
        ] {
            margin = margin.child(
                Button::new(tr("按钮"))
                    .id(format!("margin-{key}"))
                    .kind(kind)
                    .size(Size::Mini)
                    .margin_reset(),
            );
        }
        PageFrame::new(self.route)
            .title("Button")
            .desc(tr("按钮"))
            .bg(Theme::get(cx).bg_0)
            .child(first)
            .child(widths.mt_0())
            .child(medium.mt_0())
            .child(overlay.mt_0())
            .child(overlay_disabled.mt_0())
            .child(mini.mt_0())
            .child(margin.mt_0())
    }

    fn bottom_fixed(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let fg = Theme::get(cx).fg_0;
        let entity = cx.entity();
        let page = BottomFixedOprPage::new(
            "bottom-fixed-content",
            BottomFixedOpr::new()
                .button(
                    Button::new(crate::text::tr_or("阅读并同意", "Read and agree"))
                        .id("js_btn")
                        .primary(),
                )
                .button(Button::new(tr("取消")).id("js_cancel")),
        )
        .tips(
            Agree::new("weuiAgree", tr("阅读并同意"))
                .link(tr("《相关条款》"), |_, _, _| {})
                .checked(self.agreed)
                .on_change(move |v, _, cx| {
                    entity.update(cx, |this, cx| {
                        this.agreed = *v;
                        cx.notify();
                    })
                }),
        )
        .child(
            div()
                .pt(px(32.))
                .px(px(32.))
                .text_size(px(16.))
                .line_height(relative(1.6))
                .text_color(fg)
                .child(DEMO.repeat(2)),
        );
        PageFrame::new(self.route)
            .no_scroll()
            .fixed(div().absolute().inset_0().child(page))
    }
}

impl Render for ButtonPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.route {
            "button_default" => self
                .default_page(
                    cfg!(not(target_os = "ios"))
                        && window.viewport_size().width >= px(crate::desktop::BREAKPOINT),
                    cx,
                )
                .into_any_element(),
            "button_bottom_fixed" => self.bottom_fixed(cx).into_any_element(),
            _ => self.index().into_any_element(),
        }
    }
}
