use super::ClickHandler;
use crate::{Theme, accessibility::Semantics, input::TextInput};
use gpui::{prelude::*, *};

pub const LABEL_WIDTH: f32 = 105.;
pub const LABEL_MAX_WIDTH: f32 = 85.;

pub fn focus_ring(color: Hsla) -> Vec<BoxShadow> {
    vec![BoxShadow {
        color,
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(2.),
        inset: true,
    }]
}

pub fn cell_frame(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .pt(px(14.))
        .pb(px(14.))
        .pl(px(14.))
        .pr(px(14.))
        .border_2()
        .border_color(transparent_black())
        .text_size(px(17.))
        .line_height(relative(24. / 17.))
}

pub fn form_label(text: impl Into<SharedString>, width: f32) -> Div {
    div()
        .flex_shrink_0()
        .w(px(width.min(LABEL_MAX_WIDTH)))
        .mr(px(8.))
        .child(text.into())
}

pub fn cell_control(el: impl IntoElement) -> Div {
    div().my(px(8.)).ml(px(16.)).child(el)
}

pub fn cell_control_flex(el: impl IntoElement) -> Div {
    div()
        .my(px(8.))
        .ml(px(16.))
        .flex_1()
        .min_w(px(112.5))
        .child(el)
}

pub fn form_tips(text: impl IntoElement, cx: &App) -> Div {
    let t = Theme::get(cx);
    div()
        .text_size(px(14.))
        .line_height(relative(1.4))
        .text_color(t.fg_1)
        .child(text)
}

pub fn form_link(
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
    bold: bool,
    cx: &App,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let t = Theme::get(cx);
    let text = text.into();
    let brand = t.brand;
    div()
        .id(id)
        .role(Role::Link)
        .aria_label(text.clone())
        .text_color(t.link)
        .when(bold, |el| el.font_weight(FontWeight::SEMIBOLD))
        .cursor_pointer()
        .focusable()
        .tab_stop(true)
        .focus_visible(move |s| s.shadow(focus_ring(brand)))
        .active(|s| s.opacity(0.5))
        .on_click(move |e, w, cx| {
            cx.stop_propagation();
            on_click(e, w, cx)
        })
        .child(text)
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellKind {
    #[default]
    Default,
    Primary,
    Switch,
    Wrap,
    Vcode,
    Vertical,
}

#[derive(IntoElement)]
pub struct FormCell {
    id: ElementId,
    kind: CellKind,
    hd: Option<AnyElement>,
    label: Option<SharedString>,
    label_width: f32,
    label_desc: Option<SharedString>,
    bd: Vec<AnyElement>,
    bd_flex: bool,
    ft: Option<AnyElement>,
    warn: bool,
    readonly: bool,
    disabled: bool,
    active: bool,
    press_radius: f32,
    focus_target: Option<FocusHandle>,
    role: Option<Role>,
    aria_label: Option<SharedString>,
    on_tap: Option<ClickHandler>,
}

impl FormCell {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            kind: CellKind::Default,
            hd: None,
            label: None,
            label_width: LABEL_WIDTH,
            label_desc: None,
            bd: vec![],
            bd_flex: false,
            ft: None,
            warn: false,
            readonly: false,
            disabled: false,
            active: true,
            press_radius: 8.,
            focus_target: None,
            role: None,
            aria_label: None,
            on_tap: None,
        }
    }
    pub fn kind(mut self, kind: CellKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }
    pub fn label_desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.label_desc = Some(desc.into());
        self
    }
    pub fn hd(mut self, hd: impl IntoElement) -> Self {
        self.hd = Some(hd.into_any_element());
        self
    }
    pub fn bd_flex(mut self) -> Self {
        self.bd_flex = true;
        self
    }
    pub fn ft(mut self, ft: impl IntoElement) -> Self {
        self.ft = Some(ft.into_any_element());
        self
    }
    pub fn warn(mut self, warn: bool) -> Self {
        self.warn = warn;
        self
    }
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    pub fn in_primary_group(mut self) -> Self {
        self.press_radius = 0.;
        self
    }
    pub fn focus_target(mut self, handle: FocusHandle) -> Self {
        self.focus_target = Some(handle);
        self
    }
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
    pub fn on_tap(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tap = Some(Box::new(handler));
        self
    }
}

impl ParentElement for FormCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.bd.extend(elements);
    }
}

impl RenderOnce for FormCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let kind = self.kind;
        let (py, px_) = match kind {
            CellKind::Switch => (12., 16.),
            CellKind::Wrap => (8., 16.),
            CellKind::Vcode => (4., 16.),
            _ => (16., 16.),
        };
        let wrap = matches!(kind, CellKind::Wrap | CellKind::Vcode);
        let vertical = kind == CellKind::Vertical;
        let pressable = self.active
            && !self.readonly
            && !self.disabled
            && !matches!(kind, CellKind::Switch | CellKind::Vcode);
        let color = if self.warn {
            t.red
        } else if kind == CellKind::Switch && self.disabled {
            t.fg_3
        } else {
            t.fg_0
        };
        let value_color = if (self.readonly || self.disabled) && kind != CellKind::Switch {
            Some(t.fg_1)
        } else {
            None
        };
        let label_width = self.label_width;
        let label_el = self.label.map(|label| {
            form_label(label, label_width).when(wrap, |el| {
                el.mt(px(if kind == CellKind::Vcode { 12. } else { 8. }))
                    .mb(px(8.))
            })
        });
        let label_desc = self.label_desc.map(|desc| {
            div()
                .pt(px(4.))
                .text_size(px(12.))
                .line_height(relative(1.4))
                .text_color(t.fg_2)
                .child(desc)
        });
        let has_hd = self.hd.is_some() || label_el.is_some();
        let hd = has_hd.then(|| {
            div()
                .flex_shrink_0()
                .when(!wrap && !vertical, |el| el.pr(px(16.)))
                .when(vertical, |el| el.w_full().mb(px(12.)))
                .children(label_el)
                .children(label_desc)
                .children(self.hd)
        });
        let bd = div()
            .flex_1()
            .min_w_0()
            .when(vertical, |el| el.w_full())
            .when(self.bd_flex, |el| el.flex().items_center())
            .when(wrap, |el| el.flex().flex_wrap().items_center())
            .when_some(value_color, |el, c| el.text_color(c))
            .children(self.bd);
        let ft = self.ft.map(|ft| {
            div()
                .flex_shrink_0()
                .flex()
                .items_center()
                .when(!vertical, |el| el.pl(px(16.)))
                .text_color(t.fg_1)
                .child(ft)
        });
        let pressed = t.fg_3;
        let radius = self.press_radius;
        let mut el = div()
            .id(self.id)
            .relative()
            .flex()
            .when(vertical, |el| el.flex_col().items_start())
            .when(!vertical && !wrap, |el| {
                if kind == CellKind::Primary {
                    el.items_start()
                } else {
                    el.items_center()
                }
            })
            .pt(px(py))
            .pb(px(py))
            .pl(px(px_))
            .pr(px(if kind == CellKind::Vcode { 16. } else { px_ }))
            .text_size(px(17.))
            .line_height(relative(24. / 17.))
            .text_color(color)
            .when_some(self.role, |el, role| el.role(role))
            .when_some(self.aria_label, |el, label| el.aria_label(label))
            .when(pressable, |el| {
                el.active(move |s| s.bg(pressed).rounded(px(radius)))
            })
            .children(hd)
            .child(bd)
            .children(ft);
        if let Some(target) = self.focus_target.filter(|_| !self.disabled) {
            el = el.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                window.focus(&target, cx);
            });
        }
        if let Some(handler) = self.on_tap.filter(|_| !self.disabled) {
            el = el.cursor_pointer().on_click(handler);
        }
        Semantics::new(el).disabled(self.disabled)
    }
}

#[derive(IntoElement)]
pub struct FormGroup {
    id: ElementId,
    title: Option<AnyElement>,
    tips: Option<AnyElement>,
    tips_warn: bool,
    primary: bool,
    first: bool,
    role: Option<Role>,
    label: Option<SharedString>,
    rows: Vec<(AnyElement, f32)>,
}

impl FormGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            tips: None,
            tips_warn: false,
            primary: false,
            first: false,
            role: None,
            label: None,
            rows: vec![],
        }
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
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
    pub fn first(mut self) -> Self {
        self.first = true;
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
    pub fn row(mut self, row: impl IntoElement) -> Self {
        self.rows.push((row.into_any_element(), 16.));
        self
    }
    pub fn row_inset(mut self, row: impl IntoElement, inset: f32) -> Self {
        self.rows.push((row.into_any_element(), inset));
        self
    }
    pub fn rows(mut self, rows: impl IntoIterator<Item = AnyElement>, inset: f32) -> Self {
        self.rows.extend(rows.into_iter().map(|r| (r, inset)));
        self
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

impl RenderOnce for FormGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let line = t.fg_3;
        let has_title = self.title.is_some();
        let base: f32 = if self.first {
            0.
        } else if self.primary {
            32.
        } else {
            24.
        };
        let mt = if has_title { base } else { base.max(8.) };
        let rows = self.rows.into_iter().enumerate().map(|(i, (row, inset))| {
            div().relative().child(row).when(i > 0, |el| {
                el.child(hairline(line).top_0().left(px(inset)).right(px(16.)))
            })
        });
        let group = div()
            .id("form-group-cells")
            .relative()
            .mx(px(16.))
            .flex()
            .flex_col()
            .overflow_hidden()
            .when_some(self.role, |el, role| el.role(role))
            .when_some(self.label, |el, label| el.aria_label(label))
            .map(|el| {
                if self.primary {
                    el.bg(t.bg_1).rounded(px(8.))
                } else {
                    el.bg(t.bg_2)
                        .child(hairline(line).top_0().left(px(16.)).right(px(16.)))
                        .child(hairline(line).bottom_0().left(px(16.)).right(px(16.)))
                }
            })
            .children(rows);
        div()
            .id(self.id)
            .w_full()
            .flex()
            .flex_col()
            .mt(px(mt))
            .when_some(self.title, |el, title| {
                el.child(
                    div()
                        .mb(px(8.))
                        .px(px(32.))
                        .text_size(px(14.))
                        .line_height(relative(1.4))
                        .text_color(t.fg_1)
                        .child(title),
                )
            })
            .child(group)
            .when_some(self.tips, |el, tips| {
                el.child(
                    div()
                        .mt(px(8.))
                        .px(px(32.))
                        .text_size(px(14.))
                        .line_height(relative(1.4))
                        .text_color(if self.tips_warn { t.red } else { t.fg_2 })
                        .child(tips),
                )
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FtKind {
    Tips,
    Opr,
    Extra,
}

#[derive(IntoElement)]
pub struct Form {
    id: ElementId,
    title: Option<SharedString>,
    desc: Option<SharedString>,
    control: Vec<AnyElement>,
    ft: Vec<(FtKind, AnyElement)>,
    bottom_fixed: Option<ScrollHandle>,
}

impl Form {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            desc: None,
            control: vec![],
            ft: vec![],
            bottom_fixed: None,
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    pub fn tips_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Tips, el.into_any_element()));
        self
    }
    pub fn opr_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Opr, el.into_any_element()));
        self
    }
    pub fn extra_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Extra, el.into_any_element()));
        self
    }
    pub fn bottom_fixed(mut self, scroll: &ScrollHandle) -> Self {
        self.bottom_fixed = Some(scroll.clone());
        self
    }
}

impl ParentElement for Form {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.control.extend(elements);
    }
}

impl RenderOnce for Form {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let fixed = self.bottom_fixed.is_some();
        let text_area = (self.title.is_some() || self.desc.is_some()).then(|| {
            div()
                .px(px(32.))
                .text_color(t.fg_0)
                .text_center()
                .when_some(self.title, |el, title| {
                    el.child(
                        div()
                            .id("form-title")
                            .role(Role::Heading)
                            .aria_level(2)
                            .text_size(px(22.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .line_height(relative(1.36))
                            .child(title),
                    )
                })
                .when_some(self.desc, |el, desc| {
                    el.child(div().mt(px(16.)).text_size(px(17.)).child(desc))
                })
        });
        let bd = div()
            .flex_1()
            .flex()
            .flex_col()
            .when(fixed, |el| el.pt(px(56.)))
            .children(text_area)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .mt(px(48.))
                    .mb(px(if fixed { 0. } else { 48. }))
                    .children(self.control),
            );
        let kinds: Vec<FtKind> = self.ft.iter().map(|(k, _)| *k).collect();
        let n = kinds.len();
        let ft_items = self.ft.into_iter().enumerate().map(|(i, (kind, el))| {
            let prev = i.checked_sub(1).map(|j| kinds[j]);
            let last = i + 1 == n;
            let (mt, mb) = match kind {
                FtKind::Tips => {
                    let (mut mt, mut mb) = (0., 24.);
                    if prev == Some(FtKind::Opr) {
                        mt = 16.;
                        mb = 0.;
                    }
                    if last {
                        mb = 60.;
                    }
                    (mt, mb)
                }
                FtKind::Opr => (0., if last { 96. } else { 0. }),
                FtKind::Extra => (if prev == Some(FtKind::Tips) { 32. } else { 52. }, 24.),
            };
            div()
                .mt(px(mt))
                .mb(px(mb))
                .px(px(32.))
                .when(kind != FtKind::Opr, |el| {
                    el.flex().flex_col().items_center().text_center()
                })
                .child(el)
        });
        let ft = div().flex().flex_col().children(ft_items);
        match self.bottom_fixed {
            None => div()
                .id(self.id)
                .flex_1()
                .min_h_full()
                .flex()
                .flex_col()
                .pt(px(56.))
                .line_height(relative(1.4))
                .bg(t.bg_2)
                .child(bd)
                .child(ft),
            Some(scroll) => {
                let tool_bg: Hsla = if t.is_dark {
                    rgb(0x191919).into()
                } else {
                    rgb(0xffffff).into()
                };
                div()
                    .id(self.id)
                    .size_full()
                    .flex()
                    .flex_col()
                    .line_height(relative(1.4))
                    .bg(t.bg_2)
                    .child(
                        div()
                            .id("bottom-fixed-content")
                            .flex_1()
                            .min_h_0()
                            .overflow_y_scroll()
                            .track_scroll(&scroll)
                            .child(div().flex().flex_col().pb(px(80.)).child(bd)),
                    )
                    .child(
                        div()
                            .relative()
                            .pt(px(16.))
                            .px(px(32.))
                            .bg(tool_bg)
                            .child(
                                div()
                                    .absolute()
                                    .left_0()
                                    .right_0()
                                    .top(px(-79.))
                                    .h(px(80.))
                                    .bg(linear_gradient(
                                        0.,
                                        linear_color_stop(tool_bg, 0.),
                                        linear_color_stop(tool_bg.opacity(0.), 1.),
                                    )),
                            )
                            .child(ft),
                    )
            }
        }
    }
}

#[derive(IntoElement)]
pub struct TipsGroup {
    items: Vec<AnyElement>,
}

impl Default for TipsGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl TipsGroup {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl ParentElement for TipsGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}

impl RenderOnce for TipsGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let n = self.items.len();
        div()
            .max_w_full()
            .flex()
            .flex_col()
            .items_start()
            .text_left()
            .children(self.items.into_iter().enumerate().map(|(i, item)| {
                div()
                    .flex()
                    .when(i > 0, |el| el.mt(px(8.)))
                    .when(i + 1 < n, |el| el.mb(px(8.)))
                    .child(item)
            }))
    }
}

pub struct ClearableInput {
    input: Entity<TextInput>,
    clear_focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ClearableInput {
    pub fn new(input: Entity<TextInput>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let clear_focus = cx.focus_handle().tab_stop(true);
        let input_focus = input.read(cx).focus_handle(cx);
        let subscriptions = vec![
            cx.observe(&input, |_, _, cx| cx.notify()),
            cx.on_focus_in(&input_focus, window, |_, _, cx| cx.notify()),
            cx.on_blur(&input_focus, window, |_, _, cx| cx.notify()),
            cx.on_blur(&clear_focus, window, |_, _, cx| cx.notify()),
        ];
        Self {
            input,
            clear_focus,
            _subscriptions: subscriptions,
        }
    }
    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }
}

impl Render for ClearableInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let input = self.input.clone();
        let input_focus = input.read(cx).focus_handle(cx);
        let show = !input.read(cx).text().is_empty()
            && (input_focus.is_focused(window) || self.clear_focus.is_focused(window));
        let brand = t.brand;
        let (rest, pressed) = (t.fg_2, t.fg_1);
        div()
            .flex()
            .items_center()
            .w_full()
            .min_w_0()
            .child(div().flex_1().min_w_0().child(input.clone()))
            .when(show, |el| {
                el.child(
                    div()
                        .id("input-clear")
                        .role(Role::Button)
                        .aria_label("清除")
                        .group("input-clear")
                        .track_focus(&self.clear_focus)
                        .flex_shrink_0()
                        .pl(px(8.))
                        .h(px(24.))
                        .flex()
                        .items_center()
                        .cursor_pointer()
                        .focus_visible(move |s| s.shadow(focus_ring(brand)))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_click(move |_, window, cx| {
                            cx.stop_propagation();
                            input.update(cx, |input, cx| input.clear(cx));
                            window.focus(&input_focus, cx);
                        })
                        .child(
                            svg()
                                .path("icon-clear.svg")
                                .id("input-clear-icon")
                                .size(px(18.))
                                .text_color(rest)
                                .group_active("input-clear", move |s| s.text_color(pressed)),
                        ),
                )
            })
    }
}
