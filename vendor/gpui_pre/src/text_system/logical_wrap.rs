use std::ops::Range;

pub(crate) const LRM: char = '\u{200E}';
pub(crate) const RLM: char = '\u{200F}';

#[cfg(test)]
fn is_rtl_char(c: char) -> bool {
    matches!(
        unicode_bidi::bidi_class(c),
        unicode_bidi::BidiClass::R | unicode_bidi::BidiClass::AL
    )
}

pub(crate) fn base_direction_is_rtl(text: &str) -> bool {
    unicode_bidi::ParagraphBidiInfo::new(text, None)
        .paragraph_level
        .is_rtl()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Unit {
    pub index: usize,
    pub ch: char,
    pub advance: f32,
}

pub(crate) fn logical_units(visual: &[(usize, f32)], total_width: f32, text: &str) -> Vec<Unit> {
    let mut ordered: Vec<(usize, usize, f32)> = visual
        .iter()
        .enumerate()
        .map(|(ordinal, &(index, x))| {
            let next_x = visual.get(ordinal + 1).map_or(total_width, |&(_, x)| x);
            (index, ordinal, (next_x - x).max(0.))
        })
        .collect();
    ordered.sort_by_key(|&(index, ordinal, _)| (index, ordinal));
    let mut units: Vec<Unit> = Vec::with_capacity(ordered.len());
    for (index, _, advance) in ordered {
        if let Some(last) = units.last_mut()
            && last.index == index
        {
            last.advance += advance;
            continue;
        }
        let Some(ch) = text.get(index..).and_then(|rest| rest.chars().next()) else {
            return Vec::new();
        };
        units.push(Unit { index, ch, advance });
    }
    units
}

pub(crate) fn wrap(
    units: &[Unit],
    wrap_width: f32,
    max_lines: Option<usize>,
    can_break_before: impl Fn(usize, char, char) -> bool,
) -> Option<Vec<usize>> {
    wrap_with_policy(units, wrap_width, max_lines, true, can_break_before)
}

pub(crate) fn wrap_with_policy(
    units: &[Unit],
    wrap_width: f32,
    max_lines: Option<usize>,
    emergency_wrap: bool,
    can_break_before: impl Fn(usize, char, char) -> bool,
) -> Option<Vec<usize>> {
    let mut starts = Vec::new();
    let mut first_non_whitespace = None;
    let mut last_candidate: Option<(usize, f32)> = None;
    let mut last_boundary = 0;
    let mut last_boundary_x = 0.;
    let mut prev_ch = '\0';
    let mut x = 0.;
    for (i, unit) in units.iter().enumerate() {
        let ch = unit.ch;
        let next_x = x + unit.advance;
        if ch == '\n' {
            x = next_x;
            continue;
        }
        if can_break_before(unit.index, prev_ch, ch) && first_non_whitespace.is_some() {
            last_candidate = Some((i, x));
        }
        if ch != ' ' && first_non_whitespace.is_none() {
            first_non_whitespace = Some(i);
        }
        if next_x - last_boundary_x > wrap_width && i > last_boundary {
            if let Some(max_lines) = max_lines
                && starts.len() >= max_lines.saturating_sub(1)
            {
                break;
            }
            let boundary = last_candidate
                .take()
                .filter(|(ordinal, _)| *ordinal > last_boundary)
                .or_else(|| {
                    (emergency_wrap && prev_ch.is_alphanumeric() && ch.is_alphanumeric())
                        .then_some((i, x))
                });
            if let Some((ordinal, candidate_x)) = boundary {
                last_boundary = ordinal;
                last_boundary_x = candidate_x;
                starts.push(units[ordinal].index);
            }
        }
        prev_ch = ch;
        x = next_x;
    }
    Some(starts)
}

pub(crate) struct Placed<R, G> {
    pub runs: Vec<(R, Vec<G>)>,
    pub width: f32,
}

pub(crate) fn place_line<R: Copy, G>(
    runs: &[(R, &[G])],
    width: f32,
    mark_len: usize,
    line_start: usize,
    x_offset: f32,
    index_of: impl Fn(&G) -> usize,
    x_of: impl Fn(&G) -> f32,
    rebuild: impl Fn(&G, f32, usize) -> G,
) -> Option<Placed<R, G>> {
    let mut min_x = f32::INFINITY;
    let mut mark_x = None;
    for glyph in runs.iter().flat_map(|(_, glyphs)| glyphs.iter()) {
        if index_of(glyph) < mark_len {
            mark_x = Some(x_of(glyph));
        } else {
            min_x = min_x.min(x_of(glyph));
        }
    }
    if !min_x.is_finite() {
        return None;
    }
    let right_edge = match mark_x {
        Some(mark_x) if mark_x > min_x => mark_x,
        _ => width,
    };
    let runs = runs
        .iter()
        .filter_map(|&(font, glyphs)| {
            let glyphs: Vec<G> = glyphs
                .iter()
                .filter(|glyph| index_of(glyph) >= mark_len)
                .map(|glyph| {
                    rebuild(
                        glyph,
                        x_of(glyph) - min_x + x_offset,
                        index_of(glyph) - mark_len + line_start,
                    )
                })
                .collect();
            (!glyphs.is_empty()).then_some((font, glyphs))
        })
        .collect();
    Some(Placed {
        runs,
        width: right_edge - min_x,
    })
}

pub(crate) fn slice_runs<T: Copy>(runs: &[(usize, T)], range: Range<usize>) -> Vec<(usize, T)> {
    let mut sliced = Vec::new();
    let mut start = 0;
    for &(len, value) in runs {
        let end = start + len;
        let low = start.max(range.start);
        let high = end.min(range.end);
        if low < high {
            sliced.push((high - low, value));
        }
        start = end;
    }
    sliced
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(_: usize, previous: char, c: char) -> bool {
        previous == ' ' && !c.is_whitespace()
    }

    fn uniform_units(text: &str, advance: f32) -> Vec<Unit> {
        text.char_indices()
            .map(|(index, ch)| Unit { index, ch, advance })
            .collect()
    }

    #[test]
    fn rtl_characters_are_classified_by_script() {
        for c in ['ب', 'ש', 'ܐ', 'ހ', 'ﭐ', 'ﺎ', '\u{200F}'] {
            assert!(is_rtl_char(c), "{c:?} should be right to left");
        }
        for c in [
            'a', 'я', 'न', 'α', '你', '1', '٣', '۴', ' ', '\u{200E}', '.',
        ] {
            assert!(!is_rtl_char(c), "{c:?} should not be right to left");
        }
    }

    #[test]
    fn base_direction_follows_the_first_strong_character() {
        assert!(base_direction_is_rtl("مرحبا hello"));
        assert!(base_direction_is_rtl("123 (שלום) hello"));
        assert!(!base_direction_is_rtl("hello مرحبا"));
        assert!(!base_direction_is_rtl("123 ... 456"));
        assert!(!base_direction_is_rtl(""));
    }

    #[test]
    fn units_are_logical_even_when_the_shaper_returns_visual_order() {
        let text = "اب جد";
        let visual = [(7, 0.), (5, 10.), (4, 20.), (2, 30.), (0, 40.)];
        let units = logical_units(&visual, 50., text);
        assert_eq!(
            units,
            [
                Unit {
                    index: 0,
                    ch: 'ا',
                    advance: 10.
                },
                Unit {
                    index: 2,
                    ch: 'ب',
                    advance: 10.
                },
                Unit {
                    index: 4,
                    ch: ' ',
                    advance: 10.
                },
                Unit {
                    index: 5,
                    ch: 'ج',
                    advance: 10.
                },
                Unit {
                    index: 7,
                    ch: 'د',
                    advance: 10.
                },
            ]
        );
    }

    #[test]
    fn glyphs_of_one_character_share_a_unit() {
        let text = "لا ب";
        let visual = [(5, 0.), (4, 12.), (0, 22.)];
        let units = logical_units(&visual, 30., text);
        assert_eq!(units.iter().map(|u| u.index).collect::<Vec<_>>(), [0, 4, 5]);
        let split = [(5, 0.), (0, 10.), (0, 14.), (4, 24.)];
        let units = logical_units(&split, 34., text);
        assert_eq!(units.iter().map(|u| u.index).collect::<Vec<_>>(), [0, 4, 5]);
        assert_eq!(units[0].advance, 14.);
    }

    #[test]
    fn marks_with_a_negative_offset_never_shrink_the_line() {
        let text = "بَ جد";
        let visual = [(7, 0.), (5, 10.), (4, 20.), (0, 30.), (2, 28.)];
        let total: f32 = logical_units(&visual, 40., text)
            .iter()
            .map(|u| u.advance)
            .sum();
        assert!(total >= 40.);
    }

    #[test]
    fn breaks_fall_on_word_starts_in_logical_order() {
        let text = "aa bb cc";
        let units = uniform_units(text, 10.);
        assert_eq!(wrap(&units, 45., None, word), Some(vec![3, 6]));
        assert_eq!(wrap(&units, 85., None, word), Some(vec![]));
    }

    #[test]
    fn arabic_text_breaks_between_words_not_inside_them() {
        let text = "نص حكيم له سر قاطع وذو شأن عظيم";
        let units = uniform_units(text, 10.);
        let starts = wrap(&units, 150., None, word).unwrap();
        assert!(!starts.is_empty());
        for start in starts {
            assert_eq!(text[..start].chars().last(), Some(' '));
        }
    }

    #[test]
    fn a_word_wider_than_the_line_breaks_at_shaped_cluster_boundaries() {
        let units = uniform_units("aaaaaaaa", 10.);
        assert_eq!(wrap(&units, 25., None, word), Some(vec![2, 4, 6]));
        assert_eq!(
            wrap_with_policy(&units, 25., None, false, word),
            Some(vec![])
        );
        let text = "aa aaaaaaaa bb";
        let units = uniform_units(text, 10.);
        assert_eq!(
            wrap_with_policy(&units, 35., None, false, word),
            Some(vec![3, 12])
        );
        let units = uniform_units("aaaaaaaa bb", 10.);
        assert_eq!(
            wrap_with_policy(&units, 35., None, false, word),
            Some(vec![9])
        );
        let units = uniform_units("a b c", 10.);
        assert_eq!(
            wrap_with_policy(&units, 0., None, false, word),
            Some(vec![2, 4])
        );
        assert_eq!(
            wrap_with_policy(&units, 0., Some(2), false, word),
            Some(vec![2])
        );
    }

    #[test]
    fn line_clamp_stops_adding_breaks() {
        let units = uniform_units("aa bb cc dd ee ff", 10.);
        assert_eq!(wrap(&units, 45., Some(2), word).unwrap().len(), 1);
        assert_eq!(wrap(&units, 45., Some(1), word).unwrap().len(), 0);
        assert!(wrap(&units, 45., None, word).unwrap().len() > 2);
    }

    #[test]
    fn candidates_include_characters_that_are_not_word_characters() {
        let units = uniform_units("ab/cd/ef", 10.);
        let starts = wrap(&units, 35., None, |_, previous, c| {
            c != ' ' && (!c.is_alphanumeric() || previous == ' ')
        })
        .unwrap();
        assert_eq!(starts, [2, 5]);
    }

    type Glyph = (usize, f32);

    fn place(
        runs: &[(char, &[Glyph])],
        width: f32,
        line_start: usize,
        x_offset: f32,
    ) -> Option<Placed<char, Glyph>> {
        place_line(
            runs,
            width,
            3,
            line_start,
            x_offset,
            |g| g.0,
            |g| g.1,
            |_, x, index| (index, x),
        )
    }

    #[test]
    fn a_right_to_left_line_loses_its_mark_at_the_right_end() {
        let shaped: [Glyph; 6] = [(10, 0.), (8, 10.), (7, 20.), (5, 30.), (3, 40.), (0, 50.)];
        let placed = place(&[('a', &shaped)], 50., 20, 100.).unwrap();
        assert_eq!(placed.width, 50.);
        assert_eq!(
            placed.runs,
            [(
                'a',
                vec![(27, 100.), (25, 110.), (24, 120.), (22, 130.), (20, 140.)]
            )]
        );
    }

    #[test]
    fn a_left_to_right_line_loses_its_mark_at_the_left_end() {
        let shaped: [Glyph; 4] = [(0, 0.), (3, 0.), (4, 10.), (5, 20.)];
        let placed = place(&[('a', &shaped)], 30., 7, 0.).unwrap();
        assert_eq!(placed.width, 30.);
        assert_eq!(placed.runs, [('a', vec![(7, 0.), (8, 10.), (9, 20.)])]);
    }

    #[test]
    fn a_mark_with_an_advance_is_taken_out_of_the_line_width() {
        let shaped: [Glyph; 3] = [(0, 0.), (3, 2.), (4, 12.)];
        let placed = place(&[('a', &shaped)], 22., 0, 40.).unwrap();
        assert_eq!(placed.width, 20.);
        assert_eq!(placed.runs, [('a', vec![(0, 40.), (1, 50.)])]);
        let shaped: [Glyph; 3] = [(4, 0.), (3, 10.), (0, 20.)];
        let placed = place(&[('a', &shaped)], 22., 0, 0.).unwrap();
        assert_eq!(placed.width, 20.);
    }

    #[test]
    fn runs_keep_their_order_and_empty_ones_are_dropped() {
        let first: [Glyph; 1] = [(0, 0.)];
        let second: [Glyph; 2] = [(5, 4.), (3, 14.)];
        let third: [Glyph; 1] = [(8, 24.)];
        let placed = place(&[('a', &first), ('b', &second), ('c', &third)], 34., 0, 0.).unwrap();
        assert_eq!(
            placed.runs,
            [('b', vec![(2, 0.), (0, 10.)]), ('c', vec![(5, 20.)])]
        );
    }

    #[test]
    fn a_line_with_no_glyphs_but_the_mark_is_rejected() {
        let shaped: [Glyph; 1] = [(0, 0.)];
        assert!(place(&[('a', &shaped)], 0., 0, 0.).is_none());
    }

    #[test]
    fn font_runs_are_cut_to_a_line() {
        let runs = [(4, 'a'), (6, 'b'), (5, 'c')];
        assert_eq!(slice_runs(&runs, 0..15), runs);
        assert_eq!(slice_runs(&runs, 2..7), [(2, 'a'), (3, 'b')]);
        assert_eq!(slice_runs(&runs, 10..15), [(5, 'c')]);
        assert_eq!(slice_runs(&runs, 4..10), [(6, 'b')]);
        assert!(slice_runs(&runs, 7..7).is_empty());
    }
}
