use gpui::*;
use std::collections::HashSet;

pub struct Semantics<E: Element> {
    inner: E,
    disabled: bool,
    busy: bool,
    modal: bool,
    hidden: bool,
}
impl<E: Element> Semantics<E> {
    pub fn new(inner: impl IntoElement<Element = E>) -> Self {
        Self {
            inner: inner.into_element(),
            disabled: false,
            busy: false,
            modal: false,
            hidden: false,
        }
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn busy(mut self, value: bool) -> Self {
        self.busy = value;
        self
    }
    pub fn modal(mut self) -> Self {
        self.modal = true;
        self
    }
    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = value;
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
        if self.busy {
            node.set_busy();
        }
        if self.modal {
            node.set_modal();
        }
        if self.hidden {
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
struct OpenModals(HashSet<WindowId>);

impl Global for OpenModals {}

pub(crate) fn register_open_modal(window: &Window, cx: &mut App) {
    cx.default_global::<OpenModals>()
        .0
        .insert(window.window_handle().window_id());
}

pub struct ModalScope {
    inner: Stateful<Div>,
    hidden: bool,
}

impl ModalScope {
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
        cx.default_global::<OpenModals>().0.remove(&window_id);
        let result = Element::request_layout(&mut self.inner, id, inspector, window, cx);
        self.hidden = cx.global::<OpenModals>().0.contains(&window_id);
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
