mod palette;

pub use palette::Palette;

use gpui::{App, Global, Hsla, Window, WindowAppearance};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    pub is_dark: bool,
    pub care: bool,
    pub brand_override: Option<Hsla>,
    palette: Palette,
}

impl Global for Theme {}

impl Deref for Theme {
    type Target = Palette;
    fn deref(&self) -> &Palette {
        &self.palette
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::build(ThemeMode::Light, false, false, None)
    }
}

impl Theme {
    pub fn build(mode: ThemeMode, dark: bool, care: bool, brand: Option<Hsla>) -> Self {
        let mut palette = match (dark, care) {
            (false, false) => Palette::light(),
            (true, false) => Palette::dark(),
            (false, true) => Palette::care_light(),
            (true, true) => Palette::care_dark(),
        };
        if let Some(brand) = brand {
            palette.brand = brand;
            palette.steps_highlight_color = brand;
        }
        Self {
            mode,
            is_dark: dark,
            care,
            brand_override: brand,
            palette,
        }
    }
    pub fn light() -> Self {
        Self::build(ThemeMode::Light, false, false, None)
    }
    pub fn dark() -> Self {
        Self::build(ThemeMode::Dark, true, false, None)
    }
    pub fn get(cx: &App) -> &Self {
        cx.global::<Self>()
    }
    fn apply(mode: ThemeMode, care: bool, brand: Option<Hsla>, cx: &mut App) {
        let dark = match mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => matches!(
                cx.window_appearance(),
                WindowAppearance::Dark | WindowAppearance::VibrantDark
            ),
        };
        cx.set_global(Self::build(mode, dark, care, brand));
        cx.refresh_windows();
    }
    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        let t = Self::get(cx);
        let (care, brand) = (t.care, t.brand_override);
        Self::apply(mode, care, brand, cx);
    }
    pub fn set_care(care: bool, cx: &mut App) {
        let t = Self::get(cx);
        let (mode, brand) = (t.mode, t.brand_override);
        Self::apply(mode, care, brand, cx);
    }
    pub fn set_brand(brand: Option<Hsla>, cx: &mut App) {
        let t = Self::get(cx);
        let (mode, care) = (t.mode, t.care);
        Self::apply(mode, care, brand, cx);
    }
    pub fn init(cx: &mut App) {
        cx.set_global(Self::light());
        Self::set_mode(ThemeMode::System, cx);
    }
    pub fn observe(window: &mut Window) {
        window
            .observe_window_appearance(|_, cx| {
                if Self::get(cx).mode == ThemeMode::System {
                    Self::set_mode(ThemeMode::System, cx);
                }
            })
            .detach();
    }
}
