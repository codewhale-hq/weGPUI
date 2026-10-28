use gpui::{prelude::*, *};
use wegpui::{Theme, I18n, Direction, accessibility::ModalScope, components::{
    Cells, form::FormGroup, radio::RadioGroup,
}};
#[path = "/src/examples/support/platform.rs"] mod platform;

struct Probe { grouped: bool, empty: bool, form: usize, cells: usize, form_actions: usize, cells_actions: usize }
impl Probe {
    fn new() -> Self {
        Self { grouped: std::env::var("RADIO_VARIANT").as_deref() == Ok("grouped"), empty: std::env::var("RADIO_ITEMS").as_deref() == Ok("empty"), form: 0, cells: 0, form_actions: 0, cells_actions: 0 }
    }
}
impl Render for Probe {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::get(cx).clone();
        let form = RadioGroup::new("form-group-options");
        let form = if self.empty { form } else { form.option("Form one").disabled_option("Form disabled").option("Form three") };
        let form = form.selected(Some(self.form))
            .on_change(cx.listener(|this, value, _, cx| { this.form = *value; cx.notify(); }));
        let form_extra = div().id("form-add-more").role(Role::Button).aria_label("Form add more")
            .focusable().tab_stop(true).child("Form add more").on_click(cx.listener(|this, _, _, cx| { this.form_actions += 1; cx.notify(); }));
        let form: AnyElement = if self.grouped {
            let rows = form.grouped_rows("Form delivery", (32., 16.), window, cx);
            let group = FormGroup::new("form-outer").first();
            let group = if let Some(rows) = rows { group.row(rows) } else { group };
            group.row(form_extra).into_any_element()
        } else {
            let rows = form.rows(window, cx);
            FormGroup::new("form-outer").first().role(Role::RadioGroup).aria_label("Form delivery")
                .rows(rows, 32.).row(form_extra).into_any_element()
        };
        let cells = RadioGroup::new("cells-group-options");
        let cells = if self.empty { cells } else { cells.option("Cells one").disabled_option("Cells disabled").option("Cells three") };
        let cells = cells.selected(Some(self.cells))
            .on_change(cx.listener(|this, value, _, cx| { this.cells = *value; cx.notify(); }));
        let cells_extra = div().id("cells-add-more").role(Role::Button).aria_label("Cells add more")
            .focusable().tab_stop(true).child("Cells add more").on_click(cx.listener(|this, _, _, cx| { this.cells_actions += 1; cx.notify(); }));
        let cells: AnyElement = if self.grouped {
            let rows = cells.grouped_rows("Cells delivery", (36., 12.), window, cx);
            let group = Cells::new().id("cells-outer").separator_inset(36., 12.);
            let group = if let Some(rows) = rows { group.child(rows) } else { group };
            group.child(cells_extra).into_any_element()
        } else {
            let rows = cells.rows(window, cx);
            Cells::new().id("cells-outer").separator_inset(36., 12.).role(Role::RadioGroup).aria_label("Cells delivery")
                .children(rows).child(cells_extra).into_any_element()
        };
        ModalScope::new(div().id("radio-proof").size_full().flex().flex_col().bg(theme.bg_0).text_color(theme.fg_0)
            .child(form).child(cells)
            .child(div().id("form-action-count").role(Role::Status).aria_label(format!("Form actions {}", self.form_actions)).child(format!("Form actions {}", self.form_actions)))
            .child(div().id("cells-action-count").role(Role::Status).aria_label(format!("Cells actions {}", self.cells_actions)).child(format!("Cells actions {}", self.cells_actions))))
    }
}
fn main() {
    platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        let direction = if std::env::var("RADIO_DIRECTION").as_deref() == Ok("rtl") { Direction::Rtl } else { Direction::Ltr };
        I18n::set_direction(Some(direction), cx);
        cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size(px(600.), px(900.))))),
            titlebar: Some(TitlebarOptions { title: Some("weGPUI radio set proof".into()), ..Default::default() }),
            ..Default::default()
        }, |window, cx| { Theme::observe(window); cx.new(|_| Probe::new()) }).unwrap();
        cx.activate(true);
    });
}
