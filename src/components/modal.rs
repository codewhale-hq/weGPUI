use crate::accessibility::Semantics;
use gpui::{prelude::*, *};
use std::time::{Duration, Instant};

pub const TRANSITION: Duration = Duration::from_millis(300);

#[derive(Clone)]
pub struct Presence {
    visible: bool,
    changed: Option<Instant>,
    from: f32,
    span: Duration,
    opener: Option<FocusHandle>,
    focus: FocusHandle,
    generation: usize,
    duration: Duration,
}

impl Presence {
    pub fn new(cx: &mut App) -> Self {
        Self {
            visible: false,
            changed: None,
            from: 0.,
            span: TRANSITION,
            opener: None,
            focus: cx.focus_handle(),
            generation: 0,
            duration: TRANSITION,
        }
    }
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self.span = duration;
        self
    }
    pub fn is_open(&self) -> bool {
        self.visible
    }
    pub fn generation(&self) -> usize {
        self.generation
    }
    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }
    fn linear(&self, cx: &App) -> f32 {
        match self.changed {
            Some(at) if !cx.reduce_motion() && !self.span.is_zero() => {
                (at.elapsed().as_secs_f32() / self.span.as_secs_f32()).min(1.)
            }
            _ => 1.,
        }
    }
    fn value(&self, cx: &App) -> f32 {
        let to = if self.visible { 1. } else { 0. };
        self.from + (to - self.from) * ease(self.linear(cx))
    }
    fn retarget(&mut self, visible: bool, cx: &App) {
        let current = self.value(cx);
        let to = if visible { 1. } else { 0. };
        self.from = current;
        self.span = self.duration.mul_f32((to - current).abs());
        self.visible = visible;
        self.changed = Some(Instant::now());
    }
    pub fn show(&mut self, window: &mut Window, cx: &mut App) {
        if !self.visible {
            self.opener = window.focused(cx).filter(|f| f != &self.focus);
            self.retarget(true, cx);
            self.generation += 1;
        }
        window.focus(&self.focus, cx);
        window.refresh();
    }
    pub fn show_instantly(&mut self, window: &mut Window, cx: &mut App) {
        self.show(window, cx);
        self.changed = None;
        self.from = 1.;
    }
    pub fn hide(&mut self, window: &mut Window, cx: &mut App) {
        if !self.visible {
            return;
        }
        self.retarget(false, cx);
        if let Some(opener) = self.opener.take() {
            window.focus(&opener, cx);
        } else if self.focus.contains_focused(window, cx) {
            window.blur(cx);
        }
        window.refresh();
    }
    pub fn toggle(&mut self, window: &mut Window, cx: &mut App) {
        if self.visible {
            self.hide(window, cx)
        } else {
            self.show(window, cx)
        }
    }
    pub fn hide_after<T: 'static>(
        &self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<T>,
        get: fn(&mut T) -> &mut Presence,
    ) {
        let generation = self.generation;
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update_in(cx, |this, window, cx| {
                let presence = get(this);
                if presence.visible && presence.generation == generation {
                    presence.hide(window, cx);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
    pub fn progress(&self, window: &Window, cx: &App) -> Option<f32> {
        let linear = self.linear(cx);
        if linear < 1. {
            window.request_animation_frame();
        }
        if !self.visible && linear >= 1. {
            return None;
        }
        Some(self.value(cx))
    }
}

pub fn ease(t: f32) -> f32 {
    let (x1, y1, x2, y2) = (0.25f32, 0.1f32, 0.25f32, 1.0f32);
    let mut lo = 0f32;
    let mut hi = 1f32;
    let mut u = t;
    for _ in 0..24 {
        u = (lo + hi) / 2.;
        let x = 3. * (1. - u).powi(2) * u * x1 + 3. * (1. - u) * u * u * x2 + u.powi(3);
        if x < t { lo = u } else { hi = u }
    }
    3. * (1. - u).powi(2) * u * y1 + 3. * (1. - u) * u * u * y2 + u.powi(3)
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MaskKind {
    #[default]
    Dark,
    Transparent,
    None,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Placement {
    #[default]
    Center,
    Bottom,
    Top,
    Fill,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Motion {
    #[default]
    Fade,
    SlideUp,
    SlideDown,
    None,
}

type DismissHandler = Box<dyn Fn(&mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Modal {
    id: ElementId,
    presence: Presence,
    mask: MaskKind,
    placement: Placement,
    motion: Motion,
    mask_motion: bool,
    label: SharedString,
    dismiss_on_mask: bool,
    on_dismiss: Option<DismissHandler>,
    padding: Edges<Pixels>,
    children: Vec<AnyElement>,
}

impl Modal {
    pub fn new(id: impl Into<ElementId>, presence: &Presence) -> Self {
        Self {
            id: id.into(),
            presence: presence.clone(),
            mask: MaskKind::Dark,
            placement: Placement::Center,
            motion: Motion::Fade,
            mask_motion: true,
            label: SharedString::default(),
            dismiss_on_mask: true,
            on_dismiss: None,
            padding: Edges::default(),
            children: vec![],
        }
    }
    pub fn mask(mut self, mask: MaskKind) -> Self {
        self.mask = mask;
        self
    }
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
    pub fn motion(mut self, motion: Motion) -> Self {
        self.motion = motion;
        self
    }
    pub fn mask_motion(mut self, animate: bool) -> Self {
        self.mask_motion = animate;
        self
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn dismiss_on_mask(mut self, value: bool) -> Self {
        self.dismiss_on_mask = value;
        self
    }
    pub fn padding(mut self, padding: Edges<Pixels>) -> Self {
        self.padding = padding;
        self
    }
    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Modal {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(p) = self.presence.progress(window, cx) else {
            return div().into_any_element();
        };
        let viewport = window.viewport_size();
        let focus = self.presence.focus.clone();
        let interactive = self.presence.visible;
        if interactive {
            crate::accessibility::register_open_modal(window, cx);
        }
        let panel_height = window
            .use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "panel-height".into()),
                cx,
                |_, _| std::rc::Rc::new(std::cell::Cell::new(0f32)),
            )
            .read(cx)
            .clone();
        let dismiss = std::rc::Rc::new(self.on_dismiss);
        let dismiss_key = dismiss.clone();
        let mask_alpha = if self.mask_motion { p } else { 1. };
        let mask_color = match self.mask {
            MaskKind::Dark => hsla(0., 0., 0., 0.5 * mask_alpha),
            _ => transparent_black(),
        };
        let mut content = div()
            .id("modal-content")
            .relative()
            .flex()
            .flex_col()
            .occlude()
            .on_click(|_, _, cx| cx.stop_propagation())
            .child({
                let panel_height = panel_height.clone();
                canvas(
                    move |bounds, _, _| panel_height.set(bounds.size.height.as_f32()),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .children(self.children);
        let travel = match panel_height.get() {
            h if h > 0. => h,
            _ => viewport.height.as_f32(),
        };
        let shift = (1. - p) * travel;
        content = match self.motion {
            Motion::Fade => content.opacity(p),
            Motion::SlideUp => content.top(px(shift)),
            Motion::SlideDown => content.top(px(-shift)),
            Motion::None => content,
        };
        content = match self.placement {
            Placement::Bottom | Placement::Top | Placement::Fill => content.w_full(),
            Placement::Center => content,
        };
        if self.placement == Placement::Fill {
            content = content.h_full();
        }
        let dismiss_on_mask = self.dismiss_on_mask;
        let layer = div()
            .id(self.id)
            .key_context("WeuiModal")
            .track_focus(&focus)
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .flex_col()
            .pt(self.padding.top)
            .pb(self.padding.bottom)
            .pl(self.padding.left)
            .pr(self.padding.right)
            .map(|el| match self.placement {
                Placement::Center => el.items_center().justify_center(),
                Placement::Bottom => el.justify_end(),
                Placement::Top => el.justify_start(),
                Placement::Fill => el,
            })
            .when(self.mask != MaskKind::None || interactive, |el| {
                el.occlude()
            })
            .bg(mask_color)
            .capture_key_down({
                let focus = focus.clone();
                move |event, window, cx| {
                    if event.keystroke.key != "tab" {
                        return;
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                    let back = event.keystroke.modifiers.shift;
                    for _ in 0..256 {
                        if back {
                            window.focus_prev(cx)
                        } else {
                            window.focus_next(cx)
                        }
                        if focus.contains_focused(window, cx) && !focus.is_focused(window) {
                            return;
                        }
                    }
                    window.focus(&focus, cx);
                }
            })
            .on_key_down(move |event, window, cx| {
                if event.keystroke.key == "escape" && interactive {
                    if let Some(handler) = dismiss_key.as_ref() {
                        handler(window, cx);
                    }
                    cx.stop_propagation();
                }
            })
            .on_click(move |_, window, cx| {
                if dismiss_on_mask
                    && interactive
                    && let Some(handler) = dismiss.as_ref()
                {
                    handler(window, cx);
                }
            })
            .child(content);
        let semantics = Semantics::new(layer.role(Role::Dialog).aria_label(self.label)).modal();
        deferred(anchored().position(point(px(0.), px(0.))).child(semantics))
            .with_priority(1)
            .into_any_element()
    }
}
