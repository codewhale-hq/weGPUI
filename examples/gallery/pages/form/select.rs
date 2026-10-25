use super::{dismiss, input, primary_button, show_toast, toast, toast_presence};
use crate::chrome::PageFrame;
use gpui::{prelude::*, *};
use wegpui::{
    components::{
        form::{CellKind, Form, FormCell, FormGroup, cell_control, cell_control_flex},
        modal::Presence,
        picker::{Picker, PickerEvent, PickerItem, today},
        select::{SelectCell, SelectMenu},
    },
    input::{InputKind, TextInput},
};

const W: f32 = 17. * 314. / 100.;

struct Menu {
    title: &'static str,
    options: Vec<SharedString>,
    disabled: Option<usize>,
}

pub struct Select {
    route: &'static str,
    number: Entity<TextInput>,
    mobile: Entity<TextInput>,
    values: [Option<usize>; 3],
    open: Option<usize>,
    presence: Presence,
    toast: Presence,
    pickers: Vec<Entity<Picker>>,
    date_label: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl Select {
    pub fn new(
        route: &'static str,
        state: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let primary = route == "form_select_primary";
        let mut pickers = vec![];
        let mut subscriptions = vec![];
        if primary {
            let (year, month, day) = today();
            let items = |labels: &[&str], disabled: Option<usize>| -> Vec<PickerItem> {
                labels
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        PickerItem::new(l.to_string(), i.to_string()).disabled(disabled == Some(i))
                    })
                    .collect()
            };
            let phone = items(&["+86", "+80", "+84", "+87"], None);
            let ticket = items(
                &["飞机票", "火车票", "的士票", "公交票 (disabled)", "其他"],
                Some(3),
            );
            pickers.push(cx.new(|cx| {
                Picker::new("form-date", cx)
                    .date(1990, year, (year, month, day))
                    .title("多列选择器")
            }));
            pickers.push(cx.new(|cx| Picker::single("form-phone", phone, cx).title("单列选择器")));
            pickers
                .push(cx.new(|cx| Picker::single("form-ticket", ticket, cx).title("单列选择器")));
            for (which, picker) in pickers.iter().enumerate() {
                subscriptions.push(cx.subscribe(
                    picker,
                    move |this: &mut Self, picker, e: &PickerEvent, cx| {
                        if let PickerEvent::Confirm(items) = e {
                            if which == 0 {
                                let label: Vec<String> =
                                    items.iter().map(|i| i.label.to_string()).collect();
                                this.date_label = Some(label.join(""));
                            } else {
                                this.values[which] = picker.read(cx).indices().first().copied();
                            }
                        }
                        cx.notify();
                    },
                ));
            }
        }
        let mut this = Self {
            route,
            number: input("请输入号码", InputKind::Number, "12345678907", cx),
            mobile: input("请输入号码", InputKind::Number, "12345678907", cx),
            values: if primary {
                [None, Some(0), Some(2)]
            } else {
                [Some(0), Some(0), Some(0)]
            },
            open: None,
            presence: Presence::new(cx),
            toast: toast_presence(cx),
            pickers,
            _subscriptions: subscriptions,
            date_label: None,
        };
        let open = match state.as_deref() {
            Some("select1") => Some(0),
            Some("select2") => Some(1),
            Some("select3") => Some(2),
            Some("toast") => {
                this.toast.show_instantly(window, cx);
                None
            }
            _ => None,
        };
        if let Some(i) = open {
            if let Some(picker) = this.pickers.get(i).cloned() {
                window.defer(cx, move |window, cx| {
                    picker.update(cx, |p, cx| p.open_instantly(window, cx));
                });
            } else {
                this.open = Some(i);
                this.presence.show_instantly(window, cx);
            }
        }
        this
    }

    fn menu(&self, which: usize) -> Menu {
        let primary = self.route == "form_select_primary";
        let list = |items: &[&str]| {
            items
                .iter()
                .map(|s| SharedString::from(s.to_string()))
                .collect()
        };
        match (primary, which) {
            (false, 0) => Menu {
                title: "账号类型",
                options: list(&["微信号", "QQ号", "Email"]),
                disabled: None,
            },
            (_, 1) => Menu {
                title: if primary { "单列选择器" } else { "区号" },
                options: list(&["+86", "+80", "+84", "+87"]),
                disabled: None,
            },
            (false, _) => Menu {
                title: "国家",
                options: list(&["中国", "美国", "英国"]),
                disabled: None,
            },
            (true, 0) => {
                let now = 2026;
                Menu {
                    title: "多列选择器",
                    options: (1990..=now)
                        .map(|y| SharedString::from(format!("{y}年")))
                        .collect(),
                    disabled: None,
                }
            }
            (true, _) => Menu {
                title: "单列选择器",
                options: list(&["飞机票", "火车票", "的士票", "公交票 (disabled)", "其他"]),
                disabled: Some(3),
            },
        }
    }

    fn value(&self, which: usize) -> SharedString {
        match self.values[which] {
            Some(i) => self.menu(which).options[i].clone(),
            None => self
                .date_label
                .clone()
                .map(SharedString::from)
                .unwrap_or("日期".into()),
        }
    }

    fn open(&mut self, which: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(picker) = self.pickers.get(which).cloned() {
            picker.update(cx, |p, cx| p.open(window, cx));
            return;
        }
        self.open = Some(which);
        self.presence.show(window, cx);
        cx.notify();
    }
}

impl Render for Select {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let primary = self.route == "form_select_primary";
        let expanded = |i: usize| match self.pickers.get(i) {
            Some(p) => p.read(cx).is_open(),
            None => self.open == Some(i) && self.presence.is_open(),
        };
        let group = FormGroup::new("group-1")
            .first()
            .row(
                SelectCell::new("showDatePicker", self.value(0))
                    .expanded(expanded(0))
                    .on_click(cx.listener(|this, _, w, cx| this.open(0, w, cx))),
            )
            .row(
                SelectCell::new("showPhone", self.value(1))
                    .label_width(W)
                    .expanded(expanded(1))
                    .on_click(cx.listener(|this, _, w, cx| this.open(1, w, cx)))
                    .before(self.number.clone()),
            )
            .row(
                SelectCell::new("showPicker", self.value(2))
                    .after(if primary { "票种" } else { "国家" })
                    .label_width(W)
                    .expanded(expanded(2))
                    .on_click(cx.listener(|this, _, w, cx| this.open(2, w, cx))),
            )
            .row(
                FormCell::new("cell-mobile")
                    .kind(CellKind::Wrap)
                    .label("手机号")
                    .label_width(W)
                    .label_for(&self.mobile)
                    .child(cell_control("+86"))
                    .child(cell_control_flex(self.mobile.clone())),
            );
        let mut form = Form::new("weui-form")
            .title(if primary {
                "模拟选择框"
            } else {
                "原生选择框"
            })
            .child(group);
        if primary {
            form = form.desc("用于丰富原生选择框结构，使其更具有扩展性");
        } else {
            form = form.opr_area(primary_button(
                "showTooltips",
                "确定",
                false,
                cx.listener(|this, _, window, cx| show_toast(this, window, cx, |t| &mut t.toast)),
            ));
        }
        let menu = self.open.map(|which| {
            let m = self.menu(which);
            let mut menu = SelectMenu::new("form-select-menu", &self.presence)
                .title(m.title)
                .selected(self.values[which]);
            for (i, option) in m.options.into_iter().enumerate() {
                menu = if m.disabled == Some(i) {
                    menu.disabled_option(option)
                } else {
                    menu.option(option)
                };
            }
            menu.on_select(cx.listener(move |this, i: &usize, window, cx| {
                this.values[which] = Some(*i);
                this.presence.hide(window, cx);
                cx.notify();
            }))
            .on_dismiss(dismiss(cx, |this, window, cx| {
                this.presence.hide(window, cx);
                cx.notify();
            }))
        });
        PageFrame::new(self.route)
            .child(form)
            .fixed(toast(&self.toast))
            .children(menu.map(|m| div().child(m)))
            .children(self.pickers.iter().map(|p| div().child(p.clone())))
    }
}
