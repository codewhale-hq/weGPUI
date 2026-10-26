# Wrapped RTL input continuation — 25 October 2026 (Pacific)

The owner requested remote synchronisation and completion work, then explicitly approved
step 3. Canonical `main` was fast-forwarded from `b1ddcae` to `9b618ae`; the pre-sync dirty
files were preserved outside the repository. Their production changes were already carried
by the reviewed remote commits. The obsolete local-only TextArea experiment was superseded;
this continuation adds a deliberate editable paragraph example for current verification.

## Implementation and scope

- Normalise platform glyphs to visual x order before caching their layouts. Cosmic Text can
  supply right-to-left traversal, while the shared wrapping code expects ascending x. Font
  identity, glyph identity, positions and logical byte indices are preserved.
- Cache row-local clusters for input. Caret, pointer hit testing, vertical movement and partial
  selection use the same shaped row; mixed-direction selections retain separate visual spans.
  Duplicate cluster glyphs are combined and ligature selections interpolate within the glyph.
- Align TextArea rows by locale, keep hit-test endpoints on grapheme boundaries, and use the
  next row at a soft-wrap boundary in both native and accessibility caret affinity. IME range
  bounds follow the selected visual spans on the caret's row.
- Retain the existing bidi shim. Generic GPUI `LineLayout` x/index APIs and their other callers
  are not rewritten by this input implementation. Uncertain bidi character extents remain
  absent from accessibility; exact font ligature caret data and real RTL IME remain unqualified.
- Reject uniform native captures, preserving the invalid PNG and returning failure. Clear
  modifier flags on synthetic Unicode typing. Make the existing Linux sweep orchestration
  compatible with macOS Bash 3's empty-array behavior and replace its `mapfile` dependency.

The library dependency graph, pins and lockfile are unchanged. No package release, website
publication, signing, deployment or paid provider call was performed.

## Checks and native evidence

The [source manifest](source-sha256.json) and [binary hashes](binaries.json) identify the Rust
source and executables. All 59 library tests, three gallery tests and the four existing extracted
text harnesses pass. The latter run 7 word-break, 16+4 logical-wrap, 8 trailing-space and 13 CJK
cases. Native gallery, minimal and application examples build on macOS; the gallery builds on
Linux ARM64. All 33 script tests pass after the Bash fix. Logs are in `checks/`.

macOS ARM64 native keyboard/pointer events and independent AX text readback pass select-all,
three-character partial selection (`تدع`), cross-row selection, Arabic/Latin insertion
(`مرحبا 2026 hello`), word undo, complete restoration and pointer drag selection. JSON values
and selected ranges are retained under `macos/`, with representative images. Ten current-source
launch/capture checks cover Button, Form, SearchBar, TextArea and Languages in English/Arabic.
They are startup observations, not complete component acceptance.

Linux ARM64 runs actual X11 clients in a local Docker verification environment, with Xvfb,
Openbox, Mesa 25.0.7 lavapipe and Rust 1.99.1. English/Arabic and light/dark captures show complete
Arabic/Hebrew mixed paragraphs, and the Arabic TextArea aligns and selects across wrapped rows.
Real XTEST events and AT-SPI readback independently pass select-all, `تدع`, cross-row selection,
ASCII insertion, complete undo restoration and pointer drag selection. See `linux/text/result.json`.
This is native Linux software-renderer evidence on ARM64, not Linux x86_64 or hardware acceptance.

## Preserved failures and limits

The first macOS driver inherited a Command modifier during Unicode injection. Clearing the
flags fixes the driver; the failed attempt is retained. A second driver assertion expected a
single undo to restore all typed words, contrary to the existing word-grouping policy. The
corrected run verifies both word undo and complete restoration. Neither was a product input fix.

The initial Linux captures were uniformly black. A diagnostic application confirmed the
software adapter and actual drawing. Mesa was upgraded in the disposable verification container,
and a five-second capture settle produced reviewable gallery content. The old black PNG and
logs remain failures. A deliberately uniform-image control is rejected by the new capture guard;
a real native image is accepted. The first AT-SPI setup used the wrong D-Bus object path; its
failure is retained, and the corrected run uses `/org/a11y/bus`.

The verification environment lacks Japanese/Korean/emoji fallback fonts in the surrounding
Languages examples; those missing glyphs are visible and do not establish script acceptance.
The reviewed paragraphs contain the full Arabic/Hebrew/Latin samples. Native-speaker review,
real RTL IME composition, Windows, Linux x86_64/hardware/windowing diversity, styled wrapping,
complete actual-reader matrices, locale-aware dates and fresh installation/distribution remain
open. VoiceOver was left off. No component or platform is promoted to fully accepted by this slice.

Historical publication captures and receipts keep their original source identity. The 1,056-pair
matrix at `9b618ae` was not regenerated or relabelled as evidence for this later text source.
Driver copies and command logs redact host-specific absolute paths; image bytes are unchanged.
