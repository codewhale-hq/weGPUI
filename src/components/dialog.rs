use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;

pub type ActionHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
#[derive(Clone, IntoElement)]
pub struct Dialog {
    pub(crate) title: SharedString,
    pub(crate) body: SharedString,
    pub(crate) actions: Vec<SharedString>,
    pub(crate) on_action: Option<ActionHandler>,
    pub(crate) focuses: Vec<FocusHandle>,
}
impl Dialog {
    pub fn new(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            actions: vec!["确定".into()],
            on_action: None,
            focuses: vec![],
        }
    }
    pub fn actions(mut self, actions: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.actions = actions.into_iter().map(Into::into).collect();
        self
    }
    pub fn on_action(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for Dialog {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        div()
            .id("dialog")
            .role(Role::Dialog)
            .aria_label(self.title.clone())
            .w(px(320.))
            .max_w_full()
            .rounded(px(12.))
            .overflow_hidden()
            .bg(t.bg_2)
            .text_center()
            .flex()
            .flex_col()
            .child(
                div()
                    .pt(px(32.))
                    .pb(px(16.))
                    .px(px(24.))
                    .text_size(px(17.))
                    .line_height(px(23.8))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(t.fg_0)
                    .child(self.title),
            )
            .child(
                div()
                    .px(px(24.))
                    .mb(px(32.))
                    .text_size(px(17.))
                    .line_height(px(23.8))
                    .text_color(t.fg_1)
                    .child(self.body),
            )
            .child(
                div()
                    .flex()
                    .border_t(px(0.5))
                    .border_color(t.dialog_line_color)
                    .children(self.actions.iter().enumerate().map(|(i, label)| {
                        let callback = self.on_action.clone();
                        div()
                            .id(("dialog-action", i))
                            .role(Role::Button)
                            .aria_label(label.clone())
                            .focusable()
                            .tab_stop(true)
                            .when_some(self.focuses.get(i), |el, focus| el.track_focus(focus))
                            .flex_1()
                            .py(px(20.))
                            .px(px(8.))
                            .text_size(px(17.))
                            .line_height(px(24.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(if i + 1 == self.actions.len() {
                                t.brand
                            } else {
                                t.fg_half
                            })
                            .when(i > 0, |el| {
                                el.border_l(px(0.5)).border_color(t.dialog_line_color)
                            })
                            .hover(|s| s.bg(t.bg_color_active))
                            .active(|s| s.bg(t.bg_color_active))
                            .focus_visible(|s| s.border_2().border_color(t.brand))
                            .cursor_pointer()
                            .child(label.clone())
                            .on_click(move |_, w, cx| {
                                if let Some(f) = &callback {
                                    f(i, w, cx);
                                }
                            })
                    })),
            )
    }
}
