use crate::{FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString, Size, point, px};
use collections::FxHashMap;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use super::{LineWrapper, logical_wrap, text_bidi};

/// A laid out and styled line of text
#[derive(Default, Debug)]
pub struct LineLayout {
    /// The font size for this line
    pub font_size: Pixels,
    /// The width of the line
    pub width: Pixels,
    /// The ascent of the line
    pub ascent: Pixels,
    /// The descent of the line
    pub descent: Pixels,
    /// The shaped runs that make up this line
    pub runs: Vec<ShapedRun>,
    /// The length of the line in utf-8 bytes
    pub len: usize,
}

/// A run of text that has been shaped .
#[derive(Debug, Clone)]
pub struct ShapedRun {
    /// The font id for this run
    pub font_id: FontId,
    /// The glyphs that make up this run
    pub glyphs: Vec<ShapedGlyph>,
}

/// A single glyph, ready to paint.
#[derive(Clone, Debug)]
pub struct ShapedGlyph {
    /// The ID for this glyph, as determined by the text system.
    pub id: GlyphId,

    /// The position of this glyph in its containing line.
    pub position: Point<Pixels>,

    /// The index of this glyph in the original text.
    pub index: usize,

    /// Whether this glyph is an emoji
    pub is_emoji: bool,
}

impl LineLayout {
    fn normalize_visual_order(&mut self) {
        let mut previous = px(f32::NEG_INFINITY);
        let ordered = self.runs.iter().flat_map(|run| &run.glyphs).all(|glyph| {
            let ordered = glyph.position.x >= previous;
            previous = glyph.position.x;
            ordered
        });
        if ordered {
            return;
        }
        let mut glyphs: Vec<_> = std::mem::take(&mut self.runs)
            .into_iter()
            .flat_map(|run| {
                run.glyphs
                    .into_iter()
                    .map(move |glyph| (run.font_id, glyph))
            })
            .collect();
        glyphs.sort_by(|(_, a), (_, b)| a.position.x.as_f32().total_cmp(&b.position.x.as_f32()));
        for (font_id, glyph) in glyphs {
            if let Some(run) = self.runs.last_mut().filter(|run| run.font_id == font_id) {
                run.glyphs.push(glyph);
            } else {
                self.runs.push(ShapedRun {
                    font_id,
                    glyphs: vec![glyph],
                });
            }
        }
    }

    /// The index for the character at the given x coordinate
    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        if x >= self.width {
            None
        } else {
            for run in self.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    /// closest_index_for_x returns the character boundary closest to the given x coordinate
    /// (e.g. to handle aligning up/down arrow keys)
    pub fn closest_index_for_x(&self, x: Pixels) -> usize {
        let mut prev_index = 0;
        let mut prev_x = px(0.);

        for run in self.runs.iter() {
            for glyph in run.glyphs.iter() {
                if glyph.position.x >= x {
                    if glyph.position.x - x < x - prev_x {
                        return glyph.index;
                    } else {
                        return prev_index;
                    }
                }
                prev_index = glyph.index;
                prev_x = glyph.position.x;
            }
        }

        if self.len == 1 {
            if x > self.width / 2. {
                return 1;
            } else {
                return 0;
            }
        }

        self.len
    }

    /// The x position of the character at the given index
    pub fn x_for_index(&self, index: usize) -> Pixels {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x;
                }
            }
        }
        self.width
    }

    /// The corresponding Font at the given index
    pub fn font_id_for_index(&self, index: usize) -> Option<FontId> {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return Some(run.font_id);
                }
            }
        }
        None
    }

    /// Split this layout at a byte index, returning `(prefix, suffix)`.
    ///
    /// - `prefix` contains glyphs for bytes `[0, byte_index)` with original positions.
    ///   Its width equals the x-advance up to the split point.
    /// - `suffix` contains glyphs for bytes `[byte_index, len)` with positions
    ///   shifted left so the first glyph starts at x=0, and byte indices rebased to 0.
    /// - `font_size`, `ascent`, and `descent` are copied to both halves.
    pub fn split_at(&self, byte_index: usize) -> (LineLayout, LineLayout) {
        let x_offset = self.x_for_index(byte_index);

        // Partition glyph runs. A single run may contribute glyphs to both halves.
        let mut left_runs = Vec::new();
        let mut right_runs = Vec::new();

        for run in &self.runs {
            let split_pos = run.glyphs.partition_point(|g| g.index < byte_index);

            if split_pos > 0 {
                left_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: run.glyphs[..split_pos].to_vec(),
                });
            }

            if split_pos < run.glyphs.len() {
                let right_glyphs = run.glyphs[split_pos..]
                    .iter()
                    .map(|g| ShapedGlyph {
                        id: g.id,
                        position: point(g.position.x - x_offset, g.position.y),
                        index: g.index - byte_index,
                        is_emoji: g.is_emoji,
                    })
                    .collect();
                right_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: right_glyphs,
                });
            }
        }

        let left = LineLayout {
            font_size: self.font_size,
            width: x_offset,
            ascent: self.ascent,
            descent: self.descent,
            runs: left_runs,
            len: byte_index,
        };

        let right = LineLayout {
            font_size: self.font_size,
            width: self.width - x_offset,
            ascent: self.ascent,
            descent: self.descent,
            runs: right_runs,
            len: self.len - byte_index,
        };

        (left, right)
    }

    fn is_visually_reordered(&self) -> bool {
        let mut previous = 0;
        for glyph in self.runs.iter().flat_map(|run| run.glyphs.iter()) {
            if glyph.index < previous {
                return true;
            }
            previous = glyph.index;
        }
        false
    }

    fn compute_wrap_boundaries(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
    ) -> SmallVec<[WrapBoundary; 1]> {
        let mut boundaries = SmallVec::new();
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = px(0.);
        let mut last_boundary = WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_boundary_x = px(0.);
        let breaks = LineWrapper::break_indices(text);
        let mut previous_index = None;
        let mut glyphs = self
            .runs
            .iter()
            .enumerate()
            .flat_map(move |(run_ix, run)| {
                run.glyphs.iter().enumerate().map(move |(glyph_ix, glyph)| {
                    let character = text[glyph.index..].chars().next().unwrap();
                    (
                        WrapBoundary { run_ix, glyph_ix },
                        character,
                        glyph.position.x,
                        glyph.index,
                    )
                })
            })
            .filter(move |(_, _, _, index)| previous_index.replace(*index) != Some(*index))
            .peekable();

        while let Some((boundary, ch, x, index)) = glyphs.next() {
            if ch == '\n' {
                continue;
            }

            if breaks.binary_search(&index).is_ok() && first_non_whitespace_ix.is_some() {
                last_candidate_ix = Some(boundary);
                last_candidate_x = x;
            }

            if ch != ' ' && first_non_whitespace_ix.is_none() {
                first_non_whitespace_ix = Some(boundary);
            }

            let next_x = glyphs.peek().map_or(self.width, |(_, _, x, _)| *x);
            let width = next_x - last_boundary_x;

            if width > wrap_width
                && boundary > last_boundary
                && (ch != ' ' || first_non_whitespace_ix.is_none())
            {
                // When used line_clamp, we should limit the number of lines.
                if let Some(max_lines) = max_lines
                    && boundaries.len() >= max_lines.saturating_sub(1)
                {
                    break;
                }

                if let Some(last_candidate_ix) = last_candidate_ix.take() {
                    last_boundary = last_candidate_ix;
                    last_boundary_x = last_candidate_x;
                } else {
                    last_boundary = boundary;
                    last_boundary_x = x;
                }
                boundaries.push(last_boundary);
            }
        }

        boundaries
    }
}

/// A line of text that has been wrapped to fit a given width
#[derive(Default, Debug)]
pub struct WrappedLineLayout {
    /// The line layout, pre-wrapping.
    pub unwrapped_layout: Arc<LineLayout>,

    /// The boundaries at which the line was wrapped
    pub wrap_boundaries: SmallVec<[WrapBoundary; 1]>,

    /// The width of the line, if it was wrapped
    pub wrap_width: Option<Pixels>,

    pub(crate) logical_line_starts: SmallVec<[usize; 1]>,
    pub(crate) text: SharedString,
    pub(crate) bidi_rows: Vec<text_bidi::Row>,
}

/// A boundary at which a line was wrapped
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapBoundary {
    /// The index in the run just before the line was wrapped
    pub run_ix: usize,
    /// The index of the glyph just before the line was wrapped
    pub glyph_ix: usize,
}

impl WrappedLineLayout {
    #[doc = "Cached visual clusters and logical ranges for each wrapped row."]
    pub fn bidi_rows(&self) -> &[text_bidi::Row] {
        &self.bidi_rows
    }

    #[doc = "Returns separate visual rectangles for the selected byte range."]
    pub fn selection_bounds(
        &self,
        range: Range<usize>,
        line_height: Pixels,
    ) -> Vec<crate::Bounds<Pixels>> {
        self.bidi_rows
            .iter()
            .enumerate()
            .flat_map(|(row, metrics)| {
                let lo = range.start.max(metrics.range.start);
                let hi = range.end.min(metrics.range.end);
                if lo >= hi {
                    return Vec::new();
                }
                text_bidi::selection_spans(
                    &metrics.clusters,
                    &self.text[metrics.range.clone()],
                    lo - metrics.range.start,
                    hi - metrics.range.start,
                )
                .into_iter()
                .map(|(x0, x1)| {
                    crate::Bounds::new(
                        point(px(x0), line_height * row as f32),
                        crate::size(px(x1 - x0), line_height),
                    )
                })
                .collect()
            })
            .collect()
    }

    /// The length of the underlying text, in utf8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.unwrapped_layout.len
    }

    /// The width of this line, in pixels, whether or not it was wrapped.
    pub fn width(&self) -> Pixels {
        self.wrap_width
            .unwrap_or(Pixels::MAX)
            .min(self.unwrapped_layout.width)
    }

    /// The size of the whole wrapped text, for the given line_height.
    /// can span multiple lines if there are multiple wrap boundaries.
    pub fn size(&self, line_height: Pixels) -> Size<Pixels> {
        Size {
            width: self.width(),
            height: line_height * (self.wrap_boundaries.len() + 1),
        }
    }

    /// The ascent of a line in this layout
    pub fn ascent(&self) -> Pixels {
        self.unwrapped_layout.ascent
    }

    /// The descent of a line in this layout
    pub fn descent(&self) -> Pixels {
        self.unwrapped_layout.descent
    }

    /// The wrap boundaries in this layout
    pub fn wrap_boundaries(&self) -> &[WrapBoundary] {
        &self.wrap_boundaries
    }

    /// The byte index at which the given wrapped row starts; row 0 starts at 0 and the row after
    /// the last one starts at the end of the text. Prefer this to reading the index of the glyph
    /// at a wrap boundary, which is the logical start of a row only for left-to-right rows.
    pub fn wrapped_line_start_index(&self, row: usize) -> usize {
        if row == 0 {
            return 0;
        }
        if let Some(&index) = self.logical_line_starts.get(row - 1) {
            return index;
        }
        match self.wrap_boundaries.get(row - 1) {
            Some(boundary) => {
                self.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
            }
            None => self.unwrapped_layout.len,
        }
    }

    /// The font size of this layout
    pub fn font_size(&self) -> Pixels {
        self.unwrapped_layout.font_size
    }

    /// The runs in this layout, sans wrapping
    pub fn runs(&self) -> &[ShapedRun] {
        &self.unwrapped_layout.runs
    }

    /// The index corresponding to a given position in this layout for the given line height.
    ///
    /// See also [`Self::closest_index_for_position`].
    pub fn index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, false)
    }

    /// The closest index to a given position in this layout for the given line height.
    ///
    /// Closest means the character boundary closest to the given position.
    ///
    /// See also [`LineLayout::closest_index_for_x`].
    pub fn closest_index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, true)
    }

    fn _index_for_position(
        &self,
        mut position: Point<Pixels>,
        line_height: Pixels,
        closest: bool,
    ) -> Result<usize, usize> {
        if !self.bidi_rows.is_empty() {
            let row_index =
                ((position.y.max(px(0.)) / line_height) as usize).min(self.bidi_rows.len() - 1);
            let row = &self.bidi_rows[row_index];
            let text = &self.text[row.range.clone()];
            let x = f32::from(position.x);
            let index = if closest {
                text_bidi::index_for_x(&row.clusters, text, x)
            } else {
                row.clusters
                    .iter()
                    .find(|cluster| x >= cluster.x0 && x < cluster.x1)
                    .map_or_else(
                        || text_bidi::index_for_x(&row.clusters, text, x),
                        |cluster| cluster.index,
                    )
            } + row.range.start;
            return if position.y >= px(0.)
                && position.y < line_height * self.bidi_rows.len() as f32
                && position.x >= px(0.)
                && position.x < row.width
            {
                Ok(index)
            } else {
                Err(index)
            };
        }
        let wrapped_line_ix = (position.y / line_height) as usize;

        let wrapped_line_start_index;
        let wrapped_line_start_x;
        if wrapped_line_ix > 0 {
            let Some(line_start_boundary) = self.wrap_boundaries.get(wrapped_line_ix - 1) else {
                return Err(0);
            };
            let run = &self.unwrapped_layout.runs[line_start_boundary.run_ix];
            let glyph = &run.glyphs[line_start_boundary.glyph_ix];
            wrapped_line_start_index = self.wrapped_line_start_index(wrapped_line_ix);
            wrapped_line_start_x = glyph.position.x;
        } else {
            wrapped_line_start_index = 0;
            wrapped_line_start_x = Pixels::ZERO;
        };

        let wrapped_line_end_index;
        let wrapped_line_end_x;
        if wrapped_line_ix < self.wrap_boundaries.len() {
            let next_wrap_boundary_ix = wrapped_line_ix;
            let next_wrap_boundary = self.wrap_boundaries[next_wrap_boundary_ix];
            let run = &self.unwrapped_layout.runs[next_wrap_boundary.run_ix];
            let glyph = &run.glyphs[next_wrap_boundary.glyph_ix];
            wrapped_line_end_index = self.wrapped_line_start_index(wrapped_line_ix + 1);
            wrapped_line_end_x = glyph.position.x;
        } else {
            wrapped_line_end_index = self.unwrapped_layout.len;
            wrapped_line_end_x = self.unwrapped_layout.width;
        };

        let mut position_in_unwrapped_line = position;
        position_in_unwrapped_line.x += wrapped_line_start_x;
        if position_in_unwrapped_line.x < wrapped_line_start_x {
            Err(wrapped_line_start_index)
        } else if position_in_unwrapped_line.x >= wrapped_line_end_x {
            Err(wrapped_line_end_index)
        } else {
            if closest {
                Ok(self
                    .unwrapped_layout
                    .closest_index_for_x(position_in_unwrapped_line.x))
            } else {
                // The shaper can place a trailing zero-width wrap boundary glyph slightly past
                // the line's width, so the row can extend past where `index_for_x` has glyphs.
                self.unwrapped_layout
                    .index_for_x(position_in_unwrapped_line.x)
                    .ok_or(wrapped_line_end_index)
            }
        }
    }

    /// Returns the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        if !self.bidi_rows.is_empty() {
            if index > self.len() || !self.text.is_char_boundary(index) {
                return None;
            }
            let row_index = self
                .bidi_rows
                .partition_point(|row| row.range.start <= index)
                .saturating_sub(1);
            let row = &self.bidi_rows[row_index];
            let x = text_bidi::caret_x(
                &row.clusters,
                &self.text[row.range.clone()],
                index - row.range.start,
            )
            .unwrap_or(0.);
            return Some(point(px(x), line_height * row_index as f32));
        }
        let mut line_start_ix = 0;
        let mut line_end_indices = (1..=self.wrap_boundaries.len())
            .map(|row| self.wrapped_line_start_index(row))
            .chain([self.len()])
            .enumerate();
        for (ix, line_end_ix) in line_end_indices {
            let line_y = ix as f32 * line_height;
            if index < line_start_ix {
                break;
            } else if index > line_end_ix {
                line_start_ix = line_end_ix;
                continue;
            } else {
                let line_start_x = self.unwrapped_layout.x_for_index(line_start_ix);
                let x = self.unwrapped_layout.x_for_index(index) - line_start_x;
                return Some(point(x, line_y));
            }
        }

        None
    }
}

pub(crate) struct LineLayoutCache {
    previous_frame: Mutex<FrameCache>,
    current_frame: RwLock<FrameCache>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
    /// Advances when [`TextSystem::add_fonts`] successfully changes the font database.
    font_generation: Arc<AtomicUsize>,
    /// Records the generation represented by both frame caches.
    cached_font_generation: AtomicUsize,
}

#[derive(Default)]
struct FrameCache {
    lines: FxHashMap<Arc<CacheKey>, Arc<LineLayout>>,
    wrapped_lines: FxHashMap<Arc<CacheKey>, Arc<WrappedLineLayout>>,
    used_lines: Vec<Arc<CacheKey>>,
    used_wrapped_lines: Vec<Arc<CacheKey>>,

    // Content-addressable caches keyed by caller-provided text hash + layout params.
    // These allow cache hits without materializing a contiguous `SharedString`.
    //
    // IMPORTANT: To support allocation-free lookups, we store these maps using a key type
    // (`HashedCacheKeyRef`) that can be computed without building a contiguous `&str`/`SharedString`.
    // On miss, we allocate once and store under an owned `HashedCacheKey`.
    lines_by_hash: FxHashMap<Arc<HashedCacheKey>, Arc<LineLayout>>,
    wrapped_lines_by_hash: FxHashMap<Arc<HashedCacheKey>, Arc<WrappedLineLayout>>,
    used_lines_by_hash: Vec<Arc<HashedCacheKey>>,
    used_wrapped_lines_by_hash: Vec<Arc<HashedCacheKey>>,
}

#[derive(Clone, Default)]
pub(crate) struct LineLayoutIndex {
    font_generation: usize,
    lines_index: usize,
    wrapped_lines_index: usize,
    lines_by_hash_index: usize,
    wrapped_lines_by_hash_index: usize,
}

impl LineLayoutCache {
    pub fn new(
        platform_text_system: Arc<dyn PlatformTextSystem>,
        font_generation: Arc<AtomicUsize>,
    ) -> Self {
        let cached_font_generation = font_generation.load(Ordering::Acquire);
        Self {
            previous_frame: Mutex::default(),
            current_frame: RwLock::default(),
            platform_text_system,
            font_generation,
            cached_font_generation: AtomicUsize::new(cached_font_generation),
        }
    }

    pub fn layout_index(&self) -> LineLayoutIndex {
        let font_generation = self.clear_if_font_generation_changed();
        let frame = self.current_frame.read();
        LineLayoutIndex {
            font_generation,
            lines_index: frame.used_lines.len(),
            wrapped_lines_index: frame.used_wrapped_lines.len(),
            lines_by_hash_index: frame.used_lines_by_hash.len(),
            wrapped_lines_by_hash_index: frame.used_wrapped_lines_by_hash.len(),
        }
    }

    pub fn reuse_layouts(&self, range: Range<LineLayoutIndex>) {
        let font_generation = self.clear_if_font_generation_changed();
        if range.start.font_generation != font_generation
            || range.end.font_generation != font_generation
        {
            return;
        }
        let mut current_frame = &mut *self.current_frame.write();
        let mut previous_frame = &mut *self.previous_frame.lock();

        for key in &previous_frame.used_lines[range.start.lines_index..range.end.lines_index] {
            if let Some((key, line)) = previous_frame.lines.remove_entry(key) {
                current_frame.lines.insert(key, line);
            }
            current_frame.used_lines.push(key.clone());
        }

        for key in &previous_frame.used_wrapped_lines
            [range.start.wrapped_lines_index..range.end.wrapped_lines_index]
        {
            if let Some((key, line)) = previous_frame.wrapped_lines.remove_entry(key) {
                current_frame.wrapped_lines.insert(key, line);
            }
            current_frame.used_wrapped_lines.push(key.clone());
        }

        for key in &previous_frame.used_lines_by_hash
            [range.start.lines_by_hash_index..range.end.lines_by_hash_index]
        {
            if let Some((key, line)) = previous_frame.lines_by_hash.remove_entry(key) {
                current_frame.lines_by_hash.insert(key, line);
            }
            current_frame.used_lines_by_hash.push(key.clone());
        }

        for key in &previous_frame.used_wrapped_lines_by_hash
            [range.start.wrapped_lines_by_hash_index..range.end.wrapped_lines_by_hash_index]
        {
            if let Some((key, line)) = previous_frame.wrapped_lines_by_hash.remove_entry(key) {
                current_frame.wrapped_lines_by_hash.insert(key, line);
            }
            current_frame.used_wrapped_lines_by_hash.push(key.clone());
        }
    }

    pub fn truncate_layouts(&self, index: LineLayoutIndex) {
        let font_generation = self.clear_if_font_generation_changed();
        if index.font_generation != font_generation {
            return;
        }
        let mut current_frame = &mut *self.current_frame.write();
        current_frame.used_lines.truncate(index.lines_index);
        current_frame
            .used_wrapped_lines
            .truncate(index.wrapped_lines_index);
        current_frame
            .used_lines_by_hash
            .truncate(index.lines_by_hash_index);
        current_frame
            .used_wrapped_lines_by_hash
            .truncate(index.wrapped_lines_by_hash_index);
    }

    pub fn finish_frame(&self) {
        let _font_generation = self.clear_if_font_generation_changed();
        let mut curr_frame = self.current_frame.write();
        let mut prev_frame = self.previous_frame.lock();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.lines.clear();
        curr_frame.wrapped_lines.clear();
        curr_frame.used_lines.clear();
        curr_frame.used_wrapped_lines.clear();

        curr_frame.lines_by_hash.clear();
        curr_frame.wrapped_lines_by_hash.clear();
        curr_frame.used_lines_by_hash.clear();
        curr_frame.used_wrapped_lines_by_hash.clear();
    }

    pub fn layout_wrapped_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        max_lines: Option<usize>,
    ) -> Arc<WrappedLineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let _font_generation = self.clear_if_font_generation_changed();
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width,
            force_width: None,
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.wrapped_lines.get(key) {
            return layout.clone();
        }

        let previous_frame_entry = self.previous_frame.lock().wrapped_lines.remove_entry(key);
        if let Some((key, layout)) = previous_frame_entry {
            let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);
            layout
        } else {
            drop(current_frame);
            let text = SharedString::from(text);
            let unwrapped_layout = self.layout_line::<&SharedString>(&text, font_size, runs, None);
            let (unwrapped_layout, wrap_boundaries, logical_line_starts) = match wrap_width {
                Some(wrap_width) => match self.layout_logical_lines(
                    &text,
                    font_size,
                    runs,
                    &unwrapped_layout,
                    wrap_width,
                    max_lines,
                ) {
                    Some((layout, boundaries, starts)) => (Arc::new(layout), boundaries, starts),
                    None => {
                        let boundaries = unwrapped_layout.compute_wrap_boundaries(
                            text.as_ref(),
                            wrap_width,
                            max_lines,
                        );
                        (unwrapped_layout, boundaries, SmallVec::new())
                    }
                },
                None => (unwrapped_layout, SmallVec::new(), SmallVec::new()),
            };
            let mut layout = WrappedLineLayout {
                unwrapped_layout,
                wrap_boundaries,
                wrap_width,
                logical_line_starts,
                text: text.clone(),
                bidi_rows: Vec::new(),
            };
            layout.bidi_rows = text_bidi::rows(&layout, &text);
            let layout = Arc::new(layout);
            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                force_width: None,
            });

            let mut current_frame = self.current_frame.write();
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);

            layout
        }
    }

    fn layout_logical_lines(
        &self,
        text: &SharedString,
        font_size: Pixels,
        runs: &[FontRun],
        unwrapped: &LineLayout,
        wrap_width: Pixels,
        max_lines: Option<usize>,
    ) -> Option<(
        LineLayout,
        SmallVec<[WrapBoundary; 1]>,
        SmallVec<[usize; 1]>,
    )> {
        if unwrapped.width <= wrap_width || !unwrapped.is_visually_reordered() {
            return None;
        }
        let visual: Vec<(usize, f32)> = unwrapped
            .runs
            .iter()
            .flat_map(|run| run.glyphs.iter())
            .map(|glyph| (glyph.index, glyph.position.x.as_f32()))
            .collect();
        let units = logical_wrap::logical_units(&visual, unwrapped.width.as_f32(), text);
        if units.is_empty() {
            return None;
        }
        let breaks = LineWrapper::break_indices(text);
        let starts = logical_wrap::wrap(&units, wrap_width.as_f32(), max_lines, |index, _, _| {
            breaks.binary_search(&index).is_ok()
        })?;
        if starts.is_empty() {
            return None;
        }

        let mark = if logical_wrap::base_direction_is_rtl(text) {
            logical_wrap::RLM
        } else {
            logical_wrap::LRM
        };
        let mark_len = mark.len_utf8();
        let font_runs: Vec<(usize, FontId)> =
            runs.iter().map(|run| (run.len, run.font_id)).collect();
        let line_count = starts.len() + 1;
        let mut combined: Vec<ShapedRun> = Vec::new();
        let mut boundaries: SmallVec<[WrapBoundary; 1]> = SmallVec::new();
        let mut x_offset = 0f32;
        for line in 0..line_count {
            let start = if line == 0 { 0 } else { starts[line - 1] };
            let end = starts.get(line).copied().unwrap_or(text.len());
            let shaped_end = if line + 1 < line_count {
                start + text[start..end].trim_end_matches(' ').len()
            } else {
                end
            };
            if shaped_end <= start {
                return None;
            }
            let mut line_runs: Vec<FontRun> =
                logical_wrap::slice_runs(&font_runs, start..shaped_end)
                    .into_iter()
                    .map(|(len, font_id)| FontRun { len, font_id })
                    .collect();
            line_runs.first_mut()?.len += mark_len;
            let mut marked = String::with_capacity(mark_len + shaped_end - start);
            marked.push(mark);
            marked.push_str(&text[start..shaped_end]);
            let shaped = self.layout_line(marked, font_size, &line_runs, None);

            let shaped_runs: Vec<(FontId, &[ShapedGlyph])> = shaped
                .runs
                .iter()
                .map(|run| (run.font_id, run.glyphs.as_slice()))
                .collect();
            let placed = logical_wrap::place_line(
                &shaped_runs,
                shaped.width.as_f32(),
                mark_len,
                start,
                x_offset,
                |glyph| glyph.index,
                |glyph| glyph.position.x.as_f32(),
                |glyph, x, index| ShapedGlyph {
                    id: glyph.id,
                    position: point(px(x), glyph.position.y),
                    index,
                    is_emoji: glyph.is_emoji,
                },
            )?;
            if line > 0 {
                boundaries.push(WrapBoundary {
                    run_ix: combined.len(),
                    glyph_ix: 0,
                });
            }
            combined.extend(
                placed
                    .runs
                    .into_iter()
                    .map(|(font_id, glyphs)| ShapedRun { font_id, glyphs }),
            );
            x_offset += placed.width;
        }

        let layout = LineLayout {
            font_size: unwrapped.font_size,
            width: px(x_offset),
            ascent: unwrapped.ascent,
            descent: unwrapped.descent,
            runs: combined,
            len: unwrapped.len,
        };
        Some((layout, boundaries, starts.into_iter().collect()))
    }

    pub fn layout_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Arc<LineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let _font_generation = self.clear_if_font_generation_changed();
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width: None,
            force_width,
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.lines.get(key) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
        if let Some((key, layout)) = self.previous_frame.lock().lines.remove_entry(key) {
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        } else {
            let text = SharedString::from(text);
            let mut layout = self
                .platform_text_system
                .layout_line(&text, font_size, runs);
            layout.normalize_visual_order();

            if let Some(force_width) = force_width {
                apply_force_width_to_layout(&mut layout, force_width);
            }

            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width: None,
                force_width,
            });
            let layout = Arc::new(layout);
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        }
    }

    /// Try to retrieve a previously-shaped line layout using a caller-provided content hash.
    ///
    /// This is a *non-allocating* cache probe: it does not materialize any text. If the layout
    /// is not already cached in either the current frame or previous frame, returns `None`.
    ///
    /// Contract (caller enforced):
    /// - Same `text_hash` implies identical text content (collision risk accepted by caller).
    /// - `text_len` should be the UTF-8 byte length of the text (helps reduce accidental collisions).
    pub fn try_layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Option<Arc<LineLayout>> {
        let _font_generation = self.clear_if_font_generation_changed();
        let key_ref = HashedCacheKeyRef {
            text_hash,
            text_len,
            font_size,
            runs,
            wrap_width: None,
            force_width,
        };

        let current_frame = self.current_frame.read();
        if let Some((_, layout)) = current_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return Some(layout.clone());
        }

        let previous_frame = self.previous_frame.lock();
        if let Some((_, layout)) = previous_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return Some(layout.clone());
        }

        None
    }

    /// Layout a line of text using a caller-provided content hash as the cache key.
    ///
    /// This enables cache hits without materializing a contiguous `SharedString` for `text`.
    /// If the cache misses, `materialize_text` is invoked to produce the `SharedString` for shaping.
    ///
    /// Contract (caller enforced):
    /// - Same `text_hash` implies identical text content (collision risk accepted by caller).
    /// - `text_len` should be the UTF-8 byte length of the text (helps reduce accidental collisions).
    pub fn layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
        materialize_text: impl FnOnce() -> SharedString,
    ) -> Arc<LineLayout> {
        let _font_generation = self.clear_if_font_generation_changed();
        let key_ref = HashedCacheKeyRef {
            text_hash,
            text_len,
            font_size,
            runs,
            wrap_width: None,
            force_width,
        };

        // Fast path: already cached (no allocation).
        let current_frame = self.current_frame.upgradable_read();
        if let Some((_, layout)) = current_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);

        // Try to reuse from previous frame without allocating; do a linear scan to find a matching key.
        // (We avoid `drain()` here because it would eagerly move all entries.)
        let mut previous_frame = self.previous_frame.lock();
        if let Some(existing_key) = previous_frame
            .used_lines_by_hash
            .iter()
            .find(|key| {
                HashedCacheKeyRef {
                    text_hash: key.text_hash,
                    text_len: key.text_len,
                    font_size: key.font_size,
                    runs: key.runs.as_slice(),
                    wrap_width: key.wrap_width,
                    force_width: key.force_width,
                } == key_ref
            })
            .cloned()
        {
            if let Some((key, layout)) = previous_frame.lines_by_hash.remove_entry(&existing_key) {
                current_frame
                    .lines_by_hash
                    .insert(key.clone(), layout.clone());
                current_frame.used_lines_by_hash.push(key);
                return layout;
            }
        }

        let text = materialize_text();
        let mut layout = self
            .platform_text_system
            .layout_line(&text, font_size, runs);
        layout.normalize_visual_order();

        if let Some(force_width) = force_width {
            apply_force_width_to_layout(&mut layout, force_width);
        }

        let key = Arc::new(HashedCacheKey {
            text_hash,
            text_len,
            font_size,
            runs: SmallVec::from(runs),
            wrap_width: None,
            force_width,
        });
        let layout = Arc::new(layout);
        current_frame
            .lines_by_hash
            .insert(key.clone(), layout.clone());
        current_frame.used_lines_by_hash.push(key);
        layout
    }

    fn clear_if_font_generation_changed(&self) -> usize {
        let font_generation = self.font_generation.load(Ordering::Acquire);
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        let mut current_frame = self.current_frame.write();
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        *current_frame = FrameCache::default();
        *self.previous_frame.lock() = FrameCache::default();
        self.cached_font_generation
            .store(font_generation, Ordering::Release);
        font_generation
    }
}

// Combining marks (e.g. Thai vowel signs, Arabic diacritics) are shaped by
// HarfBuzz at the same x position as their base character. The force-width
// loop must not advance the cell counter for these zero-advance glyphs,
// otherwise they get displaced into the next cell. We detect them by checking
// whether shaped x has advanced by at least half a cell beyond the last base.
fn apply_force_width_to_layout(layout: &mut LineLayout, force_width: Pixels) {
    let mut glyph_pos: usize = 0;
    // NEG_INFINITY ensures the first glyph is always classified as a base.
    let mut last_base_shaped_x = px(f32::NEG_INFINITY);
    let mut last_base_actual_x = px(0.);

    for run in layout.runs.iter_mut() {
        for glyph in run.glyphs.iter_mut() {
            let shaped_x = glyph.position.x;

            if shaped_x > last_base_shaped_x + force_width * 0.5 {
                let forced_x = glyph_pos * force_width;
                if (shaped_x - forced_x).abs() > px(1.) {
                    glyph.position.x = forced_x;
                }
                last_base_shaped_x = shaped_x;
                last_base_actual_x = glyph.position.x;
                glyph_pos += 1;
            } else {
                glyph.position.x = last_base_actual_x + (shaped_x - last_base_shaped_x);
            }
        }
    }
}

/// A run of text with a single font.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[expect(missing_docs)]
pub struct FontRun {
    pub len: usize,
    pub font_id: FontId,
}

trait AsCacheKeyRef {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_>;
}

#[derive(Clone, Debug, Eq)]
struct CacheKey {
    text: SharedString,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Clone, Debug)]
struct HashedCacheKey {
    text_hash: u64,
    text_len: usize,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Copy, Clone)]
struct HashedCacheKeyRef<'a> {
    text_hash: u64,
    text_len: usize,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

impl PartialEq for dyn AsCacheKeyRef + '_ {
    fn eq(&self, other: &dyn AsCacheKeyRef) -> bool {
        self.as_cache_key_ref() == other.as_cache_key_ref()
    }
}

impl PartialEq for HashedCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.text_hash == other.text_hash
            && self.text_len == other.text_len
            && self.font_size == other.font_size
            && self.runs.as_slice() == other.runs.as_slice()
            && self.wrap_width == other.wrap_width
            && self.force_width == other.force_width
    }
}

impl Eq for HashedCacheKey {}

impl Hash for HashedCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_hash.hash(state);
        self.text_len.hash(state);
        self.font_size.hash(state);
        self.runs.as_slice().hash(state);
        self.wrap_width.hash(state);
        self.force_width.hash(state);
    }
}

impl PartialEq for HashedCacheKeyRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.text_hash == other.text_hash
            && self.text_len == other.text_len
            && self.font_size == other.font_size
            && self.runs == other.runs
            && self.wrap_width == other.wrap_width
            && self.force_width == other.force_width
    }
}

impl Eq for HashedCacheKeyRef<'_> {}

impl Hash for HashedCacheKeyRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_hash.hash(state);
        self.text_len.hash(state);
        self.font_size.hash(state);
        self.runs.hash(state);
        self.wrap_width.hash(state);
        self.force_width.hash(state);
    }
}

impl Eq for dyn AsCacheKeyRef + '_ {}

impl Hash for dyn AsCacheKeyRef + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state)
    }
}

impl AsCacheKeyRef for CacheKey {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        CacheKeyRef {
            text: &self.text,
            font_size: self.font_size,
            runs: self.runs.as_slice(),
            wrap_width: self.wrap_width,
            force_width: self.force_width,
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_cache_key_ref().eq(&other.as_cache_key_ref())
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state);
    }
}

impl<'a> Borrow<dyn AsCacheKeyRef + 'a> for Arc<CacheKey> {
    fn borrow(&self) -> &(dyn AsCacheKeyRef + 'a) {
        self.as_ref() as &dyn AsCacheKeyRef
    }
}

impl AsCacheKeyRef for CacheKeyRef<'_> {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GlyphId;

    fn glyph_at(x: f32, index: usize) -> ShapedGlyph {
        ShapedGlyph {
            id: GlyphId(0),
            position: point(px(x), px(0.)),
            index,
            is_emoji: false,
        }
    }

    fn make_layout(glyphs: Vec<ShapedGlyph>) -> LineLayout {
        LineLayout {
            font_size: px(16.),
            width: px(100.),
            ascent: px(12.),
            descent: px(4.),
            runs: vec![ShapedRun {
                font_id: FontId(0),
                glyphs,
            }],
            len: 0,
        }
    }

    fn logical_two_row_layout() -> WrappedLineLayout {
        let first_row = ShapedRun {
            font_id: FontId(0),
            glyphs: vec![glyph_at(0., 4), glyph_at(5., 2), glyph_at(15., 0)],
        };
        let second_row = ShapedRun {
            font_id: FontId(0),
            glyphs: vec![glyph_at(25., 7), glyph_at(35., 5)],
        };
        WrappedLineLayout {
            unwrapped_layout: Arc::new(LineLayout {
                font_size: px(16.),
                width: px(45.),
                ascent: px(12.),
                descent: px(4.),
                runs: vec![first_row, second_row],
                len: 9,
            }),
            wrap_boundaries: SmallVec::from_buf([WrapBoundary {
                run_ix: 1,
                glyph_ix: 0,
            }]),
            wrap_width: Some(px(30.)),
            logical_line_starts: SmallVec::from_buf([5]),
            ..Default::default()
        }
    }

    #[test]
    fn logical_rows_start_at_their_logical_index_not_at_their_first_glyph() {
        let layout = logical_two_row_layout();
        assert_eq!(layout.wrapped_line_start_index(0), 0);
        assert_eq!(layout.wrapped_line_start_index(1), 5);
        assert_eq!(layout.wrapped_line_start_index(2), 9);
        let glyph_order = WrappedLineLayout {
            logical_line_starts: SmallVec::new(),
            ..logical_two_row_layout()
        };
        assert_eq!(glyph_order.wrapped_line_start_index(1), 7);
    }

    #[test]
    fn logical_rows_place_each_index_on_the_row_that_holds_it() {
        let mut cached = logical_two_row_layout();
        cached.text = "אב גד".into();
        cached.bidi_rows = text_bidi::rows(&cached, &cached.text);
        for (index, x, row) in [
            (0, 25., 0),
            (2, 15., 0),
            (4, 5., 0),
            (5, 20., 1),
            (7, 10., 1),
            (9, 0., 1),
        ] {
            assert_eq!(
                cached.position_for_index(index, px(20.)),
                Some(point(px(x), px(20.) * row as f32))
            );
        }
        assert_eq!(
            cached.closest_index_for_position(point(px(18.), px(25.)), px(20.)),
            Ok(5)
        );
        assert_eq!(
            cached.index_for_position(point(px(18.), px(25.)), px(20.)),
            Ok(5)
        );
        assert_eq!(cached.selection_bounds(0..9, px(20.)).len(), 2);
        let layout = logical_two_row_layout();
        let line_height = px(20.);
        for (index, row) in [(0, 0), (4, 0), (5, 0), (7, 1), (8, 1), (9, 1)] {
            let position = layout.position_for_index(index, line_height).unwrap();
            assert_eq!(position.y, line_height * row as f32, "index {index}");
        }
    }

    #[test]
    fn logical_rows_hit_test_to_the_index_under_the_pointer() {
        let layout = logical_two_row_layout();
        let line_height = px(20.);
        assert_eq!(
            layout.index_for_position(point(px(2.), px(5.)), line_height),
            Ok(4)
        );
        assert_eq!(
            layout.index_for_position(point(px(2.), px(25.)), line_height),
            Ok(7)
        );
        assert_eq!(
            layout.index_for_position(point(px(30.), px(5.)), line_height),
            Err(5)
        );
        assert_eq!(
            layout.index_for_position(point(px(30.), px(25.)), line_height),
            Err(9)
        );
    }

    #[test]
    fn rows_that_were_not_wrapped_logically_keep_their_glyph_derived_starts() {
        let glyphs = [(0., 0), (10., 1), (20., 2), (0., 3), (10., 4)];
        let layout = WrappedLineLayout {
            unwrapped_layout: Arc::new(make_layout(
                glyphs
                    .iter()
                    .map(|&(x, index)| glyph_at(x, index))
                    .collect(),
            )),
            wrap_boundaries: SmallVec::from_buf([WrapBoundary {
                run_ix: 0,
                glyph_ix: 3,
            }]),
            ..Default::default()
        };
        assert_eq!(layout.wrapped_line_start_index(1), 3);
        assert_eq!(layout.wrapped_line_start_index(2), layout.len());
        let mut clusters = make_layout(vec![
            glyph_at(0., 0),
            glyph_at(5., 0),
            glyph_at(10., 3),
            glyph_at(15., 3),
            glyph_at(20., 6),
            glyph_at(25., 6),
            glyph_at(30., 9),
            glyph_at(35., 9),
        ]);
        clusters.width = px(40.);
        clusters.len = "你好世界".len();
        let boundaries = clusters.compute_wrap_boundaries("你好世界", px(25.), None);
        assert_eq!(
            boundaries
                .iter()
                .map(|boundary| boundary.glyph_ix)
                .collect::<Vec<_>>(),
            vec![4]
        );
        for (i, glyph) in clusters.runs[0].glyphs.iter_mut().enumerate() {
            glyph.index = i / 2;
            glyph.position.x = px(i as f32 * 10.);
        }
        clusters.width = px(80.);
        clusters.len = 4;
        let boundaries = clusters.compute_wrap_boundaries("abcd", px(30.), None);
        assert_eq!(
            boundaries
                .iter()
                .map(|boundary| boundary.glyph_ix)
                .collect::<Vec<_>>(),
            vec![2, 4, 6]
        );
    }

    fn glyph_x_positions(layout: &LineLayout) -> Vec<f32> {
        layout.runs[0]
            .glyphs
            .iter()
            .map(|g| f32::from(g.position.x))
            .collect()
    }

    #[test]
    fn test_force_width_latin_unchanged() {
        let cell_width = px(8.);
        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(8., 1), glyph_at(16., 2)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 8., 16.]);
    }

    #[test]
    fn test_force_width_combining_marks_not_advanced() {
        let cell_width = px(8.);
        // Simulates Thai "กี" — base consonant at x=0, combining vowel also at x=0
        let mut layout = make_layout(vec![
            glyph_at(0., 0), // ก (base)
            glyph_at(0., 3), // ี (combining mark, same x)
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0.]);
    }

    #[test]
    fn test_force_width_base_after_combining_mark() {
        let cell_width = px(8.);
        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(0., 3), glyph_at(8., 6)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0., 8.]);
    }

    #[test]
    fn test_force_width_multiple_combining_marks() {
        let cell_width = px(8.);
        // Simulates "ก้" — base + vowel + tone mark (two combining marks stacked)
        let mut layout = make_layout(vec![
            glyph_at(0., 0), // ก (base)
            glyph_at(0., 3), // vowel (combining)
            glyph_at(0., 6), // tone mark (combining)
            glyph_at(8., 9), // next base
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0., 0., 8.]);
    }

    #[test]
    fn test_force_width_corrects_drifted_base_positions() {
        let cell_width = px(8.);
        // Font metrics don't perfectly match cell grid — glyphs drift >1px from cell boundary
        let mut layout = make_layout(vec![
            glyph_at(0.5, 0),  // within 1px tolerance, kept as-is
            glyph_at(10.2, 1), // >1px off from 8.0, corrected
            glyph_at(19.8, 2), // >1px off from 16.0, corrected
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0.5, 8., 16.]);
    }

    #[test]
    fn test_force_width_combining_mark_after_within_tolerance_base() {
        let cell_width = px(8.);
        // Base glyph is within 1px of grid so it keeps its shaped position.
        // The combining mark must align to the base's actual position, not the grid slot.
        let mut layout = make_layout(vec![glyph_at(0.5, 0), glyph_at(0.5, 3)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0.5, 0.5]);
    }
}
