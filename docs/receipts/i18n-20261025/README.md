# Localisation and right-to-left — 2026-10-25

This receipt covers the localisation layer ([I18N](../../I18N.md)): message catalogs, mirrored
layouts, bidirectional text input and an English-first gallery. GitHub `main` is canonical; no CNB
writes, issues, comments, tags, releases or deployments were made.

## Source and host

- Base: `main` at `9266bb9` (fast-forwarded from `a47eabc` while working). The changes landed as
  ordinary commits on top; each source claim below was run against the working tree that became
  those commits, identified by the tree hash in [gates/tree-hash.txt](gates/tree-hash.txt):

  ```sh
  find src examples scripts Cargo.toml Cargo.lock -type f -not -path '*/__pycache__/*' -print0 \
    | sort -z | xargs -0 shasum -a 256 | shasum -a 256 | cut -c1-16      # d8ffe13e04669848
  ```

- Host: macOS 26.1 arm64, Rust and Cargo 1.99.0, `--locked --offline`. Linux and Windows were
  **not** run for this slice; their shapers, input methods and screen readers are untested here.
- The work started from twelve uncommitted paths found in the checkout, backed up byte-for-byte
  (SHA-256 manifest) before anything was touched.

## Gates (frozen source)

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo build --all-targets --locked --offline` | pass |
| `cargo test --locked --offline` | 59 unit tests pass (47 before, 7 for the catalogs, 5 for the bidirectional caret maths); 3 documentation examples ignored |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps` | pass |

Logs: [gates/](gates/). These are local macOS results; hosted CI was not run and nothing here is a
deploy, a published package or a customer outcome.

## What was observed

| Claim | Evidence | Does not show |
| --- | --- | --- |
| Every gallery route renders in English with no leftover Chinese and no clipped or glued text | 61 routes launched and captured at 375×812 ([summary](en-sweep-summary.json), sample [forms sheet](images/en-sweep-forms.png)); each sheet was read. Fixes found this way: glued link text, 85 px labels that wrapped inside words, a clipped agreement line, an "Amount / paid" wrap, a button that read "I have read and agree to" | The accessibility tree was not scanned: macOS Automation permission was not available to this session, so `scripts/i18n_sweep.py`'s leak scan did not run. English completeness rests on the source audit (every Chinese literal in the gallery goes through `tr`) and on reading the captures |
| Simplified-Chinese gallery is unchanged | 60 routes captured with `--lang zh-Hans` and compared with the stored `docs/parity/*-gpui.png` ([table](pixdiff-zh-hans.tsv)): 8 routes have no differing pixel (list, form_primary, form_access, form_select_primary, navbar, tabbar, footer, gallery), 34 routes in all (those 8 included) differ by at most 0.1 %, and the largest difference is 1.08 % (msg) | The stored images are older than HEAD: buttons were `SEMIBOLD` when they were taken and `MEDIUM` since `49268de`, which accounts for the glyph-weight differences on button-heavy pages (form index, msg, toast). No A/B build of `9266bb9` was made, because a second checkout is not allowed. Differences were not attributed route by route |
| Every script shapes | [Languages page](images/languages-page-en-scrolled.png): Arabic, Hebrew, Devanagari, Thai, Japanese, Korean, Cyrillic, Han, emoji and combining marks; [Han forms per locale](images/han-per-locale.png) | Wrapped lines of Arabic, Hindi and Thai break wrongly; see the [proposal](../../proposals/gpui-text-i18n/README.md) |
| Core components mirror in Arabic | [first](images/rtl-core-1.png) and [second](images/rtl-core-2.png) sheets: button, form, list, navbar, progress, search bar, slider, tab bar | |
| The remaining components mirror in Arabic | [steps and information bar](images/rtl-steps-infobar.png), [preview, panel, grid, uploader](images/rtl-preview-panel-grid-uploader.png), [list, article, load more, footer](images/rtl-list-article-loadmore-footer.png), [verification code, msg, checkbox](images/rtl-vcode-msg-checkbox.png), [half-screen dialog, list, verification code](images/rtl-halfscreen-list-vcode.png), [panel, icons, badge, dialog](images/rtl-panel-icons-badge-dialog.png) | `cell_control`, `action_group`, button margins and the date picker's column order are not mirrored (see I18N) |
| Bidirectional selection is correct | [scripted states](images/bidi-selection-states.png): select-all in Arabic and Hebrew paints one span; selecting `2026` in `Hello 2026 שלום` highlights only those digits. The five synthetic-layout unit tests cover the caret/hit-test maths | **No caret was captured**: the gallery window is not active when this harness launches it, and GPUI paints the caret only for a focused, active window. Real keystrokes were not driven: this session has no Post Event permission, so injected events were dropped by the system |
| Swipe cells, steps and half-screen headers behave in a mirrored row | Rendering only | Drag direction and arrow-key semantics of `SwipedCell` in right-to-left are implemented but were not driven |

### A failure that was kept

[rtl-halfscreen-picker-before-fix.png](images/rtl-halfscreen-picker-before-fix.png) shows the
half-screen dialog header before the fix: the end slot (search, more, Done) sits on the right,
on top of the close button. GPUI's `justify_end` is the physical right edge even in a reversed
row. The after-fix capture is in [rtl-halfscreen-list-vcode.png](images/rtl-halfscreen-list-vcode.png).
The same cause affected the Android dialog footer and was fixed in the same change.

## What this does not establish

- Screen-reader output in any language other than English and Simplified Chinese, and VoiceOver
  in any language (it stays off on this host by instruction).
- Native-speaker review of the twelve starter catalogs.
- Real keyboard and IME input in a right-to-left field; caret painting.
- Linux, Windows, iOS or Android behaviour.
- Pixel parity of the mirrored layouts with any reference: there is none upstream, because WeUI
  has no right-to-left skin.

## Reproducing

```sh
cargo build --example gallery
python3 -I scripts/capture.py form --only gpui --lang ar --out /tmp/rtl   # mirrored Arabic
python3 -I scripts/capture.py languages --only gpui --lang en --scroll 1050 --out /tmp/langs
python3 -I scripts/i18n_sweep.py --langs en --routes all --no-scan --out /tmp/sweep
```

The scripted typing states are `--page languages --state type:<field index>:<ops>`, where ops is a
comma list of `all`, `left`, `right`, `selleft` and `selright` (see
[PARITY-TOOLING](../../PARITY-TOOLING.md)).
