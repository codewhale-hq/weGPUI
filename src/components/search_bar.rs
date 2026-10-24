use super::Icon;
use crate::{Theme, input::TextInput};
use gpui::{prelude::*, *};
pub struct SearchBar {
    pub input: Entity<TextInput>,
    active: bool,
    focus: FocusHandle,
}
impl SearchBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| TextInput::new("搜索", cx));
        cx.on_focus_in(&input.focus_handle(cx), window, |this, _, cx| {
            this.active = true;
            cx.notify();
        })
        .detach();
        cx.observe(&input, |_, _, cx| cx.notify()).detach();
        Self {
            input,
            active: false,
            focus: cx.focus_handle(),
        }
    }
}
impl Render for SearchBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let input = self.input.clone();
        let field = div()
            .id("search-field")
            .track_focus(&self.focus)
            .flex_1()
            .min_w_0()
            .h(px(36.))
            .rounded(px(6.))
            .bg(t.bg_2)
            .flex()
            .items_center()
            .px(px(8.))
            .gap(px(4.))
            .when(!self.active, |el| {
                el.focusable()
                    .tab_stop(true)
                    .justify_center()
                    .on_click(cx.listener(|this, _, w, cx| {
                        this.active = true;
                        w.focus(&this.input.focus_handle(cx), cx);
                        cx.notify();
                    }))
            })
            .child(Icon::new("search").size(24.).color(t.fg_2))
            .when(self.active, |el| el.child(input.clone()))
            .when(!self.active, |el| {
                el.child(div().text_size(px(17.)).text_color(t.fg_2).child("搜索"))
            })
            .when(self.active && !input.read(cx).text().is_empty(), |el| {
                el.child(
                    div()
                        .id("clear-search")
                        .role(Role::Button)
                        .aria_label("清除搜索")
                        .focusable()
                        .tab_stop(true)
                        .child(Icon::new("clear").size(20.).color(t.fg_2))
                        .on_click(cx.listener(|this, _, w, cx| {
                            this.input.update(cx, |input, cx| input.clear(cx));
                            w.focus(&this.input.focus_handle(cx), cx);
                        })),
                )
            });
        div()
            .flex()
            .items_center()
            .w_full()
            .py(px(10.))
            .px(px(if self.active { 16. } else { 12. }))
            .bg(t.bg_0)
            .gap(px(12.))
            .child(field)
            .when(self.active, |el| {
                el.child(
                    div()
                        .id("cancel-search")
                        .role(Role::Button)
                        .aria_label("取消搜索")
                        .focusable()
                        .tab_stop(true)
                        .cursor_pointer()
                        .text_size(px(17.))
                        .text_color(t.link)
                        .child("取消")
                        .on_click(cx.listener(|this, _, w, cx| {
                            this.active = false;
                            this.input.update(cx, |input, cx| input.clear(cx));
                            w.focus(&this.focus, cx);
                            cx.notify();
                        })),
                )
            })
    }
}
