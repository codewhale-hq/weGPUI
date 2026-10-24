use crate::Theme;
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

pub const FADE: Duration = Duration::from_millis(100);

#[derive(Clone, Default)]
pub struct ToptipsState {
    visible: bool,
    changed: Option<Instant>,
    generation: usize,
}

impl ToptipsState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_open(&self) -> bool {
        self.visible
    }
    pub fn show(&mut self, window: &mut Window) {
        if !self.visible {
            self.visible = true;
            self.generation += 1;
            self.changed = Some(Instant::now());
        }
        window.refresh();
    }
    pub fn show_instantly(&mut self, window: &mut Window) {
        self.show(window);
        self.changed = None;
    }
    pub fn hide(&mut self, window: &mut Window) {
        if self.visible {
            self.visible = false;
            self.changed = Some(Instant::now());
            window.refresh();
        }
    }
    pub fn hide_after<T: 'static>(
        &self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<T>,
        get: fn(&mut T) -> &mut ToptipsState,
    ) {
        let generation = self.generation;
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update_in(cx, |this, window, cx| {
                let state = get(this);
                if state.visible && state.generation == generation {
                    state.hide(window);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
    fn opacity(&self, window: &Window, cx: &App) -> Option<f32> {
        let linear = match self.changed {
            Some(at) if !cx.reduce_motion() => {
                (at.elapsed().as_secs_f32() / FADE.as_secs_f32()).min(1.)
            }
            _ => 1.,
        };
        if linear < 1. {
            window.request_animation_frame();
        }
        if self.visible {
            Some(linear)
        } else if linear < 1. {
            Some(1. - linear)
        } else {
            None
        }
    }
}

#[derive(IntoElement)]
pub struct Toptips {
    id: ElementId,
    state: ToptipsState,
    text: SharedString,
    warn: bool,
}

impl Toptips {
    pub fn new(
        id: impl Into<ElementId>,
        state: &ToptipsState,
        text: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            text: text.into(),
            warn: false,
        }
    }
    pub fn warn(mut self) -> Self {
        self.warn = true;
        self
    }
}

impl RenderOnce for Toptips {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(opacity) = self.state.opacity(window, cx) else {
            return div().into_any_element();
        };
        let t = Theme::get(cx);
        let width = window.viewport_size().width;
        let tips = div()
            .id(self.id)
            .role(Role::Alert)
            .aria_label(self.text.clone())
            .w(width - px(16.))
            .p(px(10.))
            .rounded(px(8.))
            .text_size(px(14.))
            .line_height(relative(1.4))
            .text_center()
            .text_color(white())
            .when(self.warn, |el| el.bg(t.red))
            .opacity(opacity)
            .child(self.text);
        deferred(anchored().position(point(px(8.), px(8.))).child(tips))
            .with_priority(2)
            .into_any_element()
    }
}
