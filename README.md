# weGPUI

**Mac desktop and WeUI mobile components, rendered natively in Rust.**

weGPUI brings [Tencent's WeUI 2.6.26](https://github.com/Tencent/weui) design language to
[GPUI](https://gpui.rs), the GPU-accelerated framework behind Zed. It provides native controls,
forms, navigation and overlays using GPUI primitives, with no embedded browser.

[Explore the browser gallery](https://codewhale-hq.github.io/weGPUI/) ·
[Run the native examples](#quick-start) · [Project status](docs/STATUS.md) ·
[Contribute](CONTRIBUTING.md)

**Developer preview · 0.1.0 · unreleased.** The native gallery maps all 60 upstream routes and
adds Languages, a desktop form, and nine Mac desktop examples. Visual, interaction and accessibility parity are still being qualified;
no desktop platform has completed the full support criteria. See the
[acceptance matrix](docs/ACCEPTANCE.md) and [remaining work](docs/COMPLETION.md).

| Native GPUI | Upstream WeUI |
| --- | --- |
| <img src="https://raw.githubusercontent.com/codewhale-hq/weGPUI/main/docs/parity/half-screen-dialog-1-light-gpui.png" width="280" alt="Native weGPUI half-screen dialog"> | <img src="https://raw.githubusercontent.com/codewhale-hq/weGPUI/main/docs/parity/half-screen-dialog-1-light-weui.png" width="280" alt="Upstream WeUI half-screen dialog in Chromium"> |

Recorded macOS comparison. [Visual evidence](docs/PARITY.md) identifies each capture's source
and review limits; these images are not a claim of current cross-platform parity.

## What is included

- **Mac desktop:** a resizable chat shell, conversation rows, avatars, message bubbles,
  quotes, translations, attachments, composer, contacts, settings, history, media grids,
  menus, dialogs and emoji selection. [Desktop API and examples](docs/DESKTOP.md).

- **Controls and forms:** buttons, cells, text inputs, text areas, choices, switches, sliders,
  search, validation, verification codes and date fields.
- **Navigation and content:** navigation bars, tab bars, grids, panels, articles, steps,
  footers and previews.
- **Overlays and media:** dialogs, half-screen dialogs, action sheets, pickers, toasts,
  image viewers and uploader interfaces.
- **Themes:** light and dark appearance, care contrast, custom brand colours and host palettes.
- **Accessibility:** roles and names, keyboard navigation, visible focus, modal isolation,
  focus restoration and reduced-motion handling, with bounded native and reader evidence.
- **Localisation:** English defaults, 14 message catalogs, custom catalogs and mirrored RTL
  layouts. Translation review and several bidirectional-text cases remain open.

The [latest parity continuation](docs/receipts/dictionary-mobile-20261026/README.md) adds
dictionary line breaking, shaping-cluster protection and the iOS compatibility bridge. Recorded
comparisons identify their own source; capture completeness does not establish full parity.

The [component modules](src/components/mod.rs) document APIs, defaults, callbacks and known
limits. Applications own state, network requests, persistence and upload transport.

## Quick start

Install **Rust 1.95+** and your platform's [GPUI build requirements](CONTRIBUTING.md#build-and-check).
Linux additionally needs an X11 or Wayland desktop and GPUI's system libraries.

```sh
git clone https://github.com/codewhale-hq/weGPUI.git
cd weGPUI
cargo run --example minimal --locked
cargo run --example gallery --locked
cargo run --example gallery --locked -- --page desktop-chat --theme light
```

More examples:

```sh
cargo run --example application --locked
cargo run --example component_states --locked
cargo run --example gallery --locked -- --lang zh-Hans --theme dark --care
cargo doc --no-deps --open
```

`minimal` demonstrates a cell, button and dialog. `application` combines validation, controlled
state, local image selection/preview/deletion, theme switching and modal confirmation.
`component_states` exercises loading, labels, countdowns and keyboard behavior.

## Use in an application

The preview requires the matching compatibility copies in `vendor/`. **Cargo patches are not
inherited from dependencies:** configure all five at your application's workspace root.
With a sibling checkout named `weGPUI`, use:

```toml
[dependencies]
wegpui = { path = "../weGPUI" }
gpui = { package = "gpui-pre", version = "=0.3.9" }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.9", features = ["font-kit", "wayland", "x11"] }

[patch.crates-io]
gpui-pre = { path = "../weGPUI/vendor/gpui_pre" }
gpui-pre-linux = { path = "../weGPUI/vendor/gpui_pre_linux" }
gpui-pre-ios = { path = "../weGPUI/vendor/gpui_pre_ios" }
accesskit_unix = { path = "../weGPUI/vendor/accesskit_unix" }
accesskit_atspi_common = { path = "../weGPUI/vendor/accesskit_atspi_common" }
```

The desktop platform dependency includes Linux windowing features; `font-kit` alone cannot
open a Linux window. Keep the exact GPUI pins and vendor source together. Provenance and the
scope of each compatibility change are documented in [vendor/README.md](vendor/README.md).

Register `wegpui::Assets` and call `wegpui::init(cx)` before opening windows. Wrap application
roots in `accessibility::ModalScope` for modal isolation. The complete, compiled starting point
is [examples/minimal.rs](examples/minimal.rs); its platform helper and keyboard wiring are
included in the repository.

For a self-contained source setup with the patches already configured, create a
[reproducible preview bundle](docs/PREVIEW-DISTRIBUTION.md):

```sh
python3 -I scripts/package_preview.py --out target/preview
```

The bundle includes a separate consumer, integrity manifests, licences and the patched source
graph. It is a developer source distribution, not a signed installer or a published crate.

## Languages and right-to-left

Built-in control labels and messages use the active `I18n` catalog. English is the default;
applications can switch languages at runtime or supply a custom `Messages` catalog.
Text passed to component builders remains application-owned.

```rust
wegpui::I18n::set_locale(wegpui::Locale::parse("ar").unwrap(), cx);
```

The 14 catalogs cover English, Simplified and Traditional Chinese, Japanese, Korean, Spanish,
French, German, Portuguese, Russian, Hindi, Thai, Arabic and Hebrew. Twelve are starter
translations awaiting native-speaker review. RTL layouts, wrapped input caret/selection and multiline alignment are implemented, with
sampled macOS and Linux evidence. Shared Unicode bidi geometry, styled RTL colors/underlines,
locale-ordered Gregorian date entry/pickers and host-local today are implemented. Full IME,
reader and platform qualification remains open. See the
[code and parity receipt](docs/receipts/code-parity-20261026/README.md) and
[localisation guide](docs/I18N.md).

## Platform status

The first release targets macOS, Windows and Linux. Build results, native runtime samples and
complete platform support are separate milestones.

| Platform | Available evidence | Remaining qualification |
| --- | --- | --- |
| Linux | Current source gates, X11 comparisons, native interaction and Orca samples; historical nested-Wayland checks | Complete reader/IME/state coverage, hardware and distribution |
| macOS | Current build/tests/Clippy/docs and sampled RTL native input/AX checks; historical comparisons | Complete state/input/reader coverage and signed distribution |
| Windows | Current ARM64/MSVC build/tests/Clippy/docs and standalone consumer; historical native interactions | Visible UI excluded by owner; reader and distribution remain unqualified |
| iOS | Experimental simulator bundle links and launches; sampled touch navigation | Complete keyboard/IME, insets, lifecycle, accessibility and device qualification |
| Android | Library type-checks | Backend selection and native runtime |

[Platform evidence](docs/PLATFORMS.md) defines the support criteria.
[Release readiness](docs/RELEASE-READINESS.md) tracks packaging, signing and maintainer decisions.

## Component gallery website

The website source showcases all **71 native Rust gallery routes**, with light/dark captures,
component states, search, category filters and the actual Rust source beside each preview.
The Mac desktop collection comes first. Its geometry follows the observed WeChat 4.2.0.9
Mac app, its control contracts are informed by WeUiSharp, its desktop surfaces follow the sampled Mac reference, and its shared
control accents and states use WeUI tokens. The original 60 WeUI routes retain their responsive desktop and phone
compositions. These are distinct collections: the Mac set is an original GPUI implementation.
Use the Desktop/Phone switch to inspect both native layouts. The 60 upstream
HTML/CSS/JavaScript examples remain available as a separately labeled browser reference.
Mobile backends are experimental; portrait native captures do not imply mobile support.

Preview the current website locally:

```sh
python3 website/build.py
python3 -m http.server 4173 --bind 127.0.0.1 --directory website/dist
```

The [published gallery](https://codewhale-hq.github.io/weGPUI/) changes only after a separately
approved Pages deployment. Source pushes do not deploy it. [Website documentation](website/README.md)
covers the static build, source provenance and publishing workflow.

## Development and review

Run the five source gates after filling the dependency cache with `cargo fetch --locked`:

```sh
cargo fmt --all -- --check
cargo build --all-targets --locked --offline
cargo test --locked --offline
cargo clippy --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline
```

The [parity plan](docs/WEUI-PARITY-PLAN.md) pins upstream source and tracks 132 scenes across
eight appearances. [Capture tooling](docs/PARITY-TOOLING.md) records source and image hashes;
capture completeness and pixel scores do not establish parity. Component behavior, actual
reader output and platform acceptance each require their own evidence.

- [Publication review](docs/REVIEW-2026-10-26.md) and [status/known limitations](docs/STATUS.md)
- [Component acceptance](docs/ACCEPTANCE.md) and [visual review](docs/PARITY.md)
- [Upstream coverage](docs/UPSTREAM-COVERAGE.md) and [source attribution](docs/SOURCES.md)
- [Contributing](CONTRIBUTING.md), [security reporting](SECURITY.md) and [code of conduct](CODE_OF_CONDUCT.md)
- [Changelog](CHANGELOG.md)

[GitHub](https://github.com/codewhale-hq/weGPUI) is the canonical home for source, issues and
pull requests. [CNB](https://cnb.cool/codewhale.net/weGPUI) is a read-only mirror.

## Licence

Original weGPUI code is **MIT OR Apache-2.0**. See [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).

WeUI design, example content and icons are Copyright Tencent, Inc., under the MIT licence.
The distribution also identifies THL A29 Limited, a Tencent company. Their notices are
preserved in [LICENSE-WEUI](LICENSE-WEUI) and [NOTICE](NOTICE).

The text input adapts GPUI's input example, Copyright Zed Industries, Apache-2.0; its
attribution is preserved in the source and NOTICE. This is an independent project.
