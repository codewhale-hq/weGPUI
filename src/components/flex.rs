use gpui::{prelude::*, *};

pub fn flex() -> Div {
    div().flex()
}

pub fn flex_item() -> Div {
    div().flex_1().min_w_0()
}
