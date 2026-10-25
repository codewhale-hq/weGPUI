//! Native counterparts of WeUI's HTML `date` and `datetime-local` inputs.
//!
//! The default button opens a year/month/day [`Picker`], followed by hour/minute for
//! [`DateFieldMode::DateTime`]. [`DateField::set_typed_entry`] enables an editable ISO field:
//! Enter or blur commits valid text, Escape discards a draft, and Alt+Down or the calendar
//! button opens the sheets. Only the final sheet confirmation changes the value.
//!
//! Display remains `YYYY-MM-DD` / `YYYY-MM-DD HH:MM`, without locale conversion or seconds.
//! Gregorian years 1–9999 are supported; the default range is 1900–2100. Empty sheets start at
//! [`today`], which intentionally remains UTC pending the owner's policy.

use crate::{
    Theme,
    components::picker::{Picker, PickerEvent, PickerItem, days_in_month, today},
    input::{FieldSemantics, InputEvent, TextInput},
    scrolling::RevealFocus,
};
use gpui::{prelude::*, *};
use std::fmt;

/// Which HTML input the field stands in for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldMode {
    /// `type="date"`: year, month and day.
    Date,
    /// `type="datetime-local"`: date and hour/minute, without timezone conversion.
    DateTime,
}

/// A Gregorian date and optional time. Date mode normalizes time to midnight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateValue {
    /// Gregorian year, `1..=9999`.
    pub year: i32,
    /// Month, `1..=12`.
    pub month: u32,
    /// Day, valid for the month and year.
    pub day: u32,
    /// Hour, `0..=23`.
    pub hour: u32,
    /// Minute, `0..=59`.
    pub minute: u32,
}

/// Why a date draft or supplied value cannot be committed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateFieldError {
    /// Text does not use `YYYY-MM-DD` or `YYYY-MM-DD HH:MM` (`T` is also accepted).
    Format,
    /// The calendar date does not exist, or its year is outside 1–9999.
    Date,
    /// Hour or minute is outside its valid range.
    Time,
    /// A valid date lies outside the field's inclusive year range.
    OutOfRange {
        /// First permitted year.
        start: i32,
        /// Last permitted year.
        end: i32,
    },
    /// The field is disabled, read-only, or already editing a sheet.
    Unavailable,
}
impl fmt::Display for DateFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format => f.write_str("Use YYYY-MM-DD or YYYY-MM-DD HH:MM."),
            Self::Date => f.write_str("Enter a valid calendar date."),
            Self::Time => f.write_str("Use an hour from 00 to 23 and a minute from 00 to 59."),
            Self::OutOfRange { start, end } => write!(f, "Use a year from {start} to {end}."),
            Self::Unavailable => f.write_str("This date field cannot be edited."),
        }
    }
}
impl std::error::Error for DateFieldError {}

impl DateValue {
    /// Constructs a date at midnight. Use [`DateValue::validate`] to check supplied numbers.
    pub fn date(year: i32, month: u32, day: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour: 0,
            minute: 0,
        }
    }
    /// Checks the Gregorian date and time without applying a field's year range.
    pub fn validate(&self) -> Result<(), DateFieldError> {
        if !(1..=9999).contains(&self.year)
            || !(1..=12).contains(&self.month)
            || self.day == 0
            || self.day > days_in_month(self.year, self.month)
        {
            return Err(DateFieldError::Date);
        }
        if self.hour > 23 || self.minute > 59 {
            return Err(DateFieldError::Time);
        }
        Ok(())
    }
    /// Parses strict numeric ISO text, accepting a space or `T` before the time. Surrounding
    /// whitespace is ignored. [`DateField::commit_text`] handles empty text as clearing.
    pub fn parse(text: &str, mode: DateFieldMode) -> Result<Self, DateFieldError> {
        let text = text.trim();
        let bytes = text.as_bytes();
        let len = if mode == DateFieldMode::Date { 10 } else { 16 };
        if bytes.len() != len
            || bytes.get(4) != Some(&b'-')
            || bytes.get(7) != Some(&b'-')
            || bytes
                .iter()
                .enumerate()
                .any(|(i, b)| !matches!(i, 4 | 7 | 10 | 13) && !b.is_ascii_digit())
            || (mode == DateFieldMode::DateTime
                && (!matches!(bytes[10], b' ' | b'T') || bytes[13] != b':'))
        {
            return Err(DateFieldError::Format);
        }
        let number = |range: std::ops::Range<usize>| {
            text[range]
                .parse::<u32>()
                .map_err(|_| DateFieldError::Format)
        };
        let value = Self {
            year: number(0..4)? as i32,
            month: number(5..7)?,
            day: number(8..10)?,
            hour: if mode == DateFieldMode::DateTime {
                number(11..13)?
            } else {
                0
            },
            minute: if mode == DateFieldMode::DateTime {
                number(14..16)?
            } else {
                0
            },
        };
        value.validate()?;
        Ok(value)
    }
    /// Formats as `YYYY-MM-DD` or `YYYY-MM-DD HH:MM` without timezone conversion.
    pub fn format(&self, mode: DateFieldMode) -> String {
        let date = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);
        match mode {
            DateFieldMode::Date => date,
            DateFieldMode::DateTime => format!("{date} {:02}:{:02}", self.hour, self.minute),
        }
    }
    fn in_mode(mut self, mode: DateFieldMode) -> Self {
        if mode == DateFieldMode::Date {
            self.hour = 0;
            self.minute = 0;
        }
        self
    }
}
impl fmt::Display for DateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format(DateFieldMode::DateTime))
    }
}

/// Original nonempty-value events; existing subscribers keep the same variants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateFieldEvent {
    /// A sheet confirmed a value, or a typed edit changed a nonempty value.
    Change(DateValue),
    /// An edit was discarded, including cancellation by an owner setter. Once per edit.
    Cancel,
}
/// Value-change event including clearing typed text. Observe this when `None` matters;
/// [`DateFieldEvent::Change`] remains for existing nonempty subscribers. Programmatic setters
/// and confirming the current value do not emit this event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateFieldValueChanged(pub Option<DateValue>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Idle,
    Date,
    DateToTime,
    Time,
}

/// Reusable date/date-time entity with optional typed entry and picker sheets.
/// Default interaction remains a button (Enter/Space opens). Enable typed entry with
/// [`DateField::set_typed_entry`]. Invalid drafts stay visible, marked invalid with an
/// accessible error description, and never replace the committed value.
pub struct DateField {
    id: SharedString,
    label: SharedString,
    placeholder: Option<SharedString>,
    messages: std::sync::Arc<crate::i18n::Messages>,
    mode: DateFieldMode,
    value: Option<DateValue>,
    disabled: bool,
    readonly: bool,
    typed_entry: bool,
    input: Entity<TextInput>,
    dirty: bool,
    error: Option<DateFieldError>,
    repeat_error: bool,
    error_revision: u64,
    external_error: Option<SharedString>,
    focus: FocusHandle,
    window: Option<AnyWindowHandle>,
    years: (i32, i32),
    date_sheet: Option<Entity<Picker>>,
    time_sheet: Option<Entity<Picker>>,
    pending: Option<DateValue>,
    stage: Stage,
    generation: u64,
    _input_subscription: Subscription,
    _i18n_subscription: Subscription,
    _subscriptions: Vec<Subscription>,
}
impl EventEmitter<DateFieldEvent> for DateField {}
impl EventEmitter<DateFieldValueChanged> for DateField {}

impl DateField {
    /// Creates an empty picker-button field with a 1900–2100 year range.
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        mode: DateFieldMode,
        cx: &mut Context<Self>,
    ) -> Self {
        let label = label.into();
        let messages = crate::i18n::messages_arc(cx);
        let placeholder: SharedString = match mode {
            DateFieldMode::Date => messages.date_placeholder.clone(),
            DateFieldMode::DateTime => messages.datetime_placeholder.clone(),
        };
        let input = cx.new(|cx| {
            let mut input = TextInput::new(placeholder.clone(), cx);
            input.set_aria_label(label.clone(), cx);
            input
        });
        let subscription = cx.subscribe(&input, |this, input, event: &InputEvent, cx| {
            match event {
                InputEvent::Changed => {
                    this.dirty = input.read(cx).text() != this.formatted_value();
                    this.error = None;
                    this.repeat_error = false;
                    this.sync_semantics(cx);
                }
                InputEvent::Submit | InputEvent::Blur
                    if this.typed_entry
                        && this.stage == Stage::Idle
                        && this.dirty
                        && !this.disabled
                        && !this.readonly =>
                {
                    let _ = this.commit_text(cx);
                }
                _ => {}
            }
            cx.notify();
        });
        let i18n_subscription = cx.observe_global::<crate::i18n::I18n>(|this, cx| {
            this.messages = crate::i18n::messages_arc(cx);
            if this.placeholder.is_none() {
                let text = this.default_placeholder();
                this.input
                    .update(cx, |input, cx| input.set_placeholder(text, cx));
            }
            this.sync_semantics(cx);
            cx.notify();
        });
        Self {
            id: id.into(),
            label,
            placeholder: None,
            messages,
            mode,
            value: None,
            disabled: false,
            readonly: false,
            typed_entry: false,
            input,
            dirty: false,
            error: None,
            repeat_error: false,
            error_revision: 0,
            external_error: None,
            focus: cx.focus_handle().tab_stop(true),
            window: None,
            years: (1900, 2100),
            date_sheet: None,
            time_sheet: None,
            pending: None,
            stage: Stage::Idle,
            generation: 0,
            _input_subscription: subscription,
            _i18n_subscription: i18n_subscription,
            _subscriptions: vec![],
        }
    }
    /// Current committed value; invalid/incomplete drafts never replace it.
    pub fn value(&self) -> Option<DateValue> {
        self.value
    }
    /// The mode chosen at construction.
    pub fn mode(&self) -> DateFieldMode {
        self.mode
    }
    /// Underlying typed input for text-field integration.
    pub fn text_input(&self) -> &Entity<TextInput> {
        &self.input
    }
    /// Whether the field is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    /// Whether the field is read-only.
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }
    /// Current inclusive year range.
    pub fn year_range(&self) -> (i32, i32) {
        self.years
    }
    /// Sets a valid value without a change event, cancelling an unfinished edit. Invalid values
    /// leave the field unchanged; use [`DateField::try_set_value`] to inspect rejection.
    pub fn set_value(&mut self, value: Option<DateValue>, cx: &mut Context<Self>) {
        let _ = self.try_set_value(value, cx);
    }
    /// Validated counterpart of [`DateField::set_value`]. Date mode discards time fields.
    pub fn try_set_value(
        &mut self,
        value: Option<DateValue>,
        cx: &mut Context<Self>,
    ) -> Result<(), DateFieldError> {
        let value = value.map(|v| v.in_mode(self.mode));
        if let Some(v) = value {
            self.validate(v)?;
        }
        self.cancel_edit(cx);
        self.value = value;
        self.error = None;
        self.sync_text(cx);
        cx.notify();
        Ok(())
    }
    /// Sets the visible/accessible label and future sheet titles.
    pub fn set_label(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        let label = label.into();
        if self.label == label {
            return;
        }
        self.label = label;
        self.input
            .update(cx, |input, cx| input.set_aria_label(self.label.clone(), cx));
        cx.notify();
    }
    /// Sets the empty placeholder in both interaction modes.
    pub fn set_placeholder(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        self.placeholder = Some(text.clone());
        self.input
            .update(cx, |input, cx| input.set_placeholder(text, cx));
        cx.notify();
    }
    /// Enables optional typed ISO entry. Switching modes cancels unfinished editing.
    pub fn set_typed_entry(&mut self, enabled: bool, cx: &mut Context<Self>) {
        if self.typed_entry != enabled {
            self.cancel_edit(cx);
            self.typed_entry = enabled;
            self.focus = self
                .focus
                .clone()
                .tab_stop(!self.disabled && (!self.readonly || !enabled));
            self.sync_text(cx);
            cx.notify();
        }
    }
    /// Disables the field and cancels pending text/date/time editing; re-enabling preserves value.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if self.disabled != disabled {
            self.disabled = disabled;
            self.focus = self
                .focus
                .clone()
                .tab_stop(!disabled && (!self.readonly || !self.typed_entry));
            if disabled {
                self.cancel_edit(cx);
                self.sync_text(cx);
            }
            self.input
                .update(cx, |input, cx| input.set_disabled(disabled, cx));
            self.sync_semantics(cx);
            cx.notify();
        }
    }
    /// Makes the field read-only, preserving focusability while preventing text/sheet changes.
    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        if self.readonly != readonly {
            self.readonly = readonly;
            self.focus = self
                .focus
                .clone()
                .tab_stop(!self.disabled && (!readonly || !self.typed_entry));
            if readonly {
                self.cancel_edit(cx);
                self.sync_text(cx);
            }
            self.input
                .update(cx, |input, cx| input.set_readonly(readonly, cx));
            self.sync_semantics(cx);
            cx.notify();
        }
    }
    /// Sets inclusive years, reordered and clamped to 1–9999. An existing value outside a
    /// narrowed range is preserved and reported invalid until corrected.
    pub fn set_year_range(&mut self, start: i32, end: i32, cx: &mut Context<Self>) {
        let years = normalized_years(start, end);
        if self.years == years {
            return;
        }
        self.cancel_edit(cx);
        self.years = years;
        self.error = None;
        self.sync_text(cx);
        cx.notify();
    }
    /// Applies/clears an app-provided error description without changing the value.
    pub fn set_validation_error(&mut self, message: Option<SharedString>, cx: &mut Context<Self>) {
        if self.external_error == message {
            return;
        }
        self.external_error = message;
        self.repeat_error = false;
        self.sync_semantics(cx);
        cx.notify();
    }
    /// Format/calendar/range error for the draft or committed value. App-provided error text is
    /// separate; see [`DateField::set_validation_error`].
    pub fn validation_error(&self) -> Option<DateFieldError> {
        self.error
            .or_else(|| self.value.and_then(|v| self.validate(v).err()))
    }
    /// Whether there is an unfinished sheet sequence or uncommitted text draft.
    pub fn is_editing(&self, _: &App) -> bool {
        self.stage != Stage::Idle || self.dirty
    }
    /// Focuses the field unless disabled.
    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        if !self.disabled {
            window.focus(&self.focus_handle(cx), cx);
        }
    }
    /// Discards an unfinished edit and restores committed text/value; emits Cancel once.
    pub fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.window = Some(window.window_handle());
        self.cancel_edit(cx);
        self.error = None;
        self.sync_text(cx);
        cx.notify();
    }
    /// Commits valid typed text; empty text clears the field. Invalid drafts stay visible with
    /// an accessible error and leave value intact. Only value changes emit notifications.
    pub fn commit_text(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<Option<DateValue>, DateFieldError> {
        if self.disabled || self.readonly || self.stage != Stage::Idle {
            return Err(DateFieldError::Unavailable);
        }
        let text = self.input.read(cx).text().trim().to_owned();
        let parsed = if text.is_empty() {
            Ok(None)
        } else {
            DateValue::parse(&text, self.mode).and_then(|v| self.validate(v).map(|()| Some(v)))
        };
        match parsed {
            Ok(value) => {
                let changed = self.value != value;
                self.value = value;
                self.error = None;
                self.dirty = false;
                self.sync_text(cx);
                if changed {
                    if let Some(v) = value {
                        cx.emit(DateFieldEvent::Change(v));
                    }
                    cx.emit(DateFieldValueChanged(value));
                }
                cx.notify();
                Ok(value)
            }
            Err(error) => {
                let previous_description = self.description();
                self.error = Some(error);
                self.repeat_error = previous_description == self.description();
                if self.repeat_error {
                    // An unchanged description emits no event. A fresh alert gives
                    // repeated invalid submission feedback without changing value.
                    self.error_revision = self.error_revision.wrapping_add(1);
                }
                self.sync_semantics(cx);
                cx.notify();
                Err(error)
            }
        }
    }
    fn validate(&self, value: DateValue) -> Result<(), DateFieldError> {
        value.validate()?;
        if !(self.years.0..=self.years.1).contains(&value.year) {
            return Err(DateFieldError::OutOfRange {
                start: self.years.0,
                end: self.years.1,
            });
        }
        Ok(())
    }
    fn formatted_value(&self) -> String {
        self.value.map(|v| v.format(self.mode)).unwrap_or_default()
    }
    fn default_placeholder(&self) -> SharedString {
        match self.mode {
            DateFieldMode::Date => self.messages.date_placeholder.clone(),
            DateFieldMode::DateTime => self.messages.datetime_placeholder.clone(),
        }
    }
    fn placeholder_text(&self) -> SharedString {
        self.placeholder
            .clone()
            .unwrap_or_else(|| self.default_placeholder())
    }
    fn message(&self) -> Option<SharedString> {
        self.external_error.clone().or_else(|| {
            self.validation_error().map(|e| {
                let m = &self.messages;
                match (e, self.mode) {
                    (DateFieldError::Format, DateFieldMode::Date) => m.date_format_hint.clone(),
                    (DateFieldError::Format, DateFieldMode::DateTime) => {
                        m.datetime_format_hint.clone()
                    }
                    (DateFieldError::Date, _) => m.date_invalid.clone(),
                    (DateFieldError::Time, _) => m.time_invalid.clone(),
                    (DateFieldError::OutOfRange { start, end }, _) => {
                        crate::i18n::fill(&m.year_out_of_range, &[("start", &start), ("end", &end)])
                            .into()
                    }
                    (DateFieldError::Unavailable, _) => m.date_unavailable.clone(),
                }
            })
        })
    }
    fn description(&self) -> Option<SharedString> {
        let availability = if self.disabled {
            Some(self.messages.unavailable.clone())
        } else if self.readonly {
            Some(self.messages.read_only.clone())
        } else {
            None
        };
        match (self.message(), availability) {
            (Some(error), Some(state)) => Some(self.messages.join(&[&*error, &*state]).into()),
            (message, state) => message.or(state),
        }
    }
    fn sync_semantics(&self, cx: &mut Context<Self>) {
        let message = self.message();
        let description = self.description();
        self.input.update(cx, |input, cx| {
            input.set_invalid(message.is_some(), cx);
            input.set_aria_description(description, cx);
        });
    }
    fn sync_text(&mut self, cx: &mut Context<Self>) {
        let text = self.formatted_value();
        if self.input.read(cx).text() != text {
            self.input.update(cx, |input, cx| input.set_text(text, cx));
        }
        self.dirty = false;
        self.repeat_error = false;
        self.sync_semantics(cx);
    }
    fn cancel_edit(&mut self, cx: &mut Context<Self>) {
        let active = self.stage != Stage::Idle || self.dirty;
        self.stage = Stage::Idle;
        self.pending = None;
        self.dirty = false;
        self.generation = self.generation.wrapping_add(1);
        self._subscriptions.clear();
        let sheets: Vec<_> = [self.date_sheet.take(), self.time_sheet.take()]
            .into_iter()
            .flatten()
            .collect();
        if let Some(handle) = self.window {
            let disabled = self.disabled;
            let readonly_input = self.readonly && self.typed_entry;
            let focus = self.focus.clone();
            let input = self.input.clone();
            cx.defer(move |cx| {
                handle
                    .update(cx, |_, window, cx| {
                        for sheet in sheets {
                            sheet.update(cx, |p, cx| p.close(window, cx));
                        }
                        if readonly_input
                            && input.read(cx).is_readonly()
                            && focus.is_focused(window)
                        {
                            let target = input.read(cx).focus_handle(cx);
                            window.focus(&target, cx);
                        } else if disabled
                            && (focus.is_focused(window) || input.read(cx).is_focused(window))
                        {
                            // These callbacks run before drawing. Let one disabled frame
                            // finish so navigation cannot choose its stale calendar leaf.
                            window.on_next_frame(move |window, _| {
                                window.on_next_frame(move |window, cx| {
                                    if input.read(cx).is_disabled()
                                        && (focus.is_focused(window)
                                            || input.read(cx).is_focused(window)
                                            || window.focused(cx).is_none())
                                    {
                                        window.focus_next(cx);
                                    }
                                });
                            });
                        }
                    })
                    .ok();
            });
        }
        if active {
            cx.emit(DateFieldEvent::Cancel);
        }
    }
    fn ensure_sheets(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (y, m, d) = today();
        let start = self.value.unwrap_or(DateValue::date(y, m, d));
        let (lo, hi) = self.years;
        let generation = self.generation;
        let label = self.label.clone();
        let id = self.id.clone();
        let with_time = self.mode == DateFieldMode::DateTime;
        let choose_date = self.messages.choose_date.clone();
        let choose_time = self.messages.choose_time.clone();
        let date = cx.new(|cx| {
            let p = Picker::new(SharedString::from(format!("{id}-date")), cx)
                .date(lo, hi, (start.year.clamp(lo, hi), start.month, start.day))
                .title(label.clone());
            if with_time { p.desc(choose_date) } else { p }
        });
        self._subscriptions.push(cx.subscribe_in(
            &date,
            window,
            move |this, _, e: &PickerEvent, window, cx| {
                if this.generation == generation {
                    this.on_date(e, window, cx);
                }
            },
        ));
        if with_time {
            let time = cx.new(|cx| {
                Picker::new(SharedString::from(format!("{id}-time")), cx)
                    .columns(time_columns())
                    .default_value([start.hour.to_string(), start.minute.to_string()])
                    .title(label)
                    .desc(choose_time)
            });
            self._subscriptions.push(cx.subscribe_in(
                &time,
                window,
                move |this, _, e: &PickerEvent, _, cx| {
                    if this.generation == generation {
                        this.on_time(e, cx);
                    }
                },
            ));
            self.time_sheet = Some(time);
        }
        self.date_sheet = Some(date);
    }
    /// Opens a fresh date sheet unless disabled, read-only or already editing a sheet. A typed
    /// draft is discarded first; cancelled picker selections never become next-open defaults.
    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.readonly || self.stage != Stage::Idle {
            return;
        }
        self.window = Some(window.window_handle());
        self.cancel_edit(cx);
        self.error = None;
        self.sync_text(cx);
        self.ensure_sheets(window, cx);
        self.stage = Stage::Date;
        if let Some(date) = self.date_sheet.clone() {
            date.update(cx, |p, cx| p.open(window, cx));
        }
        cx.notify();
    }
    fn on_date(&mut self, e: &PickerEvent, window: &mut Window, cx: &mut Context<Self>) {
        match e {
            PickerEvent::Confirm(items)
                if self.stage == Stage::Date && !self.disabled && !self.readonly =>
            {
                if let Some(mut v) = parse_date(items).filter(|v| self.validate(*v).is_ok()) {
                    if self.mode == DateFieldMode::Date {
                        self.commit(v, cx);
                    } else {
                        if let Some(old) = self.value {
                            v.hour = old.hour;
                            v.minute = old.minute;
                        }
                        self.pending = Some(v);
                        self.stage = Stage::DateToTime;
                    }
                }
            }
            PickerEvent::Close if self.stage == Stage::DateToTime => {
                let generation = self.generation;
                let owner = cx.weak_entity();
                window.defer(cx, move |window, cx| {
                    owner
                        .update(cx, |this, cx| {
                            if this.generation == generation
                                && this.stage == Stage::DateToTime
                                && !this.disabled
                                && !this.readonly
                            {
                                this.stage = Stage::Time;
                                if let Some(time) = this.time_sheet.clone() {
                                    time.update(cx, |p, cx| p.open(window, cx));
                                }
                                cx.notify();
                            }
                        })
                        .ok();
                });
            }
            PickerEvent::Close if self.stage == Stage::Date => {
                self.stage = Stage::Idle;
                cx.emit(DateFieldEvent::Cancel);
            }
            _ => {}
        }
        cx.notify();
    }
    fn on_time(&mut self, e: &PickerEvent, cx: &mut Context<Self>) {
        match e {
            PickerEvent::Confirm(items)
                if self.stage == Stage::Time && !self.disabled && !self.readonly =>
            {
                if let (Some(mut v), [h, m]) = (self.pending, items.as_slice())
                    && let (Ok(hour), Ok(minute)) = (h.value.parse(), m.value.parse())
                {
                    v.hour = hour;
                    v.minute = minute;
                    if self.validate(v).is_ok() {
                        self.commit(v, cx);
                    }
                }
            }
            PickerEvent::Close if self.stage == Stage::Time => {
                self.stage = Stage::Idle;
                self.pending = None;
                cx.emit(DateFieldEvent::Cancel);
            }
            _ => {}
        }
        cx.notify();
    }
    fn commit(&mut self, v: DateValue, cx: &mut Context<Self>) {
        let changed = self.value != Some(v);
        self.value = Some(v);
        self.stage = Stage::Idle;
        self.pending = None;
        self.error = None;
        self.sync_text(cx);
        cx.emit(DateFieldEvent::Change(v));
        if changed {
            cx.emit(DateFieldValueChanged(Some(v)));
        }
    }
}
impl Focusable for DateField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // Both presentations are the same field. Keeping their handle stable also
        // preserves focus when an owner switches a currently focused field's mode.
        self.input.read(cx).focus_handle(cx)
    }
}
fn normalized_years(start: i32, end: i32) -> (i32, i32) {
    (start.min(end).clamp(1, 9999), start.max(end).clamp(1, 9999))
}
fn time_columns() -> Vec<Vec<PickerItem>> {
    let two = |n: u32| PickerItem::new(format!("{n:02}"), n.to_string());
    vec![(0..24).map(two).collect(), (0..60).map(two).collect()]
}
/// Parses and validates a year/month/day selection as produced by [`Picker::date`].
pub fn parse_date(items: &[PickerItem]) -> Option<DateValue> {
    let [y, m, d] = items else { return None };
    let value = DateValue::date(
        y.value.parse().ok()?,
        m.value.parse().ok()?,
        d.value.parse().ok()?,
    );
    value.validate().ok()?;
    Some(value)
}

impl Render for DateField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.window = Some(window.window_handle());
        let t = Theme::get(cx).clone();
        let message = self.message();
        let invalid = message.is_some();
        // Orca already presents a changed description on its focused field. Keep
        // live alerts for other cases, including repeated unchanged errors.
        let description_announces = cfg!(target_os = "linux")
            && self.input.read(cx).is_focused(window)
            && !self.repeat_error;
        let text = self.value.map(|v| v.format(self.mode));
        let color = if invalid {
            t.red
        } else if self.readonly {
            t.fg_1
        } else if text.is_some() {
            t.fg_0
        } else {
            t.fg_2
        };
        let name: SharedString = self
            .messages
            .join(&[
                &*self.label,
                text.as_deref().unwrap_or(&self.messages.not_selected),
            ])
            .into();
        let field = if self.typed_entry {
            let calendar = div()
                .id((ElementId::from(self.id.clone()), "calendar"))
                .role(Role::Button)
                .aria_label(crate::i18n::fill(
                    &self.messages.open_date_picker,
                    &[("label", &self.label)],
                ))
                .when(self.disabled || self.readonly, |el| {
                    el.aria_description(if self.disabled {
                        self.messages.unavailable.clone()
                    } else {
                        self.messages.read_only.clone()
                    })
                })
                .p(px(4.))
                .rounded(px(4.))
                .child(calendar_glyph(if self.readonly { t.fg_1 } else { t.fg_0 }))
                .when(!self.disabled && !self.readonly, |el| {
                    el.track_focus(&self.focus)
                        .cursor_pointer()
                        .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
                        .on_click(cx.listener(|this, _, window, cx| this.open(window, cx)))
                });
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .text_color(if invalid {
                    t.red
                } else if self.readonly {
                    t.fg_1
                } else {
                    t.fg_0
                })
                .when(self.disabled, |el| el.opacity(0.5))
                .child(self.input.clone())
                .child(
                    FieldSemantics {
                        inner: calendar,
                        disabled: self.disabled || self.readonly,
                        readonly: self.readonly,
                        invalid,
                    }
                    .reveal_focus(),
                )
        } else {
            let field_focus = self.input.read(cx).focus_handle(cx);
            let field = div()
                .id(self.id.clone())
                .role(Role::Button)
                .aria_label(name)
                .flex()
                .items_center()
                .justify_between()
                .rounded(px(4.))
                .text_color(color)
                .when_some(self.description(), |el, description| {
                    el.aria_description(description)
                })
                .when(self.disabled, |el| el.opacity(0.5))
                .when(!self.disabled, |el| {
                    el.track_focus(&field_focus)
                        .focus_visible(move |s| s.shadow(super::form::dual_focus_ring()))
                        .when(!self.readonly, |el| {
                            el.cursor_pointer()
                                .on_click(cx.listener(|this, _, window, cx| this.open(window, cx)))
                        })
                })
                .child(
                    text.map(Into::<SharedString>::into)
                        .unwrap_or(self.placeholder_text()),
                )
                .child(calendar_glyph(if self.readonly { t.fg_1 } else { t.fg_0 }));
            div().child(
                FieldSemantics {
                    inner: field,
                    disabled: self.disabled,
                    readonly: self.readonly,
                    invalid,
                }
                .reveal_focus(),
            )
        };
        div()
            .w_full()
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                if this.typed_entry && !this.disabled && !this.readonly {
                    if event.keystroke.key == "escape" && this.dirty && this.stage == Stage::Idle {
                        this.cancel(window, cx);
                        cx.stop_propagation();
                    } else if event.keystroke.key == "down" && event.keystroke.modifiers.alt {
                        this.open(window, cx);
                        cx.stop_propagation();
                    }
                }
            }))
            .child(field)
            .when_some(message, |el, message| {
                el.child(
                    crate::accessibility::Semantics::new(
                        div()
                            .id((
                                ElementId::from(self.id.clone()),
                                format!("error-{}", self.error_revision),
                            ))
                            .role(Role::Alert)
                            .aria_label(message.clone())
                            .text_size(px(12.))
                            .text_color(t.red)
                            .child(message),
                    )
                    .live(if description_announces {
                        accesskit::Live::Off
                    } else {
                        accesskit::Live::Assertive
                    }),
                )
            })
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
    use super::{
        DateFieldError, DateFieldMode, DateValue, PickerItem, normalized_years, parse_date,
    };
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
    fn parses_and_validates_calendar_and_time_text() {
        let date = DateFieldMode::Date;
        let time = DateFieldMode::DateTime;
        assert_eq!(
            DateValue::parse("2000-02-29", date),
            Ok(DateValue::date(2000, 2, 29))
        );
        assert_eq!(
            DateValue::parse("1900-02-29", date),
            Err(DateFieldError::Date)
        );
        for text in ["2026-02-30", "2026-00-01", "2026-13-01", "0000-01-01"] {
            assert_eq!(DateValue::parse(text, date), Err(DateFieldError::Date));
        }
        for text in [
            "2026-2-01",
            "2026/02/01",
            "２０２６-02-01",
            "2026-02-01extra",
        ] {
            assert_eq!(DateValue::parse(text, date), Err(DateFieldError::Format));
        }
        assert!(DateValue::parse("2026-12-31T23:59", time).is_ok());
        assert!(DateValue::parse(" 2026-12-31 23:59 ", time).is_ok());
        assert_eq!(
            DateValue::parse("2026-12-31 24:00", time),
            Err(DateFieldError::Time)
        );
        assert_eq!(
            DateValue::parse("2026-12-31 12:60", time),
            Err(DateFieldError::Time)
        );
    }
    #[test]
    fn picker_values_must_describe_a_real_date() {
        let values = |day| {
            [
                PickerItem::new("2026年", "2026"),
                PickerItem::new("2月", "2"),
                PickerItem::new("日", day),
            ]
        };
        assert_eq!(
            parse_date(&values("28")),
            Some(DateValue::date(2026, 2, 28))
        );
        assert_eq!(parse_date(&values("30")), None);
        assert_eq!(parse_date(&values("28")[..2]), None);
    }
    #[test]
    fn date_mode_normalizes_time_and_range_bounds_are_safe() {
        let v = DateValue {
            year: 2026,
            month: 1,
            day: 1,
            hour: 25,
            minute: 60,
        };
        assert_eq!(v.in_mode(DateFieldMode::Date), DateValue::date(2026, 1, 1));
        assert_eq!(normalized_years(2100, 1900), (1900, 2100));
        assert_eq!(normalized_years(i32::MIN, i32::MAX), (1, 9999));
    }
}
