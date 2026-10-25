//! Reduced-motion preference.
//!
//! weGPUI animations (overlay transitions, spinners, swipe snaps, picker inertia) consult GPUI's
//! `App::reduce_motion` and become instant when it is set.
//!
//! Known limitations: only macOS is read (from `com.apple.universalaccess`), by spawning
//! `defaults`. On other platforms the application must call `cx.set_reduce_motion` itself.

use gpui::App;

/// Reads the operating system's reduce-motion setting.
///
/// Returns `Some(true)`/`Some(false)` on macOS (a missing key reads as `false`), and `None` where
/// weGPUI cannot read the setting. This spawns a short-lived process and blocks while it runs,
/// so call it at startup or on window activation rather than per frame.
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

/// Copies [`system_reduce_motion`] into `cx.set_reduce_motion` when it is known. Leaves the
/// current setting unchanged otherwise.
pub fn sync_reduce_motion(cx: &mut App) {
    if let Some(reduce) = system_reduce_motion() {
        cx.set_reduce_motion(reduce);
    }
}
