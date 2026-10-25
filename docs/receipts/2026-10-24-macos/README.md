# macOS audit and verification — October 24, 2026

Initial source: `0c6555d82ee0e13211591b16eb5ab2dcbfc8015d`, clean `main`.
Mac arm64, Rust 1.99.0, Xcode 26.2 (17C52). Dates in this folder use America/Los_Angeles;
the earlier `takeover-20261025` folder used the Linux session's date.
Paths in logs are replaced by `<repo>`, `<scratch>`, `<cache>` or `<home>`.

## Read-only audit

The review covered README, CHANGELOG, CONTRIBUTING, SECURITY, STATUS, PLATFORMS, PARITY,
PARITY-TOOLING, SOURCES, UPSTREAM-COVERAGE, CI-PROPOSAL and the takeover receipt index, against
the examples, component sources, scripts, history and pinned Cargo registry sources.

Confirmed and corrected:

- The library has two direct dependencies, GPUI and unicode-segmentation, and no platform crate.
  Linux's missing windowing features affect both examples and the README consumer app.
- 61 upstream HTML files comprise 60 routes plus the shell. Coverage language now distinguishes
  home/layers/shell implementations from public components. Counts are four 180-field palettes,
  32 extracted icons, 72 registered states and 534 historical parity PNGs.
- Mobile type-checks belong to the `0637c25` change, and MSRV measurements to `1ea95a5`.
  The strict rustdoc run and disabled-edge picker sequence are commit-reported; their respective
  committed artifacts are the non-strict doc log and probe source, not fresh interaction logs.
- The committed Linux viewer state does not prove a thumbnail click. The zero-step probe source
  does not prove that its change handler was exercised. No Wine run log is committed.
- GPUI 0.3.7's actual gesture dispatch synthesizes mouse down/up for taps. Its touch arena has
  no pinch recognizer, despite introductory upstream rustdoc. Picker columns already accept
  scroll-wheel events. Mobile type-check failure language described the pre-0637c25 manifest.
- The small-appmsg focus-ring claim contradicts its zero-width border. This remains an open
  component defect, not a verified fix. Other audit defects remain in STATUS.
- Contributor regeneration instructions now respect CARGO_TARGET_DIR and restore only verified
  unchanged dark PNGs, preserving SVG changes. Parity report fields are aligned.
- Tooling additions from `0c6555d` are included in CHANGELOG. The CI draft is configuration,
  not a hosted run. Windows support still requires every acceptance column.

Rejected or qualified leads:

- GitHub advertised only `main` during `git ls-remote --heads origin`. The handover's
  `claude/youthful-fermi-y23wn0` branch was not advertised. Its `e23436c` tip is merged;
  no branch was deleted during this audit.
- The Linux patch passes `git apply --check` and touches only Cargo.toml/Cargo.lock.
- A tracked-text scan (853 files) found no machine paths, obvious private-key/token patterns,
  or agent/operator instructions. Pattern scanning is not an exhaustive secret-history audit.
- Fresh generation into an empty output directory produces 64 files: it cannot reproduce the
  separately authored loading.svg. With that retained asset, generation is reproducible; the
  extractor's rustdoc emission preserves the documented palette values. See extraction.txt.
- The upstream fetch initially failed because the default Python lacked a trusted TLS issuer;
  the installed Homebrew Python fetched and hash-verified all three resources successfully.
  TLS verification was retained. Both outcomes are preserved.

Tooling defects confirmed before changes:

- `sweep.sh` reports a failed capture but exits 0 (script-audit-before.txt).
- X11 capture combines PID/name with xdotool's default ANY matcher; source inspection of
  [xdotool cmd_search.c](https://github.com/jordansissel/xdotool/blob/master/cmd_search.c)
  confirms that ALL is required to restrict the capture to the launched app.
- `contact_sheets.py --per 0` crashes with ValueError instead of rejecting the argument.
- The Linux session helper discards application errors and never checks whether its process
  survived launch. AT-SPI dump exceptions are printed but do not fail the command.

These audit artifacts do not establish current macOS build, input or screen-reader success.
Runtime and build receipts below identify their own source revision and commands.

## Script repairs

`script-audit-after.txt` retains syntax checks and command-level reproductions. A failed sweep
now exits 1; invalid contact-sheet arguments exit 2; a real existing pair produces a sheet.
The X11 PID matcher, process-liveness and AT-SPI error handling are source fixes; no Linux host
was available for a new runtime check. Capture dimensions will also be exercised by the Mac run.

## macOS build and launch checkpoint

The five checks in `checks.json` all exited 0: locked fetch, formatting, locked/offline
all-target build, Clippy with `-D warnings`, and rustdoc with `-D warnings -W missing-docs`.
The recorded source is `e4a2c877fd99c8a52e1de8a145479ab613262b66`. `source-hashes.json` records
Cargo files and tracked Rust inputs; those inputs remained unchanged through `2f18b7a` and the
following runtime work. No GPUI pins, dependency features or Rust components changed here.
Cargo reports an existing future-incompatibility warning for dependency `block 0.1.6`; there
were no rustdoc diagnostics. Logs retain the wait for the shared build slot, with unrelated
checkout details removed.

`cargo run --example minimal --locked --offline` and `cargo run --example gallery --locked
--offline -- --page form_select --theme light --size 375x812 --position 60,60` launched windows.
Their compiler/launch output is in `minimal-launch.txt` and `gallery-launch.txt`. Both apps
were subsequently terminated after observation; termination is not a clean user-exit test.
The archived `probe.rs.txt` was copied locally and compiled (`probe-build.txt`) but was **not
run**. Its temporary example source was removed from the checkout and retained outside Git.

## Runtime observations and limits

Files under `runtime/` are window screenshots and AX dumps. Full-window images retain the
native titlebar and are not parity pairs.

| Check | Observed result | Receipt |
| --- | --- | --- |
| Minimal dialog | Real mouse opens it; only dialog subtree is exposed. Tab/Enter closes it and restores the Open dialog focus. A second open, two Tabs and Enter selects OK; visible Confirmed becomes 1. | `minimal-open-retargeted.*`, `minimal-confirmed-retargeted.*`, `minimal-ok.*` |
| Picker (`--state single`) | Only the picker subtree remains. Five option titles exist in raw AXTitle; the older dump omitted them. | `picker-single.txt`, `picker-single-raw.txt` |
| Select (`--state select1`) | Only the menu subtree remains; 微信号, QQ号, Email are present in raw AXTitle. | `form_select-select1*.txt` |
| Half-screen dialog (`--state 1`) | The page is absent; close/search/more/done controls remain in the dialog subtree. | `half-screen-dialog-1*.txt` |
| Toast (`--state success`) | Page controls remain and one named 已完成 node exists. The dump did not record AXSubrole, so this is not a runtime verification of alert mapping or announcements. | `toast-success*.txt` |
| Closed swipe cell | Delete action absent from the AX tree. Corrected dump exposes static-text titles. | `list-closed*.txt` |
| Article | Headings and nested lists/items are exposed. No heading-level value was returned in the raw attributes sampled. | `article-None*.txt` |

The option-label discrepancy is a **dump limitation**, not missing labels in the macOS tree:
`AXTitle` contains them while System Events' `name` property does not. `scripts/ax_dump.js` now
includes distinct nonempty raw titles. The corrected script was exercised on list/article;
`ax-raw.js.txt` preserves the separate raw-attribute diagnostic. This is tree evidence only.

Failed or incomplete attempts are retained:

- The first minimal helper selected an auxiliary 500×500 window, producing black screenshots
  and no interaction. The `*-retargeted` files use the actual 375/376-point app window.
  Despite its filename, `minimal-confirmed-retargeted` shows closing the dialog, not OK;
  only `minimal-ok.png` establishes the increment to 1.
- `dialog --state 1` is invalid: valid states include `ios1`, `ios2`, `ios3`, `android1`,
  `android2`. `dialog-1.*` shows the page, not modal isolation. The minimal app supplies the
  successful dialog-isolation evidence.
- `select-click-open.*` and subsequent `select-*.txt` did not open the menu. A separate
  `drive.py` Tab/Enter/End/Down/Home/Up/Escape attempt also failed to show the open menu in its
  images (`drive-select/`). It overlapped the final AX capture and cannot qualify input.
  A local activation/bounds-order change was tried, but did not establish success; it was
  reverted and is retained only as `drive-activation-attempt.patch.txt`. No select navigation
  or focus-restoration pass is claimed on this Mac.
- An early raw AX traversal returned `Can't get object`; that output is retained in
  `form-select-start-raw.txt` and `select-click-open-raw.txt`. The diagnostic then used indexed
  UI-element references, as the normal dump does.
- VoiceOver was enabled, but querying its last phrase timed out. No announcement transcript
  was captured. A later app inspection started it again; it was stopped and
  `NSWorkspace.isVoiceOverEnabled()` returned false (`voiceover-stopped.json`). The earlier
  process-only observation in `voiceover-minimal.json` is not the final state check.
- Pinyin, toast typing, undo/redo, UTF-16 limits, probe interactions and system reduced-motion
  activation were not completed. No input-source or reduced-motion preference was changed.

## Capture and iOS setup checkpoint

`parity/weui-capture-results.json` records 38 successful **upstream-only** captures: 18 overlay
states plus uploader default, in light and dark. The site was served from the hash-verified
cache on port 7194, with Playwright Chromium installed for this run. No matching native sweep
or new paired comparison was completed. See `docs/PARITY.md` for the qualification.

`ios-setup.json` records Xcode 26.2 and an installed iOS 26.2 (23C52) runtime. An iPhone 17
Simulator was created but not booted. No iOS app was built, packaged, installed or launched.
This setup is not simulator or device acceptance. GPUI remains pinned to 0.3.7.

## Reproduction

From the repository root (using the checked source and an external target directory if set):

```sh
cargo fetch --locked
cargo fmt --all -- --check
cargo build --all-targets --locked --offline
cargo clippy --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline
cargo run --example minimal --locked --offline
cargo run --example gallery --locked --offline -- --page picker --state single
osascript -l JavaScript scripts/ax_dump.js <gallery-pid>
python3 -I scripts/weui_upstream.py serve --port 7194
WEUI_URL=http://127.0.0.1:7194/dist/example/index.html python3 -I scripts/capture.py actionsheet --state ios --only weui --out <capture-directory>
```

Native input and capture require a dedicated macOS GUI session and its permissions. The input
attempts above must be repeated without another app taking focus. These commands do not imply
that the incomplete checks passed.
