use gpui::{prelude::*, *};
use wegpui::components::Size;
use wegpui::{Assets, Theme, ThemeMode, components::*, input::TextInput};

const FAMILIES: &[(&str, &str)] = &[
    ("button", "Button"),
    ("list", "List"),
    ("input", "Input"),
    ("badge", "Badge"),
    ("searchbar", "Search Bar"),
    ("loading", "Loading"),
    ("loadmore", "Loadmore"),
    ("toast", "Toast"),
    ("dialog", "Dialog"),
    ("actionsheet", "Actionsheet"),
];
struct Gallery {
    page: usize,
    search: Entity<SearchBar>,
    input: Entity<TextInput>,
    overlay: Entity<Overlay>,
    focus: FocusHandle,
    checked: bool,
    radio: usize,
}
impl Gallery {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Theme::observe(window);
        Self {
            page: 0,
            search: cx.new(|cx| SearchBar::new(window, cx)),
            input: cx.new(|cx| TextInput::new("请输入文本", cx)),
            overlay: cx.new(Overlay::new),
            focus: cx.focus_handle(),
            checked: true,
            radio: 0,
        }
    }
    fn show(&mut self, content: OverlayContent, w: &mut Window, cx: &mut Context<Self>) {
        self.overlay
            .update(cx, |overlay, cx| overlay.open(content, w, cx));
    }
    fn page(&self, cx: &mut Context<Self>) -> AnyElement {
        let t = Theme::get(cx).clone();
        match FAMILIES[self.page].0{
            "button"=>div().flex().flex_col().items_center().gap(px(16.)).pt(px(30.)).pb(px(40.)).children([ButtonKind::Primary,ButtonKind::Default,ButtonKind::Warn].into_iter().enumerate().flat_map(|(i,kind)|{
                let label=match kind{ButtonKind::Primary=>"主要操作",ButtonKind::Default=>"次要操作",ButtonKind::Warn=>"警示操作"};
                [Button::new(label).id(("button",i*4)).kind(kind),Button::new("").id(("button",i*4+1)).kind(kind).loading(true),Button::new(label).id(("button",i*4+2)).kind(kind).loading(true),Button::new(label).id(("button",i*4+3)).kind(kind).disabled(true)]
            })).child(Button::new("medium 按钮").id("medium").primary().size(Size::Medium)).child(Button::new("按钮").id("mini").primary().size(Size::Mini)).into_any_element(),
            "list"=>div().child(Cells::new().title("带说明的列表项").child(Cell::new("标题文字").id("list1").footer("说明文字"))).child(Cells::new().title("带跳转的列表项").child(Cell::new("标题文字").id("list2").access().on_click(|_,_,_|{})).child(Cell::new("标题文字").id("list3").access().footer("说明文字").on_click(|_,_,_|{}))).child(Cells::new().title("链接列表").child(Cell::new("查看更多").link().on_click(|_,_,_|{}))).into_any_element(),
            "input"=>div().child(Cells::new().title("单选列表项").children((0..2).map(|i|Cell::new("cell standard").id(("radio-cell",i)).footer(Radio::new(("radio",i)).checked(self.radio==i).on_change(cx.listener(move|s,_,_,cx|{s.radio=i;cx.notify();}))))))
                .child(Cells::new().title("复选列表项").child(Cell::new("standard is dealt for u.").header(Checkbox::new("checkbox").checked(self.checked).on_change(cx.listener(|s,v,_,cx|{s.checked=*v;cx.notify();})))))
                .child(Cells::new().title("表单").child(Cell::new("文本").body(div().flex().gap(px(16.)).child(div().w(px(80.)).child("文本")).child(self.input.clone()))).child(Cell::new("开关").footer(Switch::new("switch").checked(self.checked).on_change(cx.listener(|s,v,_,cx|{s.checked=*v;cx.notify();}))))).into_any_element(),
            "badge"=>Cells::new().title("新消息提示跟摘要信息后，统一在列表右侧").child(Cell::new("单行列表").footer(Badge::dot()).access()).child(Cell::new("未读数红点").footer(Badge::count(8)).access()).child(Cell::new("未读数红点").id("badge2").footer(Badge::new("99+")).access()).into_any_element(),
            "searchbar"=>self.search.clone().into_any_element(),
            "loading"=>div().flex().items_center().justify_center().gap(px(24.)).child(Loading::new()).child(Loading::new().size(32.)).child(Loading::new().size(40.).color(t.fg_1)).into_any_element(),
            "loadmore"=>div().child(LoadMore::new("正在加载")).child(LoadMore::new("暂无数据").finished()).child(LoadMore::new("").dot()).into_any_element(),
            "dialog"=>div().flex().flex_col().items_center().gap(px(16.)).child(Button::new("iOS Dialog样式一").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::Dialog(Dialog::new("弹窗标题","弹窗内容，告知当前状态、信息和解决方法，描述文字尽量控制在三行内").actions(["辅助操作","主操作"])),w,cx)))).child(Button::new("iOS Dialog样式二").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::Dialog(Dialog::new("","弹窗内容，告知当前状态、信息和解决方法")),w,cx)))).into_any_element(),
            "actionsheet"=>div().flex().justify_center().child(Button::new("iOS ActionSheet").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::ActionSheet(ActionSheet::new("这是一个标题，可以为一行或者两行。",["示例菜单","示例菜单","示例菜单","示例菜单"])),w,cx)))).into_any_element(),
            _=>div().flex().flex_col().items_center().gap(px(16.)).child(Button::new("成功提示").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::Toast(Toast::new("已完成")),w,cx)))).child(Button::new("加载中提示").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::Toast(Toast::new("加载中").loading()),w,cx)))).child(Button::new("文字提示").on_click(cx.listener(|s,_,w,cx|s.show(OverlayContent::Toast(Toast::new("文字提示").text_only()),w,cx)))).into_any_element(),
        }
    }
}
impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let body = self.page(cx);
        div()
            .id("gallery")
            .track_focus(&self.focus)
            .size_full()
            .font_family(".AppleSystemUIFont")
            .text_color(t.fg_0)
            .bg(t.bg_0)
            .flex()
            .on_key_down(|e, w, cx| {
                if e.keystroke.key == "tab" {
                    if e.keystroke.modifiers.shift {
                        w.focus_prev(cx)
                    } else {
                        w.focus_next(cx)
                    }
                    w.prevent_default();
                }
            })
            .child(
                div()
                    .id("families")
                    .w(px(230.))
                    .h_full()
                    .overflow_y_scroll()
                    .bg(t.bg_2)
                    .border_r(px(0.5))
                    .border_color(t.fg_3)
                    .child(div().p(px(24.)).text_size(px(24.)).child("weGPUI"))
                    .children(FAMILIES.iter().enumerate().map(|(i, (_, title))| {
                        Cell::new(*title)
                            .id(("family", i))
                            .access()
                            .on_click(cx.listener(move |s, _, _, cx| {
                                s.page = i;
                                cx.notify();
                            }))
                    })),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(56.))
                            .px(px(16.))
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .bg(t.bg_2)
                            .child(
                                Button::new(if t.is_dark { "Light" } else { "Dark" })
                                    .size(Size::Mini)
                                    .on_click(|_, _, cx| {
                                        let mode = if Theme::get(cx).is_dark {
                                            ThemeMode::Light
                                        } else {
                                            ThemeMode::Dark
                                        };
                                        Theme::set_mode(mode, cx);
                                    }),
                            )
                            .child(
                                Button::new("System")
                                    .size(Size::Mini)
                                    .on_click(|_, _, cx| Theme::set_mode(ThemeMode::System, cx)),
                            )
                            .child(Button::new("Brand").size(Size::Mini).on_click(|_, _, cx| {
                                let mut t = Theme::get(cx).clone();
                                t.brand = if t.brand == t.brand_100 {
                                    t.indigo
                                } else {
                                    t.brand_100
                                };
                                cx.set_global(t);
                                cx.refresh_windows();
                            })),
                    )
                    .child(
                        div()
                            .relative()
                            .flex_1()
                            .min_h_0()
                            .w_full()
                            .child(
                                div()
                                    .id(("page", self.page))
                                    .size_full()
                                    .overflow_y_scroll()
                                    .child(
                                        div()
                                            .p(px(40.))
                                            .child(
                                                div()
                                                    .text_size(px(20.))
                                                    .line_height(px(32.))
                                                    .font_weight(FontWeight::NORMAL)
                                                    .child(FAMILIES[self.page].1),
                                            )
                                            .child(
                                                div()
                                                    .mt(px(4.))
                                                    .text_size(px(14.))
                                                    .line_height(px(22.4))
                                                    .text_color(t.fg_1)
                                                    .child(match FAMILIES[self.page].0 {
                                                        "button" => "按钮",
                                                        "list" => "列表",
                                                        "input" => "表单输入",
                                                        "badge" => "徽章",
                                                        "searchbar" => "搜索栏",
                                                        "loading" => "加载中",
                                                        "loadmore" => "加载更多",
                                                        "dialog" => "对话框",
                                                        "actionsheet" => "弹出式菜单",
                                                        _ => "弹出式提示",
                                                    }),
                                            ),
                                    )
                                    .child(body),
                            )
                            .child(self.overlay.clone()),
                    ),
            )
    }
}
fn main() {
    gpui_platform::application().with_assets(Assets).run(|cx| {
        wegpui::init(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(660.), px(860.)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("weGPUI Gallery".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |w, cx| cx.new(|cx| Gallery::new(w, cx)),
        )
        .expect("gallery window");
        cx.activate(true);
    });
}
