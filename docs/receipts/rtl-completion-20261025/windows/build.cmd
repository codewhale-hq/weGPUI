@echo off
set ART=C:\CW\artifacts\wegpui-rtl-20261025
set ROOT=%ART%\wegpui-preview-7fa821c8b9c5
set CARGO_HOME=C:\Users\GUEST_USER\.cargo
set RUSTUP_HOME=C:\Users\GUEST_USER\.rustup
set CARGO_TARGET_DIR=C:\CW\artifacts\wegpui-20261024\target
set CARGO_BUILD_JOBS=2
set PATH=C:\Users\GUEST_USER\.cargo\bin;%PATH%
call C:\BuildTools2022\Common7\Tools\VsDevCmd.bat -arch=arm64 -host_arch=arm64 > "%ART%\compiler-env.txt" 2>&1
if errorlevel 1 goto fail
cd /d "%ART%"
echo extract>"%ART%\state.txt"
tar.exe -xzf source.tar.gz > "%ART%\extract.txt" 2>&1
if errorlevel 1 goto fail
cd /d "%ROOT%"
rustc -Vv > "%ART%\rustc.txt" 2>&1
cargo -V >> "%ART%\rustc.txt" 2>&1
echo build>"%ART%\state.txt"
cargo build --workspace --all-targets --locked --offline > "%ART%\build-initial.txt" 2>&1
if errorlevel 1 (
 echo fetch>"%ART%\state.txt"
 cargo fetch --locked > "%ART%\fetch.txt" 2>&1
 if errorlevel 1 goto fail
 echo build>"%ART%\state.txt"
 cargo build --workspace --all-targets --locked --offline > "%ART%\build.txt" 2>&1
 if errorlevel 1 goto fail
)
echo tests>"%ART%\state.txt"
cargo test -p wegpui --lib --example gallery --locked --offline > "%ART%\tests.txt" 2>&1
if errorlevel 1 goto fail
echo clippy>"%ART%\state.txt"
cargo clippy --workspace --all-targets --locked --offline -- -D warnings > "%ART%\clippy.txt" 2>&1
if errorlevel 1 goto fail
echo rustdoc>"%ART%\state.txt"
set RUSTDOCFLAGS=-D warnings -W missing-docs
cargo doc --workspace --no-deps --locked --offline > "%ART%\rustdoc.txt" 2>&1
if errorlevel 1 goto fail
echo PASS>"%ART%\state.txt"
exit /b 0
:fail
echo FAIL>"%ART%\state.txt"
exit /b 1
