//! Keeps keyboard focus visible inside ordinary GPUI vertical scrollers.
//!
//! GPUI's `overflow_y_scroll` divs do not move when Tab lands on an element outside the visible
//! viewport, so a focus ring can sit offscreen. Wrap the scroller in [`FocusScroll`], sharing the
//! [`ScrollHandle`] passed to `track_scroll`, and wrap focusable leaves in [`FocusTarget`] (or call
//! [`RevealFocus::reveal_focus`]). When window focus moves to a new element inside a target, the
//! scroller adjusts its offset by the smallest amount that shows the target plus
//! [`FOCUS_MARGIN`], clamped to the scroll range, and redraws on the next frame.
//!
//! Behaviour contract:
//! - Only focus *changes* scroll. Manual wheel or drag scrolling while focus stays put is kept.
//! - A target that is already fully visible never scrolls.
//! - With nested targets the innermost one containing focus is revealed.
//! - A nested `FocusScroll` handles its own targets; outer scrollers do not see them.
//! - Only the vertical axis is adjusted. Targets taller than the viewport align their top edge.
//!
//! Detection uses GPUI's `AnyElement::prepaint` result, which reports the focused handle when
//! the focused element was prepainted inside the wrapped subtree. Elements that GPUI does not
//! prepaint (for example virtualised `list` rows outside the viewport) cannot be revealed this
//! way; `gpui::list` has its own autoscroll.

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, ScrollHandle, WeakFocusHandle, Window, point, px,
};
use std::{cell::RefCell, panic, rc::Rc};

/// Extra logical pixels kept between a revealed target and the viewport edge, so focus rings
/// drawn outside the target's bounds stay visible.
pub const FOCUS_MARGIN: f32 = 8.;

type Collector = Rc<RefCell<Option<Bounds<Pixels>>>>;

thread_local! {
    static COLLECTORS: RefCell<Vec<Collector>> = const { RefCell::new(Vec::new()) };
}

/// Returns the offset `y` (GPUI convention: `0` at the top, negative further down) that reveals
/// `target` inside `viewport` with `margin`, or `None` when no change is needed. `max` is the
/// positive maximum scroll distance. Both bounds are in window coordinates measured at the
/// current `offset`.
pub fn reveal_offset(
    viewport: Bounds<Pixels>,
    target: Bounds<Pixels>,
    offset: Pixels,
    max: Pixels,
    margin: Pixels,
) -> Option<Pixels> {
    let top = viewport.top() + margin;
    let bottom = viewport.bottom() - margin;
    let next = if target.top() < top {
        offset + (top - target.top())
    } else if target.bottom() > bottom {
        let down = offset - (target.bottom() - bottom);
        let align_top = offset + (top - target.top());
        if target.size.height > bottom - top {
            align_top
        } else {
            down
        }
    } else {
        return None;
    };
    let next = next.min(px(0.)).max(-max);
    (next != offset).then_some(next)
}

/// Wraps a scroller so focus moving onto a [`FocusTarget`] inside it scrolls that target into
/// view. Pass the same [`ScrollHandle`] that the scroller tracks. `id` keys the remembered
/// focus between frames and must be stable for the scroller.
pub struct FocusScroll {
    id: ElementId,
    handle: ScrollHandle,
    child: AnyElement,
    inset_bottom: Pixels,
}

impl FocusScroll {
    /// Treats the bottom `inset` logical pixels of the viewport as covered (for example by a
    /// gradient or toolbar overlapping the scroller), so revealed targets stop above it.
    /// Default `0`.
    pub fn inset_bottom(mut self, inset: Pixels) -> Self {
        self.inset_bottom = inset;
        self
    }
    /// Wraps `scroller`, which must call `track_scroll(&handle)`.
    pub fn new(
        id: impl Into<ElementId>,
        handle: &ScrollHandle,
        scroller: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            handle: handle.clone(),
            child: scroller.into_any_element(),
            inset_bottom: px(0.),
        }
    }
}

#[derive(Default)]
struct ScrollFocusState {
    last: Option<WeakFocusHandle>,
}

impl IntoElement for FocusScroll {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for FocusScroll {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }
    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let collector = Rc::new(RefCell::new(None));
        COLLECTORS.with(|c| c.borrow_mut().push(collector.clone()));
        self.child.prepaint(window, cx);
        COLLECTORS.with(|c| c.borrow_mut().pop());
        let Some(id) = id else { return };
        let focused = window.focused(cx);
        let target = *collector.borrow();
        let handle = self.handle.clone();
        let inset = self.inset_bottom;
        window.with_element_state(id, |state: Option<ScrollFocusState>, window| {
            let mut state = state.unwrap_or_default();
            let changed = match (&state.last, &focused) {
                (Some(last), Some(now)) => last != now,
                (None, None) => false,
                _ => true,
            };
            if changed && let Some(target) = target {
                let offset = handle.offset();
                let mut viewport = handle.bounds();
                viewport.size.height = (viewport.size.height - inset).max(px(0.));
                if let Some(y) = reveal_offset(
                    viewport,
                    target,
                    offset.y,
                    handle.max_offset().y,
                    px(FOCUS_MARGIN),
                ) {
                    handle.set_offset(point(offset.x, y));
                    window.on_next_frame(|window, _| window.refresh());
                }
            }
            state.last = focused.as_ref().map(|f| f.downgrade());
            ((), state)
        });
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

/// Marks an element whose bounds should be revealed when focus lands on it or inside it. Outside
/// a [`FocusScroll`] it is a transparent wrapper with no cost beyond one element.
pub struct FocusTarget {
    child: AnyElement,
}

impl FocusTarget {
    /// Wraps `child`.
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
        }
    }
}

impl IntoElement for FocusTarget {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for FocusTarget {
    type RequestLayoutState = LayoutId;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, LayoutId) {
        let id = self.child.request_layout(window, cx);
        (id, id)
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut LayoutId,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.child.prepaint(window, cx).is_some() {
            COLLECTORS.with(|c| {
                if let Some(top) = c.borrow().last() {
                    let mut slot = top.borrow_mut();
                    if slot.is_none() {
                        *slot = Some(bounds);
                    }
                }
            });
        }
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut LayoutId,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

/// Adds [`RevealFocus::reveal_focus`] to every element.
pub trait RevealFocus: IntoElement + Sized {
    /// Wraps the element in a [`FocusTarget`].
    fn reveal_focus(self) -> FocusTarget {
        FocusTarget::new(self)
    }
}

impl<E: IntoElement> RevealFocus for E {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Point, size};

    fn b(y: f32, h: f32) -> Bounds<Pixels> {
        Bounds::new(Point::new(px(0.), px(y)), size(px(100.), px(h)))
    }

    #[test]
    fn visible_target_does_not_scroll() {
        assert_eq!(
            reveal_offset(b(0., 600.), b(100., 40.), px(0.), px(900.), px(8.)),
            None
        );
    }

    #[test]
    fn below_scrolls_minimally_and_clamps() {
        assert_eq!(
            reveal_offset(b(0., 600.), b(700., 40.), px(0.), px(900.), px(8.)),
            Some(px(-148.))
        );
        assert_eq!(
            reveal_offset(b(0., 600.), b(1600., 40.), px(0.), px(900.), px(8.)),
            Some(px(-900.))
        );
    }

    #[test]
    fn above_scrolls_back_and_clamps_at_top() {
        assert_eq!(
            reveal_offset(b(0., 600.), b(-100., 40.), px(-300.), px(900.), px(8.)),
            Some(px(-192.))
        );
        assert_eq!(
            reveal_offset(b(0., 600.), b(2., 40.), px(-3.), px(900.), px(8.)),
            Some(px(0.))
        );
    }

    #[test]
    fn tall_target_aligns_top() {
        assert_eq!(
            reveal_offset(b(0., 600.), b(300., 800.), px(0.), px(900.), px(8.)),
            Some(px(-292.))
        );
    }
}
