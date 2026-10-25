//! Direction-aware ("logical") counterparts of GPUI's left/right style methods.
//!
//! WeUI is drawn left to right. Under a right-to-left locale (see [`crate::I18n::direction`])
//! the layout mirrors: rows run from the right, the leading edge is the right edge and a
//! trailing arrow points left. The methods here take the [`Direction`] and pick the physical
//! side, so one call site serves both:
//!
//! ```ignore
//! use wegpui::{I18n, logical::Logical};
//!
//! let dir = I18n::get(cx).direction();
//! div().flex().row(dir).ps(dir, px(16.)).pe(dir, px(8.)).child(label)
//! ```
//!
//! *Start* is the leading edge of the text (left in LTR, right in RTL) and *end* the trailing
//! edge. [`Logical::row`] reverses a flex row so its first child sits at the start.

use crate::i18n::Direction;
use gpui::{DefiniteLength, Length, Styled};

/// Start/end style helpers for any GPUI element that implements [`Styled`].
pub trait Logical: Styled + Sized {
    /// Lays children out in a row whose first child is at the start edge.
    fn row(self, direction: Direction) -> Self {
        if direction.is_rtl() {
            self.flex_row_reverse()
        } else {
            self.flex_row()
        }
    }

    /// Padding on the start edge.
    fn ps(self, direction: Direction, length: impl Clone + Into<DefiniteLength>) -> Self {
        if direction.is_rtl() {
            self.pr(length)
        } else {
            self.pl(length)
        }
    }

    /// Padding on the end edge.
    fn pe(self, direction: Direction, length: impl Clone + Into<DefiniteLength>) -> Self {
        if direction.is_rtl() {
            self.pl(length)
        } else {
            self.pr(length)
        }
    }

    /// Margin on the start edge.
    fn ms(self, direction: Direction, length: impl Clone + Into<Length>) -> Self {
        if direction.is_rtl() {
            self.mr(length)
        } else {
            self.ml(length)
        }
    }

    /// Margin on the end edge.
    fn me(self, direction: Direction, length: impl Clone + Into<Length>) -> Self {
        if direction.is_rtl() {
            self.ml(length)
        } else {
            self.mr(length)
        }
    }

    /// Offset from the start edge of a positioned parent (`left` in LTR, `right` in RTL).
    fn inset_s(self, direction: Direction, length: impl Clone + Into<Length>) -> Self {
        if direction.is_rtl() {
            self.right(length)
        } else {
            self.left(length)
        }
    }

    /// Offset from the end edge of a positioned parent (`right` in LTR, `left` in RTL).
    fn inset_e(self, direction: Direction, length: impl Clone + Into<Length>) -> Self {
        if direction.is_rtl() {
            self.left(length)
        } else {
            self.right(length)
        }
    }

    /// Aligns text to the start edge.
    fn text_start(self, direction: Direction) -> Self {
        if direction.is_rtl() {
            self.text_right()
        } else {
            self.text_left()
        }
    }

    /// Aligns text to the end edge.
    fn text_end(self, direction: Direction) -> Self {
        if direction.is_rtl() {
            self.text_left()
        } else {
            self.text_right()
        }
    }
}

impl<T: Styled + Sized> Logical for T {}
