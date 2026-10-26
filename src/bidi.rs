use gpui::{LineLayout, ShapedRun};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Cluster {
    pub index: usize,
    pub x0: f32,
    pub x1: f32,
    pub rtl: bool,
}

pub(crate) fn is_rtl_char(c: char) -> bool {
    matches!(
        c as u32,
        0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF | 0x10800..=0x10FFF | 0x1E800..=0x1EFFF
    ) && !matches!(c, '\u{0660}'..='\u{0669}' | '\u{06F0}'..='\u{06F9}')
}

fn run_is_rtl(run: &ShapedRun, text: &str) -> bool {
    let (Some(first), Some(last)) = (run.glyphs.first(), run.glyphs.last()) else {
        return false;
    };
    if first.index != last.index {
        return first.index > last.index;
    }
    text.get(first.index..)
        .and_then(|rest| rest.chars().next())
        .is_some_and(is_rtl_char)
}

pub(crate) fn clusters(layout: &LineLayout, text: &str) -> Vec<Cluster> {
    let mut glyphs: Vec<(usize, f32, bool)> = Vec::new();
    for run in &layout.runs {
        let rtl = run_is_rtl(run, text);
        for glyph in &run.glyphs {
            let rtl = text
                .get(glyph.index..)
                .and_then(|s| s.chars().next())
                .map_or(rtl, |c| {
                    if c.is_alphabetic() || c.is_numeric() {
                        is_rtl_char(c)
                    } else {
                        rtl
                    }
                });
            glyphs.push((glyph.index, f32::from(glyph.position.x), rtl));
        }
    }
    let width = f32::from(layout.width);
    let mut clusters: Vec<Cluster> = Vec::new();
    for cluster in glyphs
        .iter()
        .enumerate()
        .map(|(k, &(index, x0, rtl))| Cluster {
            index,
            x0,
            x1: glyphs.get(k + 1).map_or(width, |next| next.1).max(x0),
            rtl,
        })
    {
        if let Some(previous) = clusters
            .iter_mut()
            .find(|previous| previous.index == cluster.index)
        {
            previous.x0 = previous.x0.min(cluster.x0);
            previous.x1 = previous.x1.max(cluster.x1);
        } else {
            clusters.push(cluster);
        }
    }
    clusters
}

pub(crate) fn has_rtl(clusters: &[Cluster]) -> bool {
    clusters.iter().any(|cluster| cluster.rtl)
}

fn next_logical(clusters: &[Cluster], after: usize, len: usize) -> usize {
    clusters
        .iter()
        .map(|cluster| cluster.index)
        .filter(|index| *index > after)
        .min()
        .unwrap_or(len)
}

pub(crate) fn caret_x(clusters: &[Cluster], text: &str, index: usize) -> Option<f32> {
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
    let next = next_logical(clusters, owner.index, text.len());
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

pub(crate) fn index_for_x(clusters: &[Cluster], text: &str, x: f32) -> usize {
    let len = text.len();
    let (Some(first), Some(last)) = (clusters.first(), clusters.last()) else {
        return 0;
    };
    let left = |cluster: &Cluster| {
        if cluster.rtl {
            next_logical(clusters, cluster.index, len)
        } else {
            cluster.index
        }
    };
    let right = |cluster: &Cluster| {
        if cluster.rtl {
            cluster.index
        } else {
            next_logical(clusters, cluster.index, len)
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
    let end = next_logical(clusters, cluster.index, len);
    let Some(span) = text.get(cluster.index..end) else {
        return left(cluster);
    };
    let fraction = ((x - cluster.x0) / (cluster.x1 - cluster.x0)).clamp(0., 1.);
    let fraction = if cluster.rtl { 1. - fraction } else { fraction };
    let offset = (fraction * span.chars().count() as f32).round() as usize;
    cluster.index
        + span
            .char_indices()
            .nth(offset)
            .map_or(span.len(), |(index, _)| index)
}

pub(crate) fn selection_spans(
    clusters: &[Cluster],
    text: &str,
    start: usize,
    end: usize,
) -> Vec<(f32, f32)> {
    let mut spans: Vec<(f32, f32)> = clusters
        .iter()
        .filter_map(|cluster| {
            let lo = start.max(cluster.index);
            let hi = end.min(next_logical(clusters, cluster.index, text.len()));
            if lo >= hi || cluster.x1 <= cluster.x0 {
                return None;
            }
            let edge = |index| {
                let total = text
                    .get(cluster.index..next_logical(clusters, cluster.index, text.len()))?
                    .chars()
                    .count()
                    .max(1) as f32;
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
    use gpui::{FontId, GlyphId, ShapedGlyph, point, px};

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
