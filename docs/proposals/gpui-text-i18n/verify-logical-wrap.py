"""Run the unit tests of the vendored GPUI logical-order wrapping without building GPUI.

The packaged `gpui-pre` test target cannot compile (it lacks the upstream test fonts and
dev-dependencies; see verify-word-break.py), so two stages run here:

  1. `text_system/logical_wrap.rs` depends on nothing but `std`, so it is compiled on its own as a
     test crate.
  2. The layout types of `text_system/line_layout.rs` (everything before `LineLayoutCache`) are
     compiled unmodified against a small stand-in for the GPUI items they use (`Pixels`, `Point`,
     `Size`, `FontId`, `GlyphId`, `SmallVec`) together with the real `LineWrapper::is_word_char`,
     and the layout tests named below are copied verbatim from the file's test module and run.

Usage:
  python3 verify-logical-wrap.py [--workdir DIR] [--glyph-derived-rows]

`--glyph-derived-rows` rewrites stage 2 so that rows start at the index of their first glyph, as
before logical wrapping existed; the logical-row tests are then expected to fail.

The exit status is non-zero if either stage fails.
"""

import argparse
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent
TEXT_SYSTEM = None
for candidate in [ROOT, *ROOT.parents]:
    path = candidate / "vendor/gpui_pre/src/text_system"
    if path.exists():
        TEXT_SYSTEM = path
        break

HELPERS = ["glyph_at", "make_layout", "logical_two_row_layout"]
TESTS = r"logical_rows\w*|rows_that_were_not_wrapped_logically\w*"

STUBS = r'''#![allow(dead_code, unused_imports, unused_variables, unused_mut, clippy::all)]
use std::{
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub, SubAssign},
    sync::Arc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Pixels(pub f32);
pub fn px(value: f32) -> Pixels {
    Pixels(value)
}
impl Pixels {
    pub const ZERO: Pixels = Pixels(0.);
    pub const MAX: Pixels = Pixels(f32::MAX);
    pub fn min(self, other: Self) -> Self {
        Pixels(self.0.min(other.0))
    }
    pub fn max(self, other: Self) -> Self {
        Pixels(self.0.max(other.0))
    }
    pub fn as_f32(self) -> f32 {
        self.0
    }
    pub fn half(self) -> Self {
        Pixels(self.0 / 2.)
    }
}
impl From<Pixels> for f32 {
    fn from(value: Pixels) -> f32 {
        value.0
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
impl Mul<Pixels> for f32 {
    type Output = Pixels;
    fn mul(self, rhs: Pixels) -> Pixels {
        Pixels(self * rhs.0)
    }
}
impl Div<f32> for Pixels {
    type Output = Pixels;
    fn div(self, rhs: f32) -> Pixels {
        Pixels(self.0 / rhs)
    }
}
impl Div<Pixels> for Pixels {
    type Output = f32;
    fn div(self, rhs: Pixels) -> f32 {
        self.0 / rhs.0
    }
}
impl Neg for Pixels {
    type Output = Pixels;
    fn neg(self) -> Pixels {
        Pixels(-self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
pub fn point<T>(x: T, y: T) -> Point<T> {
    Point { x, y }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FontId(pub usize);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GlyphId(pub u32);

pub trait Array {
    type Item;
}
impl<T, const N: usize> Array for [T; N] {
    type Item = T;
}
pub struct SmallVec<A: Array>(Vec<A::Item>);
impl<A: Array> SmallVec<A> {
    pub fn new() -> Self {
        SmallVec(Vec::new())
    }
}
impl<T, const N: usize> SmallVec<[T; N]> {
    pub fn from_buf(buf: [T; N]) -> Self {
        SmallVec(buf.into_iter().collect())
    }
}
impl<A: Array> Default for SmallVec<A> {
    fn default() -> Self {
        SmallVec(Vec::new())
    }
}
impl<A: Array> Clone for SmallVec<A>
where
    A::Item: Clone,
{
    fn clone(&self) -> Self {
        SmallVec(self.0.clone())
    }
}
impl<A: Array> fmt::Debug for SmallVec<A>
where
    A::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<A: Array> Deref for SmallVec<A> {
    type Target = [A::Item];
    fn deref(&self) -> &[A::Item] {
        &self.0
    }
}
impl<A: Array> DerefMut for SmallVec<A> {
    fn deref_mut(&mut self) -> &mut [A::Item] {
        &mut self.0
    }
}
impl<A: Array> SmallVec<A> {
    pub fn push(&mut self, value: A::Item) {
        self.0.push(value);
    }
}

pub mod layout {
    use super::*;

    pub(crate) struct LineWrapper;
    impl LineWrapper {
'''


def function_text(source, name):
    pattern = rf"(?:    #\[test\]\n)?    (?:pub(?:\(crate\))? )?fn {name}\b.*?\n    \}}\n"
    match = re.search(pattern, source, flags=re.S)
    if not match:
        sys.exit(f"function {name} not found")
    return match.group(0)


def run(command, **kwargs):
    return subprocess.run(command, capture_output=True, text=True, **kwargs)


def stage_module(work):
    module = TEXT_SYSTEM / "logical_wrap.rs"
    binary = work / "logical_wrap_tests"
    print(f"== stage 1: {module.relative_to(TEXT_SYSTEM.parents[3])}")
    build = run(["rustc", "--edition", "2024", "--test", "-A", "dead_code", str(module), "-o", str(binary)])
    if build.returncode != 0:
        print(build.stderr[-6000:])
        return build.returncode
    result = run([str(binary), "--test-threads=1"])
    print(result.stdout)
    if result.returncode != 0:
        print(result.stderr[-3000:])
    return result.returncode


def stage_layout(work, glyph_derived_rows=False):
    layout_source = (TEXT_SYSTEM / "line_layout.rs").read_text(encoding="utf-8")
    wrapper_source = (TEXT_SYSTEM / "line_wrapper.rs").read_text(encoding="utf-8")
    start = layout_source.index("/// A laid out and styled line of text")
    end = layout_source.index("pub(crate) struct LineLayoutCache")
    block = layout_source[start:end]
    if glyph_derived_rows:
        needle = "if let Some(&index) = self.logical_line_starts.get(row - 1) {"
        if needle not in block:
            sys.exit("wrapped_line_start_index changed; update --glyph-derived-rows")
        block = block.replace(needle, "if let Some(&index) = None::<&usize> {")
    tests_part = layout_source[layout_source.index("#[cfg(test)]\nmod tests {"):]
    is_word_char = function_text(wrapper_source, "is_word_char")
    names = list(HELPERS) + re.findall(rf"fn ({TESTS})\(\)", tests_part)
    tests = "\n".join(function_text(tests_part, name) for name in names)
    body = STUBS + is_word_char + "    }\n\n" + block
    body += "\n    #[cfg(test)]\n    mod tests {\n        use super::*;\n\n" + tests + "    }\n}\n"
    source = work / "layout_harness.rs"
    source.write_text(body, encoding="utf-8")
    binary = work / "layout_harness"
    print(f"== stage 2: line_layout.rs layout types; tests: {', '.join(n for n in names if n not in HELPERS)}")
    build = run(["rustc", "--edition", "2024", "--test", str(source), "-o", str(binary)])
    if build.returncode != 0:
        print(build.stderr[-8000:])
        return build.returncode
    result = run([str(binary), "--test-threads=1"])
    print(result.stdout)
    if result.returncode != 0:
        print(result.stderr[-3000:])
    return result.returncode


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--workdir", type=pathlib.Path)
    ap.add_argument("--glyph-derived-rows", action="store_true")
    args = ap.parse_args()
    work = args.workdir or pathlib.Path(tempfile.mkdtemp(prefix="logicalwrap-"))
    work.mkdir(parents=True, exist_ok=True)
    status = 0 if args.glyph_derived_rows else stage_module(work)
    status = stage_layout(work, args.glyph_derived_rows) or status
    sys.exit(status)


if __name__ == "__main__":
    main()
