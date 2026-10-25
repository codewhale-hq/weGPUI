//! WeUI components. Each module ports one WeUI widget family from `src/style/widget/` of the
//! pinned WeUI 2.6.26 package, with the matching example page in `examples/gallery`.
//!
//! The most common types are re-exported here: list cells ([`Cell`], [`Cells`]), [`Icon`], and
//! the overlay building blocks ([`Presence`], [`Modal`], [`MaskKind`], [`Placement`],
//! [`Motion`]). Component coverage is listed in `docs/UPSTREAM-COVERAGE.md`; behaviour evidence
//! and known differences per family are in `docs/REVIEW-2026-10-24.md`.

pub mod action_sheet;
pub mod agree;
pub mod article;
pub mod badge;
pub mod button;
pub mod cells;
pub mod checkbox;
pub mod date_field;
pub mod dialog;
pub mod flex;
pub mod footer;
pub mod form;
pub mod form_preview;
pub mod gallery;
pub mod grid;
pub mod half_screen_dialog;
pub mod icons;
pub mod information_bar;
pub mod loading;
pub mod loadmore;
pub mod modal;
pub mod msg;
pub mod navbar;
pub mod panel;
pub mod picker;
pub mod progress;
pub mod radio;
pub mod search_bar;
pub mod select;
pub mod slider;
pub mod steps;
pub mod swiped;
pub mod switch;
pub mod tabbar;
pub mod textarea;
pub mod toast;
pub mod toptips;
pub mod uploader;
pub mod vcode;

pub use cells::{Cell, Cells, CellsStyle, cells_title};
pub use icons::Icon;
pub use modal::{MaskKind, Modal, Motion, Placement, Presence};

/// Boxed click callback used by component builders. It receives GPUI's click event (mouse,
/// touch or keyboard activation), the window, and the app.
pub type ClickHandler = Box<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>;
/// Boxed value-change callback used by component builders. It receives the proposed new value;
/// components do not store it, so the owner must re-render with the new value.
pub type ChangeHandler<T> = Box<dyn Fn(&T, &mut gpui::Window, &mut gpui::App)>;
