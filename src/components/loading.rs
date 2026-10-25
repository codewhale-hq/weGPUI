//! Spinners from WeUI's `src/style/widget/weui-loading/` (`weui-loading`, `weui-primary-loading`
//! and `weui-mask-loading`), rendered by [`Loading`] from the bundled `loading.svg`.
//!
//! The ring turns once per second in 60 steps. While GPUI's `App::reduce_motion` is set it is drawn
//! still; [`crate::init`] reads that preference once at startup, on macOS only. Known limitations:
//! all three kinds draw the same ring, and live preference changes are not picked up.

use crate::Theme;
use gpui::{prelude::*, *};
use std::time::Duration;

/// The WeUI spinner class a [`Loading`] stands for.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadingKind {
    /// `weui-loading`: drawn at 90% opacity, as the upstream SVG is.
    #[default]
    Default,
    /// `weui-primary-loading`: upstream's CSS-drawn ring, here the same SVG at full opacity.
    Primary,
    /// `weui-mask-loading`: the ring in the given colour at full opacity; used inside loading
    /// buttons.
    Mask,
}

/// An animated loading spinner, 16px and `#606060` by default.
///
/// Without [`Loading::label`] it is decorative; with a label it is exposed as an image with that
/// name. Give spinners that share a parent distinct ids with [`Loading::id`].
#[derive(IntoElement)]
pub struct Loading {
    id: ElementId,
    kind: LoadingKind,
    size: f32,
    color: Option<Hsla>,
    brand: bool,
    transparent: bool,
    label: Option<SharedString>,
}

impl Default for Loading {
    fn default() -> Self {
        Self::new()
    }
}

impl Loading {
    /// Creates a [`LoadingKind::Default`] spinner with the id `"weui-loading"`.
    pub fn new() -> Self {
        Self {
            id: ElementId::Name("weui-loading".into()),
            kind: LoadingKind::Default,
            size: 16.,
            color: None,
            brand: false,
            transparent: false,
            label: None,
        }
    }
    /// Creates a [`LoadingKind::Primary`] spinner.
    pub fn primary() -> Self {
        Self::new().kind(LoadingKind::Primary)
    }
    /// Creates a [`LoadingKind::Mask`] spinner.
    pub fn mask() -> Self {
        Self::new().kind(LoadingKind::Mask)
    }
    /// Sets the spinner kind. Default: [`LoadingKind::Default`].
    pub fn kind(mut self, kind: LoadingKind) -> Self {
        self.kind = kind;
        self
    }
    /// Sets the element id that keys the animation state and the accessibility node. Default:
    /// `"weui-loading"`.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    /// Sets the width and height in logical pixels. Default: 16px (WeUI's `1em` at 16px).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    /// Sets the ring colour, taking precedence over [`Loading::brand`] and
    /// [`Loading::transparent`]. Default: `#606060`.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    /// Uses the theme brand colour (`weui-primary-loading_brand`) unless [`Loading::color`] is set.
    pub fn brand(mut self) -> Self {
        self.brand = true;
        self
    }
    /// Uses the fixed light grey `#ededed` (`weui-loading_transparent`) for dark or brand
    /// backgrounds, unless a colour or [`Loading::brand`] is set.
    pub fn transparent(mut self) -> Self {
        self.transparent = true;
        self
    }
    /// Exposes the spinner as an image (`Role::Image`) with this accessible name, for example
    /// "正在加载". Default: none (decorative).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for Loading {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or_else(|| {
            if self.brand {
                Theme::get(cx).brand
            } else if self.transparent {
                rgb(0xededed).into()
            } else {
                rgb(0x606060).into()
            }
        });
        let color = if self.kind == LoadingKind::Default {
            color.opacity(0.9)
        } else {
            color
        };
        let spinner = svg()
            .path("loading.svg")
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(color)
            .with_animation(
                self.id.clone(),
                Animation::new(Duration::from_secs(1))
                    .repeat()
                    .with_max_fps(60.)
                    .with_easing(|phase| (phase * 60.).floor() / 60.),
                |icon, delta| icon.with_transformation(Transformation::rotate(percentage(delta))),
            );
        match self.label {
            Some(label) => div()
                .id(self.id)
                .role(Role::Image)
                .aria_label(label)
                .flex_shrink_0()
                .child(spinner)
                .into_any_element(),
            None => spinner.into_any_element(),
        }
    }
}
