//! Date and date-time fields: the native equivalent of HTML `<input type="date">` and
//! `<input type="datetime-local">` used by WeUI 2.6.26 `example/input/input.html`.
//!
//! WeUI only styles the browser control; it ships no date widget of its own. weGPUI renders the
//! field text with a calendar glyph and edits through [`Picker`] sheets, following weui.js
//! `datePicker`: a year/month/day cascade, then, in [`DateFieldMode::DateTime`], a separate
//! hour/minute sheet. Only the final confirmation writes the value; closing either sheet keeps
//! the previous value. Focus returns to the field when the last sheet closes.
//!
//! Known adaptations: no typed entry, no locale-specific display (values print as
//! `YYYY-MM-DD` and `YYYY-MM-DD HH:MM`, the HTML value formats), no seconds, and a bounded year
//! range (default 1900–2100, see [`DateField::set_year_range`]).

use crate::{
    Theme,
    components::picker::{Picker, PickerEvent, PickerItem, today},
    scrolling::RevealFocus,
};
use gpui::{prelude::*, *};
use std::fmt;

/// Which HTML input the field stands in for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldMode {
    /// `type="date"`: year, month and day.
    Date,
    /// `type="datetime-local"`: a date sheet, then an hour/minute sheet.
    DateTime,
}

/// A calendar date with an optional time of day. `hour`/`minute` are `0` in date mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateValue {
    /// Gregorian year.
    pub year: i32,
    /// Month, `1..=12`.
    pub month: u32,
    /// Day of month, `1..=31` and valid for the month.
    pub day: u32,
    /// Hour, `0..=23`.
    pub hour: u32,
    /// Minute, `0..=59`.
    pub minute: u32,
}

impl DateValue {
    /// A date at midnight.
    pub fn date(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour: 0,
            minute: 0,
        }
    }
    /// Formats as the HTML value for `mode`: `YYYY-MM-DD` or `YYYY-MM-DD HH:MM`.
    pub fn format(&self, mode: DateFieldMode) -> String {
        let date = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);
        match mode {
            DateFieldMode::Date => date,
            DateFieldMode::DateTime => format!("{date} {:02}:{:02}", self.hour, self.minute),
        }
    }
}

impl fmt::Display for DateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format(DateFieldMode::DateTime))
    }
}

/// Events emitted by [`DateField`]; observe them with `cx.subscribe`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFieldEvent {
    /// The user confirmed a new value (after the time sheet in date-time mode).
    Change(DateValue),
    /// A sheet was closed without completing the edit; the value is unchanged.
    Cancel,
}

/// A focusable field that edits a [`DateValue`] through picker sheets. It is an entity: create
/// it with `cx.new(|cx| DateField::new(..., window, cx))`, render it as a child, and render
/// nothing else for the sheets (they are drawn by the field as overlays).
///
/// Accessibility: role button, name `"{label}，{value or 未选择}"`; Enter or Space opens the
/// sheet. Disabled fields are not focusable and are exposed as disabled.
pub struct DateField {
    id: SharedString,
    label: SharedString,
    placeholder: SharedString,
    mode: DateFieldMode,
    value: Option<DateValue>,
    disabled: bool,
    years: (i32, i32),
    date_sheet: Option<Entity<Picker>>,
    time_sheet: Option<Entity<Picker>>,
    pending: Option<DateValue>,
    just_committed: bool,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<DateFieldEvent> for DateField {}

impl DateField {
    /// Creates an empty field. `label` is the accessible name prefix and sheet title. The
    /// placeholder defaults to `年/月/日` or `年/月/日 --:--`.
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        mode: DateFieldMode,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            placeholder: match mode {
                DateFieldMode::Date => "年/月/日".into(),
                DateFieldMode::DateTime => "年/月/日 --:--".into(),
            },
            mode,
            value: None,
            disabled: false,
            years: (1900, 2100),
            date_sheet: None,
            time_sheet: None,
            pending: None,
            just_committed: false,
            _subscriptions: vec![],
        }
    }
    /// Current value, `None` until set or confirmed.
    pub fn value(&self) -> Option<DateValue> {
        self.value
    }
    /// The mode chosen at construction.
    pub fn mode(&self) -> DateFieldMode {
        self.mode
    }
    /// Sets the value without emitting an event; the next open starts at it.
    pub fn set_value(&mut self, value: Option<DateValue>, cx: &mut Context<Self>) {
        self.value = value;
        self.invalidate();
        cx.notify();
    }
    /// Sets the placeholder shown while empty.
    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.placeholder = text.into();
        cx.notify();
    }
    /// Disables or enables the field. Closes nothing that is already open.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }
    /// Sets the inclusive year range offered by the date sheet (default 1900–2100). Swapped
    /// bounds are reordered.
    pub fn set_year_range(&mut self, start: i32, end: i32, cx: &mut Context<Self>) {
        self.years = (start.min(end), start.max(end));
        self.invalidate();
        cx.notify();
    }
    /// Whether a sheet is open or the edit is mid-sequence.
    pub fn is_editing(&self, cx: &App) -> bool {
        self.pending.is_some()
            || [&self.date_sheet, &self.time_sheet]
                .into_iter()
                .flatten()
                .any(|p| p.read(cx).is_open())
    }

    fn invalidate(&mut self) {
        self.date_sheet = None;
        self.time_sheet = None;
        self._subscriptions.clear();
    }

    fn ensure_sheets(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.date_sheet.is_some() {
            return;
        }
        let (y, m, d) = today();
        let start = self.value.unwrap_or(DateValue::date(y, m, d));
        let (lo, hi) = self.years;
        let year = start.year.clamp(lo, hi);
        let label = self.label.clone();
        let id = self.id.clone();
        let with_time = self.mode == DateFieldMode::DateTime;
        let date = cx.new(|cx| {
            let p = Picker::new(SharedString::from(format!("{id}-date")), cx)
                .date(lo, hi, (year, start.month, start.day))
                .title(label.clone());
            if with_time { p.desc("选择日期") } else { p }
        });
        self._subscriptions.push(cx.subscribe_in(
            &date,
            window,
            |this, _, e: &PickerEvent, window, cx| this.on_date(e, window, cx),
        ));
        if self.mode == DateFieldMode::DateTime {
            let time = cx.new(|cx| {
                Picker::new(SharedString::from(format!("{id}-time")), cx)
                    .columns(time_columns())
                    .default_value([start.hour.to_string(), start.minute.to_string()])
                    .title(label)
                    .desc("选择时刻")
            });
            self._subscriptions.push(cx.subscribe_in(
                &time,
                window,
                |this, _, e: &PickerEvent, _, cx| this.on_time(e, cx),
            ));
            self.time_sheet = Some(time);
        }
        self.date_sheet = Some(date);
    }

    /// Opens the date sheet, as Enter, Space or a click on the field does. Ignored while
    /// disabled.
    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.ensure_sheets(window, cx);
        self.pending = None;
        if let Some(date) = self.date_sheet.clone() {
            date.update(cx, |p, cx| p.open(window, cx));
        }
    }

    fn on_date(&mut self, e: &PickerEvent, window: &mut Window, cx: &mut Context<Self>) {
        match e {
            PickerEvent::Confirm(items) => {
                if let Some(mut v) = parse_date(items) {
                    match self.mode {
                        DateFieldMode::Date => self.commit(v, cx),
                        DateFieldMode::DateTime => {
                            if let Some(old) = self.value {
                                v.hour = old.hour;
                                v.minute = old.minute;
                            }
                            self.pending = Some(v);
                        }
                    }
                }
            }
            PickerEvent::Close => {
                if self.pending.is_some() {
                    if let Some(time) = self.time_sheet.clone() {
                        window.defer(cx, move |window, cx| {
                            time.update(cx, |p, cx| p.open(window, cx));
                        });
                    }
                } else if self.mode == DateFieldMode::DateTime || !self.just_committed {
                    cx.emit(DateFieldEvent::Cancel);
                }
                self.just_committed = false;
            }
            PickerEvent::Change(_) => {}
        }
        cx.notify();
    }

    fn on_time(&mut self, e: &PickerEvent, cx: &mut Context<Self>) {
        match e {
            PickerEvent::Confirm(items) => {
                if let (Some(mut v), [h, m]) = (self.pending.take(), items.as_slice()) {
                    v.hour = h.value.parse().unwrap_or(0);
                    v.minute = m.value.parse().unwrap_or(0);
                    self.commit(v, cx);
                }
            }
            PickerEvent::Close => {
                if self.pending.take().is_some() {
                    cx.emit(DateFieldEvent::Cancel);
                }
            }
            PickerEvent::Change(_) => {}
        }
        cx.notify();
    }

    fn commit(&mut self, v: DateValue, cx: &mut Context<Self>) {
        self.value = Some(v);
        self.just_committed = true;
        cx.emit(DateFieldEvent::Change(v));
    }
}

fn time_columns() -> Vec<Vec<PickerItem>> {
    let two = |n: u32| PickerItem::new(format!("{n:02}"), n.to_string());
    vec![(0..24).map(two).collect(), (0..60).map(two).collect()]
}

/// Parses a year/month/day selection as produced by [`Picker::date`].
pub fn parse_date(items: &[PickerItem]) -> Option<DateValue> {
    let [y, m, d] = items else { return None };
    Some(DateValue::date(
        y.value.parse().ok()?,
        m.value.parse().ok()?,
        d.value.parse().ok()?,
    ))
}

impl Render for DateField {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx);
        let brand = t.brand;
        let text = self.value.map(|v| v.format(self.mode));
        let color = if text.is_some() { t.fg_0 } else { t.fg_2 };
        let glyph = t.fg_0;
        let name = SharedString::from(format!(
            "{}，{}",
            self.label,
            text.as_deref().unwrap_or("未选择")
        ));
        let shown: SharedString = text.map(Into::into).unwrap_or(self.placeholder.clone());
        let mut field = div()
            .id(self.id.clone())
            .role(Role::Button)
            .aria_label(name)
            .flex()
            .items_center()
            .justify_between()
            .rounded(px(4.))
            .text_color(color)
            .child(shown)
            .child(calendar_glyph(glyph));
        if self.disabled {
            field = field.opacity(0.5).aria_description("不可用");
        } else {
            field = field
                .focusable()
                .tab_stop(true)
                .cursor_pointer()
                .focus_visible(move |s| s.bg(brand.opacity(0.1)))
                .on_click(cx.listener(|this, _, window, cx| this.open(window, cx)));
        }
        div()
            .child(
                crate::accessibility::Semantics::new(field)
                    .disabled(self.disabled)
                    .reveal_focus(),
            )
            .children(self.date_sheet.clone())
            .children(self.time_sheet.clone())
    }
}

fn calendar_glyph(color: Hsla) -> Div {
    div()
        .flex_shrink_0()
        .size(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(13.))
                .h(px(13.))
                .rounded(px(1.5))
                .border_1()
                .border_t(px(3.5))
                .border_color(color),
        )
}

#[cfg(test)]
mod tests {
    use super::{DateFieldMode, DateValue, PickerItem, parse_date};

    #[test]
    fn formats_html_values() {
        let v = DateValue {
            year: 2027,
            month: 3,
            day: 4,
            hour: 5,
            minute: 6,
        };
        assert_eq!(v.format(DateFieldMode::Date), "2027-03-04");
        assert_eq!(v.format(DateFieldMode::DateTime), "2027-03-04 05:06");
    }

    #[test]
    fn parses_picker_date_values() {
        let items = [
            PickerItem::new("2026年", "2026"),
            PickerItem::new("2月", "2"),
            PickerItem::new("28日", "28"),
        ];
        assert_eq!(parse_date(&items), Some(DateValue::date(2026, 2, 28)));
        assert_eq!(parse_date(&items[..2]), None);
    }
}
