//! WeUI list cells: `.weui-cells` groups and `.weui-cell` rows (`src/style/widget/weui-cell/`:
//! `weui-cell_global.less`, `weui-access.less`, and the form variants in `weui-cells__group.less`).
//! [`Cells`] is the group container, [`Cell`] a header/body/footer row and [`cells_title`] a
//! standalone group title.
//! Known limitations: rows are separate tab stops with no arrow-key list navigation, and a
//! [`Cell`] without `on_click` is not focusable even when styled with `access` or `link`.

use super::{ClickHandler, Icon};
use crate::Theme;
use gpui::{prelude::*, *};

/// Visual style of a [`Cells`] group.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellsStyle {
    /// `.weui-cells`: full-width `bg_2` group with a top hairline, plus a bottom hairline when it
    /// has rows. The default.
    #[default]
    Plain,
    /// `.weui-cells__group_form`: `bg_2` group inset 16px from each side, outer hairlines inset
    /// 16px, 24px above.
    Form,
    /// `.weui-cells__group_form-primary`: rounded 8px `bg_1` card inset 16px from each side, no
    /// outer hairlines, 32px above.
    FormPrimary,
    /// No background or outer hairlines, only the row separators; for custom containers.
    Bare,
}

/// A group of list rows with an optional title above and tips below. Rows are any elements
/// (typically [`Cell`]) added with `child`/`children`, separated by 0.5px `fg_3` hairlines.
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
    /// Creates an empty [`CellsStyle::Plain`] group with separators inset 16px on the left only
    /// and an 8px top margin (0 when titled, since the title carries the spacing).
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
    /// Sets the element id. Defaults to `"cells"`; give sibling groups distinct ids when their
    /// rows reuse ids, so element state does not collide.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
    /// Sets the group title: [`cells_title`] styling for plain and bare groups, or the form title
    /// (14px `fg_1`, 32px side padding, 8px below) for form styles. None by default.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }
    /// Sets the tips under the group (`.weui-cells__tips`): 14px text 8px below the rows, in
    /// `fg_1` with 16px side padding, or `fg_2` with 32px for form styles. None by default.
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.tips = Some(tips.into_any_element());
        self
    }
    /// Shows the tips in red. Off by default.
    pub fn tips_warn(mut self, warn: bool) -> Self {
        self.tips_warn = warn;
        self
    }
    /// Sets the visual style. Any style other than [`CellsStyle::Plain`] also resets the
    /// separator inset to 16px on both sides, so call [`Cells::separator_inset`] afterwards to
    /// change it. Default [`CellsStyle::Plain`].
    pub fn style(mut self, style: CellsStyle) -> Self {
        self.style = style;
        if style != CellsStyle::Plain {
            self.inset = (16., 16.);
        }
        self
    }
    /// Shorthand for `style(CellsStyle::Form)`.
    pub fn form(self) -> Self {
        self.style(CellsStyle::Form)
    }
    /// Shorthand for `style(CellsStyle::FormPrimary)`.
    pub fn form_primary(self) -> Self {
        self.style(CellsStyle::FormPrimary)
    }
    /// Sets the row separators' left and right insets in logical pixels, e.g. 52 on the left to
    /// clear a leading icon. Default `(16, 0)`, or `(16, 16)` after choosing a non-plain style.
    pub fn separator_inset(mut self, left: f32, right: f32) -> Self {
        self.inset = (left, right);
        self
    }
    /// Overrides the top margin in logical pixels for [`CellsStyle::Plain`] and
    /// [`CellsStyle::Bare`]. Ignored by the form styles, which use a fixed 24px (32px primary).
    pub fn margin_top(mut self, margin: f32) -> Self {
        self.margin_top = Some(margin);
        self
    }
    /// Sets the accessibility role of the rows container, e.g. `Role::RadioGroup`. None by
    /// default.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    /// Sets the accessible name of the rows container. None by default.
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

/// A standalone group title (`.weui-cells__title`): 14px `fg_1` text with 16px side padding,
/// 16px above and 3px below. [`Cells::title`] uses it for plain groups.
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

/// One list row (`.weui-cell`): an optional header, a flexible body with an optional
/// description, and an optional right-aligned footer. It has 16px padding (including a 2px
/// transparent border that turns brand-coloured on keyboard focus) and 17px text on a 24px line.
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
    /// Creates an empty, non-interactive row exposed with the `ListItem` role.
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
    /// Sets the leading header (`.weui-cell__hd`), e.g. an icon or a fixed-width label; with
    /// [`Cell::form`] it gets 16px right padding. None by default.
    pub fn hd(mut self, hd: impl IntoElement) -> Self {
        self.hd = Some(hd.into_any_element());
        self
    }
    /// Sets the body (`.weui-cell__bd`), which fills the remaining width. [`Cell::title`]
    /// replaces it. None by default.
    pub fn bd(mut self, bd: impl IntoElement) -> Self {
        self.bd = Some(bd.into_any_element());
        self
    }
    /// Sets the trailing footer (`.weui-cell__ft`): right-aligned content in `fg_1`, e.g. a
    /// switch or a value. [`Cell::value`] replaces it. None by default.
    pub fn ft(mut self, ft: impl IntoElement) -> Self {
        self.ft = Some(ft.into_any_element());
        self
    }
    /// Sets the body to `title` text and, unless [`Cell::aria_label`] was called before, uses it
    /// as the accessible name.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        let title = title.into();
        if self.label.is_none() {
            self.label = Some(title.clone());
        }
        self.bd = Some(title.into_any_element());
        self
    }
    /// Sets the footer to `value` text, e.g. the current setting shown before an access arrow.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.ft = Some(value.into().into_any_element());
        self
    }
    /// Adds a description line under the body (`.weui-cell__desc`: 12px in `fg_2`). None by
    /// default.
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    /// Sets the accessible name, overriding any name taken from [`Cell::title`].
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Makes it a navigation row (`.weui-cell_access`): a 12×24px `fg_2` arrow after the footer
    /// and the pressed highlight. The navigation itself is the caller's `on_click`.
    pub fn access(mut self) -> Self {
        self.access = true;
        self.active = true;
        self
    }
    /// Styles the row as a link (`.weui-cell_link`): link-coloured text, the `Link` role unless
    /// [`Cell::role`] is set, and the pressed highlight.
    pub fn link(mut self) -> Self {
        self.link = true;
        self.active = true;
        self
    }
    /// Enables the pressed highlight (`.weui-cell_active`: `fg_3` background while pressed). Off
    /// by default.
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }
    /// Aligns header, body and footer to the top (`.weui-cell_primary`) instead of centring them.
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
    /// Applies form-group spacing: 16px between header, body and footer, and 8px corners on the
    /// pressed highlight. Use it for rows in a form-style [`Cells`] or a
    /// [`crate::components::form::FormGroup`].
    pub fn form(mut self) -> Self {
        self.form = true;
        self
    }
    /// Shows the row text in red (`.weui-cell_warn`); link rows keep the link colour. Off by
    /// default.
    pub fn warn(mut self, warn: bool) -> Self {
        self.warn = warn;
        self
    }
    /// Disables the row: `on_click`, focus and the pressed highlight are dropped and the node is
    /// flagged disabled. The text colour is unchanged. Off by default.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Overrides the padding, 16px on every side by default. The values include the 2px focus
    /// border, so a side below 2px lets the content overlap that border.
    pub fn padding(mut self, padding: Edges<Pixels>) -> Self {
        self.padding = Some(padding);
        self
    }
    /// Sets a minimum row height in logical pixels. None by default.
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }
    /// Overrides the accessibility role. By default it is `Link` for [`Cell::link`] rows,
    /// `Button` for enabled rows with `on_click`, and `ListItem` otherwise.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    /// Exposes a selected state (`aria-selected`) to assistive technology. Unset by default.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }
    /// Uses `focus` as the row's focus handle so the caller can focus it programmatically. Only
    /// applied while the row is clickable (it has `on_click` and is enabled).
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Calls `handler` on click or, while focused, on Enter/Space. Unless disabled, the row
    /// becomes a tab stop with a brand-coloured focus border. None by default.
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
        } else if self.active && !self.disabled {
            el = el.active(move |s| s.bg(pressed).rounded(px(radius)));
        }
        crate::accessibility::Semantics::new(el).disabled(self.disabled)
    }
}
