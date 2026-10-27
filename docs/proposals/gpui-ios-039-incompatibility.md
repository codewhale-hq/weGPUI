# Published GPUI 0.3.9 iOS/core API mismatch

Historical report, not submitted upstream. The owner subsequently approved the local
compatibility work. The existing vendored core now supplies the missing APIs and the
simulator executable links and launches. See the
[continuation](../receipts/dictionary-mobile-20261026/README.md). Registry files remain untouched.
The reproduction and failures below identify the earlier source.

`gpui-pre-ios =0.3.9` declares an exact dependency on `gpui-pre =0.3.9`, but fails
when building a native iOS simulator executable. Registry inspection on October 24,
2026 (America/Los_Angeles) found only this iOS release and no core release newer
than 0.3.9. That unmodified core/backend pair could not build an iOS application.

Reproduction: checkout weGPUI `05cae89f1373f42af30913aa65dc5e51f7c54baf`, install
`aarch64-apple-ios-sim` with Rust 1.99.1 on an Apple build host, then run:

```sh
bash examples/ios/build-simulator.sh gallery
```

Observed on GitHub's arm64 macOS 15.7.9 runner with Xcode 16.4 (16F6). Compilation
reaches `gpui-pre-ios`; core and weGPUI compile. Six E0599 diagnostics occur:

| Backend source | Missing core API |
| --- | --- |
| `src/ios/application.rs:143` and `:328` | `AppLifecyclePhase::Disconnected` |
| `src/ios/text_input.rs:163` and `:546` | `PlatformInputHandler::query_accepts_focused_text_input` |
| `src/ios/text_input.rs:432` | `PlatformInputHandler::selection_bounds_for_range` |
| `src/ios/window.rs:743` | `PlatformInputHandler::focus_id` |

Core 0.3.9 has only Active, Inactive, Background and Foreground lifecycle phases.
Its `PlatformInputHandler` has no focus-id field or the named methods. These APIs
are not conditional features. The compiler's similarly named-method suggestions
do not establish equivalent lifecycle, selection or focus semantics.

Requested upstream resolution: publish a coherent core/iOS snapshot with matching
APIs and a real simulator example build in its publication checks. A weGPUI-local
workaround would require modifying/forking dependency code, which the handoff
explicitly excludes without a separate decision. Merely upgrading Xcode cannot
supply missing Rust enum variants or methods.

[Run and retained diagnostics](../receipts/ci-20261024/README.md).
At the historical source above, no iOS application was linked, packaged, installed or launched.
The later compatibility work does not establish complete mobile or accessibility acceptance.
