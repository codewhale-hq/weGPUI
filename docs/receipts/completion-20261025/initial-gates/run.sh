set -eu
cd <earlier-preview>/wegpui-preview-*
out=<initial-gates>
rustc --version > "$out/context.txt"
cargo --version >> "$out/context.txt"
uname -srm >> "$out/context.txt"
: > "$out/summary.txt"
for gate in fmt build test clippy doc; do
set +e
case "$gate" in
fmt) cargo fmt --all -- --check > "$out/fmt.log" 2>&1 ;;
build) cargo build --all-targets --locked --offline > "$out/build.log" 2>&1 ;;
test) cargo test --locked --offline > "$out/test.log" 2>&1 ;;
clippy) cargo clippy --all-targets --locked --offline -- -D warnings > "$out/clippy.log" 2>&1 ;;
doc) RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline > "$out/doc.log" 2>&1 ;;
esac
result=$?
set -e
echo "$gate=$result" | tee -a "$out/summary.txt"
if [ "$result" -ne 0 ]; then exit "$result"; fi
done
python3 -W error -m unittest discover -s wegpui/scripts/tests -v > "$out/tool-tests.log" 2>&1
printf 'tool-tests=0\n' | tee -a "$out/summary.txt"
sha256sum "$CARGO_TARGET_DIR/debug/wegpui-preview" > "$out/consumer-binary.txt"
