use super::dialog::ActionHandler;
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;
#[derive(Clone, IntoElement)]
pub struct ActionSheet {
    title: SharedString,
    pub(crate) items: Vec<SharedString>,
    pub(crate) on_action: Option<ActionHandler>,
    pub(crate) focuses: Vec<FocusHandle>,
}
impl ActionSheet {
    pub fn new(
        title: impl Into<SharedString>,
        items: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        Self {
            title: title.into(),
            items: items.into_iter().map(Into::into).collect(),
            on_action: None,
            focuses: vec![],
        }
    }
    pub fn on_action(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for ActionSheet {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let n = self.items.len();
        div()
            .id("action-sheet")
            .role(Role::Dialog)
            .aria_label(self.title.clone())
            .w_full()
            .bg(t.bg_0)
            .rounded_t(px(12.))
            .overflow_hidden()
            .child(
                div()
                    .h(px(56.))
                    .px(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_center()
                    .text_size(px(12.))
                    .line_height(px(16.8))
                    .text_color(t.fg_1)
                    .bg(t.bg_2)
                    .child(self.title),
            )
            .children(
                self.items
                    .into_iter()
                    .chain(std::iter::once("取消".into()))
                    .enumerate()
                    .map(|(i, label)| {
                        let cb = self.on_action.clone();
                        div()
                            .id(("sheet-action", i))
                            .role(Role::Button)
                            .aria_label(label.clone())
                            .focusable()
                            .tab_stop(true)
                            .when_some(self.focuses.get(i), |el, focus| el.track_focus(focus))
                            .when(i == n, |el| el.mt(px(8.)))
                            .bg(t.bg_2)
                            .border_t(px(0.5))
                            .border_color(t.fg_3)
                            .p(px(16.))
                            .text_size(px(17.))
                            .line_height(px(24.))
                            .text_color(t.fg_0)
                            .text_center()
                            .child(label)
                            .cursor_pointer()
                            .hover(|s| s.bg(t.bg_color_active))
                            .active(|s| s.bg(t.bg_color_active))
                            .focus_visible(|s| s.border_2().border_color(t.brand))
                            .on_click(move |_, w, cx| {
                                if let Some(f) = &cb {
                                    f(i, w, cx);
                                }
                            })
                    }),
            )
    }
}
