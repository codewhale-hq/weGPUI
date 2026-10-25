//! WeUI form page and form groups: `.weui-form` (`src/style/widget/weui-page/weui-form.less`)
//! and `.weui-cells__group_form` (`src/style/widget/weui-cell/weui-cells__group.less` and
//! `weui-cell/weui-form/`). [`Form`] lays out the page areas, [`FormGroup`] a group of rows,
//! [`FormCell`] a labelled row, [`TipsGroup`] stacked tips, [`ClearableInput`] the clear button.
//! Associate a visible label with a text control through [`FormCell::label_for`].
//! Known limitations: safe-area insets are not applied, and there is no submission/validation.
use crate::scrolling::RevealFocus;

use super::ClickHandler;
use crate::{Theme, accessibility::Semantics, input::TextInput};
use gpui::{prelude::*, *};

/// Default form label width in logical pixels (WeUI `@weuiCellLabelWidth`, 105px).
pub const LABEL_WIDTH: f32 = 105.;
/// Maximum rendered label width in logical pixels: the form group's `max-width: 5em` at 17px.
/// [`form_label`] clamps every requested width to this.
pub const LABEL_MAX_WIDTH: f32 = 85.;

/// The keyboard focus indicator used across weGPUI: a 2px inset ring of `color` with no blur.
/// Pass it to `shadow` inside a `focus_visible` style; callers normally use the theme brand.
pub fn focus_ring(color: Hsla) -> Vec<BoxShadow> {
    vec![BoxShadow {
        color,
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(2.),
        inset: true,
    }]
}

/// Base row for selectable form cells (checkbox and radio rows): a vertically centred flex row
/// with 14px padding plus a 2px transparent border (16px in total) that callers recolour on
/// focus, and 17px text on a 24px line.
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

/// Form label (`.weui-label`): non-shrinking `text` that is `width` px wide, clamped to
/// [`LABEL_MAX_WIDTH`], with an 8px right margin. Visual only; it sets no accessible name.
pub fn form_label(text: impl Into<SharedString>, width: f32) -> Div {
    div()
        .flex_shrink_0()
        .w(px(width.min(LABEL_MAX_WIDTH)))
        .mr(px(8.))
        .child(text.into())
}

/// Wraps `el` as a `.weui-cell__control` item (8px vertical and 16px left margin) for the
/// wrapping [`CellKind::Wrap`] and [`CellKind::Vcode`] rows.
pub fn cell_control(el: impl IntoElement) -> Div {
    div().my(px(8.)).ml(px(16.)).child(el)
}

/// Like [`cell_control`], but grows to fill the row (`.weui-cell__control_flex`) with a 112.5px
/// minimum width (upstream `30vw` at a 375px viewport).
pub fn cell_control_flex(el: impl IntoElement) -> Div {
    div()
        .my(px(8.))
        .ml(px(16.))
        .flex_1()
        .min_w(px(112.5))
        .child(el)
}

/// Form page tip text (`.weui-form__tips`): 14px on a 1.4 line height in `fg_1`. Place it in
/// [`Form::tips_area`] or a [`TipsGroup`].
pub fn form_tips(text: impl IntoElement, cx: &App) -> Div {
    let t = Theme::get(cx);
    div()
        .text_size(px(14.))
        .line_height(relative(1.4))
        .text_color(t.fg_1)
        .child(text)
}

/// Inline link for tips and agreements: link-coloured `text`, semibold when `bold` (as in
/// `.weui-cells__tips a`). It is a `Role::Link` tab stop named by `text` with a [`focus_ring`];
/// `on_click` runs on click or Enter/Space and the click does not reach the enclosing row.
/// Call [`RevealFocus::reveal_focus`] after styling it to reveal the link inside a
/// [`crate::scrolling::FocusScroll`].
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

/// Row variant of a [`FormCell`], mirroring WeUI's `weui-cell_*` modifier classes.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellKind {
    /// Ordinary row: 16px padding with the content vertically centred. The default.
    #[default]
    Default,
    /// `.weui-cell_primary`: content aligned to the top, for multi-line bodies such as a textarea.
    Primary,
    /// `.weui-cell_switch`: 12px vertical padding to fit a
    /// [`crate::components::switch::Switch`]; no pressed highlight, disabled text in `fg_3`.
    Switch,
    /// `.weui-cell_wrap`: 8px vertical padding and a wrapping body for [`cell_control`] items,
    /// so a large-font row folds instead of overflowing.
    Wrap,
    /// `.weui-cell_vcode.weui-cell_wrap`: 4px vertical padding so a 32px verification-code button
    /// keeps the row 56px tall; wrapping body and no pressed highlight.
    Vcode,
    /// `.weui-cell_vertical`: the label sits above the body, both full width, 12px apart.
    Vertical,
}

/// One labelled row of a form group (`.weui-cells__group_form .weui-cell`): a header (label,
/// description, custom content), a body holding the children added through `ParentElement`,
/// and an optional footer. Stateless; the caller owns any input entity placed in the body.
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
    label_target: Option<Entity<TextInput>>,
    role: Option<Role>,
    aria_label: Option<SharedString>,
    on_tap: Option<ClickHandler>,
}

impl FormCell {
    /// Creates an empty [`CellKind::Default`] row with a [`LABEL_WIDTH`] label width, the pressed
    /// highlight on (8px corners), and no role, accessible name or handlers.
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
            label_target: None,
            role: None,
            aria_label: None,
            on_tap: None,
        }
    }
    /// Sets the row variant, which controls padding, alignment and pressed feedback. Default
    /// [`CellKind::Default`].
    pub fn kind(mut self, kind: CellKind) -> Self {
        self.kind = kind;
        self
    }
    /// Shows `label` as the row's [`form_label`] at the start of the header. With
    /// [`FormCell::label_for`], it also names the associated text control. Other controls need
    /// their own accessible name. No label by default.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the label width in logical pixels, clamped to [`LABEL_MAX_WIDTH`] when rendered.
    /// Default [`LABEL_WIDTH`], which renders at that 85px cap.
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }
    /// Adds a secondary line under the label (`.weui-cell__desc`: 12px in `fg_2`). None by
    /// default.
    pub fn label_desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.label_desc = Some(desc.into());
        self
    }
    /// Adds custom header content after the label and description, such as an icon. None by
    /// default; the header is omitted when there is neither a label nor custom content.
    pub fn hd(mut self, hd: impl IntoElement) -> Self {
        self.hd = Some(hd.into_any_element());
        self
    }
    /// Lays the body out as a vertically centred flex row, for bodies such as
    /// [`ClearableInput`] that pair a field with trailing controls. Off by default.
    pub fn bd_flex(mut self) -> Self {
        self.bd_flex = true;
        self
    }
    /// Sets the footer (`.weui-cell__ft`): trailing content in `fg_1` with 16px left padding
    /// (none for [`CellKind::Vertical`]), e.g. a switch or a help button. None by default.
    pub fn ft(mut self, ft: impl IntoElement) -> Self {
        self.ft = Some(ft.into_any_element());
        self
    }
    /// Shows the warning state (`.weui-cell_warn`): all row text turns red. Off by default.
    pub fn warn(mut self, warn: bool) -> Self {
        self.warn = warn;
        self
    }
    /// Styles the row as read-only (`.weui-cell_readonly`): no pressed highlight and body text in
    /// `fg_1`. The body control itself must be made read-only separately. Off by default.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }
    /// Styles the row as disabled (`.weui-cell_disabled`): no pressed highlight, body text in
    /// `fg_1` (row text in `fg_3` for [`CellKind::Switch`]), `focus_target` and `on_tap` ignored,
    /// and the node flagged disabled when a role is set. The body control is not disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Enables the pressed highlight (`fg_3` background while pressed). Default `true`; it never
    /// shows for [`CellKind::Switch`], [`CellKind::Vcode`], read-only or disabled rows.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    /// Squares the pressed highlight's corners (0 instead of 8px), as
    /// `.weui-cells__group_form-primary` does. Use it for rows inside a [`FormGroup::primary`].
    pub fn in_primary_group(mut self) -> Self {
        self.press_radius = 0.;
        self
    }
    /// Focuses `handle` when the row receives a left mouse-down, so tapping anywhere on the row
    /// focuses its input as in the upstream examples. Ignored while disabled. None by default.
    pub fn focus_target(mut self, handle: FocusHandle) -> Self {
        self.focus_target = Some(handle);
        self
    }
    /// Associates the visible label with `input` and focuses it when the row is pressed.
    /// The input may be nested in a wrapper such as `ClearableInput` or `TextArea`.
    /// Place the control in the row separately; this does not change its enabled/read-only state.
    /// The label is applied on render, and an explicit `focus_target` takes precedence.
    pub fn label_for(mut self, input: &Entity<TextInput>) -> Self {
        self.label_target = Some(input.clone());
        self
    }
    /// Sets the row's accessibility role. None by default.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    /// Sets the row's accessible name, normally together with [`FormCell::role`]. None by
    /// default.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
    /// Calls `handler` when the row is clicked and shows a pointer cursor, e.g. to toggle a switch
    /// from anywhere on a [`CellKind::Switch`] row. Ignored while disabled. The row is not
    /// focusable, so keep a focusable control in it for keyboard users. None by default.
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
    fn render(mut self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(input) = &self.label_target {
            if let Some(label) = &self.label {
                input.update(cx, |input, cx| input.set_aria_label(label.clone(), cx));
            }
            if self.focus_target.is_none() {
                self.focus_target = Some(input.read(cx).focus_handle(cx));
            }
        }
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
        Semantics::new(el).disabled(self.disabled).reveal_focus()
    }
}

/// A form group (`.weui-cells__group_form`): an optional title, a column of rows separated by
/// 0.5px `fg_3` hairlines in a container inset 16px from each side, and optional tips below.
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
    /// Creates an empty group in the standard style: `bg_2` rows between top and bottom
    /// hairlines, with a 24px top margin.
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
    /// Sets the group title (`.weui-cells__title`): 14px `fg_1` text above the rows with 32px
    /// side padding and 8px below. None by default.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }
    /// Sets the tips under the rows (`.weui-cells__tips`): 14px `fg_2` text with 32px side
    /// padding and 8px above, e.g. a [`form_link`]. None by default.
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.tips = Some(tips.into_any_element());
        self
    }
    /// Shows the tips in red (`.weui-cells__tips_warn`). Off by default.
    pub fn tips_warn(mut self, warn: bool) -> Self {
        self.tips_warn = warn;
        self
    }
    /// Uses the inverted style (`.weui-cells__group_form-primary`): rows on a rounded 8px `bg_1`
    /// card without outer hairlines, and a 32px top margin. Mark its rows `in_primary_group`.
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
    /// Marks the group as the first in the control area (`:first-child`): no top margin when it
    /// has a title, 8px otherwise.
    pub fn first(mut self) -> Self {
        self.first = true;
        self
    }
    /// Sets the accessibility role of the rows container, e.g. `Role::RadioGroup` around
    /// [`crate::components::radio::RadioGroup`] rows. None by default.
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }
    /// Sets the accessible name of the rows container. None by default.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Appends a row. From the second row on, its top separator starts 16px from the left.
    pub fn row(mut self, row: impl IntoElement) -> Self {
        self.rows.push((row.into_any_element(), 16.));
        self
    }
    /// Appends a row whose top separator starts `inset` px from the left, e.g. 56 to line up with
    /// checkbox labels (`.weui-cells_checkbox`). Separators always end 16px from the right.
    pub fn row_inset(mut self, row: impl IntoElement, inset: f32) -> Self {
        self.rows.push((row.into_any_element(), inset));
        self
    }
    /// Appends prebuilt rows, such as [`crate::components::radio::RadioGroup::rows`], all with a
    /// separator left inset of `inset` px.
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

/// A form page (`.weui-form`): a centred title and description, the control area holding the
/// children (usually [`FormGroup`]s), and footer areas for tips, operations and extra content,
/// on a `bg_2` background with 56px top padding. It fills at least its parent's height.
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
    /// Creates an empty page. It does not scroll by itself; place it in a scrolling parent
    /// unless [`Form::bottom_fixed`] is used.
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
    /// Sets the page title (`.weui-form__title`): 22px semibold text, exposed as a level-2
    /// heading named by the text. None by default.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Sets the description under the title (`.weui-form__desc`): 17px text 16px below it. None
    /// by default.
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }
    /// Appends a centred tips area (`.weui-form__tips-area`) to the footer. Footer areas keep
    /// call order and reproduce upstream's sibling margins (e.g. 16px after an operation area).
    pub fn tips_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Tips, el.into_any_element()));
        self
    }
    /// Appends an operation area (`.weui-form__opr-area`), usually the primary button, to the
    /// footer; as the last area it gets a 96px bottom margin.
    pub fn opr_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Opr, el.into_any_element()));
        self
    }
    /// Appends a centred extra area (`.weui-form__extra-area`), usually the page footer, 52px
    /// below the previous area (32px after a tips area).
    pub fn extra_area(mut self, el: impl IntoElement) -> Self {
        self.ft.push((FtKind::Extra, el.into_any_element()));
        self
    }
    /// Switches to the bottom-fixed layout (`.weui-bottom-fixed-opr-page`): the title and
    /// controls scroll inside the form, tracked by `scroll`, while the footer areas stay pinned
    /// over an 80px fade. The form then fills its parent, which must have a bounded height.
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
                            .aria_label(title.clone())
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
                        crate::scrolling::FocusScroll::new(
                            "bottom-fixed-focus-scroll",
                            &scroll,
                            div()
                                .id("bottom-fixed-content")
                                .flex_1()
                                .min_h_0()
                                .overflow_y_scroll()
                                .track_scroll(&scroll)
                                .child(div().flex().flex_col().pb(px(80.)).child(bd)),
                        )
                        .inset_bottom(px(80.)),
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

/// A left-aligned stack of tip rows (`.weui-form__tips__group_primary`) 8px apart, e.g.
/// several [`crate::components::agree::Agree`] lines in one [`Form::tips_area`].
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
    /// Creates an empty group; add rows with `child` or `children`.
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

/// A [`TextInput`] plus the WeUI clear button (`.weui-btn_input-clear`), shown while the text is
/// non-empty and either the input or the button has focus. The button is a `Role::Button` tab
/// stop named "清除"; activating it clears the text and refocuses the input.
pub struct ClearableInput {
    input: Entity<TextInput>,
    clear_focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ClearableInput {
    /// Wraps `input`, which stays shared with the caller; create the entity with
    /// `cx.new(|cx| ClearableInput::new(input, window, cx))`. It observes the input's text and
    /// focus so the button appears and hides without caller notifications.
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
    /// The wrapped text input entity, for reading or changing its text.
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
                        )
                        .reveal_focus(),
                )
            })
    }
}
