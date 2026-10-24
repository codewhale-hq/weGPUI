use super::{ClickHandler, loading::Loading};
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Size {
    #[default]
    Large,
    Medium,
    Mini,
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
impl RenderOnce for Button {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let bg = if self.disabled {
            t.fg_5
        } else {
            match self.kind {
                ButtonKind::Primary => t.brand,
                ButtonKind::Warn => t.red_100,
                ButtonKind::Default => t.fg_5,
            }
        };
        let fg = if self.disabled {
            t.fg_4
        } else if self.kind == ButtonKind::Default {
            t.fg_0
        } else {
            white()
        };
        let h = match self.size {
            Size::Large => t.btn_height,
            Size::Medium => t.btn_height_medium,
            Size::Mini => t.btn_height_small,
        };
        let min_w = match self.size {
            Size::Large => 184.,
            Size::Medium => 184.,
            Size::Mini => 0.,
        };
        let mask = t.btn_active_mask;
        let hover = t.statelayer_hovered;
        let brand = t.brand;
        let mut el = div()
            .id(self.id)
            .role(Role::Button)
            .aria_label(if self.label.is_empty() {
                SharedString::from("正在加载")
            } else {
                self.label.clone()
            })
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .h(px(h))
            .min_w(px(min_w))
            .max_w(px(280.))
            .px(px(if self.size == Size::Mini { 12. } else { 24. }))
            .flex_shrink_0()
            .rounded(px(if self.size == Size::Mini { 6. } else { 8. }))
            .bg(bg)
            .text_color(fg)
            .text_size(px(if self.size == Size::Mini { 14. } else { 17. }))
            .line_height(px(if self.size == Size::Large { 24. } else { 20. }))
            .font_weight(FontWeight::MEDIUM)
            .when(self.block, |el| el.w_full())
            .when(self.loading, |el| {
                el.child(Loading::new().size(20.).color(fg))
            })
            .child(self.label);
        if self.disabled || self.loading {
            el = el.aria_description(if self.loading {
                "正在加载"
            } else {
                "不可用"
            });
        } else {
            el = el
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .hover(move |s| s.bg(bg.blend(hover)))
                .active(move |s| s.bg(bg.blend(mask)))
                .focus_visible(move |s| s.border_2().border_color(brand));
            if let Some(focus) = self.focus {
                el = el.track_focus(&focus);
            }
            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        }
        crate::accessibility::Semantics::new(el)
            .disabled(self.disabled || self.loading)
            .busy(self.loading)
    }
}
