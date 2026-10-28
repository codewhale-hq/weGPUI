use gpui::{prelude::*, *};
use std::{cell::Cell, rc::Rc};
#[path = "/src/examples/support/platform.rs"]
mod platform;

struct Sample {
    start: usize,
    end: usize,
    row: usize,
    row_width: f32,
    selection_x: f32,
    selection_width: f32,
    caret_x: f32,
}
struct Case {
    name: String,
    text: String,
    policy: OverflowWrap,
    align: TextAlign,
    layout: TextLayout,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    samples: Vec<Sample>,
}
struct Probe;
impl Render for Probe {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut cases = Vec::new();
        let mut root = div()
            .size_full()
            .p(px(16.))
            .flex()
            .flex_col()
            .gap(px(5.))
            .bg(rgb(0xffffff))
            .text_color(rgb(0))
            .font_family(".SystemUIFont")
            .text_size(px(17.))
            .line_height(px(24.));
        for (name, text) in [
            ("ltr", "supercalifragilisticexpialidocious end"),
            ("rtl", "אבגדהוזחטיכלמנסעפצקרשתאבגדהוז סוף"),
        ] {
            for policy in [OverflowWrap::Normal, OverflowWrap::BreakWord] {
                for align in [TextAlign::Left, TextAlign::Center, TextAlign::Right] {
                    let runs = [TextRun {
                        len: text.len(),
                        font: font(".SystemUIFont"),
                        color: rgb(0).into(),
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }];
                    let line = window
                        .text_system()
                        .shape_text_with_overflow_wrap(
                            text.into(),
                            px(17.),
                            &runs,
                            Some(px(120.)),
                            None,
                            policy,
                        )
                        .unwrap()
                        .remove(0);
                    let mut samples = Vec::new();
                    for (row, metrics) in line.bidi_rows().iter().enumerate() {
                        let start = metrics.range.start;
                        if start >= metrics.range.end {
                            continue;
                        }
                        let end = start + text[start..].chars().next().unwrap().len_utf8();
                        let rect = line
                            .selection_bounds(start..end, px(24.))
                            .into_iter()
                            .next()
                            .unwrap();
                        let caret = line.position_for_index(start, px(24.)).unwrap();
                        samples.push(Sample {
                            start,
                            end,
                            row,
                            row_width: metrics.width.as_f32(),
                            selection_x: rect.origin.x.as_f32(),
                            selection_width: rect.size.width.as_f32(),
                            caret_x: caret.x.as_f32(),
                        });
                    }
                    let text_element = StyledText::new(text);
                    let layout = text_element.layout().clone();
                    let bounds = Rc::new(Cell::new(None));
                    let captured = bounds.clone();
                    let marker = canvas(move |b, _, _| captured.set(Some(b)), |_, _, _, _| {})
                        .absolute()
                        .top_0()
                        .left_0()
                        .w_full()
                        .h_full();
                    root = root.child(
                        div()
                            .relative()
                            .w(px(120.))
                            .text_align(align)
                            .overflow_wrap(policy)
                            .bg(rgb(0xf2f2f2))
                            .child(text_element)
                            .child(marker),
                    );
                    cases.push(Case {
                        name: name.into(),
                        text: text.into(),
                        policy,
                        align,
                        layout,
                        bounds,
                        samples,
                    });
                }
            }
        }
        window.on_next_frame(move |_,_| {
   let mut results=Vec::new();let mut ok=true;let mut observed_overflow=false;
   for case in cases {
    let Some(bounds)=case.bounds.get() else {ok=false;results.push(serde_json::json!({"name":case.name,"error":"missing prepaint bounds"}));continue;};
    let mut samples=Vec::new();
    for sample in case.samples {
     if sample.row_width>bounds.size.width.as_f32()+0.1 && case.policy==OverflowWrap::Normal {observed_overflow=true;}
     let shift=match case.align {TextAlign::Left=>0.,TextAlign::Center=>(bounds.size.width.as_f32()-sample.row_width)/2.,TextAlign::Right=>bounds.size.width.as_f32()-sample.row_width};
     let rects=case.layout.selection_bounds(sample.start..sample.end);
     let caret=case.layout.position_for_index(sample.start);
     if rects.len()!=1 || caret.is_none() {ok=false;samples.push(serde_json::json!({"row":sample.row,"error":"missing selection or caret","rect_count":rects.len()}));continue;}
     let rect=rects[0];let caret=caret.unwrap();
     let expected_x=bounds.origin.x.as_f32()+shift+sample.selection_x;
     let expected_caret=bounds.origin.x.as_f32()+shift+sample.caret_x;
     let mid=point(rect.origin.x+rect.size.width/2.,rect.origin.y+rect.size.height/2.);
     let hit=case.layout.index_for_position(mid);
     let hit_ok=matches!(hit,Ok(i) if i>=sample.start && i<sample.end);
     let rect_ok=(rect.origin.x.as_f32()-expected_x).abs()<0.1 && (rect.size.width.as_f32()-sample.selection_width).abs()<0.1;
     let caret_ok=(caret.x.as_f32()-expected_caret).abs()<0.1;
     ok &= rect_ok && caret_ok && hit_ok;
     samples.push(serde_json::json!({"row":sample.row,"range":[sample.start,sample.end],"row_width":sample.row_width,"shift":shift,"selection_x":rect.origin.x.as_f32(),"expected_selection_x":expected_x,"selection_width":rect.size.width.as_f32(),"expected_selection_width":sample.selection_width,"caret_x":caret.x.as_f32(),"expected_caret_x":expected_caret,"hit":format!("{hit:?}"),"rect_ok":rect_ok,"caret_ok":caret_ok,"hit_ok":hit_ok}));
    }
    results.push(serde_json::json!({"name":case.name,"text":case.text,"policy":format!("{:?}",case.policy),"align":format!("{:?}",case.align),"bounds":[bounds.origin.x.as_f32(),bounds.origin.y.as_f32(),bounds.size.width.as_f32(),bounds.size.height.as_f32()],"samples":samples}));
   }
   ok &= observed_overflow;
   let payload=serde_json::json!({"passed":ok,"observed_normal_overflow":observed_overflow,"cases":results});
   std::fs::write("/work/full-parity/overflow-geometry-result.json",serde_json::to_string_pretty(&payload).unwrap()).unwrap();
  });
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
                        size(px(900.), px(1400.)),
                    ))),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Probe),
            )
            .unwrap();
            cx.activate(true);
        });
}
