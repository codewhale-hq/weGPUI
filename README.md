# weGPUI

WeUI 2.6.26 reimplemented as native Rust GPUI components. This is an independent port of the official WeUI design system, built from upstream GPUI primitives.

The port is in development. See [STATUS](docs/STATUS.md) for build and interaction evidence and [PARITY](docs/PARITY.md) for visual comparisons. Presence in the gallery does not establish visual or behavioural parity.

```sh
cargo run --example gallery
```

## Licence and attribution

Original weGPUI code is available under either the MIT or Apache License 2.0, at your option. See LICENSE-MIT and LICENSE-APACHE.

WeUI design, example content and icon assets: **Copyright Tencent, Inc.**, MIT licence. The pinned distribution additionally identifies Copyright 2026 Tencent, Inc. and Copyright (C) 2017 THL A29 Limited, a Tencent company. Its MIT notice is preserved in NOTICE and LICENSE-WEUI.

The text input adapts GPUI's `examples/input.rs`, Copyright Zed Industries, Apache-2.0. Its attribution is preserved in the source header and NOTICE.
