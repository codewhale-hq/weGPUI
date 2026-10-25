//! Accessibility helpers layered on GPUI's AccessKit integration.
//!
//! [`Semantics`] adds AccessKit states that GPUI's element builders do not expose (disabled,
//! busy, modal, hidden) to an existing element. [`ModalScope`] hides application content from
//! assistive technology while a [`crate::components::Modal`] is open.
//!
//! Known limitations: these nodes only reach platforms whose GPUI backend wires an AccessKit
//! adapter (macOS, Windows and Linux at the pinned GPUI). Mobile backends have none. Screen-reader
//! acceptance remains incomplete. Sampled Orca runs and remaining bridge limitations are
//! recorded in `docs/receipts/orca-20261025`; VoiceOver remains off.

use gpui::*;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

thread_local! {
    static MODAL_LAYOUT_DEPTH: RefCell<HashMap<WindowId, usize>> = RefCell::default();
}

struct ModalLayoutDepth {
    window: WindowId,
    previous: usize,
}

impl ModalLayoutDepth {
    fn enter(window: WindowId) -> Self {
        let previous = MODAL_LAYOUT_DEPTH.with(|depths| {
            let mut depths = depths.borrow_mut();
            let depth = depths.entry(window).or_default();
            let previous = *depth;
            *depth += 1;
            previous
        });
        Self { window, previous }
    }
}

impl Drop for ModalLayoutDepth {
    fn drop(&mut self) {
        MODAL_LAYOUT_DEPTH.with(|depths| {
            let mut depths = depths.borrow_mut();
            if self.previous == 0 {
                depths.remove(&self.window);
            } else {
                depths.insert(self.window, self.previous);
            }
        });
    }
}

/// Wraps an element and adds AccessKit states to its node.
///
/// The wrapped element must have an id and a role for a node to exist; otherwise the flags have
/// no effect. Layout, painting and events are delegated unchanged.
pub struct Semantics<E: Element> {
    inner: E,
    disabled: bool,
    read_only: bool,
    live: Option<accesskit::Live>,
    busy: bool,
    modal: bool,
    hidden: bool,
    hidden_flag: Option<Rc<Cell<bool>>>,
    modal_layer: bool,
}
impl<E: Element> Semantics<E> {
    /// Wraps `inner` with no extra states.
    pub fn new(inner: impl IntoElement<Element = E>) -> Self {
        Self {
            inner: inner.into_element(),
            disabled: false,
            read_only: false,
            live: None,
            busy: false,
            modal: false,
            hidden: false,
            hidden_flag: None,
            modal_layer: false,
        }
    }
    /// Marks the node disabled (unavailable to activate) when `value` is true.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Marks the node as read-only when `value` is true, while leaving it available for focus
    /// and inspection.
    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = value;
        self
    }
    /// Sets the live-region politeness explicitly. A role such as `Alert` alone does not set
    /// this property in the pinned AccessKit adapter.
    pub fn live(mut self, value: accesskit::Live) -> Self {
        self.live = Some(value);
        self
    }
    /// Marks the node busy, for example a loading button, when `value` is true.
    pub fn busy(mut self, value: bool) -> Self {
        self.busy = value;
        self
    }
    /// Marks the node as a modal dialog.
    pub fn modal(mut self) -> Self {
        self.modal = true;
        self
    }
    /// Hides the node and its subtree from assistive technology when `value` is true, for
    /// example actions behind a closed swipe cell.
    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = value;
        self
    }
    pub(crate) fn hidden_flag(mut self, flag: Rc<Cell<bool>>) -> Self {
        self.hidden_flag = Some(flag);
        self
    }
    pub(crate) fn modal_layer(mut self) -> Self {
        self.modal_layer = true;
        self
    }
}
impl<E: Element> IntoElement for Semantics<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl<E: Element> Element for Semantics<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }
    fn a11y_role(&self) -> Option<Role> {
        self.inner.a11y_role()
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        if self.disabled {
            node.set_disabled();
        }
        if self.read_only {
            node.set_read_only();
        }
        if let Some(live) = self.live {
            node.set_live(live);
        }
        if self.busy {
            node.set_busy();
        }
        if self.modal {
            node.set_modal();
        }
        if self.hidden || self.hidden_flag.as_ref().is_some_and(|flag| flag.get()) {
            node.set_hidden();
        }
    }
    fn a11y_synthetic_children(
        &mut self,
        prepaint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner.a11y_synthetic_children(prepaint, builder);
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let _depth = self
            .modal_layer
            .then(|| ModalLayoutDepth::enter(window.window_handle().window_id()));
        self.inner.request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.inner
            .prepaint(id, inspector, bounds, state, window, cx)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner
            .paint(id, inspector, bounds, state, prepaint, window, cx);
    }
}

#[derive(Default)]
struct ModalLayers {
    top: Option<(usize, Rc<Cell<bool>>)>,
}

impl ModalLayers {
    fn register(&mut self, depth: usize) -> Rc<Cell<bool>> {
        if self.top.as_ref().is_some_and(|(top, _)| *top > depth) {
            return Rc::new(Cell::new(true));
        }
        let hidden = Rc::new(Cell::new(false));
        if let Some((_, previous)) = self.top.replace((depth, hidden.clone())) {
            previous.set(true);
        }
        hidden
    }
}

#[derive(Default)]
struct OpenModals(HashMap<WindowId, ModalLayers>);

impl Global for OpenModals {}

struct ModalBackgrounds {
    flags: HashMap<WindowId, Rc<Cell<bool>>>,
    _cleanup: Subscription,
}

impl Global for ModalBackgrounds {}

pub(crate) fn background_hidden_flag(window: &Window, cx: &mut App) -> Rc<Cell<bool>> {
    if !cx.has_global::<ModalBackgrounds>() {
        let cleanup = cx.on_window_closed(|cx, window| {
            if cx.has_global::<ModalBackgrounds>() {
                cx.global_mut::<ModalBackgrounds>().flags.remove(&window);
            }
            if cx.has_global::<OpenModals>() {
                cx.global_mut::<OpenModals>().0.remove(&window);
            }
        });
        cx.set_global(ModalBackgrounds {
            flags: HashMap::new(),
            _cleanup: cleanup,
        });
    }
    cx.global_mut::<ModalBackgrounds>()
        .flags
        .entry(window.window_handle().window_id())
        .or_insert_with(|| Rc::new(Cell::new(false)))
        .clone()
}

pub(crate) fn register_open_modal(window: &Window, cx: &mut App) -> Rc<Cell<bool>> {
    background_hidden_flag(window, cx).set(true);
    let window_id = window.window_handle().window_id();
    let depth = MODAL_LAYOUT_DEPTH
        .with(|depths| depths.borrow().get(&window_id).copied().unwrap_or_default());
    cx.default_global::<OpenModals>()
        .0
        .entry(window_id)
        .or_default()
        .register(depth)
}

/// Hides its content from assistive technology while a [`crate::components::Modal`] is open.
///
/// Wrap the window's root element in it. Modals register themselves when they render as open.
/// The scope checks after its children are laid out, so every `Modal` must be rendered inside
/// the scope. Modals are deferred overlays and attach to the window root of the accessibility
/// tree, so they stay exposed while the scope's group node is marked hidden. The scope fills its
/// parent (`size_full`) and adds an unnamed `Group` node to the tree.
/// Only the deepest active modal is exposed; at equal modal nesting depth the last in layout
/// order wins, matching GPUI's deferred draw rounds. Other modal layers remain painted but are
/// hidden from assistive technology. Application-supplied deferred wrappers or other overlay
/// priorities are outside this ordering contract.
pub struct ModalScope {
    inner: Stateful<Div>,
    hidden: bool,
}

impl ModalScope {
    /// Wraps `content`, typically the root view's element tree.
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            inner: div()
                .id("weui-modal-scope")
                .role(Role::Group)
                .size_full()
                .child(content),
            hidden: false,
        }
    }
}

impl IntoElement for ModalScope {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for ModalScope {
    type RequestLayoutState = <Stateful<Div> as Element>::RequestLayoutState;
    type PrepaintState = <Stateful<Div> as Element>::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        Element::id(&self.inner)
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        Element::source_location(&self.inner)
    }
    fn a11y_role(&self) -> Option<Role> {
        Element::a11y_role(&self.inner)
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        Element::write_a11y_info(&self.inner, node);
        if self.hidden {
            node.set_hidden();
        }
    }
    fn a11y_synthetic_children(
        &mut self,
        prepaint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        Element::a11y_synthetic_children(&mut self.inner, prepaint, builder);
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let window_id = window.window_handle().window_id();
        background_hidden_flag(window, cx).set(false);
        cx.default_global::<OpenModals>().0.remove(&window_id);
        let result = Element::request_layout(&mut self.inner, id, inspector, window, cx);
        self.hidden = cx.global::<OpenModals>().0.contains_key(&window_id);
        result
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        Element::prepaint(&mut self.inner, id, inspector, bounds, state, window, cx)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        Element::paint(
            &mut self.inner,
            id,
            inspector,
            bounds,
            state,
            prepaint,
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn read_only_and_live_are_forwarded_without_disabling_the_node() {
        let semantics = Semantics::new(div().id("status").role(Role::Slider))
            .read_only(true)
            .live(accesskit::Live::Assertive);
        let mut node = accesskit::Node::new(Role::Slider);
        semantics.write_a11y_info(&mut node);
        assert!(node.is_read_only());
        assert!(!node.is_disabled());
        assert_eq!(node.live(), Some(accesskit::Live::Assertive));
    }

    #[test]
    fn later_modal_hides_already_constructed_layers_at_a11y_write() {
        let mut layers = ModalLayers::default();
        let outer = Semantics::new(div().id("outer").role(Role::Dialog))
            .modal()
            .hidden_flag(layers.register(0));
        let middle = Semantics::new(div().id("middle").role(Role::Dialog))
            .modal()
            .hidden_flag(layers.register(0));
        let inner = Semantics::new(div().id("inner").role(Role::Dialog))
            .modal()
            .hidden_flag(layers.register(0));
        for (semantics, hidden) in [(outer, true), (middle, true), (inner, false)] {
            let mut node = accesskit::Node::new(Role::Dialog);
            semantics.write_a11y_info(&mut node);
            assert_eq!(node.is_hidden(), hidden);
            assert!(node.is_modal());
        }
    }

    #[test]
    fn nested_modal_stays_exposed_after_a_later_root_sibling_registers() {
        let mut layers = ModalLayers::default();
        let outer = layers.register(0);
        let inner = layers.register(1);
        let sibling = layers.register(0);
        assert!(outer.get());
        assert!(!inner.get());
        assert!(sibling.get());
        let later_inner = layers.register(1);
        assert!(inner.get());
        assert!(!later_inner.get());
    }
}
