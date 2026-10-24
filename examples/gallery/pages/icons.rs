use crate::{chrome::Placeholder, pages::Args};
use gpui::*;

pub fn build(route: &'static str, _: &Args, _: &mut Window, cx: &mut App) -> AnyView {
    Placeholder::view(route, cx)
}
