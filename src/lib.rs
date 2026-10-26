//! WeUI 2.6.26 as native [GPUI](https://gpui.rs) components.
//!
//! weGPUI is an independent Rust port of Tencent's WeUI design system. It provides the WeUI
//! tokens ([`Palette`], light/dark/care themes through [`Theme`]), the WeUI icons and example
//! images ([`Assets`]), and components in [`components`] built from GPUI primitives. It does not
//! embed a browser or run WeUI's CSS or JavaScript.
//!
//! # Using the library
//!
//! The library is platform-neutral: it depends on `gpui` and `unicode-segmentation`. An application chooses a GPUI
//! platform crate, for example `gpui_platform` (crates.io `gpui-pre-platform`, with the
//! `font-kit` feature on macOS and `wayland`/`x11` on Linux), registers [`Assets`], and calls
//! [`init`] once before opening windows:
//!
//! ```ignore
//! gpui_platform::application().with_assets(wegpui::Assets).run(|cx| {
//!     wegpui::init(cx);
//!     cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MyView)).unwrap();
//! });
//! ```
//!
//! `examples/minimal.rs` is a complete, compiled version of this, with a button that opens a
//! dialog. `examples/gallery` reproduces every upstream WeUI example page.
//!
//! # Conventions
//!
//! - Lengths are `f32` logical pixels, matching WeUI's CSS pixels.
//! - Colours and metrics come from [`Theme::get`], which dereferences to the active [`Palette`].
//! - Stateless components implement `RenderOnce` and use builder methods. Stateful ones
//!   ([`input::TextInput`], [`components::picker::Picker`], `TextArea`, `SearchBar`) are GPUI
//!   entities. Overlay visibility lives in a caller-owned [`components::Presence`].
//! - Wrap the window's root element in [`accessibility::ModalScope`] so content behind open
//!   modals is hidden from assistive technology.
//! - Every default label, accessible name and message the library produces comes from the
//!   active [`I18n`] catalog ([`i18n`]); English is the default and any language can be
//!   supplied. Text you pass to builders is used as given.
//!
//! See `docs/STATUS.md` for verified behaviour and known gaps, and `docs/PLATFORMS.md` for
//! platform support. Desktop builds are exercised on macOS and Linux; mobile platforms are not
//! supported yet.

pub mod accessibility;
pub mod assets;
pub mod components;
pub mod i18n;
pub mod input;
pub mod logical;
pub mod motion;
pub mod scrolling;
pub mod theme;

pub use assets::Assets;
pub use i18n::{Direction, I18n, Locale, Messages};
pub use theme::{Palette, Theme, ThemeMode};

/// Initialises weGPUI for an application.
///
/// Installs the light [`Theme`] and then follows the system appearance, installs English as the
/// [`I18n`] language unless one is already set, registers the
/// [`input`] key bindings, and applies the system reduced-motion preference where weGPUI can read
/// it ([`motion::system_reduce_motion`]). Call it once, inside `Application::run`, before opening
/// windows that render weGPUI components.
pub fn init(cx: &mut gpui::App) {
    Theme::init(cx);
    I18n::init(cx);
    input::init(cx);
    motion::sync_reduce_motion(cx);
}
