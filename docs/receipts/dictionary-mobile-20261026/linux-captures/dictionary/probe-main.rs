use gpui::{prelude::*, *};
#[path="/src/examples/support/platform.rs"] mod platform;
struct Probe;
impl Render for Probe {
 fn render(&mut self, window:&mut Window, _: &mut Context<Self>) -> impl IntoElement {
  let texts:std::collections::BTreeMap<String,String>=serde_json::from_str(include_str!("../../dictionary-texts.json")).unwrap();
  let mut results=Vec::new();let mut root=div().size_full().p(px(16.)).flex().gap(px(16.)).bg(rgb(0xffffff)).text_color(rgb(0)).font_family(".SystemUIFont").text_size(px(17.)).line_height(relative(1.4));
  for (language,text) in texts {
   let runs=[TextRun {len:text.len(),font:font(".SystemUIFont"),color:rgb(0).into(),background_color:None,underline:None,strikethrough:None}];
   for width in [160.,225.,327.,650.] {
    let line=window.text_system().shape_text(text.clone().into(),px(17.),&runs,Some(px(width)),None).unwrap().remove(0);
    let rows:Vec<_>=(0..=line.wrap_boundaries.len()).map(|i|{let start=line.wrapped_line_start_index(i);let end=if i<line.wrap_boundaries.len(){line.wrapped_line_start_index(i+1)}else{text.len()};text[start..end].to_string()}).collect();
    results.push(serde_json::json!({"language":language,"width":width,"text":text,"lines":rows,"full_width":line.unwrapped_layout.width.as_f32(),"breaks":icu_segmenter::LineSegmenter::new_dictionary(Default::default()).segment_str(&text).collect::<Vec<_>>(),"glyphs":line.unwrapped_layout.runs.iter().flat_map(|run|run.glyphs.iter()).map(|g|serde_json::json!([g.index,g.position.x.as_f32()])).collect::<Vec<_>>(),"row_widths":line.bidi_rows().iter().map(|row|row.width.as_f32()).collect::<Vec<_>>()}));
   }
   root=root.child(div().flex_none().w(px(225.)).child(div().mb(px(12.)).font_weight(FontWeight::MEDIUM).child(language)).child(text));
  }
  std::fs::write("/work/full-parity/dictionary-native.json",serde_json::to_string_pretty(&results).unwrap()).unwrap();root
 }
}
fn main(){platform::application().with_assets(wegpui::Assets).run(|cx|{wegpui::init(cx);cx.open_window(WindowOptions {window_bounds:Some(WindowBounds::Windowed(Bounds::new(point(px(30.),px(30.)),size(px(1024.),px(900.))))),..Default::default()},|_,cx|cx.new(|_|Probe)).unwrap();cx.activate(true);});}
