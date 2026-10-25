//! WeUI theming: light/dark appearance, care mode and brand colour over the generated
//! [`Palette`].
//!
//! [`Theme`] is a GPUI global. [`crate::init`] installs it, and components read it with
//! [`Theme::get`]. Changing the mode, care mode or brand rebuilds the palette and refreshes all
//! windows. Brand overrides replace `brand` and `steps_highlight_color` only; WeUI derives other
//! brand shades in CSS, and those remain the defaults.

mod palette;

pub use palette::Palette;

use gpui::{App, Global, Hsla, Window, WindowAppearance};
use std::ops::Deref;

/// Which appearance the [`Theme`] follows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    /// Follow the window appearance reported by the platform (dark or vibrant dark selects the
    /// dark palette). The default.
    #[default]
    System,
    /// Always use the light palette.
    Light,
    /// Always use the dark palette.
    Dark,
}

/// The active WeUI theme: a resolved [`Palette`] plus the settings that produced it.
///
/// It dereferences to [`Palette`], so `Theme::get(cx).brand` reads a token directly. Build one
/// with [`Theme::build`] for previews, or change the global with [`Theme::set_mode`],
/// [`Theme::set_care`] and [`Theme::set_brand`].
#[derive(Clone, Debug)]
pub struct Theme {
    /// The requested appearance mode.
    pub mode: ThemeMode,
    /// Whether the dark palette is in use (resolved from `mode` and the window appearance).
    pub is_dark: bool,
    /// Whether WeUI care mode (larger type, stronger contrast) is on.
    pub care: bool,
    /// Custom brand colour replacing WeUI green, if any.
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
    /// Builds a theme from explicit settings without touching the global. `dark` selects the
    /// dark palette regardless of `mode`, which is recorded for later `System` resolution.
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
    /// The light, non-care theme with the default brand.
    pub fn light() -> Self {
        Self::build(ThemeMode::Light, false, false, None)
    }
    /// The dark, non-care theme with the default brand.
    pub fn dark() -> Self {
        Self::build(ThemeMode::Dark, true, false, None)
    }
    /// The global theme. Panics if [`crate::init`] (or [`Theme::init`]) has not run.
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
    /// Sets the appearance mode, keeping care mode and brand, and refreshes all windows.
    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        let t = Self::get(cx);
        let (care, brand) = (t.care, t.brand_override);
        Self::apply(mode, care, brand, cx);
    }
    /// Turns care mode on or off, keeping mode and brand, and refreshes all windows.
    pub fn set_care(care: bool, cx: &mut App) {
        let t = Self::get(cx);
        let (mode, brand) = (t.mode, t.brand_override);
        Self::apply(mode, care, brand, cx);
    }
    /// Sets or clears the brand override, keeping mode and care mode, and refreshes all windows.
    pub fn set_brand(brand: Option<Hsla>, cx: &mut App) {
        let t = Self::get(cx);
        let (mode, care) = (t.mode, t.care);
        Self::apply(mode, care, brand, cx);
    }
    /// Installs the global theme in `System` mode. Called by [`crate::init`].
    pub fn init(cx: &mut App) {
        cx.set_global(Self::light());
        Self::set_mode(ThemeMode::System, cx);
    }
    /// Re-resolves a `System` theme whenever `window`'s appearance changes. Call it once per
    /// window, for example in the root view's constructor.
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
