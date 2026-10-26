use gpui::{prelude::*, *};
const DESC: &str = "由各种物质组成的巨型球状天体，叫做星球。星球有一定的形状，有自己的运行轨道。";
struct Probe;
impl Render for Probe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(white()).text_color(black()).font_family("Noto Sans").flex().flex_col().gap(px(24.)).p(px(24.))
            .child(div().child("267px: normal wrapping"))
            .child(div().w(px(267.)).text_size(px(14.)).line_height(relative(1.4)).child(DESC))
            .child(div().child("267px: two lines with ellipsis"))
            .child(div().w(px(267.)).text_size(px(14.)).line_height(relative(1.4)).overflow_hidden().text_ellipsis().line_clamp(2).child(DESC))
    }
}
fn main() {
    gpui_platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        cx.open_window(WindowOptions {window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(20.),px(20.)),size(px(375.),px(480.))))),..Default::default()}, |window,cx| {
            for width in [267.,343.] {
                let run=TextRun {len:DESC.len(),font:font("Noto Sans"),color:black(),background_color:None,underline:None,strikethrough:None};
                let original=window.text_system().shape_line(DESC.into(),px(14.),&[run.clone()],None);
                let mut wrapper=window.text_system().line_wrapper(run.font.clone(),px(14.));
                let (truncated,_)=wrapper.truncate_wrapped_line(DESC.into(),px(width),2,"…",&[run.clone()],TruncateFrom::End);
                eprintln!("CLAMP width={} chars={} shaped_width={:?} truncated={:?}",width,DESC.chars().count(),original.width,truncated);
                let lines=window.text_system().shape_text(DESC.into(),px(14.),&[run],Some(px(width)),None).unwrap();
                for line in lines {
                    let starts:Vec<_>=(0..=line.wrap_boundaries().len()).map(|row|line.wrapped_line_start_index(row)).collect();
                    eprintln!("WRAP width={} starts={:?}",width,starts);
                }
            }
            cx.new(|_|Probe)
        }).unwrap();
        cx.activate(true);
    });
}
