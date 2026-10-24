pub mod accessibility;
pub mod assets;
pub mod components;
pub mod input;
pub mod theme;

pub use assets::Assets;
pub use theme::{Theme, ThemeMode};

pub fn init(cx: &mut gpui::App) {
    Theme::init(cx);
    input::init(cx);
}
