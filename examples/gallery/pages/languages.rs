use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use std::time::Duration;
use wegpui::{
    I18n,
    components::{Cell, Cells, textarea::TextArea},
    i18n::fill,
    input::{
        Down, End, Home, Left, Right, SelectAll, SelectDown, SelectEnd, SelectHome, SelectLeft,
        SelectRight, SelectUp, TextInput, Up,
    },
};

pub struct LanguagesPage {
    route: &'static str,
    inputs: Vec<(&'static str, Entity<TextInput>)>,
    area: Entity<TextArea>,
}

const TYPING: [(&str, &str); 7] = [
    ("العربية", "مرحبا بالعالم 123 hello"),
    ("עברית", "שלום עולם 2026 WeUI"),
    ("Mixed", "Hello مرحبا שלום 2026"),
    ("हिन्दी", "नमस्ते दुनिया देवनागरी"),
    ("ไทย", "สวัสดีชาวโลก ภาษาไทย"),
    ("日本語", "こんにちは世界 日本語"),
    ("한국어", "안녕하세요 세계"),
];

pub fn build(route: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let mut inputs: Vec<(&'static str, Entity<TextInput>)> = TYPING
        .iter()
        .map(|&(name, text)| {
            let input = cx.new(|cx| {
                let mut input = TextInput::new("…", cx);
                input.set_text(text, cx);
                input
            });
            (name, input)
        })
        .collect();
    let area = cx.new(|cx| {
        let mut area = TextArea::new("…", cx);
        area.set_rows(6, cx);
        area.set_aria_label("Arabic paragraph", cx);
        area.set_text(PARAGRAPHS[0], cx);
        area
    });
    inputs.push(("العربية ¶", area.read(cx).input().clone()));
    if let Some(spec) = args.state.as_deref().and_then(|s| s.strip_prefix("type:")) {
        let (index, ops) = spec.split_once(':').unwrap_or((spec, ""));
        let ops: Vec<String> = ops
            .split(',')
            .filter(|op| !op.is_empty())
            .map(str::to_string)
            .collect();
        if let Some((_, input)) = index
            .parse::<usize>()
            .ok()
            .and_then(|i| inputs.get(i))
            .cloned()
        {
            let handle = window.window_handle();
            cx.spawn(async move |cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(600))
                    .await;
                handle
                    .update(cx, |_, window, cx| {
                        input.update(cx, |input, cx| input.focus(window, cx));
                        for op in &ops {
                            let action: Box<dyn Action> = match op.as_str() {
                                "all" => Box::new(SelectAll),
                                "left" => Box::new(Left),
                                "right" => Box::new(Right),
                                "selleft" => Box::new(SelectLeft),
                                "selright" => Box::new(SelectRight),
                                "up" => Box::new(Up),
                                "down" => Box::new(Down),
                                "home" => Box::new(Home),
                                "end" => Box::new(End),
                                "selup" => Box::new(SelectUp),
                                "seldown" => Box::new(SelectDown),
                                "selhome" => Box::new(SelectHome),
                                "selend" => Box::new(SelectEnd),
                                _ => continue,
                            };
                            window.dispatch_action(action, cx);
                        }
                    })
                    .ok();
            })
            .detach();
        }
    }
    cx.new(|_| LanguagesPage {
        route,
        inputs,
        area,
    })
    .into()
}

struct Sample {
    tag: &'static str,
    name: &'static str,
    hello: &'static str,
    sentence: &'static str,
}

const SAMPLES: [Sample; 14] = [
    Sample {
        tag: "en",
        name: "English",
        hello: "Hello, world",
        sentence: "The quick brown fox jumps over the lazy dog.",
    },
    Sample {
        tag: "zh-Hans",
        name: "简体中文",
        hello: "你好，世界",
        sentence: "敏捷的棕色狐狸跳过了那只懒狗。",
    },
    Sample {
        tag: "zh-Hant",
        name: "繁體中文",
        hello: "你好，世界",
        sentence: "敏捷的棕色狐狸跳過了那隻懶狗。",
    },
    Sample {
        tag: "ja",
        name: "日本語",
        hello: "こんにちは、世界",
        sentence: "色は匂えど 散りぬるを 我が世誰ぞ 常ならむ",
    },
    Sample {
        tag: "ko",
        name: "한국어",
        hello: "안녕하세요, 세계",
        sentence: "키스의 고유조건은 입술끼리 만나야 하고 특별한 기술은 필요치 않다.",
    },
    Sample {
        tag: "es",
        name: "Español",
        hello: "Hola, mundo",
        sentence: "El veloz murciélago hindú comía feliz cardillo y kiwi.",
    },
    Sample {
        tag: "fr",
        name: "Français",
        hello: "Bonjour, le monde",
        sentence: "Portez ce vieux whisky au juge blond qui fume.",
    },
    Sample {
        tag: "de",
        name: "Deutsch",
        hello: "Hallo, Welt",
        sentence: "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich.",
    },
    Sample {
        tag: "pt",
        name: "Português",
        hello: "Olá, mundo",
        sentence: "À noite, vovô Kowalsky vê o ímã cair no pé do pinguim queixoso.",
    },
    Sample {
        tag: "ru",
        name: "Русский",
        hello: "Привет, мир",
        sentence: "Съешь же ещё этих мягких французских булок, да выпей чаю.",
    },
    Sample {
        tag: "ar",
        name: "العربية",
        hello: "مرحبًا بالعالم",
        sentence: "نص حكيم له سر قاطع وذو شأن عظيم مكتوب على ثوب أخضر ومغلف بجلد أزرق.",
    },
    Sample {
        tag: "he",
        name: "עברית",
        hello: "שלום עולם",
        sentence: "דג סקרן שט בים מאוכזב ולפתע מצא חברה.",
    },
    Sample {
        tag: "hi",
        name: "हिन्दी",
        hello: "नमस्ते दुनिया",
        sentence: "ऋषियों को सताने वाले दुष्ट राक्षसों के राजा रावण का सर्वनाश करने वाले विष्णुवतार भगवान श्रीराम।",
    },
    Sample {
        tag: "th",
        name: "ไทย",
        hello: "สวัสดีชาวโลก",
        sentence: "เป็นมนุษย์สุดประเสริฐเลิศคุณค่า กว่าบรรดาฝูงสัตว์เดรัจฉาน",
    },
];

const MIXED: [&str; 5] = [
    "Han: 直 骨 令 刃 海 (ja / zh-Hans / zh-Hant / ko forms differ)",
    "اللغة العربية (Arabic) في 2026 — weGPUI",
    "שלום, WeUI 2.6.26! — עברית (Hebrew)",
    "Latin text with عربي and עברית inside, 12345.",
    "Emoji 👩‍💻 🇯🇵 and a combining accent e\u{301}",
];

const PARAGRAPHS: [&str; 3] = [
    "تدعم مكتبة weGPUI اللغة العربية والعبرية والإنجليزية في الفقرة نفسها، مع أرقام مثل 12345 وعناوين مثل weui.io في الوسط، وتنتقل الأسطر بترتيب القراءة الصحيح.",
    "הספרייה weGPUI תומכת בעברית, בערבית ובאנגלית באותה פסקה, עם מספרים כמו 12345 וכתובות כמו weui.io באמצע, והשורות מתקפלות לפי סדר הקריאה הנכון.",
    "The library wraps a mixed paragraph in reading order: العربية في المنتصف, then עברית and 2026, before the English text carries on to a second and a third line.",
];

impl Render for LanguagesPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let i18n = I18n::get(cx);
        let active = i18n.locale().catalog_tag();
        let direction = if i18n.direction().is_rtl() {
            "RTL"
        } else {
            "LTR"
        };
        let tag = i18n.locale().tag().to_string();
        let m = i18n.messages().clone();
        let catalog: Vec<(&'static str, String)> = vec![
            ("locale", tag),
            ("direction", direction.to_string()),
            ("ok", m.ok.to_string()),
            ("cancel", m.cancel.to_string()),
            ("search", m.search.to_string()),
            ("loading", m.loading.to_string()),
            ("unavailable", m.unavailable.to_string()),
            ("get_code", m.get_code.to_string()),
            ("resend_code", fill(&m.resend_code, &[("seconds", &59)])),
            (
                "step_position",
                fill(&m.step_position, &[("n", &2), ("total", &4)]),
            ),
            (
                "months",
                m.months[..3]
                    .iter()
                    .map(|s| &**s)
                    .collect::<Vec<&str>>()
                    .join(" · "),
            ),
            ("year / day", {
                let year = fill(&m.year_label, &[("year", &2026)]);
                let day = fill(&m.day_label, &[("day", &25)]);
                m.join(&[year, day])
            }),
        ];
        PageFrame::new(self.route)
            .title("Languages")
            .desc(tr("语言和文字示例"))
            .child(
                Cells::new()
                    .title(tr("试着输入"))
                    .children(self.inputs.iter().take(TYPING.len()).enumerate().map(
                        |(i, (name, input))| {
                            Cell::new(("typing", i))
                                .title(*name)
                                .bd(div().child(input.clone()))
                        },
                    ))
                    .child(
                        Cell::new(("typing", TYPING.len()))
                            .title(self.inputs[TYPING.len()].0)
                            .bd(div().child(self.area.clone())),
                    ),
            )
            .child(
                Cells::new()
                    .title(tr("切换语言"))
                    .role(Role::ListBox)
                    .children(SAMPLES.iter().enumerate().map(|(i, sample)| {
                        let tag = sample.tag;
                        Cell::new(("language", i))
                            .role(Role::ListBoxOption)
                            .title(sample.name)
                            .value(sample.hello)
                            .desc(sample.sentence)
                            .selected(active == tag)
                            .active()
                            .on_click(move |_, window, cx| {
                                window.defer(cx, move |_, cx| crate::set_language(tag, cx))
                            })
                    })),
            )
            .child(
                Cells::new().title(tr("混合方向文本")).children(
                    MIXED
                        .iter()
                        .enumerate()
                        .map(|(i, text)| Cell::new(("mixed", i)).title(*text)),
                ),
            )
            .child(
                Cells::new().title(tr("换行的混合方向文本")).children(
                    PARAGRAPHS
                        .iter()
                        .enumerate()
                        .map(|(i, text)| Cell::new(("paragraph", i)).title(*text)),
                ),
            )
            .child(
                Cells::new().title(tr("当前语言包")).children(
                    catalog.into_iter().enumerate().map(|(i, (name, value))| {
                        Cell::new(("catalog", i)).title(name).value(value)
                    }),
                ),
            )
    }
}
