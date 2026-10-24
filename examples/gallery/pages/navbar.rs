use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::navbar::{Navbar, tab_panel};

const PANELS: [&str; 3] = [
    "Sit sit quibusdam assumenda facilis magnam temporibus? Molestiae qui ad accusantium minus mollitia Esse maiores voluptates suscipit fugit veritatis numquam repellat omnis. Dolores facilis deleniti reiciendis ea beatae Quaerat soluta",
    "Consectetur neque facere modi recusandae tempore. Sit repellendus qui voluptatibus voluptas porro perspiciatis. Incidunt iusto soluta esse ipsum ex. Consectetur blanditiis dolor facere dignissimos minus quam libero? Nemo voluptas aperiam?",
    "Dolor magnam expedita ex beatae maiores assumenda Doloribus et totam numquam officiis beatae Sint eos saepe magnam error aut Doloremque necessitatibus eligendi quod fuga debitis. Error ad optio repellendus amet.",
];
const TABS: [&str; 3] = ["选项一", "选项二", "选项三"];

pub struct NavbarPage {
    selected: usize,
}

pub fn build(_: &'static str, args: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    let selected = match args.state.as_deref() {
        Some("tab2") => 1,
        Some("tab3") => 2,
        _ => 0,
    };
    cx.new(|_| NavbarPage { selected }).into()
}

impl Render for NavbarPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected;
        let entity = cx.entity().downgrade();
        PageFrame::new("navbar").no_scroll().fixed(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    Navbar::new("navbar")
                        .items(TABS)
                        .selected(selected)
                        .on_select(move |i, _, cx| {
                            entity
                                .update(cx, |this, cx| {
                                    this.selected = i;
                                    cx.notify();
                                })
                                .ok();
                        }),
                )
                .child(
                    tab_panel(("panel", selected), TABS[selected])
                        .p(px(32.))
                        .child(PANELS[selected]),
                ),
        )
    }
}
