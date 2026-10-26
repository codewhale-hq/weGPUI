"""Trial CJK closing punctuation in extracted GPUI code without editing vendor files."""

import argparse
import difflib
import importlib.util
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location("word_harness", ROOT / "docs/proposals/gpui-text-i18n/verify-word-break.py")
harness = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(harness)
SOURCE = ROOT / "vendor/gpui_pre/src/text_system/line_wrapper.rs"
NEEDLE = "        // Non-breaking glue characters\n"
ADDITION = """        matches!(c, '、' | '。' | '，' | '．' | '：' | '；' | '！' | '？' |
            '）' | '］' | '｝' | '〉' | '》' | '」' | '』' | '】' | '〕' | '〗' | '〙' | '〛') ||

"""
TESTS = r'''
    #[test]
    fn closing_marks_attach_to_the_preceding_cjk_word() {
        for c in "、。，．：；！？）］｝〉》」』】〕〗〙〛".chars() {
            assert!(LineWrapper::is_word_char(c), "{c}");
        }
    }
    #[test]
    fn closing_marks_do_not_start_a_wrapped_row() {
        for c in "、。，．：；！？）］｝〉》」』】〕〗〙〛".chars() {
            let mut wrapper = build_wrapper();
            let text = format!("你好世界{c}天地玄黄");
            let fragments = [LineFragment::text(&text)];
            let first = wrapper.wrap_line(&fragments, px(9.6 * 4.1), IndentAdjustment::NoIndent).next().unwrap();
            assert_eq!(first.ix, "你好世".len(), "{c}");
        }
    }
    #[test]
    fn panel_clamp_now_accounts_for_the_moved_punctuation() {
        let text = "由各种物质组成的巨型球状天体，叫做星球。星球有一定的形状，有自己的运行轨道。";
        let mut wrapper = build_wrapper();
        let runs = [super::TextRun { len: text.len() }];
        let (result, _) = wrapper.truncate_wrapped_line(text.into(), px(9.6 * 19.1), 2, "…", &runs, TruncateFrom::End);
        assert!(result.ends_with('…'), "{result}");
    }
    #[test]
    fn unpunctuated_text_that_fits_stays_complete() {
        let text = "天地玄黄宇宙洪荒天地玄黄宇宙洪荒天地玄黄宇宙洪荒";
        let mut wrapper = build_wrapper();
        let runs = [super::TextRun { len: text.len() }];
        let (result, _) = wrapper.truncate_wrapped_line(text.into(), px(9.6 * 19.1), 2, "…", &runs, TruncateFrom::End);
        assert_eq!(result.as_ref(), text);
    }
    #[test]
    fn opening_marks_and_other_scripts_are_unchanged() {
        for c in "（［｛〈《「『【〔〖〘〚你好ไทย한글".chars() {
            assert!(!LineWrapper::is_word_char(c), "{c}");
        }
    }
    #[test]
    fn ascii_closing_marks_still_attach() {
        for c in "（［｛〈《「『【〔〖〘〚([{‘“".chars() {
            assert!(!LineWrapper::can_break_before(c, '界'), "{c}");
        }
        for c in ")]!}…".chars() { assert!(LineWrapper::is_word_char(c)); }
    }
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workdir", required=True, type=Path)
    args = parser.parse_args()
    args.workdir.mkdir(parents=True, exist_ok=True)
    current = SOURCE.read_text()
    original = current.replace(ADDITION, "") if current.count(ADDITION) == 1 else current
    if original.count(NEEDLE) != 1:
        parser.error("Classifier changed; review this proposal")
    candidate = original.replace(NEEDLE, ADDITION + NEEDLE)
    patch = "".join(difflib.unified_diff(original.splitlines(True), candidate.splitlines(True),
                                     fromfile="a/vendor/gpui_pre/src/text_system/line_wrapper.rs",
                                     tofile="b/vendor/gpui_pre/src/text_system/line_wrapper.rs"))
    (args.workdir / "cjk-closing.patch").write_text(patch)
    for name, source in [("baseline", original), ("proposal", candidate)]:
        code, tests = harness.split(source)
        existing = "\n".join(harness.extract_tests(tests))
        generated = harness.STUBS + code + "}\n#[cfg(test)] mod tests {\n"
        generated += "use super::line_wrapper::*; use super::{FontId,px}; use std::sync::Arc;\n"
        generated += "fn build_wrapper() -> LineWrapper {LineWrapper::new(FontId(0),px(16.),Arc::new(super::TextSystem))}\n"
        generated += existing + TESTS + "}\n"
        target = args.workdir / (name + ".rs")
        target.write_text(generated)
        binary = args.workdir / name
        subprocess.run(["rustc", "--edition", "2024", "--test", str(target), "-o", str(binary)], check=True)
        result = subprocess.run([str(binary), "--test-threads=1"], capture_output=True, text=True)
        (args.workdir / (name + ".log")).write_text(result.stdout + result.stderr)
        print(name, result.returncode, flush=True)
        print(result.stdout, flush=True)
        if (name == "baseline" and result.returncode == 0) or (name == "proposal" and result.returncode != 0):
            raise SystemExit("Unexpected result; review retained logs")


if __name__ == "__main__":
    main()
