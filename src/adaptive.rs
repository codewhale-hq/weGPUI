//! Adaptive layout: one view tree that suits a desktop window and a phone screen.
//!
//! [`components`](crate::components) is phone-shaped and [`desktop`](crate::desktop) is
//! window-shaped. The types here choose between them from the space the window actually has,
//! so an application describes its navigation and panes once:
//!
//! - [`SizeClass`] reads the usable width. It is [`Compact`](SizeClass::Compact) on a phone or a
//!   narrow window and [`Regular`](SizeClass::Regular) otherwise.
//! - [`SafeArea`] keeps content clear of the status bar, home indicator and software keyboard.
//!   It does nothing where the platform reports no insets.
//! - [`AdaptiveShell`] shows top-level destinations as a [`DesktopRail`] beside the content when
//!   regular, and as a bottom [`Tabbar`] when compact.
//! - [`AdaptiveSplit`] shows a list and its detail side by side when regular. When compact it
//!   shows one at a time, with a [`NavigationBar`] and Back control above the detail.
//!
//! Everything is controlled: the caller stores the selected destination, whether a detail is
//! open and the sidebar width, and re-renders from callbacks. `examples/adaptive.rs` is a
//! complete application built this way; it runs unchanged on macOS and in the iOS simulator.
//!
//! ```ignore
//! SafeArea::new(
//!     AdaptiveShell::new("shell", content)
//!         .destination(Destination::new("Chats", DesktopIconKind::Chat).badge(3))
//!         .destination(Destination::new("Contacts", DesktopIconKind::Contacts))
//!         .selected(self.tab)
//!         .on_select(cx.listener(|this, tab: &usize, _, cx| { this.tab = *tab; cx.notify() })),
//! )
//! ```

use crate::{
    I18n, Theme,
    components::{
        ChangeHandler, ClickHandler,
        badge::Badge,
        form::dual_focus_ring,
        tabbar::{Tabbar, TabbarItem},
    },
    desktop::{
        DesktopBadge, DesktopButton, DesktopIcon, DesktopIconKind, DesktopPalette, DesktopRail,
        DesktopSplit,
    },
    logical::Logical,
    scrolling::RevealFocus,
};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Usable widths below this many logical pixels are [`SizeClass::Compact`].
pub const COMPACT_WIDTH: f32 = 760.;

/// How much horizontal room a window gives its content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeClass {
    /// A phone, or a window too narrow for side-by-side panes.
    Compact,
    /// A desktop window, or a tablet wide enough for a rail and two panes.
    Regular,
}

impl SizeClass {
    /// Classifies a usable width against [`COMPACT_WIDTH`].
    pub fn for_width(width: Pixels) -> Self {
        if width < px(COMPACT_WIDTH) {
            Self::Compact
        } else {
            Self::Regular
        }
    }

    /// Classifies `window` by the width left after system insets.
    pub fn of(window: &Window) -> Self {
        Self::for_width(window.fully_visible_bounds().size.width)
    }

    /// Whether this is [`SizeClass::Compact`].
    pub fn is_compact(self) -> bool {
        self == Self::Compact
    }
}

/// Root wrapper that places its content inside the window's fully visible bounds, clear of the
/// status bar, home indicator, display cutouts and the software keyboard.
///
/// The area outside those bounds is painted with [`background`](Self::background). On a
/// platform that reports no insets the content fills the window.
#[derive(IntoElement)]
pub struct SafeArea {
    content: AnyElement,
    background: Option<Hsla>,
}

impl SafeArea {
    /// Wraps `content`, which should be sized to fill its parent.
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
            background: None,
        }
    }

    /// Sets the colour behind the insets (default: the theme's `bg_0`).
    pub fn background(mut self, color: impl Into<Hsla>) -> Self {
        self.background = Some(color.into());
        self
    }
}

impl RenderOnce for SafeArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let visible = window.fully_visible_bounds();
        div()
            .relative()
            .size_full()
            .bg(self.background.unwrap_or(Theme::get(cx).bg_0))
            .child(
                div()
                    .absolute()
                    .left(visible.origin.x)
                    .top(visible.origin.y)
                    .w(visible.size.width)
                    .h(visible.size.height)
                    .overflow_hidden()
                    .child(self.content),
            )
    }
}

/// One top-level destination of an [`AdaptiveShell`].
pub struct Destination {
    label: SharedString,
    icon: DesktopIconKind,
    badge: usize,
}

impl Destination {
    /// Creates a destination. `label` is shown under the icon in the tab bar and is the
    /// accessible name of the rail button.
    pub fn new(label: impl Into<SharedString>, icon: DesktopIconKind) -> Self {
        Self {
            label: label.into(),
            icon,
            badge: 0,
        }
    }

    /// Shows an unread count on the icon; zero shows nothing.
    pub fn badge(mut self, count: usize) -> Self {
        self.badge = count;
        self
    }
}

/// Top-level navigation around one content area: a [`DesktopRail`] at the inline start when
/// [`Regular`](SizeClass::Regular), a bottom [`Tabbar`] when [`Compact`](SizeClass::Compact).
///
/// The shell is stateless. Pass the selected index and render that destination's content;
/// [`on_select`](Self::on_select) reports the index the user chose.
#[derive(IntoElement)]
pub struct AdaptiveShell {
    id: ElementId,
    label: SharedString,
    destinations: Vec<Destination>,
    selected: usize,
    on_select: Option<Rc<ChangeHandler<usize>>>,
    content: AnyElement,
    rail_header: Option<AnyElement>,
    rail_footer: Vec<AnyElement>,
    tab_bar_hidden: bool,
    size_class: Option<SizeClass>,
}

impl AdaptiveShell {
    /// Creates a shell around `content`, the selected destination's view.
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            destinations: vec![],
            selected: 0,
            on_select: None,
            content: content.into_any_element(),
            rail_header: None,
            rail_footer: vec![],
            tab_bar_hidden: false,
            size_class: None,
        }
    }

    /// Sets the tab bar's accessible name.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    /// Appends a destination.
    pub fn destination(mut self, destination: Destination) -> Self {
        self.destinations.push(destination);
        self
    }

    /// Sets the selected destination index (default 0).
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// Called with the index of the destination the user chose.
    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(Box::new(handler)));
        self
    }

    /// Element above the rail's destinations, such as the account avatar. The tab bar has no
    /// room for it, so it is not shown when compact.
    pub fn rail_header(mut self, header: impl IntoElement) -> Self {
        self.rail_header = Some(header.into_any_element());
        self
    }

    /// Element at the bottom of the rail. Not shown when compact; give its action another
    /// home there, such as a row in a settings destination.
    pub fn rail_footer(mut self, footer: impl IntoElement) -> Self {
        self.rail_footer.push(footer.into_any_element());
        self
    }

    /// Hides the compact tab bar, as phones do while a pushed page such as an open conversation
    /// fills the screen. The rail is unaffected.
    pub fn tab_bar_hidden(mut self, hidden: bool) -> Self {
        self.tab_bar_hidden = hidden;
        self
    }

    /// Overrides the size class read from the window, for a shell that does not span it.
    pub fn size_class(mut self, size_class: SizeClass) -> Self {
        self.size_class = Some(size_class);
        self
    }
}

impl RenderOnce for AdaptiveShell {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let size_class = self.size_class.unwrap_or_else(|| SizeClass::of(window));
        let selected = self.selected;
        let root = div().id(self.id).size_full().flex();
        if size_class.is_compact() {
            let t = Theme::get(cx);
            let (brand, plain) = (t.brand, t.fg_0);
            let mut tabs = Tabbar::new("destinations")
                .label(self.label)
                .selected(selected);
            for (i, destination) in self.destinations.into_iter().enumerate() {
                let color = if i == selected { brand } else { plain };
                let mut item = TabbarItem::new(
                    destination.label,
                    DesktopIcon::new(destination.icon).size(28.).color(color),
                );
                if destination.badge > 0 {
                    item = item
                        .badge(Badge::count(destination.badge), -2., -8.)
                        .description(destination.badge.to_string());
                }
                tabs = tabs.item(item);
            }
            if let Some(handler) = self.on_select {
                tabs = tabs.on_select(move |index, window, cx| handler(&index, window, cx));
            }
            return root
                .flex_col()
                .child(div().flex_1().min_h_0().child(self.content))
                .when(!self.tab_bar_hidden, |el| el.child(tabs));
        }
        let mut rail = DesktopRail::new().children(self.rail_header);
        for (i, destination) in self.destinations.into_iter().enumerate() {
            let handler = self.on_select.clone();
            let button = DesktopButton::new(("destination", i), destination.label)
                .icon(destination.icon)
                .icon_only()
                .selected(i == selected)
                .on_click(move |_, window, cx| {
                    if let Some(handler) = handler.as_ref() {
                        handler(&i, window, cx);
                    }
                });
            rail = rail.child(
                div()
                    .relative()
                    .child(button)
                    .when(destination.badge > 0, |el| {
                        el.child(div().absolute().top(px(-4.)).right(px(-6.)).child(
                            DesktopBadge::new(destination.badge).id(("destination-badge", i)),
                        ))
                    }),
            );
        }
        for footer in self.rail_footer {
            rail = rail.footer(footer);
        }
        root.row(I18n::get(cx).direction())
            .child(rail)
            .child(div().flex_1().min_w_0().h_full().child(self.content))
    }
}

/// A 44 px title bar for a pushed page: optional Back control at the inline start, centred
/// title, optional actions at the inline end.
#[derive(IntoElement)]
pub struct NavigationBar {
    title: SharedString,
    on_back: Option<ClickHandler>,
    actions: Vec<AnyElement>,
}

impl NavigationBar {
    /// Creates a bar titled `title`.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            on_back: None,
            actions: vec![],
        }
    }

    /// Shows a Back control, named by the active catalog's `back` message.
    pub fn on_back(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_back = Some(Box::new(handler));
        self
    }
}

impl ParentElement for NavigationBar {
    /// Appends actions at the inline end.
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements);
    }
}

impl RenderOnce for NavigationBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = DesktopPalette::get(cx);
        let i18n = I18n::get(cx);
        let direction = i18n.direction();
        let back_label = i18n.messages().back.clone();
        let chevron = if direction.is_rtl() {
            DesktopIconKind::ChevronRight
        } else {
            DesktopIconKind::Back
        };
        let back = self.on_back.map(|handler| {
            div()
                .id("back")
                .role(Role::Button)
                .aria_label(back_label)
                .size(px(44.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(5.))
                .cursor_pointer()
                .focusable()
                .tab_stop(true)
                .focus_visible(|s| s.shadow(dual_focus_ring()))
                .hover(move |s| s.bg(p.hover))
                .active(|s| s.opacity(0.65))
                .child(DesktopIcon::new(chevron).size(22.).color(p.text))
                .on_click(handler)
                .reveal_focus()
        });
        div()
            .id("navigation-bar")
            .h(px(44.))
            .w_full()
            .flex_shrink_0()
            .flex()
            .row(direction)
            .items_center()
            .bg(p.canvas)
            .border_b_1()
            .border_color(p.border)
            .text_color(p.text)
            .child(div().min_w(px(44.)).flex_1().flex().children(back))
            .child(
                div()
                    .id("title")
                    .role(Role::Heading)
                    .max_w(relative(0.6))
                    .truncate()
                    .text_size(px(17.))
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.title),
            )
            .child(
                div()
                    .min_w(px(44.))
                    .flex_1()
                    .flex()
                    .row(direction)
                    .justify_end()
                    .items_center()
                    .children(self.actions),
            )
    }
}

/// A list and its detail: side by side through a resizable [`DesktopSplit`] when
/// [`Regular`](SizeClass::Regular), one at a time when [`Compact`](SizeClass::Compact).
///
/// When compact, a set [`detail`](Self::detail) replaces the list and gains a
/// [`NavigationBar`] with [`title`](Self::title) and a Back control that calls
/// [`on_back`](Self::on_back); clear the detail there to return to the list. When regular the
/// bar is not drawn, and [`placeholder`](Self::placeholder) fills the pane while no detail is
/// open.
#[derive(IntoElement)]
pub struct AdaptiveSplit {
    id: ElementId,
    sidebar: AnyElement,
    detail: Option<AnyElement>,
    placeholder: Option<AnyElement>,
    title: SharedString,
    on_back: Option<ClickHandler>,
    width: f32,
    limits: (f32, f32),
    on_resize: Option<ChangeHandler<f32>>,
    size_class: Option<SizeClass>,
}

impl AdaptiveSplit {
    /// Creates a split whose list pane is `sidebar`.
    pub fn new(id: impl Into<ElementId>, sidebar: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            sidebar: sidebar.into_any_element(),
            detail: None,
            placeholder: None,
            title: SharedString::default(),
            on_back: None,
            width: 280.,
            limits: (220., 420.),
            on_resize: None,
            size_class: None,
        }
    }

    /// Sets the open detail. Pass `None` while only the list should show on a phone.
    pub fn detail(mut self, detail: Option<impl IntoElement>) -> Self {
        self.detail = detail.map(IntoElement::into_any_element);
        self
    }

    /// Content for the regular detail pane while no detail is open.
    pub fn placeholder(mut self, placeholder: impl IntoElement) -> Self {
        self.placeholder = Some(placeholder.into_any_element());
        self
    }

    /// Title of the compact [`NavigationBar`] above the detail.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Called by the compact Back control. Without it the detail has no bar.
    pub fn on_back(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_back = Some(Box::new(handler));
        self
    }

    /// Sets the regular list pane's width (default 280).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the regular list pane's width limits (default 220 to 420).
    pub fn limits(mut self, min: f32, max: f32) -> Self {
        self.limits = (min, max);
        self
    }

    /// Called with the proposed list pane width while the divider is dragged or keyed.
    pub fn on_resize(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_resize = Some(Box::new(handler));
        self
    }

    /// Overrides the size class read from the window, for a split that does not span it.
    pub fn size_class(mut self, size_class: SizeClass) -> Self {
        self.size_class = Some(size_class);
        self
    }
}

impl RenderOnce for AdaptiveSplit {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let size_class = self.size_class.unwrap_or_else(|| SizeClass::of(window));
        if !size_class.is_compact() {
            let detail = self
                .detail
                .or(self.placeholder)
                .unwrap_or_else(|| div().into_any_element());
            let mut split = DesktopSplit::new(self.id, self.width, self.sidebar, detail)
                .limits(self.limits.0, self.limits.1);
            if let Some(handler) = self.on_resize {
                split = split.on_resize(handler);
            }
            return split.into_any_element();
        }
        let Some(detail) = self.detail else {
            return div()
                .id(self.id)
                .size_full()
                .child(self.sidebar)
                .into_any_element();
        };
        div()
            .id(self.id)
            .size_full()
            .flex()
            .flex_col()
            .children(
                self.on_back
                    .map(|handler| NavigationBar::new(self.title).on_back(handler)),
            )
            .child(div().flex_1().min_h_0().child(detail))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn size_class_splits_at_the_compact_width() {
        assert_eq!(SizeClass::for_width(px(375.)), SizeClass::Compact);
        assert_eq!(
            SizeClass::for_width(px(COMPACT_WIDTH - 1.)),
            SizeClass::Compact
        );
        assert_eq!(SizeClass::for_width(px(COMPACT_WIDTH)), SizeClass::Regular);
        assert!(!SizeClass::for_width(px(1120.)).is_compact());
    }
}
