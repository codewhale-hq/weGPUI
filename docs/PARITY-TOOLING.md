# Parity tooling

This guide covers how to reproduce visual, interaction and accessibility evidence that compares
the native gallery with the pinned upstream WeUI 2.6.26 example. None of this tooling is a Cargo
dependency. It lives in `scripts/` and is excluded from the published package.

## What you need

| Purpose | Requirement | Notes |
| --- | --- | --- |
| Upstream fetch/serve | Python 3.10+ (standard library only) | `scripts/weui_upstream.py` |
| Reference screenshots | Python `playwright` plus Chromium | Use the browser installed for that Playwright version (`python3 -m playwright install chromium`). An existing Chromium can be selected with `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/absolute/path/to/chromium`; record both versions and any compatibility failures. Historical Linux captures used `playwright==1.63.0`, `chromium-1243` and `PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers`. |
| Image handling | Pillow | Contact sheets, pairs, sRGB conversion, generated dark PNGs |
| Native capture, macOS | `pyobjc-framework-Quartz`, `screencapture`, Screen Recording permission for the terminal | `scripts/capture.py` default path |
| Native capture, Linux X11 | `xdotool`, ImageMagick `import`, an X server (Xvfb is fine), and a gallery built with the `x11` feature (see below) | `scripts/capture.py` Linux path |
| Real input, macOS | `scripts/drive.py ROUTE --out DIR STEP...` (Quartz events), Accessibility permission | Mouse/keyboard into the native window |
| Accessibility tree, macOS | `osascript -l JavaScript scripts/ax_dump.js PID` | Whole-window AX dump, including raw AXTitle for option labels; not a VoiceOver test |
| Accessibility tree, Linux | `at-spi2-core`, `python3-pyatspi`, a D-Bus session | See *Linux procedure* |

A fresh macOS setup:

```sh
python3 -m venv .venv && . .venv/bin/activate
pip install playwright pillow pyobjc-framework-Quartz
python3 -m playwright install chromium
```

## Upstream reference page

```sh
python3 -I scripts/weui_upstream.py fetch   # download + verify SHA-256 + extract
python3 -I scripts/weui_upstream.py serve   # http://127.0.0.1:7193/dist/example/index.html
```

`fetch` writes to `$CARGO_TARGET_DIR/weui-upstream` (default `target/weui-upstream`). It verifies:

- `weui-2.6.26.tgz`, SHA-256 `f88a8f1d22ceb94e802bf423bd477a4a0cfad4cf9d70a77eedbc3cb17511b435`
- `weui.js-1.2.17.tgz`, SHA-256 `62aa1d4538bb81ac5d77aeee2b3cdbe03bfca1d28e619ed0641caa74e791182a`
- `jweixin-1.0.0.js`, SHA-256 `9e1a67f12789a5fde0ade6157154dc9bdfd17a5399b654f73740e0edaa95d94e`

The example shell (`dist/example/index.html`) loads weui.js and the WeChat JS SDK from
`res.wx.qq.com`. The served copy (`site/`) rewrites exactly those two `<script src>` attributes to
the verified local files and changes nothing else. `weui/` keeps the untouched package. On
2026-10-25 the CDN `weui.min.js` was byte-identical to the npm 1.2.17 `dist/weui.min.js`. The
jweixin hash pins the response observed that day. If the CDN changes, `fetch` fails loudly
instead of serving a different script.

## Capturing a pair

```sh
cargo build --example gallery
python3 -I scripts/capture.py ROUTE [--state S] [--theme light|dark] [--scroll Y] \
    [--brand RRGGBB] [--care] [--lang TAG] [--dir ltr|rtl] [--out DIR] [--only gpui|weui]
PAIR_DIR=/tmp/pairs python3 -I scripts/capture.py dialog --state ios1   # also writes a pair
```

- **Routes** are the gallery page names in `docs/UPSTREAM-COVERAGE.md` (the URL hash upstream).
- **States** are the keys in `scripts/parity/*.json` (`"route:state": "javascript"`). The native
  side passes `--state` to the gallery. The browser side runs the registered JavaScript after
  load. A new state needs both a gallery branch and a replay entry.
- **Themes** set `--theme` natively and `prefers-color-scheme` in Chromium.
- **Scroll** sets `--scroll Y` natively and `scrollTop` on the last `.page` upstream. Without it
  only the first 812 pt are compared.
- **Brand/care** pass `--brand`/`--care` natively and set `--weui-BRAND` / `data-weui-mode` upstream.
- **Language** is `--lang` (default `zh-Hans`, because the upstream reference is Chinese and the
  stored `docs/parity` images use it; the gallery itself now opens in English). `--dir rtl|ltr`
  forces the layout direction. Mirrored-layout captures such as `--lang ar` have no upstream
  counterpart, so pass `--only gpui`.
- **Scripted typing** for the *Languages* page: `--state type:<field index>:<ops>` focuses a field
  and dispatches the editing actions in the comma list `all`, `left`, `right`, `selleft`,
  `selright`, so selection and caret movement can be captured without synthetic input events.
- Output names are `<route>[-<state>]-<theme>-{gpui,weui}.png` at device scale 2 (750 px wide).
  Native captures with incorrect dimensions fail and retain an `-invalid.png` image for diagnosis.
- The binary comes from `$CARGO_TARGET_DIR/debug/examples/gallery` (default `target/`).

Compare pairs visually at full size, and use pixel statistics only as a pointer. Fonts, hinting and
spinner phases legitimately differ (see `docs/PARITY.md`, "Differences common to every pair").

## Language sweep

```sh
python3 -I scripts/i18n_sweep.py --langs en,ar --routes all --out DIR [--no-scan] [--settle 1.0]
```

Launches the gallery on every route in each language, captures the window and, unless
`--no-scan`, dumps the accessibility tree with `scripts/ax_dump.js` and reports Chinese characters
that survive in a non-Chinese language (the *Languages* page is exempt). The scan needs macOS
Automation permission for System Events; without it use `--no-scan` and read the captures. See
[I18N](I18N.md) and [receipts/i18n-20261025](receipts/i18n-20261025/README.md).

## Platform limits

- **macOS** is the reference platform for the committed screenshots in `docs/parity/`.
  `screencapture` includes the window's rounded bottom corners, which the browser does not have.
- **Linux X11** capture works under Xvfb with `GPUI_X11_SCALE_FACTOR=2` (set automatically). Use
  a screen big enough for a 375×812 window at 2× (for example `Xvfb :98 -screen 0 1600x3600x24`).
  Linux Chromium and GPUI both use the system CJK fonts (Noto), so Linux pairs check the
  pipeline and layout, but are not comparable with the macOS baseline images.
- Both examples enable `gpui_platform`'s `wayland` and `x11` features. The approved
  `docs/proposals/linux-windowing.patch` is applied; do not reapply it. The patch remains a
  historical proposal artifact. X11 runtime receipts do not qualify Wayland.
- **Windows, iOS, Android:** no capture path exists yet. See *Other platforms*.

## Linux procedure (interaction and accessibility)

Scripts in `scripts/linux/` reproduce the procedure recorded in
`docs/receipts/takeover-20261025/README.md`:

```sh
cargo build --example gallery
Xvfb :99 -screen 0 1280x1024x24 &
DISPLAY=:99 openbox &

# Accessibility tree, optionally after real input steps and screenshots:
DISPLAY=:99 STEPS="open:@190,170|down:Down|close:Escape" SHOTS=/tmp/select \
  dbus-run-session -- scripts/linux/session.sh \
  target/debug/examples/gallery --page form_select --position 0,0 --reduce-motion

# Visual sweep (routes, a theme, or every registered state), then contact sheets:
Xvfb :98 -screen 0 1600x3600x24 &
DISPLAY=:98 openbox &
python3 -I scripts/weui_upstream.py serve &
DISPLAY=:98 scripts/linux/sweep.sh /tmp/sweep-light        # all 60 routes, light
DISPLAY=:98 scripts/linux/sweep.sh /tmp/sweep-dark --theme dark
DISPLAY=:98 scripts/linux/sweep.sh /tmp/states-dark --theme dark --states
DISPLAY=:98 scripts/linux/sweep.sh /tmp/states-care --care --states
DISPLAY=:98 scripts/linux/sweep.sh /tmp/states-brand --brand 0055CC --states
DISPLAY=:98 scripts/linux/sweep.sh /tmp/states-dark-care --theme dark --care --states
python3 -I scripts/contact_sheets.py /tmp/states-dark/pairs /tmp/states-dark/sheets
```

`--care` and `--brand RRGGBB` are forwarded to both native and upstream captures and can be
combined with either theme. `--states` captures every registered state and cannot be combined
with route arguments. Unknown options, missing values and invalid brand colours fail before
capture. `sweep.log` records theme, care, brand and capture-side options for each invocation.

Use a **new empty output directory for every invocation**, including retries. Capture filenames
contain route, state and theme, but do not contain care/brand settings. Reusing a directory can
overwrite screenshots and per-capture stderr, mix modes or leave stale pairs. Preserve failed
directories. Record the source SHA and dirty diff, binary build command and Rust version, OS,
display/window-manager versions, Python/browser versions, exact command and reviewed images in
the receipt; the options in `sweep.log` alone do not establish build provenance or visual parity.

If the pinned upstream fetch or browser is unavailable, native state captures can still run:

```sh
DISPLAY=:98 scripts/linux/sweep.sh /tmp/native-states-dark --only gpui --theme dark --states
```

`--only gpui` does not need Playwright or a reference server, and `--only weui` captures only the
reference. These produce single-side images in `raw/`; a native-only sweep is **not a parity
result**. Keep the blocked/failed reference attempt alongside it and do not count the empty
`pairs/` directory as reviewed pairs.

`session.sh` needs `at-spi2-core`, `python3-pyatspi`, `xdotool` and `dbus` (plus ImageMagick
for screenshots). It picks the first distro Python that can import `pyatspi`, or set `PYTHON`.
`sweep.sh` uses `PYTHON` (default `python3`) for Playwright and Pillow. It returns nonzero if
any capture fails, and its summary counts only that invocation; the appended log preserves prior
runs. `session.sh` retains application output and fails if the process exits before capture.
The AT-SPI dumper fails on an empty tree or traversal error. These failures are tooling evidence,
not an interaction verdict.

- Without a window manager, X keyboard focus reaches the gallery only after a real click inside
  it (`xdotool mousemove X Y click 1`). Routes opened with `--state` at startup do not receive
  keys until then. Use plain `xdotool key`; `--window` sends synthetic events that applications
  may ignore.
- AccessKit reports the `focused` state only for an active window, which a bare Xvfb never has.
  Judge focus from screenshots.
- Run a window manager (`openbox`) on the Xvfb display when focus matters: GPUI dispatches focus
  events only to an active window, so without one `TextInput` draws no caret. Under openbox
  capture with `import -window <id>`; root-window captures can come out black.
- IME: `fcitx5` with `fcitx5-chinese-addons`, started inside the D-Bus session, and the gallery
  run with `XMODIFIERS=@im=fcitx`. Ctrl+Space switches to Pinyin.
  The smaller `fcitx5-pinyin` package is sufficient for the recorded Pinyin runs. Fcitx's
  default `UseOnTheSpot=False` shows external preedit; a private profile with True exercises
  inline preedit and caret-adjacent candidates. Record the queried setting and use a private
  session. Real selected-text cancellation, callback and state/limit sequences are in
  `receipts/ime-x11-20261025/`; direct handler calls are separate protocol evidence.
- File chooser: `xdg-desktop-portal` plus `xdg-desktop-portal-gtk` inside the D-Bus session
  (`XDG_RUNTIME_DIR` must exist). Without a portal, `prompt_for_paths` returns an error, which the
  uploader page shows.
- Transitions: `ffmpeg -f x11grab -framerate 50` records frames you can measure.
- Text screen coordinates: compare AT-SPI SCREEN minus WINDOW extents with raw `xwininfo`
  absolute client origin or direct `XTranslateCoordinates`. In this openbox setup,
  `xdotool getwindowgeometry` double-counts decorations and gives a misleading offset.
- Orca word-driving: hold modifiers through both key-down and key-up. An xdotool command
  that releases Ctrl before Right key-up can make Orca interpret a different readback command;
  preserve that driver failure separately from component behavior.
- Current root Cargo patches are part of build provenance. Hash the selected vendored sources
  as well as `src/`, examples, Cargo files and the executable. A runtime working-tree hash is
  not proof that an older copied executable was built from those bytes.

## Other platforms

- **Windows:** build natively (`cargo run --example gallery`), capture with the Snipping Tool or
  `PrintWindow`, and inspect accessibility with Accessibility Insights or Narrator/NVDA. Name the
  Windows version, scaling factor and screen reader in the report. A mingw cross-build is not a
  runtime receipt. The earlier Wine 9.0 failure is commit-reported without a committed run log.
- **iOS/Android:** iOS has an unqualified backend/bundle scaffold; Android has no backend
  (`docs/PLATFORMS.md`). Runtime evidence must come from a labelled simulator/emulator or device, with OS version, device model,
  orientation, safe-area insets, software keyboard/IME, lifecycle (background/foreground) and
  VoiceOver/TalkBack results. A narrow desktop window is not mobile evidence.

## Reporting a parity result

Every report (issue, PR or receipt) names the route, the state, the theme (plus brand/care), both
screenshots, the platform with OS version and scale, and the commit. Say what was compared
(visual, interaction, accessibility tree or screen reader) and what was not. Keep failed captures.
A script that exits 0 is not evidence that the interaction worked.
