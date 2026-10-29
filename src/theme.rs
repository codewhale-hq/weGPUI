//! WeUI theming: light/dark appearance, care mode and brand colour over the generated
//! [`Palette`].
//!
//! [`Theme`] is a GPUI global. [`crate::init`] installs it, and components read it with
//! [`Theme::get`]. Changing the mode, care mode or brand rebuilds the palette and refreshes all
//! windows. Brand overrides replace `brand` and `steps_highlight_color` only; WeUI derives other
//! brand shades in CSS, and those remain the defaults.

mod palette;

pub use palette::Palette;

use gpui::{App, Global, Hsla, Window, WindowAppearance, black, white};
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
    /// Resolves a translucent surface over `backdrop` and the theme's opaque canvas.
    ///
    /// Pass the colours in paint order. A host supplying translucent palette backgrounds must
    /// supply its actual backdrop here; an unspecified window canvas is white in light mode and
    /// black in dark mode.
    pub fn surface_color(&self, surface: Hsla, backdrop: Hsla) -> Hsla {
        let canvas = if self.is_dark { black() } else { white() };
        canvas.blend(backdrop).blend(surface)
    }

    /// A keyboard focus colour with at least 3:1 contrast against `surface`.
    ///
    /// Keeps the brand colour when its painted contrast is sufficient, otherwise selects opaque
    /// black or white. Pass the actual painted surface, using [`Self::surface_color`] for layered
    /// fills. Images and controls spanning unknown surfaces need a two-colour indicator instead.
    pub fn focus_color(&self, surface: Hsla) -> Hsla {
        self.focus_color_with(self.brand, surface)
    }

    /// Like [`Self::focus_color`], preserving a component's preferred colour when it passes.
    pub fn focus_color_with(&self, preferred: Hsla, surface: Hsla) -> Hsla {
        let canvas = if self.is_dark { black() } else { white() };
        let surface = canvas.blend(surface);
        if contrast_ratio(surface.blend(preferred), surface) >= 3. {
            preferred
        } else if contrast_ratio(black(), surface) >= contrast_ratio(white(), surface) {
            black()
        } else {
            white()
        }
    }

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
    /// Replace this theme’s palette with colors supplied by the native host.
    ///
    /// Mode, care and brand setters rebuild the standard palette; the host must
    /// reapply its mapping after those settings change.
    pub fn with_palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
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

/// WCAG relative luminance contrast for two opaque, rendered colours.
pub(crate) fn contrast_ratio(a: Hsla, b: Hsla) -> f32 {
    let luminance = |color: Hsla| {
        let color = color.to_rgb();
        let linear = |value: f32| {
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
    };
    let a = luminance(a);
    let b = luminance(b);
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{rgb, rgba};

    #[test]
    fn focus_contrast_covers_themes_fills_and_custom_brands() {
        for dark in [false, true] {
            for care in [false, true] {
                for brand in [
                    None,
                    Some(rgb(0x7c3aed).into()),
                    Some(rgba(0xffffff20).into()),
                ] {
                    let theme = Theme::build(ThemeMode::Light, dark, care, brand);
                    for surface in [
                        theme.bg_0,
                        theme.bg_1,
                        theme.bg_2,
                        theme.bg_3,
                        theme.bg_5,
                        theme.brand,
                        theme.red,
                        theme.surface_color(theme.fg_5, theme.bg_2),
                    ] {
                        let surface = if dark { black() } else { white() }.blend(surface);
                        let color = theme.focus_color(surface);
                        assert!(contrast_ratio(surface.blend(color), surface) >= 3.);
                        if contrast_ratio(surface.blend(theme.brand), surface) >= 3. {
                            assert_eq!(color, theme.brand);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn custom_palette_and_translucent_surfaces_use_painted_colours() {
        let mut palette = Palette::light();
        palette.brand = rgba(0xffffff20).into();
        palette.bg_2 = rgb(0x7c3aed).into();
        let theme = Theme::light().with_palette(palette);
        let surface = theme.surface_color(rgba(0xffffff4d).into(), theme.bg_2);
        let color = theme.focus_color(surface);
        assert!(contrast_ratio(surface.blend(color), surface) >= 3.);
        assert_ne!(color, theme.brand);
        let preferred = black();
        assert_eq!(theme.focus_color_with(preferred, white()), preferred);
        assert!((contrast_ratio(black(), white()) - 21.).abs() < 0.0001);
        assert!((contrast_ratio(rgb(0x07c160).into(), white()) - 2.385).abs() < 0.001);
    }
}
