# TextInput/TextArea editing contracts — 2026-10-25

Source base: GitHub main `9c4515c`, plus this continuation's input/textarea changes. Final
integrated gates and source/artifact hashes are recorded by the Linux continuation. This
receipt distinguishes source contracts, browser reference behavior, native keyboard/AT-SPI
runtime observations and actual screen-reader speech. `runtime.py` does not run a screen
reader; the separate Orca harness does.

Implemented contracts:

- GPUI `SetTextSelection` requests map only current synthetic text-run IDs and valid grapheme
  indices back to native byte offsets. Reversed selections, password bullets and read-only
  selection are supported; disabled or stale requests are ignored.
- GPUI textual `SetValue` uses the same sanitization, UTF-16 limit, Changed event and undoable
  replacement path as native editing. Read-only/disabled fields reject it. The operation is
  isolated from neighboring typing in undo history.
- Linux/Windows Ctrl+Left/Right, Ctrl+Shift+Left/Right and Ctrl+Backspace/Delete use the existing
  Unicode word boundaries; forward-word deletion is now implemented. The attached Chromium
  Linux reference JSON records equivalent HTML input sequences. Windows runtime is unqualified.
- Committing marked text via `unmark_text` emits Changed once when the composition differs
  from its base, applies the UTF-16 limit to only the marked range while preserving the
  untouched suffix, records one bounded undo entry, and ignores repeated unmark calls.
- Caller-owned validation can set/remove an accessible description and Invalid state on
  TextInput or TextArea without changing text, visible styling or Changed events.
- Text runs export global Unicode word starts. Runs split at 255 native characters to fit
  AccessKit's byte-sized word indices; links preserve visual lines across those chunks.
- Current prepaint geometry supplies visual-row text runs, character bounds/widths and offsets
  from the same shaped layout that paints the field. Geometry uses actual scroll and physical
  scale, preserves native caret affinity at soft-wrap boundaries, and uses masking glyphs for
  passwords. Nonmonotonic/uncertain bidi geometry is omitted while text remains readable.
  Geometry is computed only while accessibility is active; validated byte-index glyph lookup
  avoids repeated linear scans on long lines.

The pinned `accesskit_atspi_common 0.19.1` exports Text but never EditableText in
`NodeWrapper::interfaces` (`src/node.rs`, lines 449–480). Consequently Linux AT-SPI clients
cannot call text `SetValue` through an EditableText interface on this stack. The GPUI handler
is implemented for adapters that deliver textual Value actions; it is not claimed as a Linux
EditableText pass. The unmodified bridge also did not forward AccessKit Invalid in
`NodeWrapper::state` or attributes, which the baseline recordings preserve. The owner-approved compatibility
continuation now forwards Invalid; `core-scale1`, `core-scale2` and actual Orca below verify
that correction. The accessible description is a separate exported property. The input
implementation itself adds no dependencies or registry changes. Any owner-approved bridge compatibility changes in the combined build are recorded
in the integrated continuation's source and artifact provenance.

`probe.rs.txt` is the exact temporary native probe source. Copy it to
`examples/input_contracts_probe.rs` to compile it. Run the retained driver inside the prepared
Linux runtime:

```sh
OUT=/workspace/weGPUI/docs/receipts/input-contracts-20261025/scale-1 \
  EXPECT_STATE_EXPORTS=1 EXPECT_SCREEN_ORIGIN=1 docs/receipts/input-contracts-20261025/run-x11.sh
OUT=/workspace/weGPUI/docs/receipts/input-contracts-20261025/scale-2 \
  GPUI_X11_SCALE_FACTOR=2 EXPECT_STATE_EXPORTS=1 EXPECT_SCREEN_ORIGIN=1 docs/receipts/input-contracts-20261025/run-x11.sh
```

The driver starts Xvfb plus openbox, establishes a separate accessibility bus, sends real
XTEST keyboard input and actual AT-SPI Text caret/selection requests, captures each step and
records assertions in `runtime.json`. F8 exercises the public input-handler composition commit
protocol; F6 checks commit-time UTF-16 truncation. These callback checks do not qualify an
actual IME session. Failed attempts are kept in separate directories. Source formatting and Python/shell syntax checks alone do not
establish runtime behavior.

## Runtime observations

- `attempt1-scale1`: preserved harness failure after correct initial text. The deprecated
  `GetTextAtOffset` method is absent in the pinned bridge. The harness now uses its supported
  `GetStringAtOffset` method and granularity enum.
- `attempt2-scale1`: the baseline artifact passed the native keyboard, AT caret/selection,
  read-only/disabled guards, password masking, UTF-16/marked-text limit, undo and geometry
  checks. App logs show one Changed event for the double-unmark composition sequence.
- `attempt3-scale2`: the same baseline checks passed at scale 2. Rectangles scale with native
  font rounding; window-relative point/character queries round-trip. Captures at both scales
  were visually reviewed; family emoji render missing-glyph boxes in this font environment.
- Both baseline runs exposed whole-field WORD granularity, prompting the Unicode word-start
  and long-run chunk fix. `final-scale1` and `final-scale2` passed 30 native/AT-SPI steps on
  artifact `b06263ae…`: WORD returns `alpha `, the 600-character sample retains one visual
  LINE across internal chunks, and a 250..520 AT selection supports native replacement/undo.
- Visual review of that replacement found the newly inserted `x` clipped just left of the
  viewport, because the previous implementation aligned an offscreen caret at the left edge.
  The subsequent fix reveals the preceding native grapheme too. `core-scale1` and `core-scale2`
  pass the stronger glyph-inside-control assertion and all 31 steps, including a native window
  move. The inserted `x` is visibly present in reviewed captures.
- The tested core artifact is `4d089273…`, input source `9ecd888b…`; exact source/dependency
  hashes and the archived tested source/probe are retained beside this README. Later shared
  focus-style work is qualified separately by the integrated continuation.
- Baseline artifacts lacked Invalid state and screen origin. With the approved bridge/core
  corrections, both core runs expose `invalid entry`, and desktop minus window text extents
  match raw X client origins before/after movement. Scale 2 additionally cross-checks direct
  libX11 `XTranslateCoordinates(0,0)` against `xwininfo`. See
  [framework-screen-origin](framework-screen-origin.md). This qualifies these X11 coordinate
  checks, not complete magnifier behavior or other platforms.

`orca-final/session` records an actual Orca session on the word-metadata artifact. Its
transcript includes the value, `gamma selected`, the first visual notes line, read-only state
and masked password. It captured 27.66 seconds of synthesized PCM. Ctrl+Right was classified
as character navigation because `xdotool --clearmodifiers` released Control before Right;
that chord-order artifact is preserved.

`orca-held-core/session` is the actual held-modifier rerun (`run-orca.sh`, `HELD_KEYS=1`). Orca
speaks `alpha`, `é`, `gamma selected`, the first visual notes line, read-only text, password
masking bullets, `invalid entry` and the supplied validation description. It captured 43.57
seconds of synthesized mono PCM. The full debug transcript, input steps, separate AT-SPI
snapshots and screenshots are preserved. Speech Dispatcher reported a segmentation fault
when services were torn down and Orca required forced cleanup; those failures remain in the
logs. The completed speech observations are separate from a clean-shutdown claim.

The native/AT-SPI runs themselves do not establish screen-reader acceptance. Audio has not
been listened to or accepted for intelligibility. The integrated continuation records final
Cargo gates separately.
