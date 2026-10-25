//! Flex layout helpers from WeUI's `src/style/widget/weui-flex/`: [`flex`] (`weui-flex`) and
//! [`flex_item`] (`weui-flex__item`). Both return plain GPUI `Div`s for further styling.

use gpui::{prelude::*, *};

/// A flex row container, as `weui-flex`.
pub fn flex() -> Div {
    div().flex()
}

/// An equal-share flex child (`flex: 1` with zero minimum width), as `weui-flex__item`; it may
/// shrink below its content width.
pub fn flex_item() -> Div {
    div().flex_1().min_w_0()
}
