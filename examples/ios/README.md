# Experimental iOS simulator bundle

Both examples select `gpui-pre-ios =0.3.9` on iOS and `gpui-pre-platform =0.3.9`
on desktop. The desktop platform dependency is excluded on iOS because its
published 0.3.9 dispatch does not contain an iOS branch. The library's public API
is unchanged.

On a **separately authorized Apple host** with Xcode and a simulator SDK:

```sh
rustup target add aarch64-apple-ios-sim
bash examples/ios/build-simulator.sh gallery
# Or: bash examples/ios/build-simulator.sh minimal
```

For an Intel simulator use `IOS_SIMULATOR_TARGET=x86_64-apple-ios` and install
that Rust target. `CARGO_TARGET_DIR` may be absolute or relative to the repository.
The script prints a unique evidence directory containing Cargo JSON, stderr,
signing output and the resulting `WeGPUI.app`. Failed builds retain their logs.
It does not boot a simulator, install an app, launch applications or enable a
screen reader. Run those steps only on the authorized Apple host, identifying the
simulator/device and OS in the resulting receipt.

The bundle layout and scene manifest follow the Apache-2.0-licensed
`gpui-pre-ios 0.3.9` example (Zed snapshot `b4ecc454225fc1819e37e890b62df5473b2ba95b`).
The scene delegate name `GPUIIosSceneDelegate` is supplied by that backend.
No generated Xcode project or signing identity is needed for an ad-hoc simulator
bundle; physical-device provisioning is separate.

The approved local core bridge supplies the lifecycle/focus/selection APIs missing from the
published pair. A local Apple ARM64 simulator bundle now links and launches. The gallery and
minimal example use the fully visible bounds, and the gallery includes a touch Back control.
weGPUI text input supplies document-ordered selection fragments; its scrollers respond when
the visible viewport shrinks. These implementations still need the complete software-keyboard,
IME, inset, rotation, lifecycle and physical-device acceptance matrix.
The backend has no mobile AccessKit adapter. Linux library type-checks and shell
fixture tests cannot establish Apple runtime acceptance. See
[platform requirements](../../docs/PLATFORMS.md).

The first hosted build retained six missing core APIs in the published 0.3.9 pair. See the
[historical report](../../docs/proposals/gpui-ios-039-incompatibility.md) and
[current continuation](../../docs/receipts/dictionary-mobile-20261026/README.md).
Successful linking and sampled touch navigation do not establish full mobile acceptance.

On iOS, password fields deliberately reject native text sessions and expose no native
document text or length. The pinned UIKit backend has no secure-entry trait; visually
masking a plain native responder would expose password text to system UI/accessibility.
Native secure entry is required before iOS password editing can be supported. Desktop
password behavior is unchanged.
