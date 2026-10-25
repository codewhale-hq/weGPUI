//! Localisation: locale tags, text direction and the message catalogs behind every string
//! weGPUI draws or announces on its own.
//!
//! WeUI is written in Chinese; weGPUI is not. Every default label, accessible name, hint and
//! validation message the library produces comes from a [`Messages`] catalog, chosen by the
//! [`Locale`] of the global [`I18n`]. English is the default. Catalogs for Simplified and
//! Traditional Chinese, Japanese, Korean, Spanish, French, German, Portuguese, Russian, Arabic,
//! Hebrew, Hindi and Thai are built in (see [`BUILTIN_LOCALES`]); the non-English, non-Chinese
//! ones are starter translations that have not been reviewed by native speakers. Any other
//! language is supplied by the application as a [`Messages`] value.
//!
//! ```ignore
//! use wegpui::{I18n, Locale, Messages};
//!
//! wegpui::init(cx);
//! // A built-in language, switchable at any time; open windows redraw.
//! I18n::set_locale(Locale::parse("ar-EG").unwrap(), cx);
//! // Any other language: start from English and replace what you need.
//! I18n::set_locale(Locale::parse("sv").unwrap(), cx);
//! I18n::set_messages(
//!     Messages {
//!         cancel: "Avbryt".into(),
//!         ok: "OK".into(),
//!         ..Messages::en()
//!     },
//!     cx,
//! );
//! ```
//!
//! Text that an application passes to a builder (`Button::new("Save")`, `Switch::label(..)`) is
//! used verbatim; only the library's own defaults are localised. The tags are BCP 47 (`en`,
//! `zh-Hans`), so an application can pass its resolved locale straight to [`I18n::set_locale`].
//!
//! # Limits
//!
//! Numbers are always written with ASCII digits and dates are Gregorian; no CLDR number,
//! calendar or plural data is bundled. Message templates therefore avoid plural-sensitive
//! wording ("Resend (5s)", "Step 2 of 4"). Applications that need more supply their own
//! [`Messages`].

mod catalogs;

use gpui::{App, Global, SharedString};
use std::{
    fmt::{Display, Write as _},
    sync::{Arc, LazyLock},
};

/// The tags of the catalogs built into the library, in the order a language menu would list
/// them. [`Locale::catalog_tag`] always returns one of these.
pub const BUILTIN_LOCALES: &[&str] = catalogs::TAGS;

/// Text and layout direction of a locale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Left to right: the layout WeUI is designed in.
    #[default]
    Ltr,
    /// Right to left, as in Arabic and Hebrew: layouts mirror.
    Rtl,
}

impl Direction {
    /// `true` for [`Direction::Rtl`].
    pub fn is_rtl(self) -> bool {
        matches!(self, Self::Rtl)
    }

    /// The opposite direction.
    pub fn mirrored(self) -> Self {
        match self {
            Self::Ltr => Self::Rtl,
            Self::Rtl => Self::Ltr,
        }
    }
}

/// A language tag: language, optional script and optional region (`en`, `zh-Hans`, `ar-EG`).
///
/// [`Locale::parse`] accepts BCP 47 (`zh-Hant-TW`), POSIX (`zh_CN.UTF-8`, `en_US@euro`) and
/// mixed-case spellings, and stores the canonical form: lower-case language, title-case script,
/// upper-case region. Variants and extensions are dropped.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Locale {
    tag: SharedString,
}

impl Default for Locale {
    fn default() -> Self {
        Self::en()
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.tag)
    }
}

impl Locale {
    /// English, the library default.
    pub fn en() -> Self {
        Self {
            tag: SharedString::new_static("en"),
        }
    }

    /// Simplified Chinese, the language of the original WeUI.
    pub fn zh_hans() -> Self {
        Self {
            tag: SharedString::new_static("zh-Hans"),
        }
    }

    /// Parses a language tag. Returns `None` for empty input, `C`/`POSIX`, or a first subtag that
    /// is not a two- or three-letter language code.
    pub fn parse(tag: &str) -> Option<Self> {
        let base = tag.trim().split(['.', '@']).next().unwrap_or("");
        if base.is_empty() || base.eq_ignore_ascii_case("c") || base.eq_ignore_ascii_case("posix") {
            return None;
        }
        let mut parts = base.split(['-', '_']);
        let language = parts.next()?;
        if !(2..=3).contains(&language.len()) || !language.bytes().all(|b| b.is_ascii_alphabetic())
        {
            return None;
        }
        let language = match language.to_ascii_lowercase().as_str() {
            "iw" => "he".to_string(),
            "in" => "id".to_string(),
            "ji" => "yi".to_string(),
            "jw" => "jv".to_string(),
            "mo" => "ro".to_string(),
            other => other.to_string(),
        };
        let mut script: Option<String> = None;
        let mut region: Option<String> = None;
        for part in parts {
            let alphabetic = part.bytes().all(|b| b.is_ascii_alphabetic());
            if script.is_none() && region.is_none() && part.len() == 4 && alphabetic {
                let lower = part.to_ascii_lowercase();
                let mut chars = lower.chars();
                let first = chars.next()?.to_ascii_uppercase();
                script = Some(format!("{first}{}", chars.as_str()));
            } else if region.is_none()
                && ((part.len() == 2 && alphabetic)
                    || (part.len() == 3 && part.bytes().all(|b| b.is_ascii_digit())))
            {
                region = Some(part.to_ascii_uppercase());
            } else {
                break;
            }
        }
        let mut tag = language;
        for part in script.iter().chain(region.iter()) {
            tag.push('-');
            tag.push_str(part);
        }
        Some(Self { tag: tag.into() })
    }

    /// The canonical tag, for example `zh-Hans-CN`.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// The lower-case language subtag, for example `zh`.
    pub fn language(&self) -> &str {
        self.parts().0
    }

    /// The explicit script subtag, for example `Hans`; `None` when the tag does not name one.
    pub fn script(&self) -> Option<&str> {
        self.parts().1
    }

    /// The region subtag, for example `CN` or `419`.
    pub fn region(&self) -> Option<&str> {
        self.parts().2
    }

    fn parts(&self) -> (&str, Option<&str>, Option<&str>) {
        let mut parts = self.tag.split('-');
        let language = parts.next().unwrap_or("");
        let mut script = None;
        let mut region = None;
        for part in parts {
            if part.len() == 4 {
                script = Some(part);
            } else {
                region = Some(part);
            }
        }
        (language, script, region)
    }

    /// The writing direction implied by the tag: an explicit script decides, otherwise the
    /// language (Arabic, Hebrew, Persian, Urdu, Pashto, Sindhi, Uyghur, Yiddish, Divehi, Sorani
    /// Kurdish and a few others are right to left).
    pub fn direction(&self) -> Direction {
        let (language, script, _) = self.parts();
        let rtl = match script {
            Some(script) => matches!(
                script,
                "Arab" | "Hebr" | "Thaa" | "Syrc" | "Nkoo" | "Adlm" | "Rohg" | "Mand" | "Samr"
            ),
            None => matches!(
                language,
                "ar" | "he"
                    | "fa"
                    | "ur"
                    | "ps"
                    | "sd"
                    | "ug"
                    | "yi"
                    | "dv"
                    | "ckb"
                    | "ks"
                    | "azb"
                    | "prs"
                    | "syr"
                    | "arc"
                    | "nqo"
            ),
        };
        if rtl { Direction::Rtl } else { Direction::Ltr }
    }

    /// The tag of the built-in catalog that serves this locale, falling back to `en`.
    ///
    /// Chinese is split by script: `zh-Hant`, or `zh` with region `TW`, `HK` or `MO`, selects
    /// Traditional; every other `zh` selects Simplified.
    pub fn catalog_tag(&self) -> &'static str {
        let (language, script, region) = self.parts();
        match language {
            "zh" => {
                let traditional = match script {
                    Some(script) => script == "Hant",
                    None => matches!(region, Some("TW" | "HK" | "MO")),
                };
                if traditional { "zh-Hant" } else { "zh-Hans" }
            }
            other => BUILTIN_LOCALES
                .iter()
                .copied()
                .find(|tag| *tag == other)
                .unwrap_or("en"),
        }
    }

    /// Font families to consult first for characters the primary font lacks, in order.
    ///
    /// Chinese, Japanese and Korean share Han characters whose shapes differ by language, and a
    /// text system that does not know the language picks one regional form for all of them.
    /// Putting the matching regional families first (`Hiragino Sans` for Japanese, `PingFang TC`
    /// for Traditional Chinese, and so on, with the Windows and Noto equivalents after them)
    /// selects the right glyph forms. The list is empty for every other language, including
    /// Simplified Chinese, whose forms are the platform default. An explicit list can render
    /// CJK text a little lighter than the platform's own cascade does. Apply it to the
    /// application's root element: build a `gpui::FontFallbacks` with `from_fonts` and set it
    /// on the element's text style.
    pub fn font_fallbacks(&self) -> &'static [&'static str] {
        match self.catalog_tag() {
            "ja" => &["Hiragino Sans", "Yu Gothic UI", "Noto Sans CJK JP"],
            "ko" => &["Apple SD Gothic Neo", "Malgun Gothic", "Noto Sans CJK KR"],
            "zh-Hant" => &["PingFang TC", "Microsoft JhengHei UI", "Noto Sans CJK TC"],
            _ => &[],
        }
    }
}

/// Every string weGPUI supplies by default, in one language.
///
/// Build a custom catalog from English with struct update syntax (`Messages { cancel:
/// "Annuler".into(), ..Messages::en() }`). Fields written `{name}` are templates filled by
/// [`fill`]; the placeholder names are part of each field's documentation. Month names are
/// Gregorian and are the picker's row labels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Messages {
    /// Confirm button of pickers.
    pub ok: SharedString,
    /// Cancel buttons: search bar, select sheet, progress, action sheets.
    pub cancel: SharedString,
    /// Close buttons: pickers, dialogs, information bars.
    pub close: SharedString,
    /// Back buttons.
    pub back: SharedString,
    /// Clear buttons of text inputs and the search bar.
    pub clear: SharedString,
    /// Search placeholder and the outlined search bar's button.
    pub search: SharedString,
    /// Name of the search result list.
    pub search_results: SharedString,
    /// Name of the search bar's camera button.
    pub take_photo: SharedString,
    /// Delete buttons of the image viewer and uploader.
    pub delete: SharedString,
    /// Name of a select sheet that has no title.
    pub select: SharedString,
    /// Description of a button that is loading.
    pub loading: SharedString,
    /// Accessible name of a loading button whose label is empty.
    pub working: SharedString,
    /// Description of a disabled control.
    pub unavailable: SharedString,
    /// Description of a read-only control.
    pub read_only: SharedString,
    /// Accessible value of a date field with no value.
    pub not_selected: SharedString,
    /// Default accessible name of a checkbox.
    pub checkbox: SharedString,
    /// Default accessible name of a switch.
    pub switch: SharedString,
    /// Default accessible name of a progress bar.
    pub progress: SharedString,
    /// Default name of the full-screen image viewer.
    pub large_image: SharedString,
    /// Description of the focusable image in the viewer.
    pub close_preview_hint: SharedString,
    /// Default accessible name of an uploader thumbnail.
    pub image_title: SharedString,
    /// Name of the uploader's add box.
    pub upload_image: SharedString,
    /// Description of a finished uploader thumbnail.
    pub view_large_image_hint: SharedString,
    /// Name of the warning shown on a failed upload.
    pub error: SharedString,
    /// Idle label of the verification-code button.
    pub get_code: SharedString,
    /// Label of the verification-code button while counting down. Placeholder: `{seconds}`.
    pub resend_code: SharedString,
    /// Default name of the captcha image.
    pub verification_code: SharedString,
    /// Name of the success message icon.
    pub success: SharedString,
    /// Name of the warning message icons.
    pub warning: SharedString,
    /// Name of the information message icon.
    pub info: SharedString,
    /// Name of the waiting message icon.
    pub waiting: SharedString,
    /// Position of a step. Placeholders: `{n}`, `{total}`.
    pub step_position: SharedString,
    /// Appended to the name of a finished step.
    pub step_completed: SharedString,
    /// Description of an open swipe row. Placeholder: `{hide_key}`.
    pub swipe_actions_open: SharedString,
    /// Description of a closed swipe row. Placeholder: `{show_key}`.
    pub swipe_actions_closed: SharedString,
    /// Spoken name of the left arrow key.
    pub key_arrow_left: SharedString,
    /// Spoken name of the right arrow key.
    pub key_arrow_right: SharedString,
    /// Placeholder of an empty date field.
    pub date_placeholder: SharedString,
    /// Placeholder of an empty date-and-time field.
    pub datetime_placeholder: SharedString,
    /// Validation message for a malformed date.
    pub date_format_hint: SharedString,
    /// Validation message for a malformed date and time.
    pub datetime_format_hint: SharedString,
    /// Validation message for a date that does not exist.
    pub date_invalid: SharedString,
    /// Validation message for an hour or minute out of range.
    pub time_invalid: SharedString,
    /// Validation message for a year outside the field's range. Placeholders: `{start}`, `{end}`.
    pub year_out_of_range: SharedString,
    /// Message for a date field that cannot be edited.
    pub date_unavailable: SharedString,
    /// Subtitle of the date sheet of a date-and-time field.
    pub choose_date: SharedString,
    /// Subtitle of the time sheet of a date-and-time field.
    pub choose_time: SharedString,
    /// Name of the date field's picker button. Placeholder: `{label}`.
    pub open_date_picker: SharedString,
    /// Label of a year row in the date picker. Placeholder: `{year}`.
    pub year_label: SharedString,
    /// Label of a day row in the date picker. Placeholder: `{day}`.
    pub day_label: SharedString,
    /// Labels of the twelve month rows in the date picker, January first.
    pub months: [SharedString; 12],
    /// Separator for parts of an accessible name or description, such as `", "`.
    pub list_separator: SharedString,
}

impl Default for Messages {
    fn default() -> Self {
        Self::en()
    }
}

impl Messages {
    /// The English catalog, the library default.
    pub fn en() -> Self {
        catalogs::builtin("en")
    }

    /// The Simplified Chinese catalog. It reproduces the wording of the original WeUI.
    pub fn zh_hans() -> Self {
        catalogs::builtin("zh-Hans")
    }

    /// The built-in catalog serving `locale`, or English when none does.
    pub fn for_locale(locale: &Locale) -> Self {
        catalogs::builtin(locale.catalog_tag())
    }

    /// Joins `parts` with [`Messages::list_separator`].
    pub fn join<S: AsRef<str>>(&self, parts: &[S]) -> String {
        let mut out = String::new();
        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                out.push_str(&self.list_separator);
            }
            out.push_str(part.as_ref());
        }
        out
    }
}

/// Fills `{name}` placeholders in a message template.
///
/// Placeholders without a matching argument are left as written. There is no escape syntax;
/// templates never need literal braces.
pub fn fill(template: &str, args: &[(&str, &dyn Display)]) -> String {
    let mut out = String::with_capacity(template.len() + 8);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else {
            out.push('{');
            rest = after;
            continue;
        };
        let name = &after[..close];
        match args.iter().find(|(key, _)| *key == name) {
            Some((_, value)) => {
                let _ = write!(out, "{value}");
            }
            None => {
                out.push('{');
                out.push_str(name);
                out.push('}');
            }
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// The active locale, direction and message catalog: a GPUI global.
///
/// [`crate::init`] installs English if no `I18n` is set yet, so an application may call
/// [`I18n::set_locale`] before or after it. Components read the global when they render, so
/// changing it redraws every open window with the new language.
#[derive(Clone, Debug)]
pub struct I18n {
    locale: Locale,
    direction: Direction,
    messages: Arc<Messages>,
}

impl Global for I18n {}

static DEFAULT: LazyLock<I18n> = LazyLock::new(|| I18n::new(Locale::en()));

impl Default for I18n {
    fn default() -> Self {
        Self::new(Locale::en())
    }
}

impl I18n {
    /// Resolves `locale` to its built-in catalog (English when there is none) and its direction.
    pub fn new(locale: Locale) -> Self {
        let messages = Arc::new(Messages::for_locale(&locale));
        let direction = locale.direction();
        Self {
            locale,
            direction,
            messages,
        }
    }

    /// The global, or an English default when none is installed.
    pub fn get(cx: &App) -> &Self {
        cx.try_global::<Self>().unwrap_or(&DEFAULT)
    }

    /// Installs English unless an `I18n` is already set. Called by [`crate::init`].
    pub fn init(cx: &mut App) {
        if cx.try_global::<Self>().is_none() {
            cx.set_global(Self::default());
        }
    }

    /// Switches to `locale`: its built-in catalog (English when it has none) and its direction.
    /// Any catalog installed with [`I18n::set_messages`] is replaced. Refreshes all windows.
    pub fn set_locale(locale: Locale, cx: &mut App) {
        cx.set_global(Self::new(locale));
        cx.refresh_windows();
    }

    /// Replaces the message catalog, keeping the locale and direction. Call it after
    /// [`I18n::set_locale`] to supply a language the library has no catalog for. Refreshes all
    /// windows.
    pub fn set_messages(messages: Messages, cx: &mut App) {
        let mut next = Self::get(cx).clone();
        next.messages = Arc::new(messages);
        cx.set_global(next);
        cx.refresh_windows();
    }

    /// Forces the layout direction, or `None` to follow the locale again. Refreshes all windows.
    pub fn set_direction(direction: Option<Direction>, cx: &mut App) {
        let mut next = Self::get(cx).clone();
        next.direction = direction.unwrap_or_else(|| next.locale.direction());
        cx.set_global(next);
        cx.refresh_windows();
    }

    /// The active locale.
    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    /// The active layout direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// The active message catalog.
    pub fn messages(&self) -> &Messages {
        &self.messages
    }
}

pub(crate) fn messages(cx: &App) -> &Messages {
    I18n::get(cx).messages()
}

pub(crate) fn messages_arc(cx: &App) -> Arc<Messages> {
    I18n::get(cx).messages.clone()
}

pub(crate) fn direction(cx: &App) -> Direction {
    I18n::get(cx).direction()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_fields(m: &Messages) -> Vec<(&'static str, &SharedString)> {
        vec![
            ("ok", &m.ok),
            ("cancel", &m.cancel),
            ("close", &m.close),
            ("back", &m.back),
            ("clear", &m.clear),
            ("search", &m.search),
            ("search_results", &m.search_results),
            ("take_photo", &m.take_photo),
            ("delete", &m.delete),
            ("select", &m.select),
            ("loading", &m.loading),
            ("working", &m.working),
            ("unavailable", &m.unavailable),
            ("read_only", &m.read_only),
            ("not_selected", &m.not_selected),
            ("checkbox", &m.checkbox),
            ("switch", &m.switch),
            ("progress", &m.progress),
            ("large_image", &m.large_image),
            ("close_preview_hint", &m.close_preview_hint),
            ("image_title", &m.image_title),
            ("upload_image", &m.upload_image),
            ("view_large_image_hint", &m.view_large_image_hint),
            ("error", &m.error),
            ("get_code", &m.get_code),
            ("resend_code", &m.resend_code),
            ("verification_code", &m.verification_code),
            ("success", &m.success),
            ("warning", &m.warning),
            ("info", &m.info),
            ("waiting", &m.waiting),
            ("step_position", &m.step_position),
            ("step_completed", &m.step_completed),
            ("swipe_actions_open", &m.swipe_actions_open),
            ("swipe_actions_closed", &m.swipe_actions_closed),
            ("key_arrow_left", &m.key_arrow_left),
            ("key_arrow_right", &m.key_arrow_right),
            ("date_placeholder", &m.date_placeholder),
            ("datetime_placeholder", &m.datetime_placeholder),
            ("date_format_hint", &m.date_format_hint),
            ("datetime_format_hint", &m.datetime_format_hint),
            ("date_invalid", &m.date_invalid),
            ("time_invalid", &m.time_invalid),
            ("year_out_of_range", &m.year_out_of_range),
            ("date_unavailable", &m.date_unavailable),
            ("choose_date", &m.choose_date),
            ("choose_time", &m.choose_time),
            ("open_date_picker", &m.open_date_picker),
            ("year_label", &m.year_label),
            ("day_label", &m.day_label),
            ("list_separator", &m.list_separator),
        ]
    }

    #[test]
    fn locale_parsing_accepts_bcp47_and_posix_spellings() {
        for (input, tag) in [
            ("en", "en"),
            ("EN_us", "en-US"),
            ("zh_CN.UTF-8", "zh-CN"),
            ("en_US@euro", "en-US"),
            ("zh-hant-tw", "zh-Hant-TW"),
            ("sr-latn-rs", "sr-Latn-RS"),
            ("es-419", "es-419"),
            ("ar-EG-u-nu-latn", "ar-EG"),
            ("iw", "he"),
            ("  fr-CA  ", "fr-CA"),
        ] {
            assert_eq!(Locale::parse(input).unwrap().tag(), tag, "{input}");
        }
        for input in ["", "C", "POSIX", "x", "toolonglanguage", "12", "-en"] {
            assert!(Locale::parse(input).is_none(), "{input:?}");
        }
        let locale = Locale::parse("zh-Hant-TW").unwrap();
        assert_eq!(locale.language(), "zh");
        assert_eq!(locale.script(), Some("Hant"));
        assert_eq!(locale.region(), Some("TW"));
        let locale = Locale::parse("es-419").unwrap();
        assert_eq!((locale.script(), locale.region()), (None, Some("419")));
    }

    #[test]
    fn direction_follows_an_explicit_script_before_the_language() {
        for tag in [
            "ar", "ar-EG", "he", "fa", "ur", "ps", "ckb", "az-Arab", "pa-Arab",
        ] {
            assert_eq!(
                Locale::parse(tag).unwrap().direction(),
                Direction::Rtl,
                "{tag}"
            );
        }
        for tag in [
            "en", "zh-Hans", "ja", "hi", "th", "ru", "ku", "ur-Latn", "ar-Latn",
        ] {
            assert_eq!(
                Locale::parse(tag).unwrap().direction(),
                Direction::Ltr,
                "{tag}"
            );
        }
        assert!(Direction::Rtl.is_rtl() && !Direction::Ltr.is_rtl());
        assert_eq!(Direction::Ltr.mirrored(), Direction::Rtl);
    }

    #[test]
    fn catalog_selection_splits_chinese_and_falls_back_to_english() {
        for (tag, catalog) in [
            ("zh", "zh-Hans"),
            ("zh-CN", "zh-Hans"),
            ("zh-SG", "zh-Hans"),
            ("zh-Hans-TW", "zh-Hans"),
            ("zh-TW", "zh-Hant"),
            ("zh-HK", "zh-Hant"),
            ("zh-Hant", "zh-Hant"),
            ("zh-Hant-CN", "zh-Hant"),
            ("pt-BR", "pt"),
            ("iw", "he"),
            ("sv", "en"),
            ("en-GB", "en"),
        ] {
            assert_eq!(Locale::parse(tag).unwrap().catalog_tag(), catalog, "{tag}");
        }
        for tag in BUILTIN_LOCALES {
            assert_eq!(Locale::parse(tag).unwrap().catalog_tag(), *tag);
        }
    }

    #[test]
    fn fill_replaces_named_placeholders_and_keeps_unknown_text() {
        assert_eq!(
            fill("Step {n} of {total}", &[("n", &2), ("total", &4)]),
            "Step 2 of 4"
        );
        assert_eq!(fill("{a}{a}-{b}", &[("a", &"x"), ("b", &7u8)]), "xx-7");
        assert_eq!(fill("keep {unknown} here", &[]), "keep {unknown} here");
        assert_eq!(fill("open { brace", &[("n", &1)]), "open { brace");
        assert_eq!(fill("no placeholders", &[("n", &1)]), "no placeholders");
        assert_eq!(fill("", &[]), "");
    }

    #[test]
    fn join_uses_the_catalog_separator() {
        assert_eq!(Messages::en().join(&["a", "b", "c"]), "a, b, c");
        assert_eq!(Messages::zh_hans().join(&["a", "b"]), "a，b");
        assert_eq!(Messages::en().join::<&str>(&[]), "");
        assert_eq!(Messages::en().join(&["only"]), "only");
    }

    #[test]
    fn every_builtin_catalog_is_complete_and_keeps_its_placeholders() {
        let english = Messages::en();
        for tag in BUILTIN_LOCALES {
            let m = catalogs::builtin(tag);
            for (name, value) in text_fields(&m) {
                assert!(
                    !value.trim().is_empty() || name == "list_separator",
                    "{tag}.{name}"
                );
            }
            assert!(!m.list_separator.is_empty(), "{tag}.list_separator");
            let has = |name: &str, value: &SharedString, placeholders: &[&str]| {
                for placeholder in placeholders {
                    assert!(
                        value.contains(placeholder),
                        "{tag}.{name} lacks {placeholder}"
                    );
                }
            };
            has("resend_code", &m.resend_code, &["{seconds}"]);
            has("step_position", &m.step_position, &["{n}", "{total}"]);
            has("swipe_actions_open", &m.swipe_actions_open, &["{hide_key}"]);
            has(
                "swipe_actions_closed",
                &m.swipe_actions_closed,
                &["{show_key}"],
            );
            has(
                "year_out_of_range",
                &m.year_out_of_range,
                &["{start}", "{end}"],
            );
            has("open_date_picker", &m.open_date_picker, &["{label}"]);
            has("year_label", &m.year_label, &["{year}"]);
            has("day_label", &m.day_label, &["{day}"]);
            let mut months: Vec<&str> = m.months.iter().map(|month| &**month).collect();
            assert!(months.iter().all(|month| !month.is_empty()), "{tag}.months");
            months.sort_unstable();
            months.dedup();
            assert_eq!(months.len(), 12, "{tag} month names are distinct");
            if *tag != "en" {
                assert_ne!(m.cancel, english.cancel, "{tag} translates cancel");
            }
        }
    }

    #[test]
    fn zh_hans_reproduces_the_original_weui_wording() {
        let m = Messages::zh_hans();
        let step = |n: usize, total: usize, parts: &[&str]| {
            let mut all = vec![fill(&m.step_position, &[("n", &n), ("total", &total)])];
            all.extend(parts.iter().map(|part| part.to_string()));
            m.join(&all)
        };
        assert_eq!(
            step(2, 4, &["标题", "说明", "已完成"]),
            "第2步，共4步，标题，说明，已完成"
        );
        assert_eq!(
            fill(&m.swipe_actions_open, &[("hide_key", &m.key_arrow_right)]),
            "操作已展开，按右方向键或 Esc 隐藏"
        );
        assert_eq!(
            fill(&m.swipe_actions_closed, &[("show_key", &m.key_arrow_left)]),
            "操作已收起，按左方向键或回车显示操作"
        );
        assert_eq!(
            fill(&m.resend_code, &[("seconds", &59u32)]),
            "重新发送(59s)"
        );
        assert_eq!(fill(&m.year_label, &[("year", &2026)]), "2026年");
        assert_eq!(fill(&m.day_label, &[("day", &5)]), "5日");
        assert_eq!(&*m.months[9], "10月");
        assert_eq!(
            fill(&m.open_date_picker, &[("label", &"生日")]),
            "生日，打开日期选择器"
        );
        for (value, expected) in [
            (&m.ok, "确定"),
            (&m.cancel, "取消"),
            (&m.close, "关闭"),
            (&m.checkbox, "多选"),
            (&m.switch, "开关"),
            (&m.loading, "正在加载"),
            (&m.working, "正在操作"),
            (&m.unavailable, "不可用"),
            (&m.get_code, "获取验证码"),
            (&m.upload_image, "上传图片"),
        ] {
            assert_eq!(&**value, expected);
        }
    }
}
