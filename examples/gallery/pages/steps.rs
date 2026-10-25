use crate::text::tr;
use crate::{
    chrome::{self, PageFrame},
    pages::Args,
};
use gpui::{prelude::*, *};
use wegpui::components::{
    button::Button,
    steps::{Step, Steps, StepsLayout},
};

pub struct StepsPage {
    route: &'static str,
}

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| StepsPage { route }).into()
}

fn step(success: bool) -> Step {
    Step::new(tr("标题"))
        .desc(tr("描述内容详情"))
        .success(success)
}

fn group(id: &'static str, layout: StepsLayout, items: Vec<Step>) -> impl IntoElement {
    let first = matches!(id, "v1" | "h1");
    div()
        .when(first, |el| el.mt(px(40.)))
        .mb(px(80.))
        .mx_auto()
        .w(relative(0.5))
        .child(
            Steps::new(id)
                .layout(layout)
                .aria_label(tr("步骤条"))
                .items(items),
        )
}

fn vertical() -> Vec<AnyElement> {
    let v = StepsLayout::Vertical;
    vec![
        group("v1", v, (0..4).map(|_| step(false)).collect()).into_any_element(),
        group(
            "v2",
            v,
            vec![step(true), step(false), step(false), step(false)],
        )
        .into_any_element(),
        group(
            "v3",
            v,
            vec![
                step(true),
                step(true),
                step(false).icon("waiting"),
                step(false),
            ],
        )
        .into_any_element(),
        group(
            "v4",
            v,
            vec![
                step(true),
                step(true),
                step(true),
                step(true).icon("success"),
            ],
        )
        .into_any_element(),
    ]
}

fn horizontal() -> Vec<AnyElement> {
    let h = StepsLayout::Horizontal;
    (0..4)
        .map(|done| {
            let items = (0..4)
                .map(|i| step(i < done || done == 3).show_text(false))
                .collect();
            group(["h1", "h2", "h3", "h4"][done], h, items).into_any_element()
        })
        .collect()
}

impl Render for StepsPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        match self.route {
            "steps_vertical" => PageFrame::new(self.route).children(vertical()),
            "steps_horizonal" => PageFrame::new(self.route).children(horizontal()),
            _ => PageFrame::new(self.route)
                .title("Steps")
                .desc(tr("步骤条"))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(16.))
                        .child(
                            Button::new(tr("垂直型"))
                                .id("steps-vertical")
                                .on_click(chrome::go("steps_vertical")),
                        )
                        .child(
                            Button::new(tr("水平型"))
                                .id("steps-horizonal")
                                .on_click(chrome::go("steps_horizonal")),
                        ),
                ),
        }
    }
}
