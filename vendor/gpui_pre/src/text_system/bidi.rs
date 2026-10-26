use crate::{LineLayout, ShapedRun};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cluster {
    pub index: usize,
    end: usize,
    pub x0: f32,
    pub x1: f32,
    pub rtl: bool,
}

pub fn is_rtl_char(c: char) -> bool {
    matches!(
        unicode_bidi::bidi_class(c),
        unicode_bidi::BidiClass::R | unicode_bidi::BidiClass::AL
    )
}

pub fn paragraph_is_rtl(text: &str, fallback: bool) -> bool {
    match unicode_bidi::get_base_direction_full(text) {
        unicode_bidi::Direction::Rtl => true,
        unicode_bidi::Direction::Ltr => false,
        unicode_bidi::Direction::Mixed => fallback,
    }
}

pub fn clusters(layout: &LineLayout, text: &str) -> Vec<Cluster> {
    let info = unicode_bidi::ParagraphBidiInfo::new(text, None);
    clusters_with_levels(layout, &info.reordered_levels(0..text.len()), 0, text.len())
}

fn clusters_with_levels(
    layout: &LineLayout,
    levels: &[unicode_bidi::Level],
    offset: usize,
    len: usize,
) -> Vec<Cluster> {
    let mut glyphs: Vec<_> = layout.runs.iter().flat_map(|run| &run.glyphs).collect();
    glyphs.sort_by(|a, b| {
        a.position
            .x
            .partial_cmp(&b.position.x)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let width = f32::from(layout.width);
    let mut clusters: Vec<Cluster> = Vec::new();
    let mut owners = std::collections::HashMap::<usize, usize>::new();
    for (k, glyph) in glyphs.iter().enumerate() {
        let x0 = f32::from(glyph.position.x);
        let cluster = Cluster {
            index: glyph.index,
            end: len,
            x0,
            x1: glyphs
                .get(k + 1)
                .map_or(width, |next| f32::from(next.position.x))
                .max(x0),
            rtl: levels
                .get(glyph.index + offset)
                .is_some_and(|level| level.is_rtl()),
        };
        if let Some(&owner) = owners.get(&cluster.index) {
            let previous = &mut clusters[owner];
            previous.x0 = previous.x0.min(cluster.x0);
            previous.x1 = previous.x1.max(cluster.x1);
        } else {
            owners.insert(cluster.index, clusters.len());
            clusters.push(cluster);
        }
    }
    let mut indices: Vec<_> = owners.keys().copied().collect();
    indices.sort_unstable();
    for cluster in &mut clusters {
        cluster.end = indices
            .get(indices.partition_point(|index| *index <= cluster.index))
            .copied()
            .unwrap_or(len);
    }
    clusters
}

#[derive(Clone, Debug)]
pub struct Row {
    pub range: std::ops::Range<usize>,
    pub width: crate::Pixels,
    pub clusters: Vec<Cluster>,
}

pub(crate) fn rows(line: &crate::WrappedLineLayout, text: &str) -> Vec<Row> {
    let layout = &line.unwrapped_layout;
    let edges: Vec<_> = std::iter::once(crate::px(0.))
        .chain(line.wrap_boundaries().iter().map(|boundary| {
            layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix]
                .position
                .x
        }))
        .chain([layout.width])
        .collect();
    let info = unicode_bidi::ParagraphBidiInfo::new(text, None);
    (0..edges.len() - 1)
        .map(|row| {
            let start = line.wrapped_line_start_index(row);
            let end = line.wrapped_line_start_index(row + 1);
            let width = edges[row + 1] - edges[row];
            let runs = layout
                .runs
                .iter()
                .filter_map(|run| {
                    let glyphs: Vec<_> = run
                        .glyphs
                        .iter()
                        .filter(|glyph| glyph.index >= start && glyph.index < end)
                        .map(|glyph| crate::ShapedGlyph {
                            index: glyph.index - start,
                            position: crate::point(glyph.position.x - edges[row], glyph.position.y),
                            ..glyph.clone()
                        })
                        .collect();
                    (!glyphs.is_empty()).then_some(ShapedRun {
                        font_id: run.font_id,
                        glyphs,
                    })
                })
                .collect();
            let local = LineLayout {
                font_size: layout.font_size,
                width,
                ascent: layout.ascent,
                descent: layout.descent,
                runs,
                len: end - start,
            };
            let levels = info.reordered_levels(start..end);
            let mut clusters = clusters_with_levels(&local, &levels, start, end - start);
            if row + 1 < edges.len() - 1 && !line.logical_line_starts.is_empty() {
                let trimmed = text[start..end].trim_end_matches(' ').len();
                if trimmed < end - start && !clusters.iter().any(|cluster| cluster.index == trimmed)
                {
                    for cluster in &mut clusters {
                        cluster.end = cluster.end.min(trimmed);
                    }
                    let x = if info.paragraph_level.is_rtl() {
                        0.
                    } else {
                        f32::from(width)
                    };
                    clusters.push(Cluster {
                        index: trimmed,
                        end: end - start,
                        x0: x,
                        x1: x,
                        rtl: info.paragraph_level.is_rtl(),
                    });
                    clusters.sort_by(|a, b| a.x0.total_cmp(&b.x0).then(a.x1.total_cmp(&b.x1)));
                }
            }
            Row {
                range: start..end,
                width,
                clusters,
            }
        })
        .collect()
}

pub fn has_rtl(clusters: &[Cluster]) -> bool {
    clusters.iter().any(|cluster| cluster.rtl)
}

pub fn caret_x(clusters: &[Cluster], text: &str, index: usize) -> Option<f32> {
    if clusters.is_empty() {
        return None;
    }
    if let Some(cluster) = clusters.iter().find(|cluster| cluster.index == index) {
        return Some(if cluster.rtl { cluster.x1 } else { cluster.x0 });
    }
    let owner = clusters
        .iter()
        .filter(|cluster| cluster.index < index)
        .max_by_key(|cluster| cluster.index)?;
    let next = owner.end;
    if index >= next {
        return Some(if owner.rtl { owner.x0 } else { owner.x1 });
    }
    let total = text.get(owner.index..next)?.chars().count().max(1) as f32;
    let before = text.get(owner.index..index)?.chars().count() as f32;
    let fraction = before / total;
    let span = owner.x1 - owner.x0;
    Some(if owner.rtl {
        owner.x1 - fraction * span
    } else {
        owner.x0 + fraction * span
    })
}

pub fn index_for_x(clusters: &[Cluster], text: &str, x: f32) -> usize {
    let (Some(first), Some(last)) = (clusters.first(), clusters.last()) else {
        return 0;
    };
    let left = |cluster: &Cluster| {
        if cluster.rtl {
            cluster.end
        } else {
            cluster.index
        }
    };
    let right = |cluster: &Cluster| {
        if cluster.rtl {
            cluster.index
        } else {
            cluster.end
        }
    };
    if x <= first.x0 {
        return left(first);
    }
    if x >= last.x1 {
        return right(last);
    }
    let cluster = clusters
        .iter()
        .find(|cluster| x >= cluster.x0 && x < cluster.x1)
        .unwrap_or(last);
    let end = cluster.end;
    let Some(span) = text.get(cluster.index..end) else {
        return left(cluster);
    };
    let fraction = if cluster.x1 > cluster.x0 {
        ((x - cluster.x0) / (cluster.x1 - cluster.x0)).clamp(0., 1.)
    } else {
        0.
    };
    let fraction = if cluster.rtl { 1. - fraction } else { fraction };
    let offset = (fraction * span.chars().count() as f32).round() as usize;
    cluster.index
        + span
            .char_indices()
            .nth(offset)
            .map_or(span.len(), |(index, _)| index)
}

pub fn selection_spans(
    clusters: &[Cluster],
    text: &str,
    start: usize,
    end: usize,
) -> Vec<(f32, f32)> {
    let mut spans: Vec<(f32, f32)> = clusters
        .iter()
        .filter_map(|cluster| {
            let lo = start.max(cluster.index);
            let hi = end.min(cluster.end);
            if lo >= hi || cluster.x1 <= cluster.x0 {
                return None;
            }
            let edge = |index| {
                let total = text.get(cluster.index..cluster.end)?.chars().count().max(1) as f32;
                let before = text.get(cluster.index..index)?.chars().count() as f32;
                let span = (cluster.x1 - cluster.x0) * before / total;
                Some(if cluster.rtl {
                    cluster.x1 - span
                } else {
                    cluster.x0 + span
                })
            };
            let a: f32 = edge(lo)?;
            let b: f32 = edge(hi)?;
            Some((a.min(b), a.max(b)))
        })
        .collect();
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut merged: Vec<(f32, f32)> = Vec::new();
    for (x0, x1) in spans {
        match merged.last_mut() {
            Some(last) if x0 <= last.1 + 0.5 => last.1 = last.1.max(x1),
            _ => merged.push((x0, x1)),
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontId, GlyphId, ShapedGlyph, point, px};

    fn layout(glyphs: &[(usize, f32)], runs: &[usize], width: f32) -> LineLayout {
        let mut at = 0;
        let runs = runs
            .iter()
            .map(|count| {
                let run = ShapedRun {
                    font_id: FontId(0),
                    glyphs: glyphs[at..at + count]
                        .iter()
                        .map(|&(index, x)| ShapedGlyph {
                            id: GlyphId(0),
                            position: point(px(x), px(0.)),
                            index,
                            is_emoji: false,
                        })
                        .collect(),
                };
                at += count;
                run
            })
            .collect();
        LineLayout {
            font_size: px(16.),
            width: px(width),
            ascent: px(12.),
            descent: px(4.),
            runs,
            len: 0,
        }
    }

    #[test]
    fn left_to_right_text_maps_indices_to_the_obvious_positions() {
        let text = "abc";
        let line = layout(&[(0, 0.), (1, 10.), (2, 20.)], &[3], 30.);
        let clusters = clusters(&line, text);
        assert!(!has_rtl(&clusters));
        for (index, x) in [(0, 0.), (1, 10.), (2, 20.), (3, 30.)] {
            assert_eq!(caret_x(&clusters, text, index), Some(x), "caret {index}");
        }
        assert_eq!(index_for_x(&clusters, text, 12.), 1);
        assert_eq!(index_for_x(&clusters, text, 17.), 2);
        assert_eq!(index_for_x(&clusters, text, -5.), 0);
        assert_eq!(index_for_x(&clusters, text, 99.), 3);
        assert_eq!(selection_spans(&clusters, text, 1, 3), vec![(10., 30.)]);
    }

    #[test]
    fn right_to_left_text_runs_from_the_right_edge() {
        let text = "אבג";
        let line = layout(&[(4, 0.), (2, 10.), (0, 20.)], &[3], 30.);
        let clusters = clusters(&line, text);
        assert!(has_rtl(&clusters) && clusters.iter().all(|cluster| cluster.rtl));
        for (index, x) in [(0, 30.), (2, 20.), (4, 10.), (6, 0.)] {
            assert_eq!(caret_x(&clusters, text, index), Some(x), "caret {index}");
        }
        assert_eq!(index_for_x(&clusters, text, 22.), 2);
        assert_eq!(index_for_x(&clusters, text, 27.), 0);
        assert_eq!(index_for_x(&clusters, text, -3.), 6);
        assert_eq!(index_for_x(&clusters, text, 99.), 0);
        assert_eq!(selection_spans(&clusters, text, 2, 6), vec![(0., 20.)]);
    }

    #[test]
    fn mixed_direction_selection_can_split_into_two_spans() {
        let text = "abאב";
        let line = layout(&[(0, 0.), (1, 10.), (4, 20.), (2, 30.)], &[2, 2], 40.);
        let clusters = clusters(&line, text);
        let neutral_text = "א 12";
        let neutral_line = layout(&[(3, 0.), (4, 10.), (2, 20.), (0, 30.)], &[4], 40.);
        let neutral_clusters = super::clusters(&neutral_line, neutral_text);
        assert!(
            neutral_clusters
                .iter()
                .find(|cluster| cluster.index == 2)
                .unwrap()
                .rtl
        );
        assert!(
            neutral_clusters
                .iter()
                .filter(|cluster| cluster.index >= 3)
                .all(|cluster| !cluster.rtl)
        );
        assert_eq!(caret_x(&clusters, text, 1), Some(10.));
        assert_eq!(caret_x(&clusters, text, 2), Some(40.));
        assert_eq!(caret_x(&clusters, text, 6), Some(20.));
        assert_eq!(selection_spans(&clusters, text, 0, 6), vec![(0., 40.)]);
        assert_eq!(selection_spans(&clusters, text, 2, 4), vec![(30., 40.)]);
        assert_eq!(
            selection_spans(&clusters, text, 1, 4),
            vec![(10., 20.), (30., 40.)]
        );
    }

    #[test]
    fn a_single_right_to_left_glyph_is_classified_by_its_character() {
        let line = layout(&[(0, 0.)], &[1], 10.);
        assert!(has_rtl(&clusters(&line, "ש")));
        assert!(!has_rtl(&clusters(&line, "s")));
        assert!(!paragraph_is_rtl("\u{2067}א\u{2069}a", true));
        assert!(paragraph_is_rtl("123", true));
        assert!(!paragraph_is_rtl("123", false));
        assert!(is_rtl_char('ع') && is_rtl_char('א') && !is_rtl_char('क') && !is_rtl_char('中'));
    }

    #[test]
    fn a_ligature_interpolates_the_caret_inside_its_glyph() {
        let text = "لا";
        let line = layout(&[(0, 0.)], &[1], 20.);
        let clusters = clusters(&line, text);
        assert_eq!(caret_x(&clusters, text, 0), Some(20.));
        assert_eq!(caret_x(&clusters, text, 2), Some(10.));
        assert_eq!(caret_x(&clusters, text, 4), Some(0.));
        assert_eq!(index_for_x(&clusters, text, 9.), 2);
        assert_eq!(selection_spans(&clusters, text, 0, 2), vec![(10., 20.)]);
    }
}
