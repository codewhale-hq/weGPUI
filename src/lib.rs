pub mod accessibility;
pub mod assets;
pub mod components;
pub mod input;
pub mod motion;
pub mod theme;

pub use assets::Assets;
pub use theme::{Palette, Theme, ThemeMode};

pub fn init(cx: &mut gpui::App) {
    Theme::init(cx);
    input::init(cx);
    motion::sync_reduce_motion(cx);
}
