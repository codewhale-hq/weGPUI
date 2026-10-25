use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use wegpui::{
    components::{
        Cell, Cells, Presence,
        agree::Agree,
        button::Button,
        checkbox::CheckboxCell,
        date_field::{DateField, DateFieldMode},
        radio::RadioGroup,
        select::{SelectCell, SelectMenu},
        switch::Switch,
        textarea::TextArea,
        vcode::{VcodeButton, VcodeImage},
    },
    input::{InputKind, TextInput},
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
    date: Entity<DateField>,
    time: Entity<DateField>,
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
        let qq = field(tr("请输入qq号"), InputKind::Number, cx);
        let phone = field(tr("请输入手机号"), InputKind::Tel, cx);
        let code = field(tr("请输入验证码"), InputKind::Number, cx);
        let card = field(tr("请输入卡号"), InputKind::Number, cx);
        card.update(cx, |input, cx| input.set_text("weui input error", cx));
        let text = field(tr("请输入文本"), InputKind::Text, cx);
        let number = field(tr("请输入号码"), InputKind::Number, cx);
        let textarea = cx.new(|cx| {
            let mut area = TextArea::new(tr("请输入文本"), cx);
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
            (&phone, tr("手机号")),
            (&code, tr("验证码")),
            (&card, tr("卡号")),
            (&number, tr("号码")),
        ] {
            input.update(cx, |input, cx| input.set_aria_label(label, cx));
        }
        cx.observe(&textarea, |_, _, cx| cx.notify()).detach();
        let menus = [Presence::new(cx), Presence::new(cx), Presence::new(cx)];
        let date = cx.new(|cx| {
            let mut field = DateField::new("input-date-field", tr("日期"), DateFieldMode::Date, cx);
            field.set_typed_entry(true, cx);
            field
        });
        let time = cx.new(|cx| {
            let mut field =
                DateField::new("input-time-field", tr("时间"), DateFieldMode::DateTime, cx);
            field.set_typed_entry(true, cx);
            field
        });
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
                    area.set_text(tr("WeUI 是一套同微信原生视觉体验一致的基础样式库，由微信官方设计团队为微信内网页和微信小程序量身设计。"), cx)
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
            date,
            time,
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

    fn select_menu(&self, ix: usize, cx: &mut Context<Self>) -> impl IntoElement {
        SelectMenu::new(("input-select-menu", ix), &self.menus[ix])
            .options(SELECTS[ix].iter().copied().map(tr))
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
            .desc(tr("表单输入"))
            .child(
                Cells::new()
                    .title(tr("单选列表项"))
                    .role(Role::RadioGroup)
                    .aria_label(tr("单选列表项"))
                    .children(radios)
                    .child(
                        Cell::new("input-radio-more")
                            .link()
                            .title(tr("添加更多"))
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("复选列表项"))
                    .child(checkbox(0, "standard is dealt for u."))
                    .child(checkbox(1, "standard is dealicient for u."))
                    .child(
                        Cell::new("input-check-more")
                            .link()
                            .title(tr("添加更多"))
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("表单"))
                    .tips(tr("底部说明文字底部说明文字"))
                    .child(Self::row("input-qq", "qq", self.qq.clone()))
                    .child(
                        Self::row("input-phone", tr("手机号"), self.phone.clone())
                            .padding(vcode_padding)
                            .ft(self.vcode.clone()),
                    )
                    .child(Self::row("input-date", tr("日期"), self.date.clone()))
                    .child(Self::row("input-time", tr("时间"), self.time.clone()))
                    .child(
                        Self::row("input-code", tr("验证码"), self.code.clone())
                            .padding(vcode_padding)
                            .ft(VcodeImage::new("vcode.jpg")),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("表单报错"))
                    .child(Self::row("input-card", tr("卡号"), self.card.clone()).warn(true)),
            )
            .child(
                Cells::new()
                    .title(tr("开关"))
                    .child(switch_row(0, tr("标题文字")))
                    .child(switch_row(1, tr("兼容IE Edge的版本"))),
            )
            .child(
                Cells::new()
                    .title(tr("文本框"))
                    .child(Cell::new("input-text").bd(self.text.clone())),
            )
            .child(
                Cells::new()
                    .title(tr("文本域"))
                    .child(Cell::new("input-textarea").bd(self.textarea.clone())),
            )
            .child(
                Cells::new().title(tr("选择")).child(
                    SelectCell::new("input-select-before", tr(SELECTS[0][self.selected[0]]))
                        .label_width(LABEL_WIDTH)
                        .expanded(self.menus[0].is_open())
                        .before(self.number.clone())
                        .on_click(select(0, cx)),
                ),
            )
            .child(
                Cells::new()
                    .title(tr("选择"))
                    .child(
                        SelectCell::new("input-select-plain", tr(SELECTS[1][self.selected[1]]))
                            .expanded(self.menus[1].is_open())
                            .on_click(select(1, cx)),
                    )
                    .child(
                        SelectCell::new("input-select-after", tr(SELECTS[2][self.selected[2]]))
                            .label_width(LABEL_WIDTH)
                            .after(tr("国家/地区"))
                            .expanded(self.menus[2].is_open())
                            .on_click(select(2, cx)),
                    ),
            )
            .child(
                div().h(px(25.6)).pt(px(6.)).child(
                    Agree::new("input-agree", tr("阅读并同意"))
                        .link(tr("《相关条款》"), |_, _, _| {})
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
                    .child(Button::new(tr("确定")).id("showTooltips").primary()),
            )
            .child(self.select_menu(0, cx))
            .child(self.select_menu(1, cx))
            .child(self.select_menu(2, cx))
    }
}
