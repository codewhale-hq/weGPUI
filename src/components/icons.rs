use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum IconVariant {
    #[default]
    Normal,
    Msg,
    MsgPrimary,
}

#[derive(IntoElement)]
pub struct Icon {
    name: SharedString,
    w: Option<f32>,
    h: Option<f32>,
    color: Option<Hsla>,
    variant: IconVariant,
    label: Option<SharedString>,
    active: Option<(SharedString, Hsla)>,
}

impl Icon {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            w: None,
            h: None,
            color: None,
            variant: IconVariant::Normal,
            label: None,
            active: None,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.w = Some(size);
        self.h = Some(size);
        self
    }
    pub fn w(mut self, w: f32) -> Self {
        self.w = Some(w);
        self
    }
    pub fn h(mut self, h: f32) -> Self {
        self.h = Some(h);
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn msg(mut self) -> Self {
        self.variant = IconVariant::Msg;
        self
    }
    pub fn msg_primary(mut self) -> Self {
        self.variant = IconVariant::MsgPrimary;
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn group_active_color(
        mut self,
        group: impl Into<SharedString>,
        color: impl Into<Hsla>,
    ) -> Self {
        self.active = Some((group.into(), color.into()));
        self
    }
}

fn narrow(name: &str) -> bool {
    matches!(
        name,
        "arrow" | "arrow-bold" | "back-arrow" | "back-arrow-thin"
    )
}

pub fn icon_color(name: &str, variant: IconVariant, cx: &App) -> Hsla {
    let t = Theme::get(cx);
    match (variant, name) {
        (IconVariant::Msg, "warn") => return t.red,
        (IconVariant::Msg, "info-circle") => return t.blue,
        (IconVariant::MsgPrimary, "warn") => return t.yellow,
        _ => {}
    }
    match name {
        "success"
        | "success-circle"
        | "success-no-circle"
        | "success-no-circle-thin"
        | "download"
        | "safe-success" => t.brand,
        "waiting" | "info" | "waiting-circle" => t.blue,
        "warn" | "cancel" => t.red,
        "circle" | "info-circle" | "clear" | "arrow" | "arrow-bold" => t.fg_2,
        "search" => t.fg_1,
        "safe-warn" => t.yellow,
        _ => t.fg_0,
    }
}

impl RenderOnce for Icon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let base = match self.variant {
            IconVariant::Normal => 24.,
            _ => 64.,
        };
        let w = self
            .w
            .unwrap_or(if narrow(&self.name) { base / 2. } else { base });
        let h = self.h.unwrap_or(base);
        let color = self
            .color
            .unwrap_or_else(|| icon_color(&self.name, self.variant, cx));
        let icon = svg()
            .path(SharedString::from(format!("icon-{}.svg", self.name)))
            .w(px(w))
            .h(px(h))
            .flex_shrink_0()
            .text_color(color);
        let icon = match self.active {
            Some((group, active)) => icon
                .id(SharedString::from(format!("icon-svg-{}", self.name)))
                .group_active(group, move |s| s.text_color(active))
                .into_any_element(),
            None => icon.into_any_element(),
        };
        match self.label {
            Some(label) => div()
                .id(SharedString::from(format!("icon-{}-{}", self.name, label)))
                .role(Role::Image)
                .aria_label(label)
                .flex_shrink_0()
                .child(icon)
                .into_any_element(),
            None => icon.into_any_element(),
        }
    }
}

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    label: SharedString,
    btn_icon: bool,
    disabled: bool,
    focus: Option<FocusHandle>,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        let icon = icon.into();
        Self {
            id: id.into(),
            label: icon.clone(),
            icon: Icon::new(icon),
            btn_icon: false,
            disabled: false,
            focus: None,
            on_click: None,
        }
    }
    pub fn goback(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id, "back-arrow").label("返回");
        this.icon = this.icon.w(12.).h(24.);
        this
    }
    pub fn close(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id, "close").label("关闭");
        this.icon = this.icon.size(24.);
        this
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn size(mut self, size: f32) -> Self {
        self.icon = self.icon.size(size);
        self
    }
    pub fn w(mut self, w: f32) -> Self {
        self.icon = self.icon.w(w);
        self
    }
    pub fn h(mut self, h: f32) -> Self {
        self.icon = self.icon.h(h);
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.icon = self.icon.color(color);
        self
    }
    pub fn btn_icon(mut self) -> Self {
        self.btn_icon = true;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl RenderOnce for IconButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let (pressed, ring) = (t.fg_1, t.brand);
        let group = SharedString::from("weui-icon-btn");
        let mut icon = self.icon;
        if self.btn_icon {
            icon = icon.group_active_color(group.clone(), pressed);
        } else if icon.color.is_none() {
            icon = icon.color(t.fg_0);
        }
        let icon_el = div()
            .id("weui-icon-btn-icon")
            .flex()
            .when(!self.disabled && !self.btn_icon, |el| {
                el.group_active(group.clone(), |s| s.opacity(0.5))
            })
            .when(self.disabled, |el| el.opacity(0.3))
            .child(icon);
        let mut hit = div()
            .id(self.id)
            .role(Role::Button)
            .aria_label(self.label)
            .absolute()
            .top(px(-10.))
            .bottom(px(-10.))
            .left(px(-10.))
            .right(px(-10.))
            .min_w(px(44.))
            .min_h(px(44.))
            .rounded(px(8.))
            .border_2()
            .border_color(transparent_black());
        if !self.disabled {
            hit = hit
                .focusable()
                .tab_stop(true)
                .focus_visible(move |s| s.border_color(ring));
            if let Some(focus) = self.focus {
                hit = hit.track_focus(&focus);
            }
            if let Some(handler) = self.on_click {
                hit = hit.on_click(handler);
            }
        }
        let hit = crate::accessibility::Semantics::new(hit).disabled(self.disabled);
        div()
            .group(group)
            .relative()
            .flex()
            .flex_shrink_0()
            .child(icon_el)
            .child(hit)
    }
}
