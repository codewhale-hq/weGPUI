use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    input::{InputEvent, InputKind, TextInput},
};
#[path = "/src/examples/support/platform.rs"]
mod platform;
struct Probe {
    field: Entity<TextInput>,
    root_focus: FocusHandle,
    changes: usize,
    _subscription: Subscription,
}
impl Probe {
    fn new(cx: &mut Context<Self>) -> Self {
        let field = cx.new(|cx| {
            let mut f = TextInput::new("Telephone", cx);
            f.set_aria_label("Telephone", cx);
            f.set_kind(InputKind::Tel, cx);
            f
        });
        let sub = cx.subscribe(&field, |this, _, event: &InputEvent, _| {
            if *event == InputEvent::Changed {
                this.changes += 1;
            }
        });
        Self {
            field,
            root_focus: cx.focus_handle(),
            changes: 0,
            _subscription: sub,
        }
    }
    fn snapshot(&self, window: &mut Window, cx: &mut Context<Self>) {
        let mut v = self.field.update(cx, |f, cx| {
            let selection = f.selected_text_range(false, window, cx).unwrap();
            let marked = f.marked_text_range(window, cx);
            serde_json::json!({"value":f.text(),"len":f.len(),"kind":format!("{:?}",f.kind()),"readonly":f.is_readonly(),"disabled":f.is_disabled(),"selection":[selection.range.start,selection.range.end],"reversed":selection.reversed,"marked":marked.map(|r|[r.start,r.end])})
        });
        v["changes"] = self.changes.into();
        std::fs::write(
            "/work/full-parity/tel-input-snapshot.json",
            serde_json::to_string(&v).unwrap(),
        )
        .unwrap();
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        div()
            .id("tel-input-probe")
            .size_full()
            .p(px(24.))
            .flex()
            .flex_col()
            .gap(px(12.))
            .bg(t.bg_0)
            .text_color(t.fg_0)
            .track_focus(&self.root_focus)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                match event.keystroke.key.as_str() {
                    "f1" => this.field.update(cx, |f, cx| {
                        f.set_max_len(None, cx);
                        f.set_kind(InputKind::Digit, cx);
                        f.set_text("12", cx);
                        f.replace_and_mark_text_in_range(
                            Some(1..1),
                            "中7😀8",
                            Some(3..3),
                            window,
                            cx,
                        );
                        f.unmark_text(window, cx);
                    }),
                    "f13" => this.field.update(cx, |f, cx| {
                        f.set_kind(InputKind::Tel, cx);
                        f.set_max_len(Some(4), cx);
                        f.set_text("AZ", cx);
                        f.replace_and_mark_text_in_range(
                            Some(1..1),
                            "😀\0中",
                            Some(2..3),
                            window,
                            cx,
                        );
                    }),
                    "f14" => this.field.update(cx, |f, cx| f.unmark_text(window, cx)),
                    "f15" => this.field.update(cx, |f, cx| {
                        f.set_max_len(None, cx);
                        f.set_kind(InputKind::Tel, cx);
                        f.set_text("ab", cx);
                        f.replace_and_mark_text_in_range(
                            Some(1..1),
                            "\u{301}",
                            Some(0..0),
                            window,
                            cx,
                        );
                    }),
                    "f2" => {
                        cx.write_to_clipboard(ClipboardItem::new_string("+٨٦ １２３ ext. 9".into()))
                    }
                    "f3" => this.field.update(cx, |f, cx| {
                        f.set_readonly(false, cx);
                        f.set_disabled(false, cx);
                        f.set_max_len(None, cx);
                        f.set_kind(InputKind::Tel, cx);
                        f.set_text("", cx);
                        f.focus(window, cx);
                    }),
                    "f4" => this.field.update(cx, |f, cx| {
                        f.set_max_len(Some(6), cx);
                        f.set_text("A😀B", cx);
                    }),
                    "f5" => this
                        .field
                        .update(cx, |f, cx| f.set_readonly(!f.is_readonly(), cx)),
                    "f6" => {
                        let disabled = this.field.read(cx).is_disabled();
                        this.field.update(cx, |f, cx| f.set_disabled(!disabled, cx));
                        if disabled {
                            this.field.update(cx, |f, cx| f.focus(window, cx));
                        } else {
                            window.focus(&this.root_focus, cx);
                        }
                    }
                    "f7" => this
                        .field
                        .update(cx, |f, cx| f.set_kind(InputKind::Digit, cx)),
                    "f8" => this
                        .field
                        .update(cx, |f, cx| f.set_kind(InputKind::Tel, cx)),
                    "f9" => cx.write_to_clipboard(ClipboardItem::new_string("😀界".into())),
                    "f10" => this
                        .field
                        .update(cx, |f, cx| f.set_text("A\r\nB\tC\0\u{1}", cx)),
                    "f11" => cx.write_to_clipboard(ClipboardItem::new_string("x\r\ny".into())),
                    "f12" => this.snapshot(window, cx),
                    _ => return,
                }
                cx.stop_propagation();
            }))
            .child("Telephone input parity probe")
            .child(self.field.clone())
    }
}
fn main() {
    platform::application()
        .with_assets(wegpui::Assets)
        .run(|cx| {
            wegpui::init(cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(30.), px(30.)),
                        size(px(640.), px(240.)),
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI telephone probe".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    let view = cx.new(Probe::new);
                    view.read(cx)
                        .field
                        .clone()
                        .update(cx, |f, cx| f.focus(window, cx));
                    view
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
