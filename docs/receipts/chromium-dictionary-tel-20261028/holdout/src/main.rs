mod browser_dictionary;

use icu_collections::char16trie::Char16Trie;
use icu_properties::{
    CodePointMapData,
    props::{GeneralCategory, GeneralCategoryGroup, LineBreak, Script},
};
use icu_segmenter::GraphemeClusterSegmenter;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use zerovec::ZeroVec;

const ORACLE: &str = "/Volumes/VIXinSSD/CW/artifacts/wegpui-overflow-ios-20261028/chromium-icu-78-oracle/oracle-results-retry.json";

fn is_khmer_sa(ch: char) -> bool {
    CodePointMapData::<Script>::new().get(ch) == Script::Khmer
        && CodePointMapData::<LineBreak>::new().get(ch) == LineBreak::ComplexContext
}

fn sampled_cross_script_sa_seam(text: &str, index: usize) -> bool {
    let Some(left) = text[..index].chars().next_back() else { return false };
    let Some(right) = text[index..].chars().next() else { return false };
    let scripts = CodePointMapData::<Script>::new();
    let breaks = CodePointMapData::<LineBreak>::new();
    let left_script = scripts.get(left);
    let right_script = scripts.get(right);
    let sampled = |script| matches!(script, Script::Khmer | Script::Thai | Script::Lao | Script::Myanmar);
    sampled(left_script)
        && sampled(right_script)
        && left_script != right_script
        && breaks.get(left) == LineBreak::ComplexContext
        && breaks.get(right) == LineBreak::ComplexContext
}

fn is_mark(ch: char) -> bool {
    is_khmer_sa(ch)
        && GeneralCategoryGroup::Mark.contains(CodePointMapData::<GeneralCategory>::new().get(ch))
}

fn candidates(chars: &[char], start: usize, trie: &Char16Trie<'_>) -> (Vec<usize>, usize) {
    let mut cursor = trie.cursor();
    let mut matches = Vec::new();
    let mut prefix = 0;
    for (offset, &ch) in chars[start..].iter().enumerate() {
        cursor.step(ch);
        let value = cursor.value();
        if value.is_some() && matches.len() < 20 {
            matches.push(start + offset + 1);
        }
        if value.is_some() || !cursor.is_empty() {
            prefix = offset + 1;
        }
        if cursor.is_empty() {
            break;
        }
    }
    (matches, prefix)
}

fn khmer_breaks(span: &str, trie: &Char16Trie<'_>) -> Option<Vec<usize>> {
    let chars: Vec<char> = span.chars().collect();
    if chars.iter().any(|ch| ch.len_utf16() != 1) {
        return None;
    }
    if chars.len() < 4 {
        return Some(Vec::new());
    }
    let mut byte_offsets: Vec<usize> = span.char_indices().map(|(i, _)| i).collect();
    byte_offsets.push(span.len());
    let mut pos = 0;
    let mut result = Vec::new();
    while pos < chars.len() {
        let start = pos;
        let (words, _) = candidates(&chars, start, trie);
        let mut end = words.last().copied().unwrap_or(start);
        if words.len() > 1 && end < chars.len() {
            let mut chosen = end;
            'first: for &first in words.iter().rev() {
                let (second_words, _) = candidates(&chars, first, trie);
                if second_words.is_empty() {
                    continue;
                }
                chosen = first;
                if second_words.last() == Some(&chars.len()) {
                    break;
                }
                for &second in second_words.iter().rev() {
                    if !candidates(&chars, second, trie).0.is_empty() {
                        chosen = first;
                        break 'first;
                    }
                }
            }
            end = chosen;
        }
        let length = end - start;
        if end < chars.len() && length < 10 {
            let (following, prefix) = candidates(&chars, end, trie);
            if following.is_empty() && (length == 0 || prefix < 5) {
                let mut scan = end;
                while scan < chars.len() {
                    let previous = chars[scan];
                    scan += 1;
                    if scan >= chars.len() {
                        break;
                    }
                    let next = chars[scan];
                    if previous != '\u{17d2}'
                        && is_khmer_sa(previous)
                        && ('\u{1780}'..='\u{17b3}').contains(&next)
                        && !candidates(&chars, scan, trie).0.is_empty()
                    {
                        break;
                    }
                }
                end = scan;
            }
        }
        while end < chars.len() && (chars[end] == ' ' || is_mark(chars[end])) {
            end += 1;
        }
        if end <= start {
            return None;
        }
        pos = end;
        if end < chars.len() {
            result.push(byte_offsets[end]);
        }
    }
    Some(result)
}

fn spans(text: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut start = None;
    for (index, ch) in text.char_indices() {
        if is_khmer_sa(ch) {
            start.get_or_insert(index);
        } else if let Some(from) = start.take() {
            found.push((from, index));
        }
    }
    if let Some(from) = start {
        found.push((from, text.len()));
    }
    found
}

fn combined(text: &str, trie: &Char16Trie<'_>) -> (Vec<usize>, usize) {
    let base = browser_dictionary::break_indices(text);
    let mut replace = Vec::new();
    for (start, end) in spans(text) {
        if let Some(indices) = khmer_breaks(&text[start..end], trie) {
            replace.push((start, end, indices));
        }
    }
    let mut result: Vec<usize> = base
        .iter()
        .copied()
        .filter(|index| {
            !replace
                .iter()
                .any(|(start, end, _)| start < index && index < end)
        })
        .collect();
    let graphemes: Vec<usize> = GraphemeClusterSegmenter::new().segment_str(text).collect();
    let mut suppressed = 0;
    for (start, _, boundaries) in &replace {
        for boundary in boundaries {
            let candidate = start + boundary;
            if graphemes.binary_search(&candidate).is_ok() {
                result.push(candidate);
            } else {
                suppressed += 1;
            }
        }
    }
    result.sort_unstable();
    result.dedup();
    result.retain(|&index| !sampled_cross_script_sa_seam(text, index));
    let outside = |index: &usize| {
        !replace
            .iter()
            .any(|(start, end, _)| start < index && index < end)
    };
    assert_eq!(
        base.iter()
            .filter(|index| outside(index) && !sampled_cross_script_sa_seam(text, **index))
            .copied()
            .collect::<Vec<_>>(),
        result
            .iter()
            .filter(|index| outside(index) && !sampled_cross_script_sa_seam(text, **index))
            .copied()
            .collect::<Vec<_>>()
    );
    assert!(result.iter().all(|index| text.is_char_boundary(*index)));
    (result, suppressed)
}

fn main() {
    let data = ZeroVec::<u16>::parse_bytes(include_bytes!("browser_dictionary/khmerdict.u16le"))
        .expect("generated trie bytes");
    let trie = Char16Trie::new(data);
    let oracle_path = std::env::args().nth(1).unwrap_or_else(|| ORACLE.to_owned());
    let oracle: Value =
        serde_json::from_str(&std::fs::read_to_string(&oracle_path).expect("exact oracle results"))
            .unwrap();
    assert_eq!(oracle["version"], json!([78, 2, 0, 0]));
    let mut per_language: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut cases = Vec::new();
    for entry in oracle["cases"].as_array().unwrap() {
        let text = entry["text"].as_str().unwrap();
        let language = entry["language"].as_str().unwrap();
        let baseline = browser_dictionary::break_indices(text);
        let (candidate, suppressed) = combined(text, &trie);
        if language != "km" {
            assert_eq!(candidate, baseline, "non-Khmer fixture changed");
        }
        let target: Vec<usize> =
            serde_json::from_value(entry["exact_icu4c_boundaries"].clone()).unwrap();
        let count = per_language.entry(language.to_owned()).or_default();
        count.0 += 1;
        count.1 += usize::from(candidate == target);
        let matches = candidate == target;
        cases.push(
            json!({"id": entry["id"], "language": language, "text": text,
                          "baseline": baseline, "candidate": candidate, "oracle": target,
                          "candidate_matches": matches, "suppressed_non_grapheme": suppressed}),
        );
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "oracle": oracle_path, "oracle_version": [78, 2, 0, 0], "per_language": per_language, "cases": cases
    })).unwrap());
}
