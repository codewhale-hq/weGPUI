use gpui::{prelude::*, *};
use serde_json::json;
use wegpui::{Theme, components::textarea::TextArea};

const PREFIX: &str = "😀េាហ";
const COMPOSITION: &str = "ภาษาไทยภาษา";
struct Probe {
    area: Entity<TextArea>,
}
impl Probe {
    fn new(cx: &mut Context<Self>) -> Self {
        let area = cx.new(|cx| {
            let mut area = TextArea::new("Threshold input", cx);
            area.set_aria_label("Threshold input", cx);
            area.set_rows(8, cx);
            area.set_text("กກ".repeat(4096), cx);
            area
        });
        Self { area }
    }
    fn input(&self, cx: &App) -> Entity<wegpui::input::TextInput> {
        self.area.read(cx).input().clone()
    }
    fn snapshot(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.input(cx).update(cx, |input, cx| {
            let text = input.text().to_owned();
            let selected = input.selected_text_range(false, window, cx).unwrap();
            let marked = input.marked_text_range(window, cx);
            let origin = Bounds::new(point(px(0.),px(0.)), size(px(200.),px(200.)));
            let rectangles: Vec<_> = [0..0, 0..4, 4..4, 4..5].into_iter().map(|range| {
                input.bounds_for_range(range, origin, window, cx).map(|b| [b.origin.x/px(1.),b.origin.y/px(1.),b.size.width/px(1.),b.size.height/px(1.)])
            }).collect();
            eprintln!("SNAPSHOT {}", json!({"bytes":text.len(),"utf16":text.encode_utf16().count(),"prefix":text.chars().take(12).collect::<String>(),"selection":[selected.range.start,selected.range.end],"reversed":selected.reversed,"marked":marked.map(|r| [r.start,r.end]),"rectangles":rectangles}));
        });
    }
    fn action(&self, key: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.input(cx).update(cx, |input, cx| match key {
            "f1" => input.set_selected_text_range(0..0, window, cx),
            "f2" => input.set_selected_text_range(1..1, window, cx),
            "f3" => input.set_selected_text_range(0..4, window, cx),
            "f4" => input.replace_and_mark_text_in_range(None, COMPOSITION, None, window, cx),
            "f5" => input.replace_and_mark_text_in_range(None, "", None, window, cx),
            "f6" => input.replace_text_in_range(None, COMPOSITION, window, cx),
            _ => (),
        });
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xffffff))
            .p(px(20.))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if key == "f7" {
                    cx.write_to_clipboard(ClipboardItem::new_string("ก".into()));
                    cx.stop_propagation();
                } else if key == "f12" {
                    this.snapshot(window, cx);
                    cx.stop_propagation();
                } else if ["f1", "f2", "f3", "f4", "f5", "f6"].contains(&key) {
                    this.action(key, window, cx);
                    cx.stop_propagation();
                }
            }))
            .child(div().w(px(64.)).child(self.area.clone()))
    }
}
#[path = "/src/examples/support/platform.rs"]
mod platform;
fn main() {
    platform::application()
        .with_assets(wegpui::Assets)
        .run(|cx| {
            wegpui::init(cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(60.), px(60.)),
                        size(px(400.), px(320.)),
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("weGPUI threshold input probe".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    Theme::observe(window);
                    let view = cx.new(Probe::new);
                    let area = view.read(cx).area.clone();
                    area.update(cx, |area, cx| area.focus(window, cx));
                    view
                },
            )
            .expect("open window");
            cx.activate(true);
        });
}
