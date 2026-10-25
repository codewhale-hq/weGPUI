# Actual Linux XIM/Pinyin — 2026-10-25

This receipt uses real fcitx5 Pinyin, its XIM frontend, and XTEST keyboard input against the
gallery built from GitHub main `49268de7bb78a69cb56b96f6cff02a2d9284e71d`.
The immutable gallery SHA-256 is
`0966e1f0bc0a12d4e1ab6b028ce8bd4559e90a9baf6aa050d865182fba19c83f`, matching
the preceding integrated batch's manifest. Exact source/adapter hashes are in
`source-and-artifact.json`. No Rust edits or Cargo commands were used for these baseline runs.

This is actual input-method evidence, unlike the separate input-contract probe's direct
EntityInputHandler calls. AT-SPI Text getters observe the resulting field values/selections;
no screen reader was run for this receipt. Gallery observation does not establish callback
counts or cross-platform IME acceptance.

## Environment and method

The existing Debian 13 container had Xvfb, openbox, xdotool, D-Bus and AT-SPI but no input
method. With root coordination, container-only apt setup added fcitx5 `5.1.12-2`,
fcitx5-pinyin `5.1.8-1`, libimepinyin0 `1.1.10+dict20250327-1` and their minimal required
packages using `--no-install-recommends`. Update/install logs and package versions are kept;
the downloaded package cache was cleaned. No Cargo dependencies changed.

`run-session.sh` creates its own D-Bus session, Xvfb `:102` (1600×2100), openbox, accessibility
bus, fcitx profile/config/data/runtime directories and tracked processes. It uses
`GPUI_X11_SCALE_FACTOR=1`, `LC_ALL=C.UTF-8` and `XMODIFIERS=@im=fcitx`; it cleans only its own
processes/directories. Each run retains the exact scripts, fcitx configuration, XIM server
property, queried configuration, environment, application log, key traces, AT-SPI JSON and
app/root-window captures.

The desktop `XIM_SERVERS` property names `@server=fcitx`. The fcitx debug trace identifies
the gallery process and the real key events, and its remote API reports `pinyin` active
during composition. The startup warning about absent initial Pinyin history is preserved.

## What was observed

| Run | Result and scope |
| --- | --- |
| `attempt1` | Card commit/cancel, Undo/Redo and maxlength/suffix checks succeeded. The driver then stopped on its incorrect readonly label (`邮箱`; the actual accessible label is `EMail`). Preserved as a harness failure. It also queried the wrong `_XIM_SERVERS` atom; later runs query `XIM_SERVERS`. |
| `attempt2` | With fcitx's default `UseOnTheSpot=False`, card commit/cancel, cancellation over a selection, Undo/Redo, maxlength/suffix and readonly/disabled guards succeeded. The textarea check then failed because `xdotool type` omitted a literal newline in the driver. The corrected driver sends Return explicitly. |
| `textarea-default` | Corrected real multiline sequence passed: `before\nafter` → `before\n你好after`; Escape canceled later `shijie`; one Undo restored the original and Redo reapplied the composition. |
| `on-the-spot` | D-Bus confirms `UseOnTheSpot=True`. Inline preedit and caret-adjacent candidates were visually reviewed; basic Space commit, Escape cancellation at a caret and Undo/Redo passed. Cancellation after replacing selected committed text **failed**, as described below. |
| `textarea-on-the-spot` | Real inline multiline preedit/commit, caret cancellation, suffix preservation and Undo/Redo passed. |

The single-line card sequence types `ABCD`, moves after A, composes `nihao`, and Space
commits `A你好BCD`. A subsequent `shijie` plus Escape leaves that value unchanged. Ctrl+Z
returns `ABCD`, and Ctrl+Shift+Z returns `A你好BCD`. With 15 ASCII characters in a field
whose maxlength is 16, the same insertion admits only `你` and preserves every untouched
suffix character: `A你BCDEFGHIJKLMNO`. Those assertions are real native IME interactions.

The readonly `EMail` remains `1234567`; a subsequent click and attempted composition into
the disabled WeChat field leaves `WeUI` and the readonly value unchanged. These baseline
state checks use the default external-preedit configuration, not the later inline failure run.

## Proven cancellation defect on the baseline

With `UseOnTheSpot=True`, select `你好` in `A你好BCD`, type real Pinyin `shijie`, then Escape:

1. `07b-selected-preedit` shows underlined `Ashi jieBCD` inside the field.
2. `07c-selected-canceled` shows `ABCD`; the originally selected committed `你好` is lost.
3. `runtime.json` records the failing equality assertion and exact AT-SPI values/selections.

The baseline native input handler clears its marked range and composition base after replacing
the marked span with an empty string. It therefore cannot restore the original selected text
on cancellation. The prior default-server run does not expose this defect because preedit
remains outside the application. The failure is retained and is not counted as acceptance.

The source-backed candidate fix is to restore the active composition's saved content and
selection/range direction silently when an empty preedit cancels it, without using the ordinary
restore method (which emits Changed). Empty-preedit-before-commit must then allow the following
native commit to replace the restored original selection exactly once; empty-preedit-after-
commit and repeated empty callbacks must be no-ops. This needs its own source change and fresh
runtime/undo/callback evidence; no fix is claimed by the baseline receipt.

## Fixed-source continuation

The continuation adds silent cancellation before an empty preedit request resolves any range.
Only an active marked range together with its saved composition base can restore text and
selection direction. It clears the composition metadata without adding an undo entry, clearing
redo, or emitting Changed. A repeated empty preedit is a no-op; an explicit empty **commit**
still follows the ordinary deletion path.

Actual changes to disabled/readonly cancel pending preedit before applying the new state.
Changing maxlength cancels it first, then applies the new limit to the restored committed text;
the same limit leaves composition intact. This prevents an old saved snapshot from undoing a
caller's newer limit/state change. set_text still owns its documented programmatic replacement
policy. Undo/redo behavior for the cancellation itself is unchanged.

The fixed input source SHA-256 is
`2e54e3517015d2f90a78bf9fec91bfdafd7ac79f5a0c70870931f00bddeb1cf1`, archived in
`fixed-input.rs.txt` and `fixed-input-source.patch`. `observer-probe.rs.txt` records the temporary
native observer, which was built in the coordinated build, archived, compared byte-for-byte and
removed from examples before final gates. Its immutable binary SHA-256 is
`0e706422ad7e7b219e9083e98aa1056aafa3c06471e721acf0b0647caef08cfe`.
The coordinated observer build and 47 unit tests passed; the final full gates are recorded by
the integrated continuation.

`fixed-observer` verifies real forward/reversed selected-text cancellation with zero Changed,
real commit/Undo/Redo with exactly one Changed per operation, multiline notes, UTF-16/suffix
preservation, same-limit preedit and readonly/disabled changes. It then stops on a harness focus
error: after disabling Primary, a key addressed through the disabled field cannot re-enable it.
No value or source failure is inferred from the following unexecuted maxlength action. The
rerun driver first focuses another enabled input, then toggles Primary and asserts it is enabled.
This attempt remains preserved separately from the complete rerun.

`fixed-observer-rerun` completed with **PASS: 35 recorded snapshots, no failed assertions** on
the same fixed binary. The report and native callback log establish:

- Real Pinyin cancellation over forward and reversed selections restores text, selected range
  and direction with zero Changed events. Committing over the reversed selection emits exactly
  one; Undo and Redo each emit one and restore the expected text/selection.
- Real multiline selected-text cancellation emits zero changes. Commit preserves the combining
  character, newline and suffix; commit/Undo/Redo each emit exactly one change.
- Real composition in `A😀BC` with a six-unit maximum admits only `你`, preserving the original
  emoji and BC suffix. Applying the same maximum during preedit preserves the active mark and
  emits no change; committing and Undo each emit one.
- Real preedit is canceled silently by live readonly/disabled changes. Re-enabling from another
  enabled field succeeds. A new limit of seven units cancels preedit before truncating the
  restored committed value to `A你好B é`, emits one change, and is not undone by the later empty
  preedit callback.
- Clearly separate direct EntityInputHandler protocol cases pass empty-preedit-before-commit
  and commit-before-empty orderings, repeated empty cancellation, reversed-selection Undo,
  explicit empty-commit deletion and its Undo, and a changed limit with no stale restoration.
  These cases are labeled as protocol checks in `runtime.json`; they are not actual-IME claims.

The callback counts come from the temporary probe's subscriptions to native InputEvent, not
from accessibility tree events. The exact before/after counts and UTF-16 native selection/mark
snapshots are retained. Captures of restored forward/reversed selections, limited insertion,
multiline commit and changed limits were visually reviewed. The existing font environment
still shows missing-glyph boxes for the family/ZWJ sample; this does not qualify emoji typography.
Passwords remain three masking bullets in the captured trees; password mutation is outside
this particular IME matrix. No screen-reader/audio acceptance is claimed here.

## Preedit positioning and remaining limits

The historical takeover capture showed preedit in the input method's own window and did not
isolate the cause. The queried fcitx default is `UseOnTheSpot=False`: in this default run the
external preedit/candidate window appears below the gallery, far from the field. Changing only
the isolated fcitx XIM configuration to `UseOnTheSpot=True` gives inline underlined preedit and
candidates immediately below the caret on the same unmodified binary. Basic inline support is
therefore working; external preedit here cannot be attributed solely to GPUI.

`framework-xim-source.txt` retains the pinned backend path: it forces PREEDIT_CALLBACKS,
forwards preedit strings into native marked-text replacement, and ignores the XIM caret and
incremental-change offsets. Its composing branch also skips some spot updates. Those are
source observations, not independently reproduced failures for every IME. The tested matrix
does not qualify actual platform IMEs on Wayland/macOS/Windows, arbitrary preedit cursor
movement, candidate selection modes or screen-reader speech. Callback counts are qualified by
only the fixed observer sessions above; the baseline gallery runs did not observe those counts.

Reproduce the default session with `OUT=... run-session.sh`; add `XIM_ON_THE_SPOT=1` for inline
preedit, and `ONLY_TEXTAREA=1` for the isolated textarea sequence. The retained failing runs
must remain separate from any future fixed-source reruns.
