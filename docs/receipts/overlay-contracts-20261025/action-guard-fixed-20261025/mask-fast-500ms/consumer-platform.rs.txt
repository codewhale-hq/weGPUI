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
