use super::Icon;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsLayout {
    #[default]
    Vertical,
    Horizontal,
    HorizontalPrimary,
    HorizontalCenter,
}

pub struct Step {
    title: SharedString,
    desc: Option<SharedString>,
    success: bool,
    icon: Option<SharedString>,
    show_text: bool,
}

impl Step {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            desc: None,
            success: false,
            icon: None,
            show_text: true,
        }
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }
    pub fn icon(mut self, name: impl Into<SharedString>) -> Self {
        self.icon = Some(name.into());
        self
    }
    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }
}

#[derive(IntoElement)]
pub struct Steps {
    id: ElementId,
    layout: StepsLayout,
    label: Option<SharedString>,
    items: Vec<Step>,
}

impl Steps {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            layout: StepsLayout::Vertical,
            label: None,
            items: vec![],
        }
    }
    pub fn layout(mut self, layout: StepsLayout) -> Self {
        self.layout = layout;
        self
    }
    pub fn vertical(self) -> Self {
        self.layout(StepsLayout::Vertical)
    }
    pub fn horizontal(self) -> Self {
        self.layout(StepsLayout::Horizontal)
    }
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn item(mut self, step: Step) -> Self {
        self.items.push(step);
        self
    }
    pub fn items(mut self, steps: impl IntoIterator<Item = Step>) -> Self {
        self.items.extend(steps);
        self
    }
}

struct Ctx {
    font: f32,
    line: f32,
    dot: f32,
    icon: f32,
    gap_v: f32,
    gap_h: f32,
    default: Hsla,
    highlight: Hsla,
    fg2: Hsla,
    bg2: Hsla,
}

fn text_block(step: &Step, c: &Ctx) -> Div {
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .child(step.title.clone()),
        )
        .when_some(step.desc.clone(), |el, desc| {
            el.child(
                div()
                    .mt(px(4.))
                    .text_size(px(14.))
                    .line_height(relative(1.4))
                    .text_color(c.fg2)
                    .child(desc),
            )
        })
}

fn dot(size: f32, color: Hsla) -> Div {
    div()
        .size(px(size))
        .rounded_full()
        .bg(color)
        .flex_shrink_0()
}

fn state_label(step: &Step, index: usize, total: usize) -> SharedString {
    let state = if step.success { "，已完成" } else { "" };
    let text = match &step.desc {
        Some(desc) => format!(
            "第{}步，共{}步，{}，{}{}",
            index + 1,
            total,
            step.title,
            desc,
            state
        ),
        None => format!("第{}步，共{}步，{}{}", index + 1, total, step.title, state),
    };
    SharedString::from(text)
}

impl RenderOnce for Steps {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let font = t.steps_font_size;
        let c = Ctx {
            font,
            line: t.steps_lineheight,
            dot: t.steps_dot_size,
            icon: t.steps_icon_size,
            gap_v: t.steps_vertical_dot_gap,
            gap_h: t.steps_horizonal_dot_gap,
            default: t.steps_default_color,
            highlight: t.steps_highlight_color,
            fg2: t.fg_2,
            bg2: t.bg_2,
        };
        let n = self.items.len();
        let reached: Vec<bool> = (0..n)
            .map(|i| self.items[i].success || (i > 0 && self.items[i - 1].success))
            .collect();
        let lit: Vec<bool> = (0..n).map(|i| i == 0 || reached[i]).collect();
        let root = div()
            .id(self.id)
            .role(Role::List)
            .when_some(self.label, |el, label| el.aria_label(label))
            .text_size(px(font))
            .line_height(relative(c.line));
        match self.layout {
            StepsLayout::Vertical => {
                root.relative()
                    .flex()
                    .flex_col()
                    .children(self.items.iter().enumerate().map(|(i, step)| {
                        let em = c.font;
                        let has_icon = step.icon.is_some();
                        let icon_prev = self.items.get(i + 1).is_some_and(|s| s.icon.is_some());
                        let line_color = if step.success { c.highlight } else { c.default };
                        let top = if has_icon {
                            c.gap_v + c.icon - 0.14 * em
                        } else {
                            (c.line - (c.line - 1.) / 2.) * em
                        };
                        let bottom = if icon_prev {
                            c.gap_v - (c.line - 1.) / 2. * em + 0.14 * em
                        } else {
                            -(c.line - 1.) / 2. * em
                        };
                        let mid = c.line / 2. * em;
                        div()
                            .id(ElementId::NamedInteger("step".into(), i as u64))
                            .role(Role::ListItem)
                            .aria_label(state_label(step, i, n))
                            .relative()
                            .pb(px(32.))
                            .when(i + 1 < n, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .left_0()
                                        .w(px(0.5))
                                        .top(px(top))
                                        .bottom(px(bottom))
                                        .bg(line_color),
                                )
                            })
                            .child(
                                div()
                                    .relative()
                                    .pl(px(36.))
                                    .child(text_block(step, &c))
                                    .when(!has_icon, |el| {
                                        el.child(
                                            dot(
                                                c.dot,
                                                if lit[i] { c.highlight } else { c.default },
                                            )
                                            .absolute()
                                            .left(px(-c.dot / 2.))
                                            .top(px(mid - c.dot / 2.)),
                                        )
                                    })
                                    .when_some(step.icon.clone(), |el, name| {
                                        let color = match name.as_ref() {
                                            "success" | "success-circle" | "success-no-circle" => {
                                                t.brand
                                            }
                                            "warn" => t.red,
                                            "waiting" | "waiting-circle" | "info" => t.blue,
                                            _ => t.fg_2,
                                        };
                                        let margin = (c.icon - em) / 2. - 0.28 * em;
                                        el.child(
                                            div()
                                                .absolute()
                                                .left(px(-c.icon / 2.))
                                                .top(px(mid - c.icon / 2. + margin))
                                                .child(Icon::new(name).size(c.icon).color(color)),
                                        )
                                    }),
                            )
                    }))
            }
            StepsLayout::Horizontal => {
                root.flex()
                    .items_center()
                    .children(self.items.iter().enumerate().map(|(i, step)| {
                        let last = i + 1 == n;
                        div()
                            .id(ElementId::NamedInteger("step".into(), i as u64))
                            .role(Role::ListItem)
                            .aria_label(state_label(step, i, n))
                            .flex()
                            .items_center()
                            .when(!last, |el| el.flex_1())
                            .when(last, |el| el.flex_none())
                            .child(dot(c.dot, if lit[i] { c.highlight } else { c.default }))
                            .when(step.show_text, |el| {
                                el.child(text_block(step, &c).ml(px(8.)))
                            })
                            .when(!last, |el| {
                                el.child(
                                    div()
                                        .flex_1()
                                        .h(px(0.5))
                                        .mx(px(c.gap_h))
                                        .bg(if step.success { c.highlight } else { c.default }),
                                )
                            })
                    }))
            }
            StepsLayout::HorizontalPrimary => {
                root.flex()
                    .children(self.items.iter().enumerate().map(|(i, step)| {
                        let last = i + 1 == n;
                        let mask = c.dot + 2. * c.gap_h;
                        div()
                            .id(ElementId::NamedInteger("step".into(), i as u64))
                            .role(Role::ListItem)
                            .aria_label(state_label(step, i, n))
                            .relative()
                            .when(!last, |el| el.flex_1())
                            .when(last, |el| el.flex_none())
                            .when(!last, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .top_0()
                                        .left_0()
                                        .right_0()
                                        .h(px(0.5))
                                        .bg(if step.success { c.highlight } else { c.default }),
                                )
                            })
                            .child(
                                div()
                                    .relative()
                                    .pt(px(36.))
                                    .child(
                                        div()
                                            .absolute()
                                            .top(px(-mask / 2.))
                                            .left(px(-mask / 2. + c.dot / 2.))
                                            .size(px(mask))
                                            .bg(c.bg2),
                                    )
                                    .child(
                                        dot(
                                            c.dot,
                                            if reached[i] { c.highlight } else { c.default },
                                        )
                                        .absolute()
                                        .top(px(-c.dot / 2.))
                                        .left_0(),
                                    )
                                    .when(step.show_text, |el| el.child(text_block(step, &c))),
                            )
                    }))
            }
            StepsLayout::HorizontalCenter => {
                root.flex()
                    .text_center()
                    .children(self.items.iter().enumerate().map(|(i, step)| {
                        let mask = c.dot + 2. * c.gap_h;
                        let before = if reached[i] { c.highlight } else { c.default };
                        let after = if step.success { c.highlight } else { c.default };
                        div()
                            .id(ElementId::NamedInteger("step".into(), i as u64))
                            .role(Role::ListItem)
                            .aria_label(state_label(step, i, n))
                            .relative()
                            .flex_1()
                            .when(i > 0, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .top_0()
                                        .left_0()
                                        .right(relative(0.5))
                                        .h(px(0.5))
                                        .bg(before),
                                )
                            })
                            .when(i + 1 < n, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .top_0()
                                        .left(relative(0.5))
                                        .right_0()
                                        .h(px(0.5))
                                        .bg(after),
                                )
                            })
                            .child(
                                div()
                                    .relative()
                                    .pt(px(36.))
                                    .child(
                                        div()
                                            .absolute()
                                            .top(px(-mask / 2.))
                                            .left(relative(0.5))
                                            .ml(px(-mask / 2.))
                                            .size(px(mask))
                                            .bg(c.bg2),
                                    )
                                    .child(
                                        dot(
                                            c.dot,
                                            if reached[i] { c.highlight } else { c.default },
                                        )
                                        .absolute()
                                        .top(px(-c.dot / 2.))
                                        .left(relative(0.5))
                                        .ml(px(-c.dot / 2.)),
                                    )
                                    .when(step.show_text, |el| el.child(text_block(step, &c))),
                            )
                    }))
            }
        }
    }
}
