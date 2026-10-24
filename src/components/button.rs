use super::{ClickHandler, loading::Loading};
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Size {
    #[default]
    Large,
    Medium,
    Mini,
    XMini,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonKind {
    #[default]
    Default,
    Primary,
    Warn,
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    kind: ButtonKind,
    size: Size,
    disabled: bool,
    loading: bool,
    block: bool,
    inline: bool,
    overlay: bool,
    cell: bool,
    margin_reset: bool,
    width: Option<f32>,
    min_width: Option<f32>,
    padding_x: Option<f32>,
    flex_1: bool,
    icon: Option<AnyElement>,
    title: Option<SharedString>,
    focus: Option<FocusHandle>,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            id: label.clone().into(),
            label,
            kind: ButtonKind::Default,
            size: Size::Large,
            disabled: false,
            loading: false,
            block: false,
            inline: false,
            overlay: false,
            cell: false,
            margin_reset: false,
            width: None,
            min_width: None,
            padding_x: None,
            flex_1: false,
            icon: None,
            title: None,
            focus: None,
            on_click: None,
        }
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn primary(mut self) -> Self {
        self.kind = ButtonKind::Primary;
        self
    }
    pub fn warn(mut self) -> Self {
        self.kind = ButtonKind::Warn;
        self
    }
    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn medium(self) -> Self {
        self.size(Size::Medium)
    }
    pub fn mini(self) -> Self {
        self.size(Size::Mini)
    }
    pub fn xmini(self) -> Self {
        self.size(Size::XMini)
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn loading(mut self, value: bool) -> Self {
        self.loading = value;
        self
    }
    pub fn block(mut self) -> Self {
        self.block = true;
        self
    }
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }
    pub fn overlay(mut self) -> Self {
        self.overlay = true;
        self
    }
    pub fn cell(mut self) -> Self {
        self.cell = true;
        self
    }
    pub fn margin_reset(mut self) -> Self {
        self.margin_reset = true;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }
    pub fn padding_x(mut self, padding: f32) -> Self {
        self.padding_x = Some(padding);
        self
    }
    pub fn flex_1(mut self) -> Self {
        self.flex_1 = true;
        self
    }
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
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

    fn colors(&self, t: &Theme) -> (Hsla, Hsla) {
        let white: Hsla = white();
        if self.overlay {
            if self.disabled {
                return (rgba(0xffffff4d).into(), rgba(0xf7f7f74d).into());
            }
            return match self.kind {
                ButtonKind::Primary => (white, rgba(0x000000e6).into()),
                ButtonKind::Default => (rgba(0xf7f7f74d).into(), white),
                ButtonKind::Warn => (t.red_100, white),
            };
        }
        if self.disabled {
            return (t.fg_5, t.fg_4);
        }
        match self.kind {
            ButtonKind::Primary => (t.brand, white),
            ButtonKind::Default => (t.fg_5, t.fg_0),
            ButtonKind::Warn => (t.red_100, white),
        }
    }

    fn render_cell(self, t: &Theme) -> AnyElement {
        let fg = match self.kind {
            ButtonKind::Default => t.fg_0,
            ButtonKind::Primary => t.link,
            ButtonKind::Warn => t.red,
        };
        let (bg, active, ring) = (t.bg_5, t.bg_color_active, t.brand);
        let mut el = div()
            .id(self.id)
            .role(Role::Button)
            .aria_label(self.title.unwrap_or_else(|| self.label.clone()))
            .relative()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .p(px(14.))
            .border_2()
            .border_color(transparent_black())
            .overflow_hidden()
            .bg(bg)
            .text_color(fg)
            .text_size(px(17.))
            .line_height(px(24.))
            .when_some(self.icon, |el, icon| {
                el.child(div().size(px(24.)).mr(px(17. * 0.34)).child(icon))
            })
            .child(self.label);
        if self.disabled {
            el = el.opacity(0.5);
        } else {
            el = el
                .focusable()
                .tab_stop(true)
                .active(move |s| s.bg(active))
                .focus_visible(move |s| s.border_color(ring));
            if let Some(focus) = self.focus {
                el = el.track_focus(&focus);
            }
            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        }
        crate::accessibility::Semantics::new(el)
            .disabled(self.disabled)
            .into_any_element()
    }
}

pub fn clamp_lines(
    text: &SharedString,
    size: f32,
    weight: FontWeight,
    max_width: f32,
    lines: usize,
    window: &Window,
) -> SharedString {
    let mut font = window.text_style().font();
    font.weight = weight;
    let count = |s: &str| -> usize {
        let run = TextRun {
            len: s.len(),
            font: font.clone(),
            color: black(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        window
            .text_system()
            .shape_text(
                SharedString::from(s.to_string()),
                px(size),
                &[run],
                Some(px(max_width)),
                None,
            )
            .map(|ls| ls.iter().map(|l| l.wrap_boundaries().len() + 1).sum())
            .unwrap_or(1)
    };
    if text.is_empty() || count(text) <= lines {
        return text.clone();
    }
    let bounds: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    let (mut lo, mut hi) = (0, bounds.len());
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if count(&format!("{}…", &text[..bounds[mid]])) <= lines {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    SharedString::from(format!("{}…", &text[..bounds[lo]]))
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        if self.cell {
            return self.render_cell(&t);
        }
        let (bg, fg) = self.colors(&t);
        let small = matches!(self.size, Size::Mini | Size::XMini);
        let (h, font, pad_x, radius) = match self.size {
            Size::Large => (t.btn_height, 17., 24., 8.),
            Size::Medium => (t.btn_height_medium, 14., 24., 8.),
            Size::Mini => (t.btn_height_small, 14., 12., 6.),
            Size::XMini => (28., 14., 12., 4.),
        };
        let lh = match self.size {
            Size::Large => h - 24.,
            Size::Medium => h - 20.,
            Size::Mini => 20.,
            Size::XMini => 20.,
        };
        let pad_y = (h - lh) / 2.;
        let pad_x = self.padding_x.unwrap_or(pad_x);
        let min_w = self
            .min_width
            .unwrap_or(if small || self.inline { 0. } else { 184. });
        let mask = t.btn_active_mask;
        let ring = match (self.overlay, self.kind) {
            (false, ButtonKind::Default) | (true, ButtonKind::Primary) => t.brand,
            _ => white(),
        };
        let spinner_color: Hsla = if self.kind == ButtonKind::Default && !self.overlay {
            rgb(0x606060).into()
        } else {
            fg
        };
        let interactive = !self.disabled && !self.loading;
        let text_max =
            self.width.unwrap_or(280.).min(280.) - 2. * pad_x - if self.loading { 24. } else { 0. };
        let label = clamp_lines(&self.label, font, FontWeight::SEMIBOLD, text_max, 2, window);
        let group = SharedString::from("weui-btn");
        let label_only = self.label.is_empty();
        let aria = self.title.clone().unwrap_or_else(|| {
            if label_only {
                SharedString::from("正在操作")
            } else {
                self.label.clone()
            }
        });
        let mut el = div()
            .id(self.id)
            .group(group.clone())
            .role(Role::Button)
            .aria_label(aria)
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .min_w(px(min_w))
            .max_w(px(280.))
            .pl(px(pad_x - 2.))
            .pr(px(pad_x - 2.))
            .pt(px(pad_y - 2.))
            .pb(px(pad_y - 2.))
            .border_2()
            .border_color(transparent_black())
            .rounded(px(radius))
            .bg(bg)
            .text_color(fg)
            .text_size(px(font))
            .line_height(px(lh))
            .font_weight(FontWeight::SEMIBOLD)
            .when(
                !small && !self.inline && !self.margin_reset && !self.flex_1,
                |el| el.mx_auto(),
            )
            .when(self.block, |el| el.w_full())
            .when_some(self.width, |el, w| el.w(px(w)))
            .when(self.flex_1, |el| el.flex_1().w_full().min_w_0())
            .when(self.loading, |el| {
                el.child(
                    div()
                        .when(!label_only, |el| el.mr(px(4.)))
                        .child(Loading::mask().size(20.).color(spinner_color)),
                )
            })
            .children(self.icon)
            .when(!label_only, |el| {
                el.child(
                    div()
                        .min_w_0()
                        .max_w(px(text_max))
                        .text_center()
                        .child(label),
                )
            })
            .when(label_only && !self.loading, |el| el.h(px(h)));
        if label_only && self.loading {
            el = el.h(px(h));
        }
        if interactive {
            el = el
                .child(
                    div()
                        .id("weui-btn-mask")
                        .absolute()
                        .top(px(-2.))
                        .left(px(-2.))
                        .right(px(-2.))
                        .bottom(px(-2.))
                        .rounded(px(radius))
                        .group_active(group, move |s| s.bg(mask)),
                )
                .focusable()
                .tab_stop(true)
                .focus_visible(move |s| s.border_color(ring));
            if let Some(focus) = self.focus {
                el = el.track_focus(&focus);
            }
            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        } else {
            el = el.aria_description(if self.loading {
                "正在加载"
            } else {
                "不可用"
            });
        }
        crate::accessibility::Semantics::new(el)
            .disabled(self.disabled)
            .busy(self.loading)
            .into_any_element()
    }
}

#[derive(IntoElement)]
pub struct ButtonArea {
    inline: bool,
    margin: Option<Edges<Pixels>>,
    buttons: Vec<Button>,
}

impl Default for ButtonArea {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonArea {
    pub fn new() -> Self {
        Self {
            inline: false,
            margin: None,
            buttons: vec![],
        }
    }
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }
    pub fn margin(mut self, margin: Edges<Pixels>) -> Self {
        self.margin = Some(margin);
        self
    }
    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }
    pub fn buttons(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.buttons.extend(buttons);
        self
    }
}

impl RenderOnce for ButtonArea {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let m = self.margin.unwrap_or(Edges {
            top: px(48.),
            right: px(16.),
            bottom: px(8.),
            left: px(16.),
        });
        let inline = self.inline;
        div()
            .mt(m.top)
            .mr(m.right)
            .mb(m.bottom)
            .ml(m.left)
            .flex()
            .gap(px(16.))
            .when(inline, |el| el.flex_row())
            .when(!inline, |el| el.flex_col())
            .children(
                self.buttons
                    .into_iter()
                    .map(move |b| if inline { b.flex_1() } else { b }),
            )
    }
}

#[derive(IntoElement)]
pub struct BottomFixedOpr {
    wrap: bool,
    padding: Option<Edges<Pixels>>,
    buttons: Vec<Button>,
}

impl Default for BottomFixedOpr {
    fn default() -> Self {
        Self::new()
    }
}

impl BottomFixedOpr {
    pub fn new() -> Self {
        Self {
            wrap: false,
            padding: None,
            buttons: vec![],
        }
    }
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    pub fn padding(mut self, padding: Edges<Pixels>) -> Self {
        self.padding = Some(padding);
        self
    }
    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }
    pub fn buttons(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.buttons.extend(buttons);
        self
    }
}

impl RenderOnce for BottomFixedOpr {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let many = self.buttons.len() > 1;
        let wrap = self.wrap;
        div()
            .when_some(self.padding, |el, p| {
                el.pt(p.top).pr(p.right).pb(p.bottom).pl(p.left)
            })
            .flex()
            .items_center()
            .justify_center()
            .when(wrap, |el| el.flex_col().gap(px(16.)))
            .when(!wrap, |el| el.gap(px(16.)))
            .children(self.buttons.into_iter().map(move |b| {
                let b = b.margin_reset();
                if wrap && many {
                    b.width(184.)
                } else if many {
                    b.width(136.).min_width(0.).padding_x(16.)
                } else {
                    b.width(184.).padding_x(16.)
                }
            }))
    }
}

#[derive(IntoElement)]
pub struct BottomFixedOprPage {
    id: ElementId,
    content: Vec<AnyElement>,
    tips: Option<AnyElement>,
    opr: BottomFixedOpr,
    tool_bg: Option<Hsla>,
}

impl BottomFixedOprPage {
    pub fn new(id: impl Into<ElementId>, opr: BottomFixedOpr) -> Self {
        Self {
            id: id.into(),
            content: vec![],
            tips: None,
            opr,
            tool_bg: None,
        }
    }
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.tips = Some(tips.into_any_element());
        self
    }
    pub fn tool_bg(mut self, bg: Hsla) -> Self {
        self.tool_bg = Some(bg);
        self
    }
}

impl ParentElement for BottomFixedOprPage {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements);
    }
}

impl RenderOnce for BottomFixedOprPage {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let solid: Hsla = self.tool_bg.unwrap_or_else(|| {
            if Theme::get(cx).is_dark {
                rgb(0x191919).into()
            } else {
                rgb(0xffffff).into()
            }
        });
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(self.id)
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(div().pb(px(80.)).children(self.content)),
            )
            .child(
                div()
                    .relative()
                    .flex_shrink_0()
                    .pt(px(16.))
                    .px(px(32.))
                    .pb(px(24.))
                    .bg(solid)
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .top(px(-79.))
                            .h(px(80.))
                            .bg(linear_gradient(
                                0.,
                                linear_color_stop(solid, 0.),
                                linear_color_stop(solid.opacity(0.), 1.),
                            )),
                    )
                    .when_some(self.tips, |el, tips| {
                        el.child(
                            div()
                                .mb(px(24.))
                                .px(px(32.))
                                .flex()
                                .justify_center()
                                .child(tips),
                        )
                    })
                    .child(self.opr),
            )
    }
}
