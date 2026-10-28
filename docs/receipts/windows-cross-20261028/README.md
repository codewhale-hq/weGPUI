# Windows cross-check — 2026-10-28

The canonical checkout was clean at `1f293a4b86005edc2bf23242b7d28b6adfd8b30f` before and after the checks, matching the local `origin/main` ref. A preview source archive verified with `scripts/package_preview.py --verify` has source SHA-256 `87f4bb0dce2e2435426ec60520e03a9c576482893a25990a2357cd3fc695eae1` and archive SHA-256 `e175224c84b43b352c76a628e4c9b8ceedfb01fdc908bf175c51849c7af8f03f`. The archive was evidence only; the checks below ran against the canonical checkout.

On macOS ARM64 with Rust/Cargo 1.99.0 and the installed `x86_64-pc-windows-gnu` Rust target, these commands exited 0 under the shared build lock, offline and with the default dependency features:

```sh
cargo check --lib --target x86_64-pc-windows-gnu --locked --offline
cargo check --all-targets --target x86_64-pc-windows-gnu --locked --offline
```

The host lacked `x86_64-w64-mingw32-windres`, and the initial check failed in GPUI's build script; that failure is retained in `cross-check.log`. The successful checks used an artifact-local wrapper selecting already installed Homebrew LLVM 21.1.6 `llvm-windres`, plus Zig 0.16.0 targeting x86-64 Windows GNU as the resource preprocessor. A separate invocation compiled GPUI's actual Windows manifest; LLVM `readobj` identified an x86-64 COFF object containing a MANIFEST resource. The wrapper and object are retained alongside the logs. All-targets log SHA-256: `8308c969a8ef7f0058d50132140ca72fdbea159928f94f97443f53f73f494216`.

The Windows 11 ARM VM was stopped. A single `utmctl start --hide` attempt emitted `OSStatus error -10004` twice and stalled; the operator terminated the stalled command and UTM still reported the VM stopped. The cause was not established. The attempt is recorded in `vm-start-attempt.txt`; no guest build, test, UI, or UI automation was run.

These are Windows **x64 GNU cross-target type checks**, not native ARM64/MSVC compilation, linked executables, executed tests, UI or screen-reader verification. The VM and platform acceptance remain open.
