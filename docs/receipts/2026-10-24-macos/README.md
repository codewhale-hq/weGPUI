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
Runtime and build receipts will identify their own source revision and commands.
