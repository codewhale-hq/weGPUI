use gpui::{prelude::*, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesktopIconKind {
    Chat,
    Contacts,
    Favorites,
    Discover,
    Phone,
    More,
    Search,
    Add,
    Close,
    ChevronDown,
    ChevronRight,
    Back,
    Check,
    Emoji,
    Folder,
    Scissors,
    Microphone,
    Video,
    History,
    Settings,
    Bell,
    Keyboard,
    File,
    Link,
    Photo,
    Play,
    Download,
    Pin,
    Mute,
    Reply,
    Cube,
    Send,
    Lock,
    Info,
    Calendar,
}

impl DesktopIconKind {
    fn geometry(self) -> &'static str {
        match self {
            Self::Chat => {
                "<path d='M20 10.5a8 7 0 0 1-8 7H9l-5 3 1-5a7 7 0 0 1-2-5 8 7 0 0 1 8-7h1a8 7 0 0 1 8 7Z'/><path d='M8 10h.01M12 10h.01M16 10h.01'/>"
            }
            Self::Contacts => {
                "<circle cx='10' cy='7' r='3.5'/><path d='M3 20v-2c0-3 3-5 7-5s7 2 7 5v2ZM18 6h4M19 10h3M20 14h2'/>"
            }
            Self::Favorites => "<path d='m12 3 9 5v9l-9 5-9-5V8Zm0 10v9M3 8l9 5 9-5'/>",
            Self::Discover => "<circle cx='12' cy='12' r='9'/><path d='m15.5 8.5-2 5-5 2 2-5Z'/>",
            Self::Phone => {
                "<path d='m7 3 3 5-3 2c2 4 3 5 7 7l2-3 5 3-1 4C10 23 1 14 3 4Z'/><path d='M15 3a7 7 0 0 1 6 6'/>"
            }
            Self::More => {
                "<circle cx='5' cy='12' r='.8'/><circle cx='12' cy='12' r='.8'/><circle cx='19' cy='12' r='.8'/>"
            }
            Self::Search => "<circle cx='10' cy='10' r='6.5'/><path d='m15 15 6 6'/>",
            Self::Add => "<path d='M12 4v16M4 12h16'/>",
            Self::Close => "<path d='m5 5 14 14M19 5 5 19'/>",
            Self::ChevronDown => "<path d='m5 9 7 7 7-7'/>",
            Self::ChevronRight => "<path d='m9 5 7 7-7 7'/>",
            Self::Back => "<path d='m15 5-7 7 7 7'/>",
            Self::Check => "<path d='m4 12 5 5L20 6'/>",
            Self::Emoji => {
                "<circle cx='12' cy='12' r='9'/><path d='M8 9h.01M16 9h.01M7.5 14a5 5 0 0 0 9 0'/>"
            }
            Self::Folder => "<path d='M3 7V5h7l2 3h9v12H3Zm0 5h18'/>",
            Self::Scissors => {
                "<circle cx='6' cy='18' r='3'/><circle cx='18' cy='18' r='3'/><path d='m4 2 12 13M20 2 8 15'/>"
            }
            Self::Microphone => {
                "<rect x='9' y='3' width='6' height='12' rx='3'/><path d='M6 11v1a6 6 0 0 0 12 0v-1M12 18v4M8 22h8'/>"
            }
            Self::Video => {
                "<rect x='3' y='5' width='12' height='14' rx='2'/><path d='m15 9 6-3v12l-6-3'/>"
            }
            Self::History => "<path d='M3 9a9 9 0 1 1 0 7M3 3v6h6M12 6v6l4 2'/>",
            Self::Settings => {
                "<path d='m9 3-1 3-3 1-2 4 2 2 1 4 3 1 2 3 4-1 1-3 3-1 2-4-2-2-1-4-3-1-2-3Z'/><circle cx='12' cy='12' r='3'/>"
            }
            Self::Bell => "<path d='M5 17h14l-2-3V9a5 5 0 0 0-10 0v5ZM10 21h4'/>",
            Self::Keyboard => {
                "<rect x='2' y='5' width='20' height='14' rx='2'/><path d='M6 9h.01M10 9h.01M14 9h.01M18 9h.01M6 12h.01M10 12h.01M14 12h.01M18 12h.01M7 16h10'/>"
            }
            Self::File => "<path d='M5 2h9l5 5v15H5Zm9 0v6h5M8 13h8M8 17h6'/>",
            Self::Link => {
                "<path d='m10 8 3-3a5 5 0 0 1 7 7l-3 3M14 16l-3 3a5 5 0 0 1-7-7l3-3M8 16l8-8'/>"
            }
            Self::Photo => {
                "<rect x='3' y='3' width='18' height='18' rx='2'/><circle cx='8' cy='8' r='2'/><path d='m3 18 6-6 4 4 3-4 5 6'/>"
            }
            Self::Play => "<path d='m8 4 12 8-12 8Z'/>",
            Self::Download => "<path d='M12 3v12m-5-5 5 5 5-5M4 17v4h16v-4'/>",
            Self::Pin => "<path d='m8 3 9 1-2 7 3 3-5 1-4 6v-7l-4-3 3-2Z'/>",
            Self::Mute => "<path d='M4 9h4l5-5v16l-5-5H4ZM17 9l5 6m0-6-5 6'/>",
            Self::Reply => "<path d='m9 5-6 6 6 6v-5c6 0 10 2 12 7 0-9-5-11-12-11Z'/>",
            Self::Cube => "<path d='m12 2 9 5v10l-9 5-9-5V7Zm0 10v10M3 7l9 5 9-5'/>",
            Self::Send => "<path d='m3 3 19 9-19 9 4-9Zm4 9h15'/>",
            Self::Lock => {
                "<rect x='5' y='10' width='14' height='11' rx='2'/><path d='M8 10V6a4 4 0 0 1 8 0v4M12 14v3'/>"
            }
            Self::Info => "<circle cx='12' cy='12' r='9'/><path d='M12 11v6M12 7h.01'/>",
            Self::Calendar => {
                "<rect x='3' y='5' width='18' height='16' rx='2'/><path d='M7 2v6M17 2v6M3 10h18M7 14h2M13 14h2M7 18h2'/>"
            }
        }
    }
}

#[derive(IntoElement)]
pub struct DesktopIcon {
    kind: DesktopIconKind,
    size: f32,
    color: Option<Hsla>,
}

impl DesktopIcon {
    pub fn new(kind: DesktopIconKind) -> Self {
        Self {
            kind,
            size: 22.,
            color: None,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl RenderOnce for DesktopIcon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let data = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='1.65' stroke-linecap='round' stroke-linejoin='round'>{}</svg>",
            self.kind.geometry()
        );
        svg()
            .data(data.as_bytes())
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(self.color.unwrap_or(super::DesktopPalette::get(cx).text))
    }
}
