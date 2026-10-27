pub fn application() -> gpui::Application {
    #[cfg(target_os = "ios")]
    {
        gpui::Application::with_platform(gpui_ios::current_platform(false))
    }
    #[cfg(not(target_os = "ios"))]
    {
        gpui_platform::application()
    }
}

#[cfg(target_os = "ios")]
pub fn safe_area(
    content: impl gpui::IntoElement,
    window: &gpui::Window,
    cx: &gpui::App,
) -> gpui::AnyElement {
    use gpui::{prelude::*, *};
    let visible = window.fully_visible_bounds();
    div()
        .relative()
        .size_full()
        .bg(wegpui::Theme::get(cx).bg_0)
        .child(
            div()
                .absolute()
                .left(visible.origin.x)
                .top(visible.origin.y)
                .w(visible.size.width)
                .h(visible.size.height)
                .overflow_hidden()
                .child(content),
        )
        .into_any_element()
}
