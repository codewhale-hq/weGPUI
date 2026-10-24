use crate::Theme;
use gpui::{prelude::*, *};
use std::time::Duration;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadingKind {
    #[default]
    Default,
    Primary,
    Mask,
}

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
    pub fn primary() -> Self {
        Self::new().kind(LoadingKind::Primary)
    }
    pub fn mask() -> Self {
        Self::new().kind(LoadingKind::Mask)
    }
    pub fn kind(mut self, kind: LoadingKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn brand(mut self) -> Self {
        self.brand = true;
        self
    }
    pub fn transparent(mut self) -> Self {
        self.transparent = true;
        self
    }
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
