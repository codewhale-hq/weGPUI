use gpui::{prelude::*, *};
#[path = "/src/vendor/gpui_pre/src/text_system/browser_dictionary.rs"]
mod browser_dictionary;
#[path = "/src/examples/support/platform.rs"]
mod platform;

struct Probe;

impl Render for Probe {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let corpus: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string("/work/full-parity/dictionary-extended/corpus.json").unwrap(),
        )
        .unwrap();
        let mut results = Vec::new();
        let mut root = div()
            .size_full()
            .p(px(16.))
            .flex()
            .gap(px(16.))
            .bg(rgb(0xffffff))
            .text_color(rgb(0))
            .font_family(".SystemUIFont")
            .text_size(px(17.))
            .line_height(relative(1.4));
        let stock = icu_segmenter::LineSegmenter::new_dictionary(Default::default());
        for case in corpus["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            let language = case["language"].as_str().unwrap();
            let text = case["text"].as_str().unwrap();
            let runs = [TextRun {
                len: text.len(),
                font: font(".SystemUIFont"),
                color: rgb(0).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            }];
            let stock_breaks: Vec<_> = stock.segment_str(text).collect();
            let candidate_breaks = browser_dictionary::break_indices(text);
            for width in corpus["widths"].as_array().unwrap() {
                let width = width.as_f64().unwrap() as f32;
                let line = window
                    .text_system()
                    .shape_text_with_overflow_wrap(
                        text.into(),
                        px(17.),
                        &runs,
                        Some(px(width)),
                        None,
                        OverflowWrap::Normal,
                    )
                    .unwrap()
                    .remove(0);
                let rows: Vec<_> = (0..=line.wrap_boundaries.len())
                    .map(|i| {
                        let start = line.wrapped_line_start_index(i);
                        let end = if i < line.wrap_boundaries.len() {
                            line.wrapped_line_start_index(i + 1)
                        } else {
                            text.len()
                        };
                        text[start..end].to_owned()
                    })
                    .collect();
                results.push(serde_json::json!({
                    "id": id,
                    "language": language,
                    "width": width,
                    "text": text,
                    "lines": rows,
                    "stock_breaks": stock_breaks,
                    "candidate_breaks": candidate_breaks,
                    "row_widths": line.bidi_rows().iter().map(|row| row.width.as_f32()).collect::<Vec<_>>(),
                }));
            }
            if id.ends_with("original") {
                root = root.child(
                    div()
                        .flex_none()
                        .w(px(225.))
                        .overflow_wrap(OverflowWrap::Normal)
                        .child(
                            div()
                                .mb(px(12.))
                                .font_weight(FontWeight::MEDIUM)
                                .child(language.to_owned()),
                        )
                        .child(text.to_owned()),
                );
            }
        }
        std::fs::write(
            "/work/full-parity/dictionary-extended/native.json",
            serde_json::to_string_pretty(&results).unwrap(),
        )
        .unwrap();
        root
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
                        size(px(1024.), px(900.)),
                    ))),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Probe),
            )
            .unwrap();
            cx.activate(true);
        });
}
