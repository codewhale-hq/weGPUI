//! WeUI picker (`weui-picker/weui-picker.less`) with the column, cascade and date behaviour of the
//! weui.js 1.2.17 `weui.picker` and `weui.datePicker` calls the upstream examples make, shown in a
//! flush [`HalfScreenDialog`].
//!
//! [`Picker`] is a GPUI entity emitting [`PickerEvent`]s; [`PickerItem`] trees describe the
//! options. [`today`] uses the host's local date. A column whose rows are all disabled keeps
//! Confirm disabled.

use super::{
    button::Button,
    half_screen_dialog::{HalfScreenDialog, HdIconButton},
    modal::{Presence, ease},
};
use crate::{Theme, accessibility::Semantics, logical::Logical};
use gpui::{prelude::*, *};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

/// Height of one picker row in px (`@pickerItemHeight`). Five rows are visible, with the current
/// row in the middle.
pub const ROW_HEIGHT: f32 = 56.;
const OFFSET: f32 = 2.;
const VISIBLE_ROWS: f32 = 5.;
const SNAP: Duration = Duration::from_millis(300);
const WHEEL_IDLE: Duration = Duration::from_millis(120);

/// One option in a picker column; in cascade mode its `children` fill the next column.
#[derive(Clone, Debug, PartialEq)]
pub struct PickerItem {
    /// Text shown in the row; also its accessible name.
    pub label: SharedString,
    /// Value matched by [`Picker::default_value`]; returned with the item in events.
    pub value: SharedString,
    /// Drawn greyed and skipped by dragging, the wheel and keys; never returned by
    /// [`Picker::selected`].
    pub disabled: bool,
    /// Options for the next column while this item is selected in a cascade; ignored by plain
    /// columns.
    pub children: Vec<PickerItem>,
}

impl PickerItem {
    /// Creates an enabled item with no children.
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
            children: vec![],
        }
    }
    /// Sets whether the item is disabled (default `false`).
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Replaces the child options used by cascade pickers.
    pub fn children(mut self, children: impl IntoIterator<Item = PickerItem>) -> Self {
        self.children = children.into_iter().collect();
        self
    }
}

/// Events emitted by [`Picker`]; observe them with `cx.subscribe`.
#[derive(Clone, Debug)]
pub enum PickerEvent {
    /// A column's current row changed by drag, wheel or keys. Carries [`Picker::selected`] after
    /// the dependent cascade columns were rebuilt.
    Change(Vec<PickerItem>),
    /// Confirm was pressed, or Enter in a column, while [`Picker::can_confirm`] held. Carries the
    /// selected items and is followed by `Close`.
    Confirm(Vec<PickerItem>),
    /// The picker closed: close button, Escape, mask click, [`Picker::close`] or after a confirm.
    Close,
}

#[derive(Clone)]
enum Source {
    Columns(Vec<Vec<PickerItem>>),
    Cascade {
        items: Vec<PickerItem>,
        depth: usize,
    },
    Date {
        start: i32,
        end: i32,
    },
}

enum ColumnItems {
    Stored(Vec<PickerItem>),
    Years {
        start: i32,
        end: i32,
        label: SharedString,
    },
}

impl ColumnItems {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn len(&self) -> usize {
        match self {
            Self::Stored(items) => items.len(),
            Self::Years { start, end, .. } => (i64::from(*end) - i64::from(*start) + 1)
                .max(0)
                .try_into()
                .unwrap_or(usize::MAX),
        }
    }
    fn get(&self, index: usize) -> Option<PickerItem> {
        if index >= self.len() {
            return None;
        }
        match self {
            Self::Stored(items) => items.get(index).cloned(),
            Self::Years { start, label, .. } => {
                let year = i64::from(*start) + index as i64;
                Some(PickerItem::new(
                    crate::i18n::fill(label, &[("year", &year)]),
                    year.to_string(),
                ))
            }
        }
    }
    fn enabled_at(&self, index: usize) -> bool {
        match self {
            Self::Stored(items) => enabled_at(items, index),
            Self::Years { .. } => index < self.len(),
        }
    }
    fn years(&self) -> bool {
        matches!(self, Self::Years { .. })
    }
}

struct Drag {
    start: f32,
    last: f32,
    last_time: Instant,
    history: Vec<(Instant, f32)>,
}

struct Column {
    items: ColumnItems,
    year_index: usize,
    offset: f32,
    from: f32,
    anim: Option<Instant>,
    drag: Option<Drag>,
    wheel: usize,
    wheel_dir: f32,
    focus: FocusHandle,
    bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl Column {
    fn index(&self) -> usize {
        if self.items.years() {
            self.year_index
        } else {
            (OFFSET - (self.offset / ROW_HEIGHT).round()).max(0.) as usize
        }
    }
    fn max_offset(&self) -> f32 {
        if self.items.years() {
            (OFFSET + self.year_index as f32) * ROW_HEIGHT
        } else {
            OFFSET * ROW_HEIGHT
        }
    }
    fn min_offset(&self) -> f32 {
        if self.items.years() {
            (OFFSET - self.items.len().saturating_sub(self.year_index + 1) as f32) * ROW_HEIGHT
        } else {
            -(ROW_HEIGHT * (self.items.len() as f32 - OFFSET - 1.))
        }
    }
    fn current(&self, cx: &App) -> f32 {
        match self.anim {
            Some(at) if !cx.reduce_motion() => {
                let p = (at.elapsed().as_secs_f32() / SNAP.as_secs_f32()).min(1.);
                self.from + (self.offset - self.from) * ease(p)
            }
            _ => self.offset,
        }
    }
    fn visual(&mut self, window: &Window, cx: &App) -> f32 {
        match self.anim {
            Some(at) if !cx.reduce_motion() => {
                let p = (at.elapsed().as_secs_f32() / SNAP.as_secs_f32()).min(1.);
                if p < 1. {
                    window.request_animation_frame();
                    self.from + (self.offset - self.from) * ease(p)
                } else {
                    self.anim = None;
                    self.offset
                }
            }
            _ => self.offset,
        }
    }
}

fn enabled_at(items: &[PickerItem], index: usize) -> bool {
    items.get(index).is_some_and(|item| !item.disabled)
}

fn default_index(items: &[PickerItem]) -> usize {
    let len = items.len();
    if len == 0 {
        return 0;
    }
    let mut i = len / 2;
    let mut n = 0;
    while items[i].disabled && n < len {
        i = (i + 1) % len;
        n += 1;
    }
    i
}

/// Wheel picker entity rendered as a bottom sheet with a title, columns and a Confirm button.
///
/// Create it with `cx.new(|cx| Picker::single(...))`, or `Picker::new` plus `columns`, `cascade` or
/// `date`; render the entity in your view and call [`Picker::open`]. Columns are rebuilt on every
/// open. Each level remembers its last row for the next open, falling back to
/// [`Picker::default_value`] and then to the first enabled row from the middle; a disabled row is
/// never chosen while an enabled one exists. Keys in a focused column: Up and Down move one row,
/// PageUp and PageDown five, Home and End to the ends, and Enter confirms. A move that lands on a
/// disabled row continues in the direction of travel, then searches back for the nearest enabled
/// row.
pub struct Picker {
    presence: Presence,
    id: SharedString,
    title: SharedString,
    desc: SharedString,
    show_close: bool,
    confirm_text: Option<SharedString>,
    close_text: Option<SharedString>,
    source: Source,
    default_value: Vec<SharedString>,
    memory: Vec<Option<usize>>,
    columns: Vec<Column>,
}

impl EventEmitter<PickerEvent> for Picker {}

impl Picker {
    /// Creates an empty, closed picker with the localized [`crate::Messages::ok`] Confirm text,
    /// the localized [`crate::Messages::close`] close button label and the close button shown.
    pub fn new(id: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            presence: Presence::new(cx),
            id: id.into(),
            title: SharedString::default(),
            desc: SharedString::default(),
            show_close: true,
            confirm_text: None,
            close_text: None,
            source: Source::Columns(vec![]),
            default_value: vec![],
            memory: vec![],
            columns: vec![],
        }
    }
    /// Creates a one-column picker from `items`.
    pub fn single(
        id: impl Into<SharedString>,
        items: impl IntoIterator<Item = PickerItem>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(id, cx).columns(vec![items.into_iter().collect()])
    }
    /// Uses independent columns, like the weui.js multi-column picker; replaces any previous
    /// source.
    pub fn columns(mut self, columns: Vec<Vec<PickerItem>>) -> Self {
        self.source = Source::Columns(columns);
        self
    }
    /// Uses a cascading tree: each column lists the children of the row selected to its left, up to
    /// `depth` columns or until an item has no children. Replaces any previous source.
    pub fn cascade(mut self, items: Vec<PickerItem>, depth: usize) -> Self {
        self.source = Source::Cascade { items, depth };
        self
    }
    /// Uses a year, month and day cascade over `start_year..=end_year`, labelled by the locale's
    /// [`crate::Messages::year_label`], [`crate::Messages::months`] and
    /// [`crate::Messages::day_label`] with unpadded numeric values, and makes `today`
    /// (`(year, month, day)`) the default value. Pass the user's local date; [`today`] reads the host local clock.
    /// Years are generated as visible rows, and months/days only for the current selection.
    /// Reversed bounds produce an empty year column, as an empty inclusive Rust range does.
    pub fn date(mut self, start_year: i32, end_year: i32, today: (i32, u32, u32)) -> Self {
        self.source = Source::Date {
            start: start_year,
            end: end_year,
        };
        self.default_value = vec![
            today.0.to_string().into(),
            today.1.to_string().into(),
            today.2.to_string().into(),
        ];
        self
    }
    /// Sets the sheet title, also part of each column's accessible name (default empty).
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    /// Sets a subtitle under the title (default none).
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = desc.into();
        self
    }
    /// Whether to show the header close button (default `true`).
    pub fn show_close(mut self, value: bool) -> Self {
        self.show_close = value;
        self
    }
    /// Sets the Confirm button text (default: the localized [`crate::Messages::ok`]).
    pub fn confirm_text(mut self, text: impl Into<SharedString>) -> Self {
        self.confirm_text = Some(text.into());
        self
    }
    /// Sets the close button's accessible label (default: the localized
    /// [`crate::Messages::close`]).
    pub fn close_text(mut self, text: impl Into<SharedString>) -> Self {
        self.close_text = Some(text.into());
        self
    }
    /// Sets, per column level, the item value to select when that level has no remembered row.
    /// Missing or disabled values fall back to the first enabled row from the middle.
    pub fn default_value(
        mut self,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.default_value = values.into_iter().map(Into::into).collect();
        self
    }
    /// Whether the picker is open; `false` as soon as closing starts.
    pub fn is_open(&self) -> bool {
        self.presence.is_open()
    }
    /// Items at each column's current row, omitting disabled ones, so the list can be shorter than
    /// the column count. Empty before the first [`Picker::open`]. Date rows contain their label
    /// and value without eagerly generated `children`; ordinary cascade rows retain theirs.
    pub fn selected(&self) -> Vec<PickerItem> {
        self.columns
            .iter()
            .filter_map(|c| c.items.get(c.index()).filter(|item| !item.disabled))
            .collect()
    }
    /// Current row index of every column, including rows that are disabled. Empty before the first
    /// open.
    pub fn indices(&self) -> Vec<usize> {
        self.columns.iter().map(Column::index).collect()
    }
    /// `true` only when there is at least one column and every column's current row exists and is
    /// enabled. The Confirm button is disabled otherwise.
    pub fn can_confirm(&self) -> bool {
        !self.columns.is_empty()
            && self
                .columns
                .iter()
                .all(|column| column.items.enabled_at(column.index()))
    }

    /// Rebuilds the columns, slides the sheet up and focuses the first column. The previously
    /// focused element gets focus back when the picker closes.
    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rebuild(cx);
        self.presence.show(window, cx);
        if let Some(first) = self
            .columns
            .get(self.display_order(cx).first().copied().unwrap_or(0))
        {
            window.focus(&first.focus.clone(), cx);
        }
        cx.notify();
    }
    /// Like [`Picker::open`] without the opening transition.
    pub fn open_instantly(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rebuild(cx);
        self.presence.show_instantly(window, cx);
        if let Some(first) = self
            .columns
            .get(self.display_order(cx).first().copied().unwrap_or(0))
        {
            window.focus(&first.focus.clone(), cx);
        }
        cx.notify();
    }
    /// Slides the sheet down and emits [`PickerEvent::Close`]; does nothing when already closed.
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.presence.is_open() {
            self.presence.hide(window, cx);
            cx.emit(PickerEvent::Close);
            cx.notify();
        }
    }
    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_open() || !self.can_confirm() {
            return;
        }
        cx.emit(PickerEvent::Confirm(self.selected()));
        self.close(window, cx);
    }

    fn initial_index(&self, level: usize, items: &[PickerItem]) -> usize {
        if let Some(Some(i)) = self.memory.get(level)
            && enabled_at(items, *i)
        {
            return *i;
        }
        if let Some(value) = self.default_value.get(level)
            && let Some(i) = items.iter().position(|it| &it.value == value)
            && enabled_at(items, i)
        {
            return i;
        }
        default_index(items)
    }

    fn make_column(&mut self, level: usize, items: Vec<PickerItem>, cx: &mut App) -> Column {
        let index = self.initial_index(level, &items);
        if self.memory.len() <= level {
            self.memory.resize(level + 1, None);
        }
        self.memory[level] = Some(index);
        let offset = (OFFSET - index as f32) * ROW_HEIGHT;
        let (focus, bounds, wheel) = match self.columns.get(level) {
            Some(c) => (c.focus.clone(), c.bounds.clone(), c.wheel.wrapping_add(1)),
            None => (
                cx.focus_handle().tab_stop(true),
                Rc::new(Cell::new(Bounds::default())),
                0,
            ),
        };
        Column {
            items: ColumnItems::Stored(items),
            year_index: 0,
            offset,
            from: offset,
            anim: None,
            drag: None,
            wheel,
            wheel_dir: 0.,
            focus,
            bounds,
        }
    }

    fn rebuild(&mut self, cx: &mut App) {
        match self.source.clone() {
            Source::Columns(cols) => {
                let built: Vec<Column> = cols
                    .into_iter()
                    .enumerate()
                    .map(|(level, items)| self.make_column(level, items, cx))
                    .collect();
                self.columns = built;
            }
            Source::Cascade { items, .. } => {
                let col = self.make_column(0, items, cx);
                self.columns.clear();
                self.columns.push(col);
                self.rebuild_from(0, cx);
            }
            Source::Date { start, end } => {
                let items = ColumnItems::Years {
                    start,
                    end,
                    label: crate::i18n::messages(cx).year_label.clone(),
                };
                let len = items.len();
                let index = self
                    .memory
                    .first()
                    .copied()
                    .flatten()
                    .filter(|i| *i < len)
                    .or_else(|| {
                        let year: i32 = self.default_value.first()?.parse().ok()?;
                        (start..=end)
                            .contains(&year)
                            .then_some((i64::from(year) - i64::from(start)) as usize)
                    })
                    .unwrap_or(len / 2);
                let mut column = self.make_column(0, vec![], cx);
                column.items = items;
                column.year_index = index;
                column.offset = OFFSET * ROW_HEIGHT;
                column.from = column.offset;
                self.memory[0] = Some(index);
                self.columns = vec![column];
                self.rebuild_from(0, cx);
            }
        }
    }

    fn rebuild_from(&mut self, level: usize, cx: &mut App) {
        if matches!(self.source, Source::Date { .. }) {
            let Some(year) = self.columns[0]
                .items
                .get(self.columns[0].index())
                .and_then(|item| item.value.parse::<i32>().ok())
            else {
                self.columns.truncate(1);
                return;
            };
            if level == 0 {
                let names = crate::i18n::messages_arc(cx);
                let months = (1..=12u32)
                    .map(|month| {
                        PickerItem::new(names.months[month as usize - 1].clone(), month.to_string())
                    })
                    .collect();
                let column = self.make_column(1, months, cx);
                if self.columns.len() > 1 {
                    self.columns[1] = column;
                } else {
                    self.columns.push(column);
                }
            }
            if level <= 1 {
                let month = self.columns[1].index() as u32 + 1;
                let day_label = crate::i18n::messages(cx).day_label.clone();
                let days = (1..=days_in_month(year, month))
                    .map(|day| {
                        PickerItem::new(
                            crate::i18n::fill(&day_label, &[("day", &day)]),
                            day.to_string(),
                        )
                    })
                    .collect::<Vec<_>>();
                if let Some(Some(day)) = self.memory.get_mut(2) {
                    *day = (*day).min(days.len() - 1);
                }
                let column = self.make_column(2, days, cx);
                self.columns.truncate(2);
                self.columns.push(column);
            }
            return;
        }
        let Source::Cascade { depth, .. } = self.source else {
            return;
        };
        let mut level = level;
        loop {
            let children = self.columns[level]
                .items
                .get(self.columns[level].index())
                .map(|it| it.children.clone())
                .unwrap_or_default();
            if children.is_empty() || level + 1 >= depth {
                self.columns.truncate(level + 1);
                return;
            }
            let col = self.make_column(level + 1, children, cx);
            if self.columns.len() > level + 1 {
                self.columns[level + 1] = col;
            } else {
                self.columns.push(col);
            }
            level += 1;
        }
    }

    fn snap(
        &mut self,
        level: usize,
        delta: f32,
        dir: f32,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_open() {
            return;
        }
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        let before = col.index();
        let mut k = col.offset + delta;
        k = (k / ROW_HEIGHT).round() * ROW_HEIGHT;
        k = k.min(col.max_offset()).max(col.min_offset());
        let len = col.items.len() as isize;
        if len == 0 {
            return;
        }
        let target = if col.items.years() {
            (before as isize + (OFFSET - k / ROW_HEIGHT).round() as isize).clamp(0, len - 1)
        } else {
            (OFFSET - k / ROW_HEIGHT).round() as isize
        };
        let forward = if dir > 0. { -1 } else { 1 };
        let enabled_from = |step: isize| {
            let mut j = target;
            while j >= 0 && j < len {
                if col.items.enabled_at(j as usize) {
                    return Some(j);
                }
                j += step;
            }
            None
        };
        let i = enabled_from(forward)
            .or_else(|| enabled_from(-forward))
            .unwrap_or(before as isize);
        self.set_index(level, i as usize, cx);
    }

    fn set_index(&mut self, level: usize, after: usize, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        if !col.items.enabled_at(after) {
            return;
        }
        let before = col.index();
        let visual = col.current(cx);
        if col.items.years() {
            let distance = after as i64 - before as i64;
            col.from = if distance.unsigned_abs() <= VISIBLE_ROWS as u64 {
                visual + distance as f32 * ROW_HEIGHT
            } else {
                OFFSET * ROW_HEIGHT
            };
            col.offset = OFFSET * ROW_HEIGHT;
            col.year_index = after;
        } else {
            col.from = visual;
            col.offset = (OFFSET - after as f32) * ROW_HEIGHT;
        }
        col.anim = Some(Instant::now());
        if after != before {
            self.memory[level] = Some(after);
            self.rebuild_from(level, cx);
            cx.emit(PickerEvent::Change(self.selected()));
        }
        cx.notify();
    }

    fn step(&mut self, level: usize, rows: f32, window: &Window, cx: &mut Context<Self>) {
        self.snap(level, -rows * ROW_HEIGHT, -rows, window, cx);
    }

    fn drag_start(&mut self, level: usize, y: f32, _window: &Window, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        if let Some(col) = self.columns.get_mut(level) {
            let visual = col.current(cx);
            col.offset = visual;
            col.anim = None;
            col.drag = Some(Drag {
                start: y,
                last: y,
                last_time: Instant::now(),
                history: vec![],
            });
        }
    }

    fn drag_move(&mut self, level: usize, y: f32, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        if let Some(col) = self.columns.get_mut(level)
            && let Some(drag) = col.drag.as_mut()
        {
            drag.last = y;
            drag.last_time = Instant::now();
            drag.history.push((drag.last_time, y));
            if drag.history.len() > 40 {
                drag.history.remove(0);
            }
            cx.notify();
        }
    }

    fn drag_end(&mut self, level: usize, y: f32, window: &Window, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        let Some(drag) = col.drag.take() else {
            return;
        };
        let bounds = col.bounds.get();
        let center = bounds.origin.y.as_f32() + VISIBLE_ROWS * ROW_HEIGHT / 2.;
        let moved = y - drag.start;
        let delta = if drag.last_time.elapsed() > Duration::from_millis(100) {
            if moved.abs() > 10. { moved } else { center - y }
        } else if moved.abs() > 10. {
            let last = drag.history.len().saturating_sub(1);
            let mut first = last;
            let mut r = last;
            while r > 0
                && drag.last_time.duration_since(drag.history[r].0) < Duration::from_millis(100)
            {
                first = r;
                r -= 1;
            }
            if first != last {
                let (t1, y1) = drag.history[last];
                let (t0, y0) = drag.history[first];
                let dt = t1.duration_since(t0).as_secs_f32() * 1000.;
                let v = if dt > 0. { (y1 - y0) / dt } else { 0. };
                150. * v + moved
            } else {
                moved
            }
        } else {
            center - y
        };
        self.snap(level, delta, delta, window, cx);
    }

    fn wheel(&mut self, level: usize, dy: f32, window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_open() {
            return;
        }
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        let visual = col.current(cx);
        col.anim = None;
        col.offset = (visual + dy)
            .min(col.max_offset() + ROW_HEIGHT / 2.)
            .max(col.min_offset() - ROW_HEIGHT / 2.);
        col.wheel += 1;
        col.wheel_dir = dy;
        let generation = col.wheel;
        let opening = self.presence.generation();
        cx.notify();
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(WHEEL_IDLE).await;
            this.update_in(cx, |this, window, cx| {
                if this.is_open()
                    && this.presence.generation() == opening
                    && let Some(col) = this.columns.get(level)
                    && col.wheel == generation
                {
                    let dir = col.wheel_dir;
                    this.snap(level, 0., dir, window, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    fn display_order(&self, cx: &App) -> Vec<usize> {
        if matches!(self.source, Source::Date { .. }) {
            super::date_field::DatePresentation::for_locale(
                cx.global::<crate::i18n::I18n>().locale(),
            )
            .order
            .into_iter()
            .filter(|i| *i < self.columns.len())
            .collect()
        } else {
            (0..self.columns.len()).collect()
        }
    }

    fn render_column(
        &mut self,
        level: usize,
        count: usize,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let t = Theme::get(cx).clone();
        let dir = crate::i18n::direction(cx);
        let dark = t.is_dark;
        let shade = |a: f32| -> Hsla {
            if dark {
                Rgba {
                    r: 25. / 255.,
                    g: 25. / 255.,
                    b: 25. / 255.,
                    a,
                }
                .into()
            } else {
                Rgba {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                    a,
                }
                .into()
            }
        };
        let title = self.title.clone();
        let col = &mut self.columns[level];
        let mut offset = col.visual(window, cx);
        if let Some(drag) = col.drag.as_ref() {
            offset = col.offset + (drag.last - drag.start);
        }
        let index = col.index();
        let selected = col.items.get(index).map(|it| it.label).unwrap_or_default();
        let bounds = col.bounds.clone();
        let focus = col.focus.clone();
        let keyboard_focus = focus.is_focused(window) && window.last_input_was_keyboard();
        let first = level == 0;
        let last = level + 1 == count;
        let brand = t.focus_color(t.surface_color(t.bg_3, t.bg_2));
        let row_range = if col.items.years() && !col.items.is_empty() {
            let visual_index = (index as i64 + (OFFSET - offset / ROW_HEIGHT).round() as i64)
                .clamp(0, col.items.len() as i64 - 1) as usize;
            visual_index.saturating_sub(4)..(visual_index + 5).min(col.items.len())
        } else {
            0..col.items.len()
        };
        let render_offset = if col.items.years() {
            offset + (row_range.start as i64 - index as i64) as f32 * ROW_HEIGHT
        } else {
            offset
        };
        let item_count = col.items.len();
        let items: Vec<_> = row_range
            .filter_map(|i| col.items.get(i).map(|item| (i, item)))
            .map(|(i, item)| {
                Semantics::new(
                    div()
                        .id(("weui-picker-item", i))
                        .role(Role::ListBoxOption)
                        .aria_label(item.label.clone())
                        .aria_selected(i == index)
                        .aria_position_in_set(i)
                        .aria_size_of_set(item_count)
                        .h(px(ROW_HEIGHT))
                        .line_height(px(ROW_HEIGHT))
                        .text_center()
                        .whitespace_nowrap()
                        .overflow_hidden()
                        .text_ellipsis()
                        .text_color(if item.disabled { t.fg_1 } else { t.fg_0 })
                        .child(item.label),
                )
                .disabled(item.disabled)
            })
            .collect();
        div()
            .id(("weui-picker-group", level))
            .role(Role::ListBox)
            .aria_label(SharedString::from(format!("{} {}", title, selected)))
            .track_focus(&focus)
            .tab_stop(true)
            .relative()
            .flex_1()
            .h_full()
            .overflow_hidden()
            .text_size(px(17.))
            .child(
                canvas(move |b, _, _| bounds.set(b), |_, _, _, _| {})
                    .absolute()
                    .size_full(),
            )
            .child(
                div()
                    .absolute()
                    .top(px(OFFSET * ROW_HEIGHT))
                    .h(px(ROW_HEIGHT))
                    .inset_s(dir, px(if first { 8. } else { 0. }))
                    .inset_e(dir, px(if last { 8. } else { 0. }))
                    .when(first && !dir.is_rtl(), |el| el.rounded_l(px(8.)))
                    .when(first && dir.is_rtl(), |el| el.rounded_r(px(8.)))
                    .when(last && !dir.is_rtl(), |el| el.rounded_r(px(8.)))
                    .when(last && dir.is_rtl(), |el| el.rounded_l(px(8.)))
                    .bg(t.bg_3),
            )
            .child(
                div()
                    .absolute()
                    .top(px(render_offset))
                    .left_0()
                    .w_full()
                    .flex()
                    .flex_col()
                    .children(items),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .w_full()
                    .h(px(OFFSET * ROW_HEIGHT))
                    .bg(linear_gradient(
                        180.,
                        linear_color_stop(shade(0.95), 0.),
                        linear_color_stop(shade(0.6), 1.),
                    )),
            )
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .w_full()
                    .h(px(OFFSET * ROW_HEIGHT))
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(shade(0.95), 0.),
                        linear_color_stop(shade(0.6), 1.),
                    )),
            )
            .child(
                div()
                    .absolute()
                    .top(px(OFFSET * ROW_HEIGHT))
                    .h(px(ROW_HEIGHT))
                    .inset_s(dir, px(if first { 8. } else { 0. }))
                    .inset_e(dir, px(if last { 8. } else { 0. }))
                    .when(first && !dir.is_rtl(), |el| el.rounded_l(px(8.)))
                    .when(first && dir.is_rtl(), |el| el.rounded_r(px(8.)))
                    .when(last && !dir.is_rtl(), |el| el.rounded_r(px(8.)))
                    .when(last && dir.is_rtl(), |el| el.rounded_l(px(8.)))
                    .border_2()
                    .border_color(transparent_black())
                    .when(keyboard_focus, |s| s.border_color(brand)),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, e: &MouseDownEvent, window, cx| {
                    this.drag_start(level, e.position.y.as_f32(), window, cx);
                    cx.stop_propagation();
                }),
            )
            .on_mouse_move(cx.listener(move |this, e: &MouseMoveEvent, _, cx| {
                if e.pressed_button == Some(MouseButton::Left) {
                    this.drag_move(level, e.position.y.as_f32(), cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, e: &MouseUpEvent, window, cx| {
                    this.drag_end(level, e.position.y.as_f32(), window, cx);
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(move |this, e: &MouseUpEvent, window, cx| {
                    this.drag_end(level, e.position.y.as_f32(), window, cx);
                }),
            )
            .on_scroll_wheel(cx.listener(move |this, e: &ScrollWheelEvent, window, cx| {
                let dy = e.delta.pixel_delta(px(ROW_HEIGHT)).y.as_f32();
                this.wheel(level, dy, window, cx);
                cx.stop_propagation();
            }))
            .on_key_down(cx.listener(move |this, e: &KeyDownEvent, window, cx| {
                if matches!(e.keystroke.key.as_str(), "home" | "end") {
                    if let Some(column) = this.columns.get(level) {
                        let mut indices = 0..column.items.len();
                        let target = if e.keystroke.key == "home" {
                            indices.find(|i| column.items.enabled_at(*i))
                        } else {
                            indices.rev().find(|i| column.items.enabled_at(*i))
                        };
                        if let Some(target) = target {
                            this.set_index(level, target, cx);
                        }
                    }
                    cx.stop_propagation();
                    return;
                }
                let rows = match e.keystroke.key.as_str() {
                    "up" => -1.,
                    "down" => 1.,
                    "pageup" => -VISIBLE_ROWS,
                    "pagedown" => VISIBLE_ROWS,
                    "enter" => {
                        this.confirm(window, cx);
                        cx.stop_propagation();
                        return;
                    }
                    _ => return,
                };
                this.step(level, rows, window, cx);
                cx.stop_propagation();
            }))
            .into_any_element()
    }
}

impl Render for Picker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.presence.progress(window, cx).is_none() {
            return div().into_any_element();
        }
        let count = self.columns.len();
        let columns: Vec<AnyElement> = self
            .display_order(cx)
            .into_iter()
            .map(|level| self.render_column(level, count, window, cx))
            .collect();
        let entity = cx.entity();
        let close_entity = entity.clone();
        let confirm_entity = entity.clone();
        let can_confirm = self.can_confirm();
        let dir = crate::i18n::direction(cx);
        let mut dialog =
            HalfScreenDialog::new(SharedString::from(format!("weui-picker-{}", self.id)))
                .flush(true)
                .title(self.title.clone())
                .presence(&self.presence)
                .on_dismiss(move |window, cx| {
                    entity.update(cx, |this, cx| this.close(window, cx));
                })
                .child(
                    div()
                        .flex()
                        .row(dir)
                        .relative()
                        .h(px(VISIBLE_ROWS * ROW_HEIGHT))
                        .overflow_hidden()
                        .children(columns),
                )
                .footer(
                    Button::new(
                        self.confirm_text
                            .clone()
                            .unwrap_or_else(|| crate::i18n::messages(cx).ok.clone()),
                    )
                    .id(SharedString::from(format!(
                        "weui-picker-confirm-{}",
                        self.id
                    )))
                    .primary()
                    .disabled(!can_confirm)
                    .on_click(move |_, window, cx| {
                        confirm_entity.update(cx, |this, cx| this.confirm(window, cx));
                    }),
                );
        if !self.desc.is_empty() {
            dialog = dialog.subtitle(self.desc.clone());
        }
        if self.show_close {
            dialog = dialog.left(
                HdIconButton::new(
                    SharedString::from(format!("weui-picker-close-{}", self.id)),
                    "close-thin",
                    self.close_text
                        .clone()
                        .unwrap_or_else(|| crate::i18n::messages(cx).close.clone()),
                )
                .on_click(move |_, window, cx| {
                    close_entity.update(cx, |this, cx| this.close(window, cx));
                }),
            );
        }
        dialog.into_any_element()
    }
}

/// Days in `month` (1 to 12) of `year` in the proleptic Gregorian calendar. Any other month value
/// is treated as February.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
    }
}

/// Today's Gregorian date as `(year, month, day)` in the host's local time zone.
pub fn today() -> (i32, u32, u32) {
    use chrono::Datelike;
    let date = chrono::Local::now();
    (date.year(), date.month(), date.day())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn generated_years_cover_full_i32_range_without_a_year_vector() {
        let years = ColumnItems::Years {
            start: i32::MIN,
            end: i32::MAX,
            label: "{year}".into(),
        };
        assert_eq!(years.len(), 1usize << 32);
        assert_eq!(years.get(0).unwrap().value.as_ref(), "-2147483648");
        assert_eq!(
            years.get(years.len() - 1).unwrap().value.as_ref(),
            "2147483647"
        );
        assert!(years.get(years.len()).is_none());
        assert!(years.get(2_147_485_648).unwrap().children.is_empty());
        assert_eq!(years.get(2_147_485_648).unwrap().value.as_ref(), "2000");
    }

    #[test]
    fn generated_year_ranges_are_inclusive_and_reversed_ranges_are_empty() {
        let years = ColumnItems::Years {
            start: 1900,
            end: 2100,
            label: "{year}".into(),
        };
        assert_eq!(years.len(), 201);
        assert_eq!(years.get(200).unwrap().value.as_ref(), "2100");
        assert!(
            ColumnItems::Years {
                start: 2100,
                end: 1900,
                label: "{year}".into(),
            }
            .is_empty()
        );
    }

    #[test]
    fn disabled_default_wraps_to_an_enabled_option() {
        let items = vec![
            PickerItem::new("enabled", "0"),
            PickerItem::new("middle", "1").disabled(true),
            PickerItem::new("last", "2").disabled(true),
        ];
        assert_eq!(default_index(&items), 0);
        assert!(enabled_at(&items, default_index(&items)));
    }

    #[test]
    fn empty_and_all_disabled_columns_have_no_enabled_default() {
        assert_eq!(default_index(&[]), 0);
        assert!(!enabled_at(&[], 0));
        let items = vec![PickerItem::new("disabled", "0").disabled(true)];
        assert!(!enabled_at(&items, default_index(&items)));
        assert!(!enabled_at(&items, usize::MAX));
    }

    #[test]
    fn gregorian_centuries_and_month_lengths() {
        assert_eq!(days_in_month(1900, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        for month in [4, 6, 9, 11] {
            assert_eq!(days_in_month(2024, month), 30);
        }
        for month in [1, 3, 5, 7, 8, 10, 12] {
            assert_eq!(days_in_month(2024, month), 31);
        }
    }
}
