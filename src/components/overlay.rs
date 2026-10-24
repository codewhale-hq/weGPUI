use super::{ActionSheet, Dialog, Toast};
use crate::Theme;
use gpui::{prelude::*, *};
use std::rc::Rc;
#[derive(Clone)]
pub enum OverlayContent {
    Dialog(Dialog),
    ActionSheet(ActionSheet),
    Toast(Toast),
}
pub struct Overlay {
    content: Option<OverlayContent>,
    opener: Option<FocusHandle>,
    focus: FocusHandle,
    actions: Vec<FocusHandle>,
}
impl Overlay {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            content: None,
            opener: None,
            focus: cx.focus_handle(),
            actions: vec![],
        }
    }
    pub fn is_open(&self) -> bool {
        self.content.is_some()
    }
    pub fn open(&mut self, content: OverlayContent, window: &mut Window, cx: &mut Context<Self>) {
        if self.content.is_none() {
            self.opener = window.focused(cx);
        }
        let count = match &content {
            OverlayContent::Dialog(d) => d.actions.len(),
            OverlayContent::ActionSheet(s) => s.items.len() + 1,
            OverlayContent::Toast(_) => 0,
        };
        self.actions = (0..count)
            .map(|_| cx.focus_handle().tab_stop(true))
            .collect();
        self.content = Some(content);
        window.focus(self.actions.first().unwrap_or(&self.focus), cx);
        cx.notify();
    }
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.content = None;
        if let Some(opener) = self.opener.take() {
            window.focus(&opener, cx);
        }
        cx.notify();
    }
}
impl Render for Overlay {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(content) = self.content.clone() else {
            return div().into_any_element();
        };
        let toast = matches!(content, OverlayContent::Toast(_));
        let sheet = matches!(content, OverlayContent::ActionSheet(_));
        let weak = cx.entity().downgrade();
        let element = match content {
            OverlayContent::Dialog(mut dialog) => {
                let cb = dialog.on_action.take();
                dialog.focuses = self.actions.clone();
                dialog.on_action = Some(Rc::new(move |i, w, cx| {
                    let _ = weak.update(cx, |s, cx| s.close(w, cx));
                    if let Some(cb) = &cb {
                        cb(i, w, cx);
                    }
                }));
                dialog.into_any_element()
            }
            OverlayContent::ActionSheet(mut sheet) => {
                let cb = sheet.on_action.take();
                sheet.focuses = self.actions.clone();
                sheet.on_action = Some(Rc::new(move |i, w, cx| {
                    let _ = weak.update(cx, |s, cx| s.close(w, cx));
                    if let Some(cb) = &cb {
                        cb(i, w, cx);
                    }
                }));
                sheet.into_any_element()
            }
            OverlayContent::Toast(toast) => toast.into_any_element(),
        };
        div()
            .id("overlay")
            .absolute()
            .inset_0()
            .track_focus(&self.focus)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .when(sheet, |el| el.justify_end())
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, w, cx| {
                if event.keystroke.key == "escape" {
                    this.close(w, cx);
                    cx.stop_propagation();
                }
                if event.keystroke.key == "tab" {
                    let n = this.actions.len();
                    if n > 0 {
                        let index = this
                            .actions
                            .iter()
                            .position(|h| h.is_focused(w))
                            .unwrap_or(0);
                        let next = if event.keystroke.modifiers.shift {
                            (index + n - 1) % n
                        } else {
                            (index + 1) % n
                        };
                        w.focus(&this.actions[next], cx);
                    }
                    w.prevent_default();
                    cx.stop_propagation();
                }
            }))
            .child(
                div()
                    .id("backdrop")
                    .absolute()
                    .inset_0()
                    .bg(if toast {
                        transparent_black()
                    } else {
                        Theme::get(cx).overlay
                    })
                    .occlude()
                    .on_click(cx.listener(|this, _, w, cx| this.close(w, cx))),
            )
            .child(
                div()
                    .relative()
                    .when(sheet, |el| el.w_full())
                    .when(toast, |el| el.mb(relative(0.1)))
                    .child(element),
            )
            .into_any_element()
    }
}
