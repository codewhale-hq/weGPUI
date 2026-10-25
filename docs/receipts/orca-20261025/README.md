# Actual Orca speech collection — Linux X11, 2026-10-25

These are real Orca 48.1 runs, with Speech Dispatcher 0.12.0, eSpeak NG 1.52.0 and
PulseAudio 17.0 in a Debian 13 container. Xvfb :91/:92 and openbox provide working X11
sessions; `GPUI_X11_SCALE_FACTOR=1`. Audio is recorded from a private PulseAudio null
sink as 16 kHz mono PCM. This establishes generated screen-reader speech reaching
an audio sink, separately from the AT-SPI dumps. It is partial evidence, **not full
screen-reader acceptance for any inventory row**.

The agent reviewed the generated Orca speech transcripts and selected screenshots,
and measured nonzero PCM. It could not listen to the audio: the model's available
audio tool does not accept audio input. Pronunciation/intelligibility and human
screen-reader usability sign-off remain open. The English examples do not qualify
Chinese-label pronunciation or every component/state/theme.

## Provenance

Each run has `provenance.txt` with HEAD, dirty status, a contemporaneous source-diff
hash, executable SHA-256, toolchain and package versions. HEAD was
`52650f3b830ed7e8d1cee7921d495bff06e0628d`; the production binaries were built by
Rust 1.98.1. The checkout also contained this continuation's uncommitted work.
A contemporaneous checkout hash is not a build-source attestation: other agents
were making subsequent accessibility changes while baseline binaries were tested.
Use the executable hash to distinguish before/after artifacts and the integrated
Linux build receipt for source/build provenance. Nothing here uses the Mac's
uncommitted candidate or qualifies macOS/Windows.

## Preserved runs

| Directory | Observation |
| --- | --- |
| `attempt1-harness` | GTK button, not weGPUI: confirms Orca/Speech Dispatcher/audio pipeline. 13.62 s nonzero audio. Initial transcript extraction used unavailable `rg`; extracted afterward from the retained log using Python. |
| `attempt2-component-states-tree-failed` | Failed harness: independent pyatspi capture exited 133 before keyboard driving. Orca announced the actual GPUI window. Two a11y-bus launches caused by setup ordering were corrected; failure logs/audio retained. |
| `component-states` | First GPUI sequence; 54.53 s nonzero audio. Spoken input/button/panel/radio names and countdown-name changes. Save activation removed focus while loading; timing made later action labels drift, so use actual trees/transcript/callback log, not intended labels. |
| `component-linear` | Clean sequence without activating loading Save; 59.14 s nonzero audio. Input, button/link names, radio selection changes and separate inline-link activation. All five intended callbacks fired once. Exposed radio grouping defect described below. |
| `input-summary` | 27.54 s nonzero audio. Actual Orca input readback exposes missing editable text interface, described below. |
| `component-fixed` | 46.85 s nonzero audio. Actual Alex readback, caret/selection speech, undo, and corrected “2 of 2” radio groups. |
| `minimal-fixed` | 33.80 s nonzero audio. Initial dialog body/button/trap/Escape speech works; reopening exposes the stale/defunct AT-SPI cache blocker below. |
| `nested-fixed` | 39.36 s nonzero audio. Actual inner dialog/body/Tab/Shift-Tab speech and inner-only tree; reader focus restoration remains blocked by stale cached objects. |
| `text-final` | 56.04 s nonzero audio on the later grapheme-aware text implementation. Unicode clusters, masked password, multiline content/selection, empty placeholder and ASCII readback. Preserves the Ctrl+Home keyboard gap subsequently fixed. |
| `boundaries-fixed` | 36.71 s nonzero audio. Ctrl+Home/End and both Shift variants now reach/select the whole document; ordinary Home/End retain first-line boundaries. |

`speech-transcript.txt` contains Orca's own `SPEECH OUTPUT` records; `speech.wav` is
the captured synthesis. `orca-debug.log` retains the full reader diagnostic log.
Speech Dispatcher logs are gzip-compressed losslessly. `*-atspi.txt` are independent
tree observations, not speech results. `actions.txt` records actual XTEST actions
and timestamps. `run-status.txt` records harness exit where available; a zero exit
means collection finished, not acceptance.

## Reviewed baseline results and findings

- Input focus says “Display name entry Enter a display name”; typed character echo
  says A/l/e/x. Input name, role and placeholder are delivered to Orca.
- In `input-summary`, the screenshot visibly contains `Alex` but native Orca
  `KP_Enter` (“where am I”) still says the placeholder and omits `Alex`. Home/End
  gives no text/caret readback. Ctrl+Z visibly empties the field. Source exposed
  only `.aria_value` on TextInput, without an AccessKit TextRun/selection subtree;
  character-key echo alone does not establish editable-text accessibility.
- In `component-linear`, ArrowDown selects SMS and Weekly, with actual spoken
  “selected radio button” and the correct visual ticks. `KP_Enter` says “2 of 4”
  for each, although the two groups each have two options. The example put both
  groups under one `Role::Group`, contrary to RadioGroup's documented requirement
  for caller-provided separate `Role::RadioGroup` containers.
- Orca identifies “Profile details link”, “the profile guide link” and the long
  wrapped preferences link. Return triggers each callback exactly once; panel,
  guide and preferences counts are each 1 in `application.log`. Screenshot review
  confirms the selected radio ticks and wrapped-link focus fill.
- Save's loading transition moves focus to the frame. The description-change
  event contains `正在加载`, but the recorded spoken output announces the frame;
  no loading-description speech acceptance is claimed. Later Tab can revisit Save
  when its three-second loading timer expires.
- Disabled Unavailable is skipped in ordinary Tab navigation. The attempted Orca
  flat-review sequence spoke the frame and did not reach Unavailable, so its
  “不可用” description was **not** verified as spoken. AccessKit's known disabled
  button state limitation remains unresolved. The older receipt's statement
  “which Orca reads” was unsupported by this baseline.
- AccessKit cache `AddAccessible` signature warnings are retained in `orca.log`.
  Subsequent focus/name/selection speech and tree reads succeeded despite them.

## Follow-up results after source fixes

`component-fixed` used executable SHA-256
`1241c4455b1ff3aaac2ea69f1a08823dee6d0c708e56edbbf8198afd86186e50`:

- Orca `KP_Enter` now says “Display name entry Alex.” after typing Alex. Left
  says “x”; Shift+Left says “e”, “selected”. The Text probe independently reads
  `Alex`, count 4, caret 4 → 3 → 2. Ctrl+Z says “Text unselected”, clears the visible
  field, and summary returns to the placeholder.
- Both SMS and Weekly now announce “2 of 2”. Separate Delivery/Frequency group
  names are spoken. The example's grouping fix addresses the previous “2 of 4”.
- The supplementary probe initially used nonexistent pyatspi `Text.nSelections`.
  Those retained files correctly report text/count/caret before that diagnostic
  error; their “Text interface unavailable: AttributeError” suffix describes the
  helper bug, not absence of the application's Text interface. The helper now
  uses verified `getNSelections()`. A final probe rerun is recorded separately.

`minimal-fixed` used executable SHA-256
`37560b6ab675a47bbdab7dcbffb0b4fc7c39c7b593e8bf4e6e08fe91b7d95b53`:

- Initial open actually speaks “weGPUI dialog A WeUI dialog rendered by native
  GPUI components.” Tab speaks Cancel → OK → Cancel; Escape speaks “Open dialog
  button.” The fixed static body is no longer silent.
- Reopen renders the correct visual dialog and the fresh tree reports its focused
  container and subsequently focused OK. Confirmation closes it and the tree
  reports `Confirmed` with description `1`. **Orca does not announce the reopened
  controls**: it logs “Ignoring defunct object” for the readded dialog/buttons.
  Only raw Tab/Return key echo remains. A fresh tree is not evidence that the
  already-running reader recovered.

`nested-fixed` used executable SHA-256
`6349e3fd4ac0efb89c1dc09ac95ee51f1a16d9c191b7189ee34f877b786360f6`:

- Orca speaks Outer sheet, Open inner dialog, then “Inner dialog dialog Opened
  from inside the half-screen dialog.” Cancel/OK forward and backward cycling
  speaks the expected names. Fresh trees while inner is open expose only inner
  dialog/body/buttons; outer/background controls are absent.
- Inner Escape restores native/AT-SPI focus to Open inner dialog, but Orca's
  subsequent `KP_Enter` incorrectly reads its old OK focus. Outer Escape does
  speak Open sheet. Screen-reader focus restoration is not accepted.

The pinned dependency stack is AccessKit Unix 0.22.1 / AT-SPI common 0.13.0 /
zbus 5.19.0 / zvariant 5.15.0. Its cache XML expects one structure argument,
`((so)(so)(so)iiassusau)`, while actual Orca warnings report
`(so)(so)(so)iiassusau`. `accesskit_unix/src/atspi/bus.rs::emit_cache_add` passes
the CacheItem directly to `emit_signal`. This static evidence is consistent with
the reader retaining defunct cached objects after nodes are removed and readded;
it does not establish every cause of the reader failure. No dependency patches,
registry edits or upstream posts were made. This blocker remains separate from
the known disabled-button state limitation.

## Final grapheme-aware text probe

`text-final` used executable SHA-256
`02ea9451778225ef9adeea9b6bd7c1e50d6aff3429f33059631aae165f56ec0d`.
Its deterministic fixture source is retained in
`../a11y-text-20261025/probe.rs.txt`; the temporary example is removed from the
final source after runtime collection.

- Unicode text is `é 👩‍👩‍👧‍👦`, 10 Unicode scalar offsets. Native Left crosses
  the whole family cluster (caret 10 → 3); backward selection reports `(3, 10)`.
  Home/Shift+Right selects the complete combining sequence `(0, 2)`. Orca's
  generated output contains the family/combining strings and “selected”. The
  screenshots show fallback boxes for emoji because this container lacks the
  corresponding font; no emoji appearance or spoken pronunciation acceptance is
  claimed.
- Password text reports three mask characters, caret 3 → 2 → 1 and selection
  `(1, 2)`. Orca speaks `Password sample password text •••` and masked character
  selections. The unmasked dummy value is not exposed in these speech/Text probes.
- Multiline Text returns both hard lines **and** the trailing newline (24 scalar
  offsets); Ctrl+A reports selection `(0, 24)`. Orca generates both lines followed
  by “selected”. Initial focus/readback at the final empty line says only the
  field name/role, so this does not establish complete line-navigation acceptance.
- Empty input reports text `''`, count/caret 0 and no selection; its initial
  speech includes “Type here”. After typing Ready, native reader summary says
  “Empty sample entry Ready.”, with count/caret 5 and no selection.
- **Failed keyboard sample preserved:** `multiline-start-text-interface.txt`
  shows Ctrl+Home left the caret at 24. Source had no Linux Ctrl+Home/End aliases.
  The next follow-up verifies the new bindings; this failed run is not rewritten.

The initial derived transcript clipped newline continuations from Orca's multiline
utterance. The original full log/audio were intact. The extractor now retains
continuations; `speech-transcript.txt` was regenerated from that log and the old
derived file remains as `speech-transcript-initial-extraction.txt`, with a note.
All final Text probes use the corrected `getNSelections()` helper and report no
capture errors. Example artifacts:
[generated transcript](text-final/speech-transcript.txt),
[captured speech](text-final/speech.wav),
[Unicode selection](text-final/unicode-family-selection-text-interface.txt),
[masked password selection](text-final/password-selection-text-interface.txt),
[multiline selection](text-final/multiline-selection-text-interface.txt).

## Linux document-boundary shortcut follow-up

`boundaries-fixed` used executable SHA-256
`9626474fe80783ee8bddd109ad322cd520c9a8803ba6038a945b5126fdae5ff4`
after adding the four Ctrl+Home/End/Shift aliases to the existing document actions.
Actual retained Text probes show:

| Action | Multiline caret | Selection |
| --- | --- | --- |
| Ctrl+Home | 0 | none |
| Ctrl+End | 24 | none |
| Ctrl+Shift+Home from end | 0 | `(0, 24)` |
| Ctrl+Home, Ctrl+Shift+End | 24 | `(0, 24)` |
| Plain End from first-line start | 8 | none |
| Plain Home | 0 | none |
| Shift+End from first-line start | 8 | `(0, 8)` |

The harness finished with exit 0 and no capture errors; the 36.71 s recording is
nonzero. This verifies those native shortcuts and corresponding Text state, not
every editor action or platform. The failed pre-fix Ctrl+Home sample remains in
`text-final`. The deeper dependency-cache source trace is in
[cache-investigation.md](cache-investigation.md); modal reopening/restoration
remains a real Orca blocker.

## Reproduce

Install the tools listed in `scripts/linux/orca-session.sh`. With no `DISPLAY`
supplied, it starts Xvfb :91 and openbox; with `DISPLAY` supplied, provide an active
X server and window manager yourself. The script creates a private D-Bus session,
a11y bus, audio socket, Speech Dispatcher and Orca instance. OUT must be new: the
script refuses to overwrite prior evidence.

```sh
STEPS='field:@220,64|typed:text=Alex|readback:KP_Enter|undo:ctrl+z|readback-after-undo:KP_Enter' \
  scripts/linux/orca-session.sh NEW-OUT /absolute/path/to/component_states
```

Coordinates are relative to the client window. `Tab`, `Down`, `KP_Enter` and other
key names are sent by xdotool. Every new capture also runs `orca-text.py` to record
input Text-interface contents/caret/selection separately from speech; password
contents are never printed. Readable transcripts, WAVs and failure logs must be
reviewed before making any claim beyond collection.
