use gpui::{prelude::*, *};
use wegpui::{I18n, Locale, Theme, accessibility::ModalScope,
    components::date_field::{DateField, DateFieldMode, DateValue}};
#[path = "/src/examples/support/platform.rs"] mod platform;
struct Probe { field: Entity<DateField>, mode: String }
impl Probe {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mode = std::env::var("DATE_PROBE_MODE").unwrap_or_else(|_| "display".into());
        let date_mode = if std::env::var_os("DATE_PROBE_DATETIME").is_some() { DateFieldMode::DateTime } else { DateFieldMode::Date };
        let field = cx.new(|cx| {
            let mut f = DateField::new("date", "Date", date_mode, cx);
            f.set_value(Some(DateValue { hour: 5, minute: 6, ..DateValue::date(2027, 3, 4) }), cx);
            f.set_typed_entry(mode != "picker", cx);
            f
        });
        if mode == "picker" {
            let copy = field.clone();
            window.defer(cx, move |window, cx| copy.update(cx, |f, cx| f.open(window, cx)));
        } else if let Ok(text) = std::env::var("DATE_PROBE_ENTRY") && text != "__NONE__" {
            let copy = field.clone();
            window.defer(cx, move |_, cx| {
                copy.update(cx, |f, cx| {
                    let input = f.text_input().clone();
                    input.update(cx, |input, cx| input.set_text(text, cx));
                    eprintln!("commit={:?}", f.commit_text(cx));
                });
            });
        }
        Self { field, mode }
    }
}
impl Render for Probe {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let f = self.field.read(cx);
        let text = f.text_input().read(cx).text();
        let value = f.value().map(|v|v.format(f.mode())).unwrap_or_default();
        let status = format!("Text {text}; ISO {value}");
        let page = div().size_full().p(px(32.)).flex().flex_col().gap(px(20.))
            .bg(Theme::get(cx).bg_0).child(self.field.clone())
            .child(div().id("status").role(Role::Status).aria_label(status.clone()).child(status));
        let _ = &self.mode;
        ModalScope::new(page)
    }
}
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        let locale = std::env::var("DATE_PROBE_LOCALE").unwrap_or_else(|_| "en-CA".into());
        I18n::set_locale(Locale::parse(&locale).unwrap(), cx);
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.),px(0.)),size(px(600.),px(800.))))),
            titlebar: Some(TitlebarOptions { title: Some("weGPUI date locale probe".into()), ..Default::default() }), ..Default::default()
        }, |window,cx| { Theme::observe(window); cx.new(|cx| Probe::new(window,cx)) }).unwrap();
        cx.activate(true);
    });
}
