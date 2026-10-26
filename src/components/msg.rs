//! WeUI result page (`weui-page/weui-msg.less`): icon, text, operation, tips and extra areas, plus
//! [`LinkText`] for paragraphs with inline links.
//!
//! [`Msg`] lays its areas out in the order they are added. Inline links retain their text layout
//! and expose individual keyboard targets with bounds derived from the shaped text.

use super::Icon;
use crate::{Theme, logical::Logical};
use gpui::{prelude::*, *};
use std::rc::Rc;

type LinkHandler = Rc<dyn Fn(&mut Window, &mut App)>;

enum Segment {
    Text(SharedString),
    Link(SharedString, Option<LinkHandler>),
}

/// Paragraph mixing plain text and inline links, exposed as one `Paragraph` named by the full text.
///
/// Each actionable link gets its own keyboard focus stop and `Link` accessibility node with
/// bounds covering its wrapped text. Enter or Space activates it, and keyboard focus
/// underlines the link text with a contrasting 2px stroke. Clicking the link text runs the same handler.
#[derive(IntoElement)]
pub struct LinkText {
    id: ElementId,
    segments: Vec<Segment>,
}

impl LinkText {
    /// Creates an empty paragraph; segments are appended in order.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            segments: vec![],
        }
    }
    /// Appends plain text.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.segments.push(Segment::Text(text.into()));
        self
    }
    /// Appends link-coloured text whose `handler` runs on click or keyboard activation.
    pub fn link(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.segments
            .push(Segment::Link(label.into(), Some(Rc::new(handler))));
        self
    }
    /// Appends link-coloured text with no action and no focus stop.
    pub fn inert_link(mut self, label: impl Into<SharedString>) -> Self {
        self.segments.push(Segment::Link(label.into(), None));
        self
    }
}

impl RenderOnce for LinkText {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        crate::scrolling::FocusTarget::new({
            let t = Theme::get(cx).clone();
            let mut full = String::new();
            let mut links: Vec<(std::ops::Range<usize>, SharedString, Option<LinkHandler>)> =
                vec![];
            for segment in self.segments {
                match segment {
                    Segment::Text(text) => full.push_str(&text),
                    Segment::Link(label, handler) => {
                        let start = full.len();
                        full.push_str(&label);
                        links.push((start..full.len(), label, handler));
                    }
                }
            }
            let actionable = links.iter().filter(|(_, _, h)| h.is_some()).count();
            let handles =
                window.use_keyed_state(self.id.clone(), cx, |_, _| Vec::<FocusHandle>::new());
            let handles = handles.update(cx, |v, cx| {
                while v.len() < actionable {
                    v.push(cx.focus_handle().tab_stop(true));
                }
                v.clone()
            });
            let focus_ring = t.focus_color(t.bg_2);
            let mut actionable_index = 0;
            let mut highlights = vec![];
            let mut targets = vec![];
            for (range, label, handler) in &links {
                let mut style = HighlightStyle {
                    color: Some(t.link),
                    ..Default::default()
                };
                if let Some(handler) = handler {
                    let handle = handles[actionable_index].clone();
                    if handle.is_focused(window) && window.last_input_was_keyboard() {
                        style.underline = Some(UnderlineStyle {
                            thickness: px(2.),
                            color: Some(focus_ring),
                            wavy: false,
                        });
                    }
                    let click_handler = handler.clone();
                    targets.push((range.clone(), label.clone(), handle, click_handler));
                    actionable_index += 1;
                }
                highlights.push((range.clone(), style));
            }
            let full = SharedString::from(full);
            let ranges: Vec<_> = links.iter().map(|(range, _, _)| range.clone()).collect();
            let mut focus_index = 0;
            let handlers: Vec<_> = links
                .into_iter()
                .map(|(_, _, handler)| {
                    handler.map(|handler| {
                        let focus = handles[focus_index].clone();
                        focus_index += 1;
                        (handler, focus)
                    })
                })
                .collect();
            let styled = StyledText::new(full.clone()).with_highlights(highlights);
            let layout = styled.layout().clone();
            let text = InteractiveText::new("link-text", styled).on_click(
                ranges,
                move |ix, window, cx| {
                    if let Some(Some((handler, focus))) = handlers.get(ix) {
                        window.focus(focus, cx);
                        handler(window, cx);
                    }
                },
            );
            div()
                .id(self.id)
                .relative()
                .role(Role::Paragraph)
                .aria_label(full)
                .child(text)
                .child(
                    canvas(
                        move |_, window, cx| {
                            targets
                                .into_iter()
                                .enumerate()
                                .filter_map(|(index, (range, label, handle, handler))| {
                                    let bounds = link_bounds(&layout, range)?;
                                    let accessible_handler = handler.clone();
                                    let mut target = div()
                                        .id(("link-text-target", index))
                                        .role(Role::Link)
                                        .aria_label(label)
                                        .w(bounds.size.width)
                                        .h(bounds.size.height)
                                        .track_focus(&handle)
                                        .tab_stop(true)
                                        .on_mouse_down(MouseButton::Left, |_, window, _| {
                                            window.prevent_default()
                                        })
                                        .on_a11y_action(
                                            accesskit::Action::Click,
                                            move |_, window, cx| {
                                                accessible_handler(window, cx);
                                            },
                                        )
                                        .on_click(move |event, window, cx| {
                                            if matches!(event, ClickEvent::Keyboard(_)) {
                                                handler(window, cx);
                                            }
                                        })
                                        .into_any_element();
                                    target.prepaint_as_root(
                                        bounds.origin,
                                        bounds.size.map(AvailableSpace::Definite),
                                        window,
                                        cx,
                                    );
                                    Some(target)
                                })
                                .collect::<Vec<_>>()
                        },
                        |_, targets, window, cx| {
                            for mut target in targets {
                                target.paint(window, cx);
                            }
                        },
                    )
                    .absolute()
                    .size_full(),
                )
        })
    }
}

fn link_bounds(layout: &TextLayout, range: std::ops::Range<usize>) -> Option<Bounds<Pixels>> {
    let mut result: Option<Bounds<Pixels>> = None;
    let mut offset = 0;
    let mut origin = layout.bounds().origin;
    let height = layout.line_height();
    for line in layout.line_layouts() {
        let mut start = 0;
        let ends = (1..=line.wrap_boundaries.len())
            .map(|row| line.wrapped_line_start_index(row))
            .chain([line.len()]);
        for end in ends {
            let from = range.start.max(offset + start);
            let to = range.end.min(offset + end);
            if from < to {
                let left = line.unwrapped_layout.x_for_index(from - offset)
                    - line.unwrapped_layout.x_for_index(start);
                let right = line.unwrapped_layout.x_for_index(to - offset)
                    - line.unwrapped_layout.x_for_index(start);
                let bounds = Bounds::new(origin + point(left, px(0.)), size(right - left, height));
                result = Some(result.map_or(bounds, |previous| previous.union(&bounds)));
            }
            start = end;
            origin.y += height;
        }
        offset += line.len() + 1;
    }
    result
}

/// Standard 64 px icon for a [`Msg`] icon area, exposed as an `Image`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MsgIcon {
    /// Brand-coloured `success` icon, named by the localized [`crate::Messages::success`].
    Success,
    /// Red `warn` icon, named by the localized [`crate::Messages::warning`].
    Warn,
    /// Yellow `warn` icon, named by the localized [`crate::Messages::warning`].
    WarnPrimary,
    /// Blue `info-circle` icon, named by the localized [`crate::Messages::info`].
    Info,
    /// Blue `waiting` icon, named by the localized [`crate::Messages::waiting`].
    Waiting,
}

enum Area {
    Icon(AnyElement),
    Text(Vec<(TextKind, AnyElement)>),
    Opr(AnyElement),
    Tips(AnyElement),
    Extra(AnyElement),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextKind {
    Title,
    Desc,
    DescPrimary,
    Custom,
}

/// Result page column with centred text on the `BG-2` background, at least as tall as its parent.
///
/// Consecutive `title`, `desc`, `desc_primary` and `custom_area` calls share one text area; other
/// calls start a new area. A leading text area gets 96 px of top padding unless [`Msg::align_top`]
/// is set.
#[derive(IntoElement)]
pub struct Msg {
    id: ElementId,
    areas: Vec<Area>,
    align_top: bool,
}

impl Msg {
    /// Creates an empty result page.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            areas: vec![],
            align_top: false,
        }
    }
    /// Adds an icon area (`weui-msg__icon-area`) with a standard [`MsgIcon`].
    pub fn icon(self, icon: MsgIcon) -> Self {
        let (name, primary) = match icon {
            MsgIcon::Success => ("success", false),
            MsgIcon::Warn => ("warn", false),
            MsgIcon::WarnPrimary => ("warn", true),
            MsgIcon::Info => ("info-circle", false),
            MsgIcon::Waiting => ("waiting", false),
        };
        self.icon_element(MsgIconView { name, primary })
    }
    /// Adds an icon area with a custom element.
    pub fn icon_element(mut self, icon: impl IntoElement) -> Self {
        self.areas.push(Area::Icon(icon.into_any_element()));
        self
    }
    fn push_text(mut self, kind: TextKind, el: AnyElement) -> Self {
        if let Some(Area::Text(items)) = self.areas.last_mut() {
            items.push((kind, el));
        } else {
            self.areas.push(Area::Text(vec![(kind, el)]));
        }
        self
    }
    /// Adds a 22 px medium-weight title (`weui-msg__title`) to the current text area.
    pub fn title(self, title: impl Into<SharedString>) -> Self {
        let title: SharedString = title.into();
        self.push_text(TextKind::Title, title.into_any_element())
    }
    /// Adds a 17 px description (`weui-msg__desc`) to the current text area.
    pub fn desc(self, desc: impl IntoElement) -> Self {
        self.push_text(TextKind::Desc, desc.into_any_element())
    }
    /// Adds a 14 px secondary-colour description (`weui-msg__desc-primary`) to the current text
    /// area.
    pub fn desc_primary(self, desc: impl IntoElement) -> Self {
        self.push_text(TextKind::DescPrimary, desc.into_any_element())
    }
    /// Adds a left-aligned custom block (`weui-msg__custom-area`), such as cells or a preview, to
    /// the current text area; it is spaced 32 px after a title or 24 px after a description.
    pub fn custom_area(self, custom: impl IntoElement) -> Self {
        self.push_text(TextKind::Custom, custom.into_any_element())
    }
    /// Adds an operation area of centred buttons (`weui-msg__opr-area`).
    pub fn opr(mut self, opr: impl IntoElement) -> Self {
        self.areas.push(Area::Opr(opr.into_any_element()));
        self
    }
    /// Adds a 14 px tips area (`weui-msg__tips-area`).
    pub fn tips(mut self, tips: impl IntoElement) -> Self {
        self.areas.push(Area::Tips(tips.into_any_element()));
        self
    }
    /// Adds a 12 px extra area (`weui-msg__extra-area`), such as footer links and copyright.
    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.areas.push(Area::Extra(extra.into_any_element()));
        self
    }
    /// Removes the 96 px top padding of a leading text area (`weui-msg_align-top`). Default
    /// `false`.
    pub fn align_top(mut self, align_top: bool) -> Self {
        self.align_top = align_top;
        self
    }
}

#[derive(IntoElement)]
struct MsgIconView {
    name: &'static str,
    primary: bool,
}

impl RenderOnce for MsgIconView {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx);
        let color = match (self.name, self.primary) {
            ("success", _) => t.brand,
            ("warn", true) => t.yellow,
            ("warn", false) => t.red,
            _ => t.blue,
        };
        div()
            .id("msg-icon")
            .role(Role::Image)
            .aria_label({
                let m = crate::i18n::messages(cx);
                match (self.name, self.primary) {
                    ("success", _) => m.success.clone(),
                    ("warn", _) => m.warning.clone(),
                    ("waiting", _) => m.waiting.clone(),
                    _ => m.info.clone(),
                }
            })
            .flex()
            .justify_center()
            .child(Icon::new(self.name).size(64.).color(color))
    }
}

impl RenderOnce for Msg {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let dir = crate::i18n::direction(cx);
        let n = self.areas.len();
        let kinds: Vec<u8> = self
            .areas
            .iter()
            .map(|a| match a {
                Area::Icon(_) => 0,
                Area::Text(_) => 1,
                Area::Opr(_) => 2,
                Area::Tips(_) => 3,
                Area::Extra(_) => 4,
            })
            .collect();
        let align_top = self.align_top;
        let areas = self.areas.into_iter().enumerate().map(|(i, area)| {
            let last = i + 1 == n;
            let prev = i.checked_sub(1).map(|p| kinds[p]);
            match area {
                Area::Icon(icon) => div().mb(px(32.)).child(icon),
                Area::Text(items) => {
                    let mut prev_kind: Option<TextKind> = None;
                    let children: Vec<AnyElement> = items
                        .into_iter()
                        .map(|(kind, el)| {
                            let wrapper = match kind {
                                TextKind::Title => div()
                                    .mb(px(16.))
                                    .text_size(px(22.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(t.fg_0),
                                TextKind::Desc => {
                                    div().mb(px(16.)).text_size(px(17.)).text_color(t.fg_0)
                                }
                                TextKind::DescPrimary => {
                                    div().mb(px(16.)).text_size(px(14.)).text_color(t.fg_1)
                                }
                                TextKind::Custom => div()
                                    .flex()
                                    .flex_col()
                                    .mb(px(16.))
                                    .text_start(dir)
                                    .when(prev_kind == Some(TextKind::Title), |el| el.mt(px(32.)))
                                    .when(
                                        matches!(
                                            prev_kind,
                                            Some(TextKind::Desc | TextKind::DescPrimary)
                                        ),
                                        |el| el.mt(px(24.)),
                                    ),
                            };
                            prev_kind = Some(kind);
                            wrapper.child(el).into_any_element()
                        })
                        .collect();
                    div()
                        .flex_1()
                        .mb(px(32.))
                        .px(px(32.))
                        .line_height(relative(1.6))
                        .when(i == 0 && !align_top, |el| el.pt(px(96.)))
                        .flex()
                        .flex_col()
                        .children(children)
                }
                Area::Opr(opr) => div()
                    .mb(px(if last { 96. } else { 16. }))
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(opr),
                Area::Tips(tips) => div()
                    .mb(px(if last {
                        64.
                    } else if prev == Some(2) {
                        48.
                    } else {
                        16.
                    }))
                    .px(px(40.))
                    .text_size(px(14.))
                    .text_color(t.fg_1)
                    .child(tips),
                Area::Extra(extra) => div()
                    .when(prev == Some(2), |el| el.mt(px(48.)))
                    .mb(px(24.))
                    .px(px(32.))
                    .text_size(px(12.))
                    .text_color(t.fg_1)
                    .child(extra),
            }
        });
        div()
            .id(self.id)
            .min_h_full()
            .flex_grow(1.)
            .w_full()
            .flex()
            .flex_col()
            .pt(px(48.))
            .text_center()
            .line_height(relative(1.4))
            .bg(t.bg_2)
            .children(areas)
    }
}
