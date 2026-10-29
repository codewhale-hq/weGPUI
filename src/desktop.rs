use gpui::{prelude::*, *};

use crate::{I18n, logical::Logical};

#[derive(IntoElement)]
pub struct FormRow {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    label_width: Option<DefiniteLength>,
    control_basis: Length,
    focus_target: Option<FocusHandle>,
    stacked: bool,
    controls: Vec<AnyElement>,
}

impl FormRow {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            label_width: None,
            control_basis: rems(12.).into(),
            focus_target: None,
            stacked: false,
            controls: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn label_width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.label_width = Some(width.into());
        self
    }

    pub fn focus_target(mut self, focus_target: FocusHandle) -> Self {
        self.focus_target = Some(focus_target);
        self
    }

    pub fn control_basis(mut self, basis: impl Into<Length>) -> Self {
        self.control_basis = basis.into();
        self
    }

    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }
}

impl ParentElement for FormRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.controls.extend(elements);
    }
}

impl RenderOnce for FormRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let direction = I18n::get(cx).direction();
        let group_label = self.label.clone();
        let label = div()
            .id("label")
            .min_w_0()
            .max_w_full()
            .text_start(direction)
            .when(!self.stacked, |label| {
                label.when_some(self.label_width, |label, width| {
                    label.w(width).flex_shrink_0()
                })
            })
            .child(self.label)
            .children(self.description.map(|description| div().child(description)))
            .when_some(self.focus_target, |label, focus_target| {
                label
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        window.focus(&focus_target, cx)
                    })
            });
        let controls = div()
            .flex()
            .flex_col()
            .min_w_0()
            .max_w_full()
            .when(self.stacked, |controls| controls.w_full())
            .when(!self.stacked, |controls| {
                controls.flex_1().flex_basis(self.control_basis)
            })
            .children(self.controls);

        div()
            .id(self.id)
            .role(Role::Group)
            .aria_label(group_label)
            .flex()
            .flex_wrap()
            .w_full()
            .min_w_0()
            .gap(rems(0.5))
            .py(rems(0.25))
            .when(self.stacked, |row| row.flex_col().items_start())
            .when(!self.stacked, |row| row.row(direction).items_center())
            .child(label)
            .child(controls)
    }
}
