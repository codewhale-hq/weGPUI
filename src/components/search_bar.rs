//! WeUI search bar (`weui-searchbar/weui-searchbar.less`) in its filled, filled-grey, outlined and
//! homepage variants, plus a plain result list.
//!
//! [`SearchBar`] is a GPUI entity wrapping a [`TextInput`] and emitting [`SearchBarEvent`]s;
//! [`SearchResults`] renders result strings as cells. Known limitations: searching, filtering and
//! history are application-owned, and `words` is not drawn in the outlined variant.

use super::{Cell, Cells, Icon};
use crate::{
    Theme,
    input::{InputEvent, TextInput},
};
use gpui::{prelude::*, *};
use std::rc::Rc;

/// Visual variant of a [`SearchBar`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchBarVariant {
    /// `BG-2` field on the `BG-0` page background (`weui-search-bar`); the default.
    #[default]
    Filled,
    /// Grey field on a `BG-2` bar (`weui-search-bar_filled-grey`).
    FilledGrey,
    /// Outlined field with an attached 搜索 button (`weui-search-bar_outlined`); always in the
    /// focusing state.
    Outlined,
    /// Tappable placeholder plus a camera button (`weui-search-bar_homepage`). It has no text field
    /// and only emits `Open` and `Camera`.
    Homepage,
}

/// Events emitted by [`SearchBar`]; observe them with `cx.subscribe`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchBarEvent {
    /// The input text changed.
    Changed,
    /// Enter was pressed in the input, or the outlined variant's 搜索 button was activated.
    Submit,
    /// The bar left the focusing state, through 取消, [`SearchBar::cancel`] or a blur with empty
    /// text.
    Cancel,
    /// The clear button emptied the text; the input keeps focus.
    Clear,
    /// The back arrow shown while focusing was activated; the bar does not cancel by itself.
    Back,
    /// The homepage variant's camera button was activated.
    Camera,
    /// The homepage placeholder was activated, or [`SearchBar::activate`] was called on a homepage
    /// bar.
    Open,
    /// The input gained focus.
    Focus,
    /// The input lost focus, after any automatic `Cancel`.
    Blur,
}

/// Search bar entity: create it with `cx.new(|cx| SearchBar::new(cx).variant(...))`, render it and
/// subscribe to [`SearchBarEvent`].
///
/// Unfocused, it shows a centred placeholder label; activating it enters the focusing state with
/// back and 取消 buttons, plus a clear button while there is text. Blurring with empty text
/// cancels automatically. Its focus handle is the inner input's.
pub struct SearchBar {
    input: Entity<TextInput>,
    variant: SearchBarVariant,
    placeholder: SharedString,
    words: Option<SharedString>,
    focusing: bool,
    clear_on_cancel: bool,
    _subscription: Subscription,
}

impl SearchBar {
    /// Creates a filled search bar with placeholder "搜索" and clear-on-cancel on; the inner input
    /// has the `SearchInput` role.
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = TextInput::new("搜索", cx);
            input.set_accessible_role(Some(Role::SearchInput), cx);
            input
        });
        let subscription = cx.subscribe(&input, |this, input, event: &InputEvent, cx| {
            match event {
                InputEvent::Changed => cx.emit(SearchBarEvent::Changed),
                InputEvent::Submit => cx.emit(SearchBarEvent::Submit),
                InputEvent::Focus => {
                    this.focusing = true;
                    cx.emit(SearchBarEvent::Focus);
                }
                InputEvent::Blur => {
                    if input.read(cx).is_empty() {
                        this.finish_cancel(cx);
                    }
                    cx.emit(SearchBarEvent::Blur);
                }
            }
            cx.notify();
        });
        Self {
            input,
            variant: SearchBarVariant::Filled,
            placeholder: "搜索".into(),
            words: None,
            focusing: false,
            clear_on_cancel: true,
            _subscription: subscription,
        }
    }

    /// Sets the variant while building (default [`SearchBarVariant::Filled`]).
    pub fn variant(mut self, variant: SearchBarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Shows a brand-coloured keyword before the input (`weui-search-bar__words`), such as "微信".
    /// Default none; not drawn in the outlined variant.
    pub fn words(mut self, words: impl Into<SharedString>) -> Self {
        self.words = Some(words.into());
        self
    }

    /// Whether cancelling also clears the text (default `true`).
    pub fn clear_on_cancel(mut self, clear: bool) -> Self {
        self.clear_on_cancel = clear;
        self
    }

    /// Changes the variant and re-renders.
    pub fn set_variant(&mut self, variant: SearchBarVariant, cx: &mut Context<Self>) {
        self.variant = variant;
        cx.notify();
    }

    /// Sets or removes the keyword shown before the input, then re-renders.
    pub fn set_words(&mut self, words: Option<SharedString>, cx: &mut Context<Self>) {
        self.words = words;
        cx.notify();
    }

    /// Sets the placeholder used by the label, the homepage button and the input (default "搜索").
    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.placeholder = text.into();
        let placeholder = self.placeholder.clone();
        self.input
            .update(cx, |input, cx| input.set_placeholder(placeholder, cx));
        cx.notify();
    }

    /// The inner [`TextInput`] entity, for direct text, length or focus control.
    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }

    /// Current input text.
    pub fn text<'a>(&self, cx: &'a App) -> &'a str {
        self.input.read(cx).text()
    }

    /// Replaces the input text; the bar emits `Changed` if it differs.
    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.input.update(cx, |input, cx| input.set_text(text, cx));
        cx.notify();
    }

    /// Whether the bar is in the focusing state (always `true` for the outlined variant).
    pub fn is_focusing(&self) -> bool {
        self.focusing || self.variant == SearchBarVariant::Outlined
    }

    /// Whether results should show: focusing with non-empty text. Also exposed as the expanded
    /// state of the form's `ComboBox` node.
    pub fn is_expanded(&self, cx: &App) -> bool {
        self.is_focusing() && !self.input.read(cx).is_empty()
    }

    /// Enters the focusing state and focuses the input; for the homepage variant it only emits
    /// `Open`.
    pub fn activate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.variant == SearchBarVariant::Homepage {
            cx.emit(SearchBarEvent::Open);
            return;
        }
        self.focusing = true;
        self.input.update(cx, |input, cx| input.focus(window, cx));
        cx.notify();
    }

    /// Empties the text, focuses the input and emits `Clear`.
    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.focus(window, cx);
        });
        cx.emit(SearchBarEvent::Clear);
        cx.notify();
    }

    /// Leaves the focusing state, clearing the text if `clear_on_cancel`, emitting `Cancel` if it
    /// was focusing, and blurring the input if it has focus.
    pub fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.finish_cancel(cx);
        if self.input.read(cx).is_focused(window) {
            window.blur(cx);
        }
        cx.notify();
    }

    fn finish_cancel(&mut self, cx: &mut Context<Self>) {
        let was_focusing = self.focusing;
        if self.clear_on_cancel {
            self.input.update(cx, |input, cx| input.clear(cx));
        }
        self.focusing = false;
        if was_focusing {
            cx.emit(SearchBarEvent::Cancel);
        }
    }

    fn form_bg(&self, cx: &App) -> Hsla {
        let t = Theme::get(cx);
        match self.variant {
            SearchBarVariant::FilledGrey if t.is_dark => rgb(0x383838).into(),
            SearchBarVariant::FilledGrey => rgb(0xededed).into(),
            SearchBarVariant::Outlined => transparent_black(),
            _ => t.bg_2,
        }
    }

    fn render_homepage(&mut self, cx: &mut Context<Self>) -> Div {
        let t = Theme::get(cx).clone();
        div()
            .flex()
            .w_full()
            .py(px(12.))
            .px(px(20.))
            .bg(t.bg_0)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .rounded(px(8.))
                    .bg(t.bg_2)
                    .px(px(16.))
                    .child(
                        div()
                            .id("search-bar-open")
                            .role(Role::Button)
                            .aria_label(self.placeholder.clone())
                            .focusable()
                            .tab_stop(true)
                            .flex_1()
                            .min_w_0()
                            .py(px(12.))
                            .text_size(px(17.))
                            .line_height(px(24.))
                            .text_color(t.fg_2)
                            .rounded(px(4.))
                            .focus_visible(|s| s.bg(t.fg_3))
                            .child(self.placeholder.clone())
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(SearchBarEvent::Open))),
                    )
                    .child(
                        div()
                            .id("search-bar-camera")
                            .role(Role::Button)
                            .aria_label("拍照")
                            .focusable()
                            .tab_stop(true)
                            .relative()
                            .flex()
                            .items_center()
                            .rounded(px(4.))
                            .focus_visible(|s| s.bg(t.fg_3))
                            .active(|s| s.opacity(0.5))
                            .child(Icon::new("camera").w(18.8).h(20.).color(t.fg_0))
                            .child(hot_area(44., 44.))
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(SearchBarEvent::Camera))),
                    ),
            )
    }
}

impl EventEmitter<SearchBarEvent> for SearchBar {}

impl Focusable for SearchBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

fn hot_area(w: f32, h: f32) -> Div {
    div()
        .absolute()
        .top(relative(0.5))
        .left(relative(0.5))
        .w(px(w))
        .h(px(h))
        .mt(px(-h / 2.))
        .ml(px(-w / 2.))
}

impl Render for SearchBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            if self.variant == SearchBarVariant::Homepage {
                return crate::scrolling::FocusTarget::new(self.render_homepage(cx))
                    .into_any_element();
            }
            let t = Theme::get(cx).clone();
            let variant = self.variant;
            let outlined = variant == SearchBarVariant::Outlined;
            let focusing = self.is_focusing();
            let input = self.input.read(cx);
            let empty = input.is_empty();
            let input_focused = input.is_focused(window);
            let form_bg = self.form_bg(cx);
            let mask_bg = if outlined { t.bg_2 } else { form_bg };
            let action_size = if outlined { 15. } else { 17. };
            let pressed = t.fg_3;
            let brand = t.brand;

            let back = div()
                .id("search-bar-back")
                .role(Role::Button)
                .aria_label("返回")
                .focusable()
                .tab_stop(true)
                .relative()
                .flex()
                .items_center()
                .mr(px(12.))
                .rounded(px(4.))
                .focus_visible(move |s| s.bg(pressed))
                .active(|s| s.opacity(0.5))
                .child(Icon::new("back-arrow-thin").w(12.).h(24.).color(t.fg_0))
                .child(hot_area(44., 44.))
                .on_click(cx.listener(|_, _, _, cx| cx.emit(SearchBarEvent::Back)));

            let field = div()
                .relative()
                .flex_1()
                .min_w_0()
                .py(px(6.))
                .text_size(px(17.))
                .line_height(px(24.))
                .text_color(t.fg_0)
                .child(self.input.clone())
                .when(!input_focused && !empty, |el| {
                    el.child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom(px(2.))
                            .right_0()
                            .w(px(40.))
                            .rounded(px(6.))
                            .bg(linear_gradient(
                                90.,
                                linear_color_stop(mask_bg.opacity(0.), 0.),
                                linear_color_stop(mask_bg, 0.83),
                            )),
                    )
                });

            let clear = (!empty).then(|| {
                div()
                    .id("search-bar-clear")
                    .role(Role::Button)
                    .aria_label("清除")
                    .focusable()
                    .tab_stop(true)
                    .relative()
                    .flex()
                    .items_center()
                    .flex_shrink_0()
                    .ml(px(8.))
                    .mr(px(12.))
                    .rounded_full()
                    .focus_visible(move |s| s.bg(pressed))
                    .child(Icon::new("clear").size(20.).color(t.fg_2))
                    .child(hot_area(44., 36.))
                    .on_click(cx.listener(|this, _, window, cx| this.clear(window, cx)))
            });

            let box_ = div()
                .flex()
                .items_center()
                .flex_1()
                .min_w_0()
                .when(outlined, |el| el.pl(px(12.)))
                .when(!outlined, |el| {
                    el.child(
                        div()
                            .ml(px(8.))
                            .mr(px(4.))
                            .flex_shrink_0()
                            .child(Icon::new("search").size(24.).color(t.fg_2)),
                    )
                    .when_some(self.words.clone(), |el, words| {
                        el.child(
                            div()
                                .flex_shrink_0()
                                .mr(px(8.))
                                .text_size(px(17.))
                                .text_color(t.brand)
                                .child(words),
                        )
                    })
                })
                .child(field)
                .children(clear);

            let label = (!focusing).then(|| {
                div()
                    .id("search-bar-label")
                    .absolute()
                    .inset_0()
                    .rounded(px(6.))
                    .bg(form_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(17.))
                    .text_color(t.fg_2)
                    .cursor_text()
                    .role(Role::Button)
                    .aria_label(self.placeholder.clone())
                    .focusable()
                    .tab_stop(true)
                    .border_2()
                    .border_color(transparent_black())
                    .focus_visible(move |s| s.border_color(brand))
                    .child(
                        div()
                            .mr(px(4.))
                            .child(Icon::new("search").size(24.).color(t.fg_2)),
                    )
                    .child(self.placeholder.clone())
                    .on_click(cx.listener(|this, _, window, cx| this.activate(window, cx)))
            });

            let form = div()
                .id("search-bar-form")
                .role(Role::ComboBox)
                .aria_expanded(!empty && focusing)
                .relative()
                .flex()
                .items_center()
                .flex_1()
                .min_w_0()
                .bg(form_bg)
                .when(outlined, |el| {
                    el.border_l(px(0.5))
                        .border_t(px(0.5))
                        .border_b(px(0.5))
                        .border_color(t.fg_3)
                        .rounded_l(px(6.))
                })
                .when(!outlined, |el| el.rounded(px(6.)))
                .child(box_)
                .children(label);

            let search_btn = outlined.then(|| {
                let mask = t.btn_active_mask;
                div()
                    .id("search-bar-search")
                    .role(Role::Button)
                    .aria_label("搜索")
                    .focusable()
                    .tab_stop(true)
                    .flex()
                    .items_center()
                    .flex_shrink_0()
                    .py(px(8.))
                    .px(px(12.))
                    .text_size(px(15.))
                    .line_height(px(20.))
                    .text_color(t.white)
                    .bg(brand)
                    .rounded_r(px(6.))
                    .active(move |s| s.bg(brand.blend(mask)))
                    .focus_visible(move |s| s.bg(brand.blend(mask)))
                    .child("搜索")
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(SearchBarEvent::Submit)))
            });

            let cancel = focusing.then(|| {
                div()
                    .id("search-bar-cancel")
                    .role(Role::Button)
                    .aria_label("取消")
                    .focusable()
                    .tab_stop(true)
                    .flex()
                    .items_center()
                    .flex_shrink_0()
                    .ml(px(12.))
                    .text_size(px(action_size))
                    .text_color(t.link)
                    .rounded(px(4.))
                    .focus_visible(move |s| s.bg(pressed))
                    .active(|s| s.opacity(0.5))
                    .child("取消")
                    .on_click(cx.listener(|this, _, window, cx| this.cancel(window, cx)))
            });

            div()
                .id("search-bar")
                .role(Role::Search)
                .relative()
                .flex()
                .w_full()
                .py(px(10.))
                .pr(px(if outlined { 16. } else { 12. }))
                .pl(px(if focusing { 16. } else { 12. }))
                .bg(match variant {
                    SearchBarVariant::Filled => t.bg_0,
                    _ => t.bg_2,
                })
                .when(focusing, |el| el.child(back))
                .child(form)
                .children(search_btn)
                .children(cancel)
                .into_any_element()
        })
        .into_any_element()
    }
}

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Result list (`ListBox` role named "搜索结果") of focusable text cells; what a selection does
/// is up to the caller.
#[derive(IntoElement)]
pub struct SearchResults {
    id: ElementId,
    items: Vec<SharedString>,
    on_select: Option<SelectHandler>,
}

impl SearchResults {
    /// Creates an empty result list.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            on_select: None,
        }
    }

    /// Appends one result.
    pub fn item(mut self, text: impl Into<SharedString>) -> Self {
        self.items.push(text.into());
        self
    }

    /// Appends several results, in order.
    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    /// Called with the result's index when it is clicked or activated with Enter or Space.
    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for SearchResults {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let on_select = self.on_select;
        Cells::new()
            .id(self.id)
            .role(Role::ListBox)
            .aria_label("搜索结果")
            .margin_top(0.)
            .children(self.items.into_iter().enumerate().map(|(ix, text)| {
                let on_select = on_select.clone();
                Cell::new(("search-result", ix))
                    .role(Role::ListBoxOption)
                    .aria_label(text.clone())
                    .active()
                    .bd(div().py(px(2.)).pl(px(20.)).text_color(t.fg_1).child(text))
                    .on_click(move |_, window, cx| {
                        if let Some(handler) = on_select.as_ref() {
                            handler(ix, window, cx);
                        }
                    })
                    .into_any_element()
            }))
    }
}
