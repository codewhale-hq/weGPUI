use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellsStyle {
    #[default]
    Plain,
    Form,
    FormPrimary,
    Bare,
}

#[derive(IntoElement)]
pub struct Cells {
    id: Option<ElementId>,
    title: Option<AnyElement>,
    tips: Option<AnyElement>,
    tips_warn: bool,
    style: CellsStyle,
    inset: (f32, f32),
    margin_top: Option<f32>,
    role: Option<Role>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Default for Cells {
    fn default() -> Self {
        Self::new()
    }
}

impl Cells {
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            tips: None,
            tips_warn: false,
            style: CellsStyle::Plain,
            inset: (16., 0.),
            margin_top: None,
            role: None,
            label: None,
            children: vec![],
        }
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.tips = Some(tips.into_any_element());
        self
    }
    pub fn tips_warn(mut self, warn: bool) -> Self {
        self.tips_warn = warn;
        self
    }
    pub fn style(mut self, style: CellsStyle) -> Self {
        self.style = style;
        if style != CellsStyle::Plain {
            self.inset = (16., 16.);
        }
        self
    }
    pub fn form(self) -> Self {
        self.style(CellsStyle::Form)
    }
    pub fn form_primary(self) -> Self {
        self.style(CellsStyle::FormPrimary)
    }
    pub fn separator_inset(mut self, left: f32, right: f32) -> Self {
        self.inset = (left, right);
        self
    }
    pub fn margin_top(mut self, margin: f32) -> Self {
        self.margin_top = Some(margin);
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl ParentElement for Cells {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

pub fn cells_title(text: impl IntoElement, cx: &App) -> Div {
    let t = Theme::get(cx);
    div()
        .mt(px(16.))
        .mb(px(3.))
        .px(px(16.))
        .text_size(px(14.))
        .line_height(relative(1.4))
        .text_color(t.fg_1)
        .child(text)
}

impl RenderOnce for Cells {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let form = matches!(self.style, CellsStyle::Form | CellsStyle::FormPrimary);
        let (inset_l, inset_r) = self.inset;
        let line = t.fg_3;
        let has_title = self.title.is_some();
        let margin_top = self
            .margin_top
            .unwrap_or(if has_title && !form { 0. } else { 8. });
        let count = self.children.len();
        let rows = self.children.into_iter().enumerate().map(|(i, item)| {
            div().relative().child(item).when(i > 0, |el| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left(px(inset_l))
                        .right(px(inset_r))
                        .h(px(0.5))
                        .bg(line),
                )
            })
        });
        let mut group = div()
            .id("cells-group")
            .relative()
            .flex()
            .flex_col()
            .overflow_hidden()
            .mt(px(if form { 0. } else { margin_top }))
            .when_some(self.role, |el, role| el.role(role))
            .when_some(self.label, |el, label| el.aria_label(label))
            .children(rows);
        group = match self.style {
            CellsStyle::Plain => group
                .bg(t.bg_2)
                .child(hairline(line).top_0().left_0().right_0())
                .when(count > 0, |el| {
                    el.child(hairline(line).bottom_0().left_0().right_0())
                }),
            CellsStyle::Form => group
                .mx(px(16.))
                .bg(t.bg_2)
                .child(hairline(line).top_0().left(px(16.)).right(px(16.)))
                .child(hairline(line).bottom_0().left(px(16.)).right(px(16.))),
            CellsStyle::FormPrimary => group.mx(px(16.)).bg(t.bg_1).rounded(px(8.)),
            CellsStyle::Bare => group,
        };
        let tips_color = if self.tips_warn {
            t.red
        } else if form {
            t.fg_2
        } else {
            t.fg_1
        };
        div()
            .id(self.id.unwrap_or_else(|| ElementId::Name("cells".into())))
            .w_full()
            .flex()
            .flex_col()
            .when(form, |el| {
                el.mt(px(match self.style {
                    CellsStyle::FormPrimary => 32.,
                    _ => 24.,
                }))
            })
            .when_some(self.title, |el, title| {
                el.child(if form {
                    div()
                        .mb(px(8.))
                        .px(px(32.))
                        .text_size(px(14.))
                        .line_height(relative(1.4))
                        .text_color(t.fg_1)
                        .child(title)
                } else {
                    cells_title(title, cx)
                })
            })
            .child(group)
            .when_some(self.tips, |el, tips| {
                el.child(
                    div()
                        .mt(px(8.))
                        .px(px(if form { 32. } else { 16. }))
                        .text_size(px(14.))
                        .line_height(relative(1.4))
                        .text_color(tips_color)
                        .child(tips),
                )
            })
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

#[derive(IntoElement)]
pub struct Cell {
    id: ElementId,
    hd: Option<AnyElement>,
    bd: Option<AnyElement>,
    ft: Option<AnyElement>,
    desc: Option<SharedString>,
    label: Option<SharedString>,
    access: bool,
    link: bool,
    active: bool,
    primary: bool,
    form: bool,
    warn: bool,
    disabled: bool,
    padding: Option<Edges<Pixels>>,
    min_height: Option<f32>,
    role: Option<Role>,
    selected: Option<bool>,
    focus: Option<FocusHandle>,
    on_click: Option<ClickHandler>,
}

impl Cell {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            hd: None,
            bd: None,
            ft: None,
            desc: None,
            label: None,
            access: false,
            link: false,
            active: false,
            primary: false,
            form: false,
            warn: false,
            disabled: false,
            padding: None,
            min_height: None,
            role: None,
            selected: None,
            focus: None,
            on_click: None,
        }
    }
    pub fn hd(mut self, hd: impl IntoElement) -> Self {
        self.hd = Some(hd.into_any_element());
        self
    }
    pub fn bd(mut self, bd: impl IntoElement) -> Self {
        self.bd = Some(bd.into_any_element());
        self
    }
    pub fn ft(mut self, ft: impl IntoElement) -> Self {
        self.ft = Some(ft.into_any_element());
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        if self.label.is_none() {
            self.label = Some(title.clone());
        }
        self.bd = Some(title.into_any_element());
        self
    }
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.ft = Some(value.into().into_any_element());
        self
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn access(mut self) -> Self {
        self.access = true;
        self.active = true;
        self
    }
    pub fn link(mut self) -> Self {
        self.link = true;
        self.active = true;
        self
    }
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
    pub fn form(mut self) -> Self {
        self.form = true;
        self
    }
    pub fn warn(mut self, warn: bool) -> Self {
        self.warn = warn;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn padding(mut self, padding: Edges<Pixels>) -> Self {
        self.padding = Some(padding);
        self
    }
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Cell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let padding = self.padding.unwrap_or(Edges {
            top: px(16.),
            bottom: px(16.),
            left: px(16.),
            right: px(16.),
        });
        let interactive = self.on_click.is_some() && !self.disabled;
        let role = self.role.unwrap_or(if self.link {
            Role::Link
        } else if interactive {
            Role::Button
        } else {
            Role::ListItem
        });
        let color = if self.link {
            t.link
        } else if self.warn {
            t.red
        } else {
            t.fg_0
        };
        let bd = div()
            .flex_1()
            .min_w_0()
            .children(self.bd)
            .when_some(self.desc, |el, desc| {
                el.child(
                    div()
                        .pt(px(4.))
                        .text_size(px(12.))
                        .line_height(relative(1.4))
                        .text_color(t.fg_2)
                        .child(desc),
                )
            });
        let ft = (self.ft.is_some() || self.access).then(|| {
            div()
                .relative()
                .flex()
                .items_center()
                .justify_end()
                .text_right()
                .text_color(t.fg_1)
                .when(self.form, |el| el.pl(px(16.)))
                .when(self.access, |el| el.pr(px(24.)))
                .children(self.ft)
                .when(self.access, |el| {
                    el.child(
                        div()
                            .absolute()
                            .right_0()
                            .top_0()
                            .bottom_0()
                            .flex()
                            .items_center()
                            .child(Icon::new("arrow").w(12.).h(24.).color(t.fg_2)),
                    )
                })
        });
        let pressed = t.fg_3;
        let brand = t.brand;
        let radius = if self.form { 8. } else { 0. };
        let mut el = div()
            .id(self.id)
            .role(role)
            .when_some(self.label, |el, label| el.aria_label(label))
            .when_some(self.selected, |el, s| el.aria_selected(s))
            .relative()
            .flex()
            .when(self.primary, |el| el.items_start())
            .when(!self.primary, |el| el.items_center())
            .pt(padding.top - px(2.))
            .pb(padding.bottom - px(2.))
            .pl(padding.left - px(2.))
            .pr(padding.right - px(2.))
            .border_2()
            .border_color(transparent_black())
            .when_some(self.min_height, |el, h| el.min_h(px(h)))
            .text_size(px(17.))
            .line_height(relative(24. / 17.))
            .text_color(color)
            .when_some(self.hd, |el, hd| {
                el.child(div().when(self.form, |el| el.pr(px(16.))).child(hd))
            })
            .child(bd)
            .children(ft);
        if let Some(handler) = self.on_click.filter(|_| !self.disabled) {
            el = el
                .when_some(self.focus, |el, f| el.track_focus(&f))
                .when(self.active, |el| {
                    el.active(move |s| s.bg(pressed).rounded(px(radius)))
                })
                .focusable()
                .tab_stop(true)
                .focus_visible(move |s| s.border_color(brand))
                .on_click(handler);
        } else if self.active {
            el = el.active(move |s| s.bg(pressed).rounded(px(radius)));
        }
        crate::accessibility::Semantics::new(el).disabled(self.disabled)
    }
}
