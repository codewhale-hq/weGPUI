//! WeUI form preview (`src/style/widget/weui-cell/weui-form/weui-form-preview.less`, examples
//! `src/example/preview/preview.html` and `src/example/msg/msg_custom_area_preview.html`).
//! [`FormPreview`] is the read-only summary card with header, items and footer buttons;
//! [`FormPreviewList`] is the `.weui-form-preview__list` block for msg page custom areas.
//! Summary rows are exposed individually within a `List` node.
use crate::scrolling::RevealFocus;

use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

/// Colour of a [`FormPreviewButton`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FormPreviewButtonKind {
    /// `.weui-form-preview__btn_primary`: link colour. The default.
    #[default]
    Primary,
    /// `.weui-form-preview__btn_default`: the secondary `fg_half` colour.
    Default,
}

/// A footer action of a [`FormPreview`] (`.weui-form-preview__btn`). Buttons share the footer
/// width equally, separated by hairlines, and are focusable `Role::Button` nodes named by their
/// label.
pub struct FormPreviewButton {
    label: SharedString,
    kind: FormPreviewButtonKind,
    on_click: Option<ClickHandler>,
}

impl FormPreviewButton {
    /// Creates a [`FormPreviewButtonKind::Primary`] button; `label` is its text and accessible
    /// name.
    pub fn primary(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: FormPreviewButtonKind::Primary,
            on_click: None,
        }
    }
    /// Creates a [`FormPreviewButtonKind::Default`] (secondary) button. This inherent
    /// constructor takes a label; it is not `Default::default`.
    pub fn default(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: FormPreviewButtonKind::Default,
            on_click: None,
        }
    }
    /// Calls `handler` on click or Enter/Space. Without it the button still renders, takes focus
    /// and shows its pressed state, but does nothing.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

/// A form preview card (`.weui-form-preview`): an optional header with a large value, label and
/// value items, and a footer of buttons, on `bg_2` between top and bottom hairlines.
#[derive(IntoElement)]
pub struct FormPreview {
    id: ElementId,
    header: Option<(SharedString, SharedString)>,
    items: Vec<(SharedString, SharedString)>,
    buttons: Vec<FormPreviewButton>,
}

impl FormPreview {
    /// Creates an empty card; sections without content are not rendered.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            items: vec![],
            buttons: vec![],
        }
    }
    /// Sets the header (`.weui-form-preview__hd`): `label` in `fg_1` and `value` right-aligned at
    /// 25.6px (1.6em), e.g. an amount, exposed as one `Label` named by label and value.
    pub fn header(
        mut self,
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        self.header = Some((label.into(), value.into()));
        self
    }
    /// Appends a body item (`.weui-form-preview__item`): the label in a 60.48px (4.2em) `fg_1`
    /// column and the value right-aligned in `fg_0`, 14.4px text on a 28.8px line.
    pub fn item(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.items.push((label.into(), value.into()));
        self
    }
    /// Appends a footer button; buttons render left to right in call order.
    pub fn button(mut self, button: FormPreviewButton) -> Self {
        self.buttons.push(button);
        self
    }
}

fn hairline(color: Hsla) -> Div {
    div().absolute().h(px(0.5)).bg(color)
}

fn preview_item(
    label: SharedString,
    value: impl IntoElement,
    label_width: f32,
    gap: f32,
    label_color: Hsla,
) -> Div {
    div()
        .flex()
        .items_start()
        .child(
            div()
                .flex_shrink_0()
                .w(px(label_width))
                .mr(px(gap))
                .text_left()
                .text_color(label_color)
                .child(label),
        )
        .child(div().flex_1().min_w_0().child(value))
}

impl RenderOnce for FormPreview {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let id = self.id.clone();
        let header = self.header.map(|(label, value)| {
            let aria = SharedString::from(format!("{label} {value}"));
            div()
                .id("form-preview-hd")
                .role(Role::Label)
                .aria_label(aria)
                .relative()
                .p(px(16.))
                .text_size(px(16.))
                .line_height(px(40.))
                .child(preview_item(
                    label,
                    div()
                        .text_right()
                        .text_size(px(25.6))
                        .text_color(t.fg_0)
                        .child(value),
                    67.2,
                    16.,
                    t.fg_1,
                ))
                .child(hairline(t.fg_3).bottom_0().left(px(16.)).right_0())
        });
        let fg0 = t.fg_0;
        let fg1 = t.fg_1;
        let count = self.items.len();
        let body = (count > 0).then(|| {
            div()
                .id("form-preview-bd")
                .role(Role::List)
                .p(px(16.))
                .text_size(px(14.4))
                .line_height(px(28.8))
                .text_color(fg1)
                .children(
                    self.items
                        .into_iter()
                        .enumerate()
                        .map(move |(i, (label, value))| {
                            let name = SharedString::from(format!("{label} {value}"));
                            preview_item(
                                label,
                                div().text_right().text_color(fg0).child(value),
                                60.48,
                                14.4,
                                fg1,
                            )
                            .id(("preview-item", i))
                            .role(Role::ListItem)
                            .aria_label(name)
                            .aria_position_in_set(i + 1)
                            .aria_size_of_set(count)
                        }),
                )
        });
        let line = t.dialog_line_color;
        let pressed = t.bg_color_active;
        let brand = t.brand;
        let footer = (!self.buttons.is_empty()).then(|| {
            div()
                .relative()
                .flex()
                .text_size(px(16.))
                .line_height(px(50.))
                .children(self.buttons.into_iter().enumerate().map(|(i, b)| {
                    let color = match b.kind {
                        FormPreviewButtonKind::Primary => t.link,
                        FormPreviewButtonKind::Default => t.fg_half,
                    };
                    div()
                        .id(ElementId::NamedInteger("form-preview-btn".into(), i as u64))
                        .role(Role::Button)
                        .aria_label(b.label.clone())
                        .relative()
                        .flex_1()
                        .text_center()
                        .text_color(color)
                        .border_2()
                        .border_color(transparent_black())
                        .my(px(-2.))
                        .cursor_pointer()
                        .focusable()
                        .tab_stop(true)
                        .active(move |s| s.bg(pressed))
                        .focus_visible(move |s| s.border_color(brand))
                        .when_some(b.on_click, |el, h| el.on_click(h))
                        .child(b.label)
                        .when(i > 0, |el| {
                            el.child(
                                div()
                                    .absolute()
                                    .left(px(-2.))
                                    .top(px(-2.))
                                    .bottom(px(-2.))
                                    .w(px(0.5))
                                    .bg(line),
                            )
                        })
                        .reveal_focus()
                }))
                .child(hairline(line).top_0().left_0().right_0())
        });
        div()
            .id(id)
            .relative()
            .flex()
            .flex_col()
            .bg(t.bg_2)
            .children(header)
            .children(body)
            .children(footer)
            .child(hairline(t.fg_3).top_0().left_0().right_0())
            .child(hairline(t.fg_3).bottom_0().left_0().right_0())
    }
}

/// A compact label/value list (`.weui-form-preview__list`) for msg page custom areas: 14px rows
/// 12px apart with an 84px (6em) `fg_1` label column, below a top hairline and 24px of padding.
#[derive(IntoElement)]
pub struct FormPreviewList {
    id: ElementId,
    items: Vec<(SharedString, SharedString)>,
    last: bool,
}

impl Default for FormPreviewList {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

impl FormPreviewList {
    /// Creates an empty list treated as the last block in its container (no bottom padding).
    #[track_caller]
    pub fn new() -> Self {
        Self {
            id: ElementId::from(std::panic::Location::caller()),
            items: vec![],
            last: true,
        }
    }
    /// Sets a stable list id; use distinct ids for lists created in the same loop.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    /// Appends a row, exposed as a `ListItem` named by the label and value.
    pub fn item(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.items.push((label.into(), value.into()));
        self
    }
    /// Sets whether this is the last block in its container (`:last-child`); `false` adds 24px
    /// bottom padding. Default `true`.
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }
}

impl RenderOnce for FormPreviewList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let (fg0, fg1) = (t.fg_0, t.fg_1);
        let count = self.items.len();
        div()
            .id(self.id)
            .role(Role::List)
            .relative()
            .pt(px(24.))
            .when(!self.last, |el| el.pb(px(24.)))
            .flex()
            .flex_col()
            .gap(px(12.))
            .text_size(px(14.))
            .line_height(relative(1.4))
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(move |(i, (label, value))| {
                        div()
                            .id(ElementId::NamedInteger(
                                "form-preview-list-item".into(),
                                i as u64,
                            ))
                            .role(Role::ListItem)
                            .aria_position_in_set(i + 1)
                            .aria_size_of_set(count)
                            .aria_label(SharedString::from(format!("{label} {value}")))
                            .child(preview_item(
                                label,
                                div().text_color(fg0).child(value),
                                84.,
                                14.,
                                fg1,
                            ))
                    }),
            )
            .child(hairline(t.fg_3).top_0().left_0().right_0())
    }
}
