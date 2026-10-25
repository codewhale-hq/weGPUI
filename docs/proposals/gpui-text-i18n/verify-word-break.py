"""Run the word-break unit tests of the vendored GPUI line wrapper without building GPUI.

The vendored `gpui-pre` test target cannot compile from the packaged crate (it lacks the
upstream test fonts and dev-dependencies), so this extracts two things from the real source:

  * the non-test part of `vendor/gpui_pre/src/text_system/line_wrapper.rs`, compiled unmodified
    against a small stand-in for the GPUI items it uses (`Pixels`, `FontId`, `SharedString`,
    `TextRun`, and a `TextSystem` whose every character is 9.6 px wide, the 0.6 em advance of the
    mono font the upstream tests use at 16 px);
  * the unit tests named below, copied verbatim from the test module.

Usage:
  python3 verify-word-break.py [--code FILE] [--tests FILE] [--workdir DIR]

`--code` and `--tests` default to the vendored file. Pointing `--code` at the unpatched
registry copy and `--tests` at the patched file shows the new tests failing without the change.
Exit status is the test binary's.
"""

import argparse
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent
DEFAULT = None
for candidate in [ROOT, *ROOT.parents]:
    path = candidate / "vendor/gpui_pre/src/text_system/line_wrapper.rs"
    if path.exists():
        DEFAULT = path
        break

TEST_NAMES = r"test_is_word_char\w*|test_wrap_line_breaks_only_between_words\w*"
MARKER = "#[cfg(test)]\nmod tests {"

STUBS = r'''#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use std::{
    fmt,
    ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub, SubAssign},
    sync::Arc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Pixels(pub f32);
pub fn px(value: f32) -> Pixels {
    Pixels(value)
}
impl Pixels {
    pub fn floor(self) -> Self {
        Pixels(self.0.floor())
    }
    pub fn max(self, other: Self) -> Self {
        Pixels(self.0.max(other.0))
    }
    pub fn min(self, other: Self) -> Self {
        Pixels(self.0.min(other.0))
    }
}
impl Add for Pixels {
    type Output = Pixels;
    fn add(self, rhs: Pixels) -> Pixels {
        Pixels(self.0 + rhs.0)
    }
}
impl Sub for Pixels {
    type Output = Pixels;
    fn sub(self, rhs: Pixels) -> Pixels {
        Pixels(self.0 - rhs.0)
    }
}
impl AddAssign for Pixels {
    fn add_assign(&mut self, rhs: Pixels) {
        self.0 += rhs.0;
    }
}
impl SubAssign for Pixels {
    fn sub_assign(&mut self, rhs: Pixels) {
        self.0 -= rhs.0;
    }
}
impl Mul<f32> for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: f32) -> Pixels {
        Pixels(self.0 * rhs)
    }
}
impl Mul<usize> for Pixels {
    type Output = Pixels;
    fn mul(self, rhs: usize) -> Pixels {
        Pixels(self.0 * rhs as f32)
    }
}
impl std::iter::Sum for Pixels {
    fn sum<I: Iterator<Item = Pixels>>(iter: I) -> Pixels {
        Pixels(iter.map(|p| p.0).sum())
    }
}
impl Div<f32> for Pixels {
    type Output = Pixels;
    fn div(self, rhs: f32) -> Pixels {
        Pixels(self.0 / rhs)
    }
}
impl Neg for Pixels {
    type Output = Pixels;
    fn neg(self) -> Pixels {
        Pixels(-self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SharedString(pub Arc<str>);
impl From<String> for SharedString {
    fn from(value: String) -> Self {
        SharedString(value.into())
    }
}
impl From<&str> for SharedString {
    fn from(value: &str) -> Self {
        SharedString(value.into())
    }
}
impl Deref for SharedString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextRun {
    pub len: usize,
}

pub struct TextSystem;
impl TextSystem {
    pub fn layout_width(&self, _font: FontId, _size: Pixels, _c: char) -> Pixels {
        Pixels(9.6)
    }
}

pub mod collections {
    pub use std::collections::HashMap;
}

pub mod line_wrapper {
    use super::collections;
'''


def split(text):
    at = text.index(MARKER)
    return text[:at], text[at:]


def extract_tests(tests_part):
    found = re.findall(
        rf"(    #\[test\]\n    fn (?:{TEST_NAMES})\(\) \{{\n.*?\n    \}}\n)",
        tests_part,
        flags=re.S,
    )
    if not found:
        sys.exit("no matching tests found")
    return found


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--code", type=pathlib.Path, default=DEFAULT)
    ap.add_argument("--tests", type=pathlib.Path, default=DEFAULT)
    ap.add_argument("--workdir", type=pathlib.Path)
    args = ap.parse_args()
    code, _ = split(args.code.read_text(encoding="utf-8"))
    _, tests_part = split(args.tests.read_text(encoding="utf-8"))
    tests = extract_tests(tests_part)
    names = [re.search(r"fn (\w+)", t).group(1) for t in tests]
    body = STUBS + code + "}\n\n#[cfg(test)]\nmod tests {\n"
    body += "    use super::line_wrapper::*;\n    use super::{FontId, px};\n    use std::sync::Arc;\n\n"
    body += "    fn build_wrapper() -> LineWrapper {\n        LineWrapper::new(FontId(0), px(16.), Arc::new(super::TextSystem))\n    }\n\n"
    body += "\n".join(tests) + "}\n"
    work = args.workdir or pathlib.Path(tempfile.mkdtemp(prefix="wordbreak-"))
    work.mkdir(parents=True, exist_ok=True)
    src = work / "harness.rs"
    src.write_text(body, encoding="utf-8")
    binary = work / "harness"
    print(f"code: {args.code}\ntests: {args.tests}\ntests extracted: {', '.join(names)}")
    build = subprocess.run(
        ["rustc", "--edition", "2024", "--test", str(src), "-o", str(binary)],
        capture_output=True,
        text=True,
    )
    if build.returncode != 0:
        print(build.stderr[-6000:])
        sys.exit(build.returncode)
    run = subprocess.run([str(binary), "--test-threads=1"], capture_output=True, text=True)
    print(run.stdout)
    if run.returncode != 0:
        print(run.stderr[-3000:])
    sys.exit(run.returncode)


if __name__ == "__main__":
    main()
