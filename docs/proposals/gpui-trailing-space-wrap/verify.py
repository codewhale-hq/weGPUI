"""Check a proposed shaped-row trailing-space change without modifying vendored source."""

import argparse
import difflib
import importlib.util
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location(
    "logical_harness", ROOT / "docs/proposals/gpui-text-i18n/verify-logical-wrap.py"
)
harness = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(harness)
SOURCE = ROOT / "vendor/gpui_pre/src/text_system/line_layout.rs"
NEEDLE = "if width > wrap_width && boundary > last_boundary {"
REPLACEMENT = """if width > wrap_width
                && boundary > last_boundary
                && (ch != ' ' || first_non_whitespace_ix.is_none())
            {"""
TESTS = r'''
    #[cfg(test)]
    mod trailing_space_tests {
        use super::*;
        fn boundaries(text: &str, width: f32) -> Vec<usize> {
            let glyphs = text.char_indices().enumerate().map(|(ordinal, (index, _))| ShapedGlyph {
                id: GlyphId(0), position: point(px(ordinal as f32), px(0.)), index, is_emoji: false,
            }).collect();
            let layout = LineLayout {
                font_size: px(1.), width: px(text.chars().count() as f32), ascent: px(1.), descent: px(0.),
                runs: vec![ShapedRun { font_id: FontId(0), glyphs }], len: text.len(),
            };
            layout.compute_wrap_boundaries(text, px(width), None).iter().map(|boundary| {
                layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
            }).collect()
        }
        #[test] fn fitting_word_before_space_stays_on_row() { assert_eq!(boundaries("aa bb cc",5.),vec![6]); }
        #[test] fn fitting_word_before_multiple_spaces_stays_on_row() { assert_eq!(boundaries("aa bb   cc",5.),vec![8]); }
        #[test] fn trailing_space_does_not_add_row() { assert_eq!(boundaries("aa bb ",5.),vec![]); }
        #[test] fn exact_fit_without_trailing_space() { assert_eq!(boundaries("aa bb",5.),vec![]); }
        #[test] fn genuinely_overflowing_word_moves() { assert_eq!(boundaries("aa bbb cc",5.),vec![3,7]); }
        #[test] fn oversized_word_still_splits() { assert_eq!(boundaries("abcdeX",5.),vec![5]); }
        #[test] fn leading_space_policy_is_preserved() { assert_eq!(boundaries("   aa bb",3.),vec![3,6]); }
        #[test] fn unspaced_cjk_still_wraps() { assert_eq!(boundaries("你好世界",3.),vec![9]); }
    }
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workdir", required=True, type=Path)
    args = parser.parse_args()
    args.workdir.mkdir(parents=True, exist_ok=True)
    current = SOURCE.read_text()
    original = current.replace(REPLACEMENT, NEEDLE) if current.count(REPLACEMENT) == 1 else current
    if original.count(NEEDLE) != 1:
        parser.error("Source changed; review the proposed replacement")
    patched = original.replace(NEEDLE, REPLACEMENT)
    patch = "".join(difflib.unified_diff(original.splitlines(True), patched.splitlines(True),
                                     fromfile="a/vendor/gpui_pre/src/text_system/line_layout.rs",
                                     tofile="b/vendor/gpui_pre/src/text_system/line_layout.rs"))
    (args.workdir / "trailing-space.patch").write_text(patch)
    wrapper = (SOURCE.parent / "line_wrapper.rs").read_text()
    classifier = harness.function_text(wrapper, "is_word_char")
    for name, source in [("baseline", original), ("proposal", patched)]:
        block = source[source.index("/// A laid out and styled line of text"):source.index("pub(crate) struct LineLayoutCache")]
        generated = harness.STUBS + classifier + "    }\n" + block + TESTS
        target = args.workdir / (name + ".rs")
        target.write_text(generated)
        binary = args.workdir / name
        subprocess.run(["rustc", "--edition", "2024", "--test", str(target), "-o", str(binary)], check=True)
        result = subprocess.run([str(binary), "--test-threads=1"], capture_output=True, text=True)
        print(name, "exit", result.returncode, flush=True)
        print(result.stdout, flush=True)
        (args.workdir / (name + ".log")).write_text(result.stdout + result.stderr)
        if (name == "baseline" and result.returncode == 0) or (name == "proposal" and result.returncode != 0):
            raise SystemExit("Unexpected result; inspect the retained logs")


if __name__ == "__main__":
    main()
