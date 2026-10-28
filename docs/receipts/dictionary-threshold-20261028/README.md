# Guarded dictionary threshold editing samples — 2026-10-28

Two local Linux/X11 probes exercise an unshipped guarded dictionary candidate in the real `TextArea`/`TextInput` widgets. They use the canonical source snapshot at `384ded7721f75ed8222c9590ad364d3b5f39fbbf`, with a bounded GPUI overlay. Both build manifests hash all selected source, overlay and fixture files, and each source snapshot and binary hash matched immediately before and after runtime. The `src/input.rs` SHA-256 is `b54072bdabf718cd9bb727ac4332c95e5c6b4d8a618eebea4dcb000cd1e325d5`; the overlay matcher is `345ffba52745fa1867dcadae96edaae861c8f72d0b390a79091371278077659e`. The first binary SHA-256 is `3a9e17eda7e35a15e4a5814439d31a73c0b9063f795a0d55833c604737a7d0c9`; the second is `67f855ff8cdd3c7f00511132eb18cdb775e0469b8d9b9a46efb0f9d4c6b27dee`.

## 65,536-byte threshold

A multiline input was seeded with a Khmer combining-mark/emoji prefix followed by a long ASCII tail, exactly 65,536 UTF-8 bytes. Actual X11 key events inserted and deleted one byte, crossing 65,536 ↔ 65,537. Selection and caret readback tracked the edits; an attempted UTF-16 caret position inside the emoji surrogate pair was rejected. A selected emoji-and-mark grapheme was then replaced using the public input protocol's synthetic preedit method, taking the text above the limit. Cancel restored the original text and selection; a second preedit/commit and a real Ctrl+Z restored it again. These are direct protocol calls, **not real IME events**.

A second run used real keyboard navigation and an end-of-document insertion/deletion so the original prefix stayed unchanged. The sampled geometry for the first prefix ranges did not change at 65,537 bytes and was identical again after deletion. This is a bounded geometry observation, not proof that every row, selection or caret remains unchanged through a threshold transition. The `attempt1/` folder preserves the first runtime's result, application log and screenshots; the tail run is in `result-tail.json` and `tail-run.log`.

## 8,192-script-span threshold

A separate real `TextArea` began with 8,192 alternating Thai/Lao characters (24,576 UTF-8 bytes). A local X11 clipboard was populated by the fixture, then actual Ctrl+V pasted a Thai scalar at the end, creating 8,193 spans. Selection and UTF-16 readback remained valid; the first four characters were selectable. Actual Ctrl+Z and Ctrl+Shift+Z reversed and restored the paste; Backspace removed the final scalar and Ctrl+Z restored it. The sampled first-prefix geometry did not change between these states. Results, X11 log, screenshots and source manifest are in sibling `dictionary-span-guard-probe/`.

Prior provider-only controls found changed opportunities earlier in the paragraph when the guard switches: three offsets for its byte-threshold sample and 8,191 offsets for its alternating-script span sample. The native fixtures above only measured selected prefix ranges, and the unchanged sampled geometry does not exclude reflow elsewhere. No browser-parity, whole-document geometry, real XIM/fcitx, screen-reader, macOS/iOS, or Windows UI result is implied.

## Setup and identity

The initial byte fixture build stopped before compilation because its manifest was outside the Linux container mount (`build.log`). After the manifest was copied and checked, the next attempt failed on a delimiter in the fixture (`build-retry.log`); a host rustfmt attempt also could not resolve the container-only `/src` path. The corrected fixture was formatted and built under the shared lock (`build-retry3.log`). The span fixture then built under the same lock. All failed attempts are fixture/setup evidence, not product failures.

| Evidence | SHA-256 |
| --- | --- |
| Byte fixture binary | `3a9e17eda7e35a15e4a5814439d31a73c0b9063f795a0d55833c604737a7d0c9` |
| Byte build manifest | `533627a37abfbffabfcefd4db201faac5c9db74465f32acc98b71aa5bc2dfcb4` |
| Span fixture binary | `67f855ff8cdd3c7f00511132eb18cdb775e0469b8d9b9a46efb0f9d4c6b27dee` |
| Span build manifest | `c064d9bd1d3c7aab791bba282a9d6d5281c2fe0522278f30cd62205a8213a613` |

Do not publish or ship the candidate based on these samples alone. The long/mixed-paragraph fallback can alter earlier wrap opportunities when thresholds are crossed; complex-cluster editing, actual platform IME, full geometry and target performance remain open.
