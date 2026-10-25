use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::components::{
    button::Button,
    picker::{Picker, PickerEvent, PickerItem, today},
};

pub struct PickerPage {
    single: Entity<Picker>,
    date: Entity<Picker>,
    multi: Entity<Picker>,
    cascade: Entity<Picker>,
    _subscriptions: Vec<Subscription>,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| PickerPage::new(state.as_deref(), window, cx))
        .into()
}

fn region(label: &str, children: Vec<PickerItem>) -> PickerItem {
    PickerItem::new(label.to_string(), label.to_string()).children(children)
}

impl PickerPage {
    fn new(state: Option<&str>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let single = cx.new(|cx| {
            Picker::single(
                "single",
                [
                    PickerItem::new(tr("飞机票"), "0"),
                    PickerItem::new(tr("火车票"), "1"),
                    PickerItem::new(tr("的士票"), "2"),
                    PickerItem::new(tr("公交票 (disabled)"), "3").disabled(true),
                    PickerItem::new(tr("其他"), "4"),
                ],
                cx,
            )
            .title(tr("单列选择器"))
            .show_close(false)
        });
        let (year, month, day) = today();
        let date = cx.new(|cx| {
            Picker::new("date", cx)
                .date(1990, year, (year, month, day))
                .title(tr("多列选择器"))
        });
        let multi = cx.new(|cx| {
            Picker::new("multi", cx)
                .columns(vec![
                    ["1", "2", "3"]
                        .iter()
                        .map(|v| PickerItem::new(*v, *v))
                        .collect(),
                    ["A", "B", "C"]
                        .iter()
                        .map(|v| PickerItem::new(*v, *v))
                        .collect(),
                ])
                .default_value(["3", "A"])
                .title(tr("多列选择器"))
        });
        let cascade = cx.new(|cx| {
            Picker::new("cascade", cx)
                .cascade(
                    vec![
                        region(
                            tr("广东"),
                            vec![
                                region(
                                    tr("广州"),
                                    vec![region(tr("海珠"), vec![]), region(tr("番禺"), vec![])],
                                ),
                                region(
                                    tr("佛山"),
                                    vec![region(tr("禅城"), vec![]), region(tr("南海"), vec![])],
                                ),
                            ],
                        ),
                        region(
                            tr("广西"),
                            vec![
                                region(
                                    tr("南宁"),
                                    vec![region(tr("青秀"), vec![]), region(tr("兴宁"), vec![])],
                                ),
                                region(
                                    tr("桂林"),
                                    vec![region(tr("象山"), vec![]), region(tr("秀峰"), vec![])],
                                ),
                            ],
                        ),
                    ],
                    3,
                )
                .default_value([tr("广东"), tr("广州"), tr("海珠")])
                .title(tr("级联选择器"))
        });
        let pickers = [single.clone(), date.clone(), multi.clone(), cascade.clone()];
        let subscriptions = pickers
            .iter()
            .map(|p| cx.subscribe(p, |_, _, _: &PickerEvent, cx| cx.notify()))
            .collect();
        let target = match state {
            Some("single") => Some(single.clone()),
            Some("date") => Some(date.clone()),
            Some("multi") => Some(multi.clone()),
            Some("cascade") => Some(cascade.clone()),
            _ => None,
        };
        if let Some(target) = target {
            window.defer(cx, move |window, cx| {
                target.update(cx, |p, cx| p.open_instantly(window, cx));
            });
        }
        Self {
            single,
            date,
            multi,
            cascade,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for PickerPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let open = |picker: &Entity<Picker>| {
            let picker = picker.clone();
            move |_: &ClickEvent, window: &mut Window, cx: &mut App| {
                picker.update(cx, |p, cx| p.open(window, cx));
            }
        };
        PageFrame::new("picker")
            .title("Picker")
            .desc(tr("多列选择器，需要配合js实现"))
            .spacing()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(
                        Button::new(tr("单列选择器"))
                            .id("showPicker")
                            .on_click(open(&self.single)),
                    )
                    .child(
                        Button::new(tr("日期选择器"))
                            .id("showDatePicker")
                            .on_click(open(&self.date)),
                    ),
            )
            .child(self.single.clone())
            .child(self.date.clone())
            .child(self.multi.clone())
            .child(self.cascade.clone())
    }
}
