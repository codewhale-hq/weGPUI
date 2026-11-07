# Integrating weGPUI

weGPUI is an unreleased developer preview. Its compatibility copies must match the library: Cargo patches in a dependency are not inherited by the consuming workspace.

## Add the dependencies and patches

For a standalone application whose `Cargo.toml` is also its workspace root, and a sibling checkout named `weGPUI`, add:

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

In a multi-package workspace, put the dependencies in the consuming package manifest and the patches in the workspace root; adjust the paths relative to each manifest.

The desktop platform dependency includes Linux windowing features; `font-kit` alone cannot open a Linux window. Keep the exact GPUI pins and vendor source together. See the [vendor guide](../vendor/README.md) for provenance and the scope of each compatibility change.

Register `wegpui::Assets` and call `wegpui::init(cx)` before opening windows. Wrap application roots in `accessibility::ModalScope` for modal isolation. The complete starting point is [examples/minimal.rs](../examples/minimal.rs); its platform helper and keyboard wiring are included in the repository.

Applications own their state, network requests, persistence and upload transport. Component APIs, defaults, callbacks and known limits are documented in the [component modules](../src/components/mod.rs).

## More examples

Run these from the weGPUI checkout after installing Rust 1.95+ and the [platform build requirements](../CONTRIBUTING.md#build-and-check):

```sh
cargo run --example application --locked
cargo run --example component_states --locked
cargo run --example gallery --locked -- --lang zh-Hans --theme dark --care
cargo doc --no-deps --open
```

`application` combines validation, controlled state, local image selection/preview/deletion, theme switching and modal confirmation. `component_states` exercises loading, labels, countdowns and keyboard behavior.

Built-in control labels use the active `I18n` catalog. English is the default; applications can switch languages at runtime or supply a custom catalog. Text passed to component builders remains application-owned.

```rust
wegpui::I18n::set_locale(wegpui::Locale::parse("ar").unwrap(), cx);
```

The [localisation guide](I18N.md) covers the 14 catalogs and RTL behavior, including the remaining translation and platform qualification work.

## Create a source preview bundle

```sh
python3 -I scripts/package_preview.py --out target/preview
```

The bundle includes a separate consumer, integrity manifests, licences and the patched source graph. It is a developer source distribution, not a signed installer or a published crate. See the [preview distribution guide](PREVIEW-DISTRIBUTION.md) for verification and usage.
