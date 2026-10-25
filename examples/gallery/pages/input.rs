use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    Theme,
    components::{
        Cell, Cells, Presence,
        agree::Agree,
        button::Button,
        checkbox::CheckboxCell,
        picker::{Picker, PickerEvent, PickerItem, today},
        radio::RadioGroup,
        select::{SelectCell, SelectMenu},
        switch::Switch,
        textarea::TextArea,
        vcode::{VcodeButton, VcodeImage},
    },
    input::{InputKind, TextInput},
    scrolling::RevealFocus,
};

const LABEL_WIDTH: f32 = 105.;
const SELECTS: [&[&str]; 3] = [
    &["+86", "+80", "+84", "+87"],
    &["微信号", "QQ号", "Email"],
    &["中国", "美国", "英国"],
];

pub struct InputPage {
    radio: Option<usize>,
    checks: [bool; 2],
    switches: [bool; 2],
    agree: bool,
    selected: [usize; 3],
    menus: [Presence; 3],
    qq: Entity<TextInput>,
    phone: Entity<TextInput>,
    date: Option<String>,
    time: Option<String>,
    date_picker: Entity<Picker>,
    datetime_date: Entity<Picker>,
    datetime_time: Entity<Picker>,
    pending_date: Option<String>,
    _subscriptions: Vec<Subscription>,
    code: Entity<TextInput>,
    card: Entity<TextInput>,
    text: Entity<TextInput>,
    textarea: Entity<TextArea>,
    number: Entity<TextInput>,
    vcode: Entity<VcodeButton>,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| InputPage::new(state.as_deref(), window, cx))
        .into()
}

fn field(
    placeholder: &'static str,
    kind: InputKind,
    cx: &mut Context<InputPage>,
) -> Entity<TextInput> {
    cx.new(|cx| {
        let mut input = TextInput::new(placeholder, cx);
        input.set_kind(kind, cx);
        input
    })
}

impl InputPage {
    fn new(state: Option<&str>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let qq = field("请输入qq号", InputKind::Number, cx);
        let phone = field("请输入手机号", InputKind::Tel, cx);
        let code = field("请输入验证码", InputKind::Number, cx);
        let card = field("请输入卡号", InputKind::Number, cx);
        card.update(cx, |input, cx| input.set_text("weui input error", cx));
        let text = field("请输入文本", InputKind::Text, cx);
        let number = field("请输入号码", InputKind::Number, cx);
        let textarea = cx.new(|cx| {
            let mut area = TextArea::new("请输入文本", cx);
            area.set_max_len(Some(200), cx);
            area.set_show_count(true, cx);
            area
        });
        let vcode = cx.new(|cx| {
            let mut button = VcodeButton::new(cx);
            button.set_plain(true, cx);
            button
        });
        for (input, label) in [
            (&qq, "qq"),
            (&phone, "手机号"),
            (&code, "验证码"),
            (&card, "卡号"),
            (&number, "号码"),
        ] {
            input.update(cx, |input, cx| input.set_aria_label(label, cx));
        }
        cx.observe(&textarea, |_, _, cx| cx.notify()).detach();
        let menus = [Presence::new(cx), Presence::new(cx), Presence::new(cx)];
        let now = today();
        let date_picker = cx.new(|cx| {
            Picker::new("input-date-picker", cx)
                .date(1900, 2100, now)
                .title("日期")
        });
        let datetime_date = cx.new(|cx| {
            Picker::new("input-datetime-date", cx)
                .date(1900, 2100, now)
                .title("时间")
                .desc("选择日期")
        });
        let datetime_time = cx.new(|cx| {
            Picker::new("input-datetime-time", cx)
                .columns(time_columns())
                .default_value(["0", "0"])
                .title("时间")
                .desc("选择时刻")
        });
        let subscriptions = vec![
            cx.subscribe_in(&date_picker, window, |this, _, e: &PickerEvent, _, cx| {
                if let PickerEvent::Confirm(items) = e {
                    this.date = format_date(items);
                }
                cx.notify();
            }),
            cx.subscribe_in(
                &datetime_date,
                window,
                |this, _, e: &PickerEvent, window, cx| {
                    match e {
                        PickerEvent::Confirm(items) => this.pending_date = format_date(items),
                        PickerEvent::Close if this.pending_date.is_some() => {
                            let time = this.datetime_time.clone();
                            window.defer(cx, move |window, cx| {
                                time.update(cx, |p, cx| p.open(window, cx));
                            });
                        }
                        _ => {}
                    }
                    cx.notify();
                },
            ),
            cx.subscribe_in(&datetime_time, window, |this, _, e: &PickerEvent, _, cx| {
                match e {
                    PickerEvent::Confirm(items) => {
                        if let (Some(date), [h, m]) = (this.pending_date.take(), items.as_slice()) {
                            this.time = Some(format!("{date} {}:{}", h.label, m.label));
                        }
                    }
                    PickerEvent::Close => this.pending_date = None,
                    _ => {}
                }
                cx.notify();
            }),
        ];
        match state {
            Some("focused") => text.update(cx, |input, cx| input.focus(window, cx)),
            Some("typed") => {
                qq.update(cx, |input, cx| input.set_text("12345678", cx));
                phone.update(cx, |input, cx| input.set_text("13800138000", cx));
                text.update(cx, |input, cx| {
                    input.set_text("weui input", cx);
                    input.focus(window, cx);
                });
                textarea.update(cx, |area, cx| {
                    area.set_text("WeUI 是一套同微信原生视觉体验一致的基础样式库，由微信官方设计团队为微信内网页和微信小程序量身设计。", cx)
                });
            }
            _ => {}
        }
        Self {
            radio: Some(1),
            checks: [true, false],
            switches: [false, true],
            agree: false,
            selected: [0, 0, 0],
            menus,
            qq,
            phone,
            date: None,
            time: None,
            date_picker,
            datetime_date,
            datetime_time,
            pending_date: None,
            _subscriptions: subscriptions,
            code,
            card,
            text,
            textarea,
            number,
            vcode,
        }
    }

    fn row(id: &'static str, label: &'static str, body: impl IntoElement) -> Cell {
        Cell::new(id)
            .aria_label(label)
            .hd(div().w(px(LABEL_WIDTH)).child(label))
            .bd(body)
    }

    fn picker_field(
        &self,
        id: &'static str,
        label: &'static str,
        value: Option<String>,
        placeholder: &'static str,
        picker: Entity<Picker>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let t = Theme::get(cx);
        let brand = t.brand;
        let shown: SharedString = value.clone().unwrap_or_else(|| placeholder.into()).into();
        let color = if value.is_some() { t.fg_0 } else { t.fg_2 };
        div()
            .id(id)
            .role(Role::Button)
            .aria_label(SharedString::from(format!(
                "{label}，{}",
                value.as_deref().unwrap_or("未选择")
            )))
            .focusable()
            .tab_stop(true)
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_between()
            .rounded(px(4.))
            .text_color(color)
            .focus_visible(move |s| s.bg(brand.opacity(0.1)))
            .on_click(move |_, window, cx| {
                picker.update(cx, |p, cx| p.open(window, cx));
            })
            .child(shown)
            .child(calendar_glyph(t.fg_0))
            .reveal_focus()
    }

    fn select_menu(&self, ix: usize, cx: &mut Context<Self>) -> impl IntoElement {
        SelectMenu::new(("input-select-menu", ix), &self.menus[ix])
            .options(SELECTS[ix].iter().copied())
            .selected(Some(self.selected[ix]))
            .on_select({
                let entity = cx.entity();
                move |choice, window, cx| {
                    let choice = *choice;
                    entity.update(cx, |this, cx| {
                        this.selected[ix] = choice;
                        this.menus[ix].hide(window, cx);
                        cx.notify();
                    });
                }
            })
            .on_dismiss({
                let entity = cx.entity();
                move |window, cx| {
                    entity.update(cx, |this, cx| {
                        this.menus[ix].hide(window, cx);
                        cx.notify();
                    });
                }
            })
    }

    fn open_select(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.menus[ix].show(window, cx);
        cx.notify();
    }
}

fn time_columns() -> Vec<Vec<PickerItem>> {
    let two = |n: u32| PickerItem::new(format!("{n:02}"), n.to_string());
    vec![(0..24).map(two).collect(), (0..60).map(two).collect()]
}

fn format_date(items: &[PickerItem]) -> Option<String> {
    let [y, m, d] = items else { return None };
    let n = |i: &PickerItem| i.value.parse::<u32>().ok();
    Some(format!("{}-{:02}-{:02}", y.value, n(m)?, n(d)?))
}

fn calendar_glyph(color: Hsla) -> Div {
    div()
        .flex_shrink_0()
        .size(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(13.))
                .h(px(13.))
                .rounded(px(1.5))
                .border_1()
                .border_t(px(3.5))
                .border_color(color),
        )
}

impl Render for InputPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let radios = RadioGroup::new("input-radio")
            .option("cell standard")
            .option("cell standard")
            .selected(self.radio)
            .on_change({
                let entity = entity.clone();
                move |ix, _, cx| {
                    let ix = *ix;
                    entity.update(cx, |this, cx| {
                        this.radio = Some(ix);
                        cx.notify();
                    })
                }
            })
            .rows(window, cx);

        let checkbox = |ix: usize, label: &'static str| {
            let entity = entity.clone();
            CheckboxCell::new(("input-check", ix), label)
                .checked(self.checks[ix])
                .on_change(move |value, _, cx| {
                    let value = *value;
                    entity.update(cx, |this, cx| {
                        this.checks[ix] = value;
                        cx.notify();
                    })
                })
        };

        let switch_row = |ix: usize, label: &'static str| {
            let entity = entity.clone();
            Cell::new(("input-switch", ix))
                .padding(Edges {
                    top: px(12.),
                    bottom: px(12.),
                    left: px(16.),
                    right: px(16.),
                })
                .bd(label)
                .ft(Switch::new(("input-switch-control", ix))
                    .label(label)
                    .checked(self.switches[ix])
                    .on_change(move |value, _, cx| {
                        let value = *value;
                        entity.update(cx, |this, cx| {
                            this.switches[ix] = value;
                            cx.notify();
                        })
                    }))
        };

        let vcode_padding = Edges {
            top: px(0.),
            bottom: px(0.),
            left: px(16.),
            right: px(0.),
        };

        let select = |ix: usize, cx: &mut Context<Self>| {
            let entity = cx.entity();
            move |_: &ClickEvent, window: &mut Window, cx: &mut App| {
                entity.update(cx, |this, cx| this.open_select(ix, window, cx))
            }
        };

        let agree_entity = entity.clone();

        PageFrame::new("input")
            .title("Input")
            .desc("表单输入")
            .child(
                Cells::new()
                    .title("单选列表项")
                    .role(Role::RadioGroup)
                    .aria_label("单选列表项")
                    .children(radios)
                    .child(
                        Cell::new("input-radio-more")
                            .link()
                            .title("添加更多")
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title("复选列表项")
                    .child(checkbox(0, "standard is dealt for u."))
                    .child(checkbox(1, "standard is dealicient for u."))
                    .child(
                        Cell::new("input-check-more")
                            .link()
                            .title("添加更多")
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title("表单")
                    .tips("底部说明文字底部说明文字")
                    .child(Self::row("input-qq", "qq", self.qq.clone()))
                    .child(
                        Self::row("input-phone", "手机号", self.phone.clone())
                            .padding(vcode_padding)
                            .ft(self.vcode.clone()),
                    )
                    .child(Self::row(
                        "input-date",
                        "日期",
                        self.picker_field(
                            "input-date-field",
                            "日期",
                            self.date.clone(),
                            "年/月/日",
                            self.date_picker.clone(),
                            cx,
                        ),
                    ))
                    .child(Self::row(
                        "input-time",
                        "时间",
                        self.picker_field(
                            "input-time-field",
                            "时间",
                            self.time.clone(),
                            "年/月/日 --:--",
                            self.datetime_date.clone(),
                            cx,
                        ),
                    ))
                    .child(
                        Self::row("input-code", "验证码", self.code.clone())
                            .padding(vcode_padding)
                            .ft(VcodeImage::new("vcode.jpg")),
                    ),
            )
            .child(
                Cells::new()
                    .title("表单报错")
                    .child(Self::row("input-card", "卡号", self.card.clone()).warn(true)),
            )
            .child(
                Cells::new()
                    .title("开关")
                    .child(switch_row(0, "标题文字"))
                    .child(switch_row(1, "兼容IE Edge的版本")),
            )
            .child(
                Cells::new()
                    .title("文本框")
                    .child(Cell::new("input-text").bd(self.text.clone())),
            )
            .child(
                Cells::new()
                    .title("文本域")
                    .child(Cell::new("input-textarea").bd(self.textarea.clone())),
            )
            .child(
                Cells::new().title("选择").child(
                    SelectCell::new("input-select-before", SELECTS[0][self.selected[0]])
                        .label_width(LABEL_WIDTH)
                        .expanded(self.menus[0].is_open())
                        .before(self.number.clone())
                        .on_click(select(0, cx)),
                ),
            )
            .child(
                Cells::new()
                    .title("选择")
                    .child(
                        SelectCell::new("input-select-plain", SELECTS[1][self.selected[1]])
                            .expanded(self.menus[1].is_open())
                            .on_click(select(1, cx)),
                    )
                    .child(
                        SelectCell::new("input-select-after", SELECTS[2][self.selected[2]])
                            .label_width(LABEL_WIDTH)
                            .after("国家/地区")
                            .expanded(self.menus[2].is_open())
                            .on_click(select(2, cx)),
                    ),
            )
            .child(
                div().h(px(25.6)).pt(px(6.)).child(
                    Agree::new("input-agree", "阅读并同意")
                        .link("《相关条款》", |_, _, _| {})
                        .checked(self.agree)
                        .on_change(move |value, _, cx| {
                            let value = *value;
                            agree_entity.update(cx, |this, cx| {
                                this.agree = value;
                                cx.notify();
                            })
                        }),
                ),
            )
            .child(
                div()
                    .mt(px(48.))
                    .mx(px(16.))
                    .mb(px(8.))
                    .flex()
                    .justify_center()
                    .child(Button::new("确定").id("showTooltips").primary()),
            )
            .child(self.select_menu(0, cx))
            .child(self.select_menu(1, cx))
            .child(self.select_menu(2, cx))
            .child(self.date_picker.clone())
            .child(self.datetime_date.clone())
            .child(self.datetime_time.clone())
    }
}
