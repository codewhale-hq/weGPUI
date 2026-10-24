use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
pub struct Assets;
impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "icon-arrow-bold.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-arrow-bold.svg").as_slice(),
            )),
            "icon-arrow.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-arrow.svg").as_slice(),
            )),
            "icon-back-arrow-thin.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-back-arrow-thin.svg").as_slice(),
            )),
            "icon-back-arrow.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-back-arrow.svg").as_slice(),
            )),
            "icon-back-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-back-circle.svg").as_slice(),
            )),
            "icon-back.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-back.svg").as_slice(),
            )),
            "icon-camera.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-camera.svg").as_slice(),
            )),
            "icon-cancel.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-cancel.svg").as_slice(),
            )),
            "icon-checked.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-checked.svg").as_slice(),
            )),
            "icon-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-circle.svg").as_slice(),
            )),
            "icon-clear.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-clear.svg").as_slice(),
            )),
            "icon-close-thin.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-close-thin.svg").as_slice(),
            )),
            "icon-close.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-close.svg").as_slice(),
            )),
            "icon-delete.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-delete.svg").as_slice(),
            )),
            "icon-download.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-download.svg").as_slice(),
            )),
            "icon-info-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-info-circle.svg").as_slice(),
            )),
            "icon-info.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-info.svg").as_slice(),
            )),
            "icon-outlined-warn.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-outlined-warn.svg").as_slice(),
            )),
            "icon-safe-success.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-safe-success.svg").as_slice(),
            )),
            "icon-safe-warn.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-safe-warn.svg").as_slice(),
            )),
            "icon-search.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-search.svg").as_slice(),
            )),
            "icon-success-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-success-circle.svg").as_slice(),
            )),
            "icon-success-no-circle-thin.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-success-no-circle-thin.svg").as_slice(),
            )),
            "icon-success-no-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-success-no-circle.svg").as_slice(),
            )),
            "icon-success.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-success.svg").as_slice(),
            )),
            "icon-waiting-circle.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-waiting-circle.svg").as_slice(),
            )),
            "icon-waiting.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-waiting.svg").as_slice(),
            )),
            "icon-warn.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon-warn.svg").as_slice(),
            )),
            "icon_footer.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_footer.png").as_slice(),
            )),
            "icon_footer_link.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_footer_link.png").as_slice(),
            )),
            "icon_intro.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_intro.png").as_slice(),
            )),
            "icon_nav_feedback.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_feedback.png").as_slice(),
            )),
            "icon_nav_form.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_form.png").as_slice(),
            )),
            "icon_nav_layout.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_layout.png").as_slice(),
            )),
            "icon_nav_nav.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_nav.png").as_slice(),
            )),
            "icon_nav_search.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_search.png").as_slice(),
            )),
            "icon_nav_special.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_special.png").as_slice(),
            )),
            "icon_nav_z-index.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_nav_z-index.png").as_slice(),
            )),
            "icon_tabbar.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/icon_tabbar.png").as_slice(),
            )),
            "loading.svg" => Some(Cow::Borrowed(
                include_bytes!("../assets/loading.svg").as_slice(),
            )),
            "logo.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/logo.png").as_slice(),
            )),
            "pic_160.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/pic_160.png").as_slice(),
            )),
            "pic_article.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/pic_article.png").as_slice(),
            )),
            "pic_gongan.png" => Some(Cow::Borrowed(
                include_bytes!("../assets/pic_gongan.png").as_slice(),
            )),
            "vcode.jpg" => Some(Cow::Borrowed(
                include_bytes!("../assets/vcode.jpg").as_slice(),
            )),
            _ => None,
        })
    }
    fn list(&self, _: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
