use gpui::App;

pub fn system_reduce_motion() -> Option<bool> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("defaults")
            .args(["read", "com.apple.universalaccess", "reduceMotion"])
            .output()
            .ok()?;
        if !output.status.success() {
            return Some(false);
        }
        Some(String::from_utf8_lossy(&output.stdout).trim() == "1")
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

pub fn sync_reduce_motion(cx: &mut App) {
    if let Some(reduce) = system_reduce_motion() {
        cx.set_reduce_motion(reduce);
    }
}
