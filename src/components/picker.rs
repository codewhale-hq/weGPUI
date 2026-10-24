use super::{
    button::Button,
    half_screen_dialog::{HalfScreenDialog, HdIconButton},
    modal::{Presence, ease},
};
use crate::Theme;
use gpui::{prelude::*, *};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

pub const ROW_HEIGHT: f32 = 56.;
const OFFSET: f32 = 2.;
const VISIBLE_ROWS: f32 = 5.;
const SNAP: Duration = Duration::from_millis(300);
const WHEEL_IDLE: Duration = Duration::from_millis(120);

#[derive(Clone, Debug, PartialEq)]
pub struct PickerItem {
    pub label: SharedString,
    pub value: SharedString,
    pub disabled: bool,
    pub children: Vec<PickerItem>,
}

impl PickerItem {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
            children: vec![],
        }
    }
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn children(mut self, children: impl IntoIterator<Item = PickerItem>) -> Self {
        self.children = children.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub enum PickerEvent {
    Change(Vec<PickerItem>),
    Confirm(Vec<PickerItem>),
    Close,
}

#[derive(Clone)]
enum Source {
    Columns(Vec<Vec<PickerItem>>),
    Cascade {
        items: Vec<PickerItem>,
        depth: usize,
    },
}

struct Drag {
    start: f32,
    last: f32,
    last_time: Instant,
    history: Vec<(Instant, f32)>,
}

struct Column {
    items: Vec<PickerItem>,
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
        (OFFSET - (self.offset / ROW_HEIGHT).round()).max(0.) as usize
    }
    fn max_offset(&self) -> f32 {
        OFFSET * ROW_HEIGHT
    }
    fn min_offset(&self) -> f32 {
        -(ROW_HEIGHT * (self.items.len() as f32 - OFFSET - 1.))
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

pub struct Picker {
    presence: Presence,
    id: SharedString,
    title: SharedString,
    desc: SharedString,
    show_close: bool,
    confirm_text: SharedString,
    close_text: SharedString,
    source: Source,
    default_value: Vec<SharedString>,
    memory: Vec<Option<usize>>,
    columns: Vec<Column>,
}

impl EventEmitter<PickerEvent> for Picker {}

impl Picker {
    pub fn new(id: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            presence: Presence::new(cx),
            id: id.into(),
            title: SharedString::default(),
            desc: SharedString::default(),
            show_close: true,
            confirm_text: "确定".into(),
            close_text: "关闭".into(),
            source: Source::Columns(vec![]),
            default_value: vec![],
            memory: vec![],
            columns: vec![],
        }
    }
    pub fn single(
        id: impl Into<SharedString>,
        items: impl IntoIterator<Item = PickerItem>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(id, cx).columns(vec![items.into_iter().collect()])
    }
    pub fn columns(mut self, columns: Vec<Vec<PickerItem>>) -> Self {
        self.source = Source::Columns(columns);
        self
    }
    pub fn cascade(mut self, items: Vec<PickerItem>, depth: usize) -> Self {
        self.source = Source::Cascade { items, depth };
        self
    }
    pub fn date(mut self, start_year: i32, end_year: i32, today: (i32, u32, u32)) -> Self {
        let years = (start_year..=end_year)
            .map(|y| {
                PickerItem::new(format!("{y}年"), y.to_string()).children((1..=12u32).map(|m| {
                    PickerItem::new(format!("{m}月"), m.to_string()).children(
                        (1..=days_in_month(y, m))
                            .map(|d| PickerItem::new(format!("{d}日"), d.to_string())),
                    )
                }))
            })
            .collect();
        self.source = Source::Cascade {
            items: years,
            depth: 3,
        };
        self.default_value = vec![
            today.0.to_string().into(),
            today.1.to_string().into(),
            today.2.to_string().into(),
        ];
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = desc.into();
        self
    }
    pub fn show_close(mut self, value: bool) -> Self {
        self.show_close = value;
        self
    }
    pub fn confirm_text(mut self, text: impl Into<SharedString>) -> Self {
        self.confirm_text = text.into();
        self
    }
    pub fn close_text(mut self, text: impl Into<SharedString>) -> Self {
        self.close_text = text.into();
        self
    }
    pub fn default_value(
        mut self,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.default_value = values.into_iter().map(Into::into).collect();
        self
    }
    pub fn is_open(&self) -> bool {
        self.presence.is_open()
    }
    pub fn selected(&self) -> Vec<PickerItem> {
        self.columns
            .iter()
            .filter_map(|c| c.items.get(c.index()).cloned())
            .collect()
    }
    pub fn indices(&self) -> Vec<usize> {
        self.columns.iter().map(Column::index).collect()
    }

    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rebuild(cx);
        self.presence.show(window, cx);
        if let Some(first) = self.columns.first() {
            window.focus(&first.focus.clone(), cx);
        }
        cx.notify();
    }
    pub fn open_instantly(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rebuild(cx);
        self.presence.show_instantly(window, cx);
        if let Some(first) = self.columns.first() {
            window.focus(&first.focus.clone(), cx);
        }
        cx.notify();
    }
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.presence.is_open() {
            self.presence.hide(window, cx);
            cx.emit(PickerEvent::Close);
            cx.notify();
        }
    }
    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PickerEvent::Confirm(self.selected()));
        self.close(window, cx);
    }

    fn initial_index(&self, level: usize, items: &[PickerItem]) -> usize {
        if let Some(Some(i)) = self.memory.get(level)
            && *i < items.len()
        {
            return *i;
        }
        if let Some(value) = self.default_value.get(level)
            && let Some(i) = items.iter().position(|it| &it.value == value)
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
        let (focus, bounds) = match self.columns.get(level) {
            Some(c) => (c.focus.clone(), c.bounds.clone()),
            None => (
                cx.focus_handle().tab_stop(true),
                Rc::new(Cell::new(Bounds::default())),
            ),
        };
        Column {
            items,
            offset,
            from: offset,
            anim: None,
            drag: None,
            wheel: 0,
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
        }
    }

    fn rebuild_from(&mut self, level: usize, cx: &mut App) {
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
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        let before = col.index();
        let visual = col.visual(window, cx);
        let mut k = col.offset + delta;
        k = (k / ROW_HEIGHT).round() * ROW_HEIGHT;
        k = k.min(col.max_offset()).max(col.min_offset());
        let len = col.items.len() as isize;
        let mut i = (OFFSET - k / ROW_HEIGHT).round() as isize;
        let mut guard = 0;
        while i >= 0 && i < len && col.items[i as usize].disabled && guard < len {
            if dir > 0. {
                i -= 1
            } else {
                i += 1
            }
            guard += 1;
        }
        if i < 0 || i >= len || col.items[i as usize].disabled {
            i = before as isize;
        }
        col.from = visual;
        col.offset = (OFFSET - i as f32) * ROW_HEIGHT;
        col.anim = Some(Instant::now());
        let after = i as usize;
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

    fn drag_start(&mut self, level: usize, y: f32, window: &Window, cx: &mut Context<Self>) {
        if let Some(col) = self.columns.get_mut(level) {
            let visual = col.visual(window, cx);
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
        let Some(col) = self.columns.get_mut(level) else {
            return;
        };
        let visual = col.visual(window, cx);
        col.anim = None;
        col.offset = (visual + dy)
            .min(col.max_offset() + ROW_HEIGHT / 2.)
            .max(col.min_offset() - ROW_HEIGHT / 2.);
        col.wheel += 1;
        col.wheel_dir = dy;
        let generation = col.wheel;
        cx.notify();
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(WHEEL_IDLE).await;
            this.update_in(cx, |this, window, cx| {
                if let Some(col) = this.columns.get(level)
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

    fn render_column(
        &mut self,
        level: usize,
        count: usize,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let t = Theme::get(cx).clone();
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
        let selected = col
            .items
            .get(index)
            .map(|it| it.label.clone())
            .unwrap_or_default();
        let bounds = col.bounds.clone();
        let focus = col.focus.clone();
        let first = level == 0;
        let last = level + 1 == count;
        let brand = t.brand;
        let items = col.items.iter().enumerate().map(|(i, item)| {
            div()
                .id(("weui-picker-item", i))
                .role(Role::ListBoxOption)
                .aria_label(item.label.clone())
                .aria_selected(i == index)
                .h(px(ROW_HEIGHT))
                .line_height(px(ROW_HEIGHT))
                .text_center()
                .whitespace_nowrap()
                .overflow_hidden()
                .text_ellipsis()
                .text_color(if item.disabled { t.fg_1 } else { t.fg_0 })
                .child(item.label.clone())
        });
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
                    .left(px(if first { 8. } else { 0. }))
                    .right(px(if last { 8. } else { 0. }))
                    .when(first, |el| el.rounded_l(px(8.)))
                    .when(last, |el| el.rounded_r(px(8.)))
                    .bg(t.bg_3),
            )
            .child(
                div()
                    .absolute()
                    .top(px(offset))
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
                    .left(px(if first { 8. } else { 0. }))
                    .right(px(if last { 8. } else { 0. }))
                    .when(first, |el| el.rounded_l(px(8.)))
                    .when(last, |el| el.rounded_r(px(8.)))
                    .border_2()
                    .border_color(transparent_black())
                    .focus_visible(move |s| s.border_color(brand)),
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
                let rows = match e.keystroke.key.as_str() {
                    "up" => -1.,
                    "down" => 1.,
                    "pageup" => -VISIBLE_ROWS,
                    "pagedown" => VISIBLE_ROWS,
                    "home" => -10000.,
                    "end" => 10000.,
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
        let columns: Vec<AnyElement> = (0..count)
            .map(|level| self.render_column(level, count, window, cx))
            .collect();
        let entity = cx.entity();
        let close_entity = entity.clone();
        let confirm_entity = entity.clone();
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
                        .relative()
                        .h(px(VISIBLE_ROWS * ROW_HEIGHT))
                        .overflow_hidden()
                        .children(columns),
                )
                .footer(
                    Button::new(self.confirm_text.clone())
                        .id(SharedString::from(format!(
                            "weui-picker-confirm-{}",
                            self.id
                        )))
                        .primary()
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
                    self.close_text.clone(),
                )
                .on_click(move |_, window, cx| {
                    close_entity.update(cx, |this, cx| this.close(window, cx));
                }),
            );
        }
        dialog.into_any_element()
    }
}

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

pub fn today() -> (i32, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let z = secs.div_euclid(86_400) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = (yoe + era * 400 + if m <= 2 { 1 } else { 0 }) as i32;
    (y, m, d)
}
