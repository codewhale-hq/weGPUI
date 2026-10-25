# weGPUI

weGPUI is [WeUI 2.6.26](https://github.com/Tencent/weui), Tencent's design system for WeChat, as
native Rust components for [GPUI](https://gpui.rs), the GPU-accelerated UI framework behind Zed.
It renders WeUI's tokens, icons, components and overlays with GPUI primitives, without a WebView,
and is written to stay close to upstream behaviour.

| Native GPUI (macOS) | Upstream WeUI in Chromium |
| --- | --- |
| <img src="docs/parity/half-screen-dialog-1-light-gpui.png" width="280" alt="weGPUI half-screen dialog"> | <img src="docs/parity/half-screen-dialog-1-light-weui.png" width="280" alt="WeUI half-screen dialog"> |

The library is in development: **0.1.0, unreleased**. All 61 upstream example files map to the
gallery (60 routes plus the index shell), but parity is sampled, not complete. [docs/STATUS.md](docs/STATUS.md) records what is
verified, what differs, and what has not been tested.

## Quick start

```sh
git clone https://github.com/codewhale-hq/weGPUI && cd weGPUI
cargo run --example minimal    # a small app: a list cell, a button and a dialog
cargo run --example gallery    # every WeUI example page, with theme and care-mode menus
```

Rust 1.95 or newer is required. Linux needs GPUI's system libraries (see
[CONTRIBUTING](CONTRIBUTING.md)) and, for now, a local change that enables `gpui_platform`'s
`wayland`/`x11` features before a window can open ([docs/PLATFORMS.md](docs/PLATFORMS.md)).

## Using the library

weGPUI depends on `gpui` and `unicode-segmentation`, with no platform crate in the library's
dependencies. Your application picks the GPUI platform crate:

```rust
use gpui::{prelude::*, *};
use wegpui::components::{Cell, Cells, button::Button};

struct Home;

impl Render for Home {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = wegpui::Theme::get(cx);
        div().size_full().bg(t.bg_0)
            .child(Cells::new().child(Cell::new("account").title("Account").access()))
            .child(Button::new("Continue").id("continue").primary())
    }
}

fn main() {
    gpui_platform::application().with_assets(wegpui::Assets).run(|cx| {
        wegpui::init(cx);
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Home)).unwrap();
    });
}
```

Until a crates.io release, depend on the Git repository and the same exact GPUI pins:

```toml
[dependencies]
wegpui = { git = "https://github.com/codewhale-hq/weGPUI" }
gpui = { package = "gpui-pre", version = "=0.3.9" }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.9", features = ["font-kit"] }
```

On Linux, that application dependency also needs `"wayland"` or `"x11"` in `features`;
`font-kit` alone builds but panics when opening a window. This applies to both examples too.

[`examples/minimal.rs`](examples/minimal.rs) is the compiled, complete version, including
`ModalScope` (hides the page from screen readers while a modal is open), Tab navigation and a
`Presence`-driven dialog. Public items carry rustdoc: run `cargo doc --open`.

## Platform support

Nothing is called supported until build, launch, rendering, interaction, accessibility and
packaging all have receipts from that platform. Details and evidence are in
[docs/PLATFORMS.md](docs/PLATFORMS.md).

| Platform | Today |
| --- | --- |
| macOS | Build, Clippy and strict rustdoc pass at `e4a2c87` (Rust 1.99); minimal and gallery launch, with sampled AX evidence. VoiceOver attempt incomplete. Not packaged. |
| Linux | Windowing features enabled. Both examples build and render under isolated Xvfb; sampled dialog/select XTEST input and AT-SPI trees verified. No Wayland runtime or Orca acceptance. Not packaged. |
| Windows | Cross-compiles (mingw). No native run, accessibility or packaging evidence. |
| iOS | 0.3.9 backend entry point and simulator bundle scaffold implemented. No Apple app build/runtime acceptance. Not supported. |
| Android | Library type-checks only. No upstream backend. Not supported. |

## Documentation

- [STATUS](docs/STATUS.md): done, partial, and not done, with missing receipts
- [Independent review](docs/REVIEW-2026-10-24.md): per-family behaviour table
- [Visual parity](docs/PARITY.md) and [parity tooling](docs/PARITY-TOOLING.md)
- [Upstream coverage](docs/UPSTREAM-COVERAGE.md) and [sources](docs/SOURCES.md)
- [Platform feasibility](docs/PLATFORMS.md) and the [CI proposal](docs/CI-PROPOSAL.md)
- [Contributing](CONTRIBUTING.md), [Code of Conduct](CODE_OF_CONDUCT.md),
  [Security](SECURITY.md), [Changelog](CHANGELOG.md)

[GitHub](https://github.com/codewhale-hq/weGPUI) is the canonical home for issues and pull
requests; [CNB](https://cnb.cool/codewhale.net/weGPUI) is a mirror. The repository builds
independently of any parent workspace.

## Licence and attribution

Original weGPUI code is available under either the MIT or Apache License 2.0, at your option. See
LICENSE-MIT and LICENSE-APACHE.

WeUI design, example content and icon assets: **Copyright Tencent, Inc.**, MIT licence. The pinned
distribution additionally identifies Copyright 2026 Tencent, Inc. and Copyright (C) 2017 THL A29
Limited, a Tencent company. Its MIT notice is preserved in NOTICE and LICENSE-WEUI.

The text input adapts GPUI's `examples/input.rs`, Copyright Zed Industries, Apache-2.0. Its
attribution is preserved in the source header and NOTICE.
