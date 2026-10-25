//! WeUI icons (`src/style/icon/weui-icon.less`) drawn as tinted SVGs from the bundled `icon-*.svg`
//! assets, plus [`IconButton`], the tappable icon of `weui-icon-btn` and `weui-btn_icon`.
//!
//! [`Icon`] loads `icon-{name}.svg` through the app's `AssetSource`, so register [`crate::Assets`]
//! (or serve the same names) first. Known limitations: an unknown name paints nothing in a box of
//! the icon's size, and an icon without a label is invisible to assistive technology.
use crate::scrolling::RevealFocus;

use super::ClickHandler;
use crate::Theme;
use gpui::{prelude::*, *};

/// Size and colour preset of an [`Icon`].
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum IconVariant {
    /// Inline icon: 24px square (WeUI's `2.4em` at a 10px icon font size).
    #[default]
    Normal,
    /// `weui-icon_msg`: 64px result icon; `warn` is red and `info-circle` is blue.
    Msg,
    /// `weui-icon_msg-primary`: 64px result icon; `warn` is yellow.
    MsgPrimary,
}

/// A single WeUI icon, tinted with one colour.
///
/// By default it is 24px square (64px for the message variants); the narrow arrows (`arrow`,
/// `arrow-bold`, `back-arrow`, `back-arrow-thin`) default to half width. The tint defaults to
/// [`icon_color`]. Without [`Icon::label`] the icon is decorative and has no accessibility node.
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
    /// Creates the icon `name`: the asset file name without the `icon-` prefix and `.svg` suffix,
    /// for example `"success"` or `"arrow"`.
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
    /// Sets the width and height to `size` logical pixels.
    pub fn size(mut self, size: f32) -> Self {
        self.w = Some(size);
        self.h = Some(size);
        self
    }
    /// Sets the width in logical pixels, keeping the default or previously set height.
    pub fn w(mut self, w: f32) -> Self {
        self.w = Some(w);
        self
    }
    /// Sets the height in logical pixels, keeping the default or previously set width.
    pub fn h(mut self, h: f32) -> Self {
        self.h = Some(h);
        self
    }
    /// Overrides the tint. Default: [`icon_color`] for the name and variant.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    /// Uses the [`IconVariant::Msg`] preset (64px, message colours).
    pub fn msg(mut self) -> Self {
        self.variant = IconVariant::Msg;
        self
    }
    /// Uses the [`IconVariant::MsgPrimary`] preset (64px, yellow `warn`).
    pub fn msg_primary(mut self) -> Self {
        self.variant = IconVariant::MsgPrimary;
        self
    }
    /// Exposes the icon to assistive technology as an image (`Role::Image`) named `label`. Default:
    /// none (decorative).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Switches the tint to `color` while an ancestor marked with GPUI's `group(group)` is pressed,
    /// mirroring WeUI's `:active` icon colours.
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

/// Returns WeUI's default tint for icon `name` in `variant`, taken from the active [`Theme`]: for
/// example brand for success icons, blue for waiting and info, red for warn and cancel, and `fg_2`
/// for arrows, circle and clear. Names it does not list get `fg_0`.
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
        let mirror = crate::i18n::direction(cx).is_rtl()
            && matches!(
                &*self.name,
                "arrow"
                    | "arrow-bold"
                    | "back-arrow"
                    | "back-arrow-thin"
                    | "back-circle"
                    | "back"
                    | "btn_goback"
            );
        let icon = svg()
            .path(SharedString::from(format!("icon-{}.svg", self.name)))
            .w(px(w))
            .h(px(h))
            .flex_shrink_0()
            .text_color(color);
        let icon = if mirror {
            icon.with_transformation(Transformation::scale(size(-1., 1.)))
        } else {
            icon
        };
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

/// An icon that acts as a button (`weui-icon-btn`), with a hit area extending 10px beyond the icon
/// on every side and at least 44px square.
///
/// It exposes `Role::Button` with an accessible label, is a Tab stop and activates on click, Enter
/// or Space. Pressing dims the icon to 50% (or recolours it in [`IconButton::btn_icon`] mode);
/// keyboard focus shows two contrasting inset bands.
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    label: SharedString,
    localized: Option<fn(&crate::i18n::Messages) -> &SharedString>,
    btn_icon: bool,
    disabled: bool,
    focus: Option<FocusHandle>,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    /// Creates a button with element id `id` showing the icon named `icon`. The accessible label
    /// defaults to the icon name; set a meaningful one with [`IconButton::label`].
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        let icon = icon.into();
        Self {
            id: id.into(),
            label: icon.clone(),
            localized: None,
            icon: Icon::new(icon),
            btn_icon: false,
            disabled: false,
            focus: None,
            on_click: None,
        }
    }
    /// Creates a 12×24px `back-arrow` button labelled with the localized
    /// [`crate::Messages::back`], as `weui-icon-btn_goback`.
    pub fn goback(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id, "back-arrow");
        this.localized = Some(|m| &m.back);
        this.icon = this.icon.w(12.).h(24.);
        this
    }
    /// Creates a 24px `close` button labelled with the localized [`crate::Messages::close`], as
    /// `weui-icon-btn_close`.
    pub fn close(id: impl Into<ElementId>) -> Self {
        let mut this = Self::new(id, "close");
        this.localized = Some(|m| &m.close);
        this.icon = this.icon.size(24.);
        this
    }
    /// Sets the accessible name of the button.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self.localized = None;
        self
    }
    /// Sets the icon's width and height in logical pixels. Default: 24px.
    pub fn size(mut self, size: f32) -> Self {
        self.icon = self.icon.size(size);
        self
    }
    /// Sets the icon width in logical pixels.
    pub fn w(mut self, w: f32) -> Self {
        self.icon = self.icon.w(w);
        self
    }
    /// Sets the icon height in logical pixels.
    pub fn h(mut self, h: f32) -> Self {
        self.icon = self.icon.h(h);
        self
    }
    /// Sets the icon tint. Default: `fg_0` (the `weui-icon-btn` colour), or the icon's own default
    /// colour in [`IconButton::btn_icon`] mode.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.icon = self.icon.color(color);
        self
    }
    /// Switches to `weui-btn_icon` behaviour: the icon keeps its default colour and turns `fg_1`
    /// while pressed instead of dimming.
    pub fn btn_icon(mut self) -> Self {
        self.btn_icon = true;
        self
    }
    /// Disables the button when `disabled` is true (default false): the icon dims to 30%, the
    /// button is not focusable, never calls `on_click`, and is exposed as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Uses `focus` as the button's focus handle so the owner can focus it programmatically.
    /// Ignored while disabled.
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }
    /// Sets the activation callback. It receives the click event on a pointer click, or on Enter or
    /// Space while focused, and is not attached while disabled.
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
        let pressed = t.fg_1;
        let group = SharedString::from("weui-icon-btn");
        let mut icon = self.icon;
        if self.btn_icon {
            if !self.disabled {
                icon = icon.group_active_color(group.clone(), pressed);
            }
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
            .aria_label(match self.localized {
                Some(pick) => pick(crate::i18n::messages(cx)).clone(),
                None => self.label,
            })
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
                .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()));
            if let Some(focus) = self.focus {
                hit = hit.track_focus(&focus);
            }
            if let Some(handler) = self.on_click {
                hit = hit.on_click(handler);
            }
        }
        let hit = crate::accessibility::Semantics::new(hit)
            .disabled(self.disabled)
            .reveal_focus();
        div()
            .group(group)
            .relative()
            .flex()
            .flex_shrink_0()
            .child(icon_el)
            .child(hit)
    }
}
