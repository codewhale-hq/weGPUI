set -u
cd <repo>
export TMPDIR=<prior-evidence>/executable-tmp
result=0
run() {
    printf '\n%s\n' "$*"
    "$@"
    status=$?
    printf 'GATE_EXIT=%s\n' "$status"
    if [ "$status" -ne 0 ]; then result=1; fi
}
run cargo fmt --all -- --check
run cargo build --all-targets --locked --offline
run cargo test --locked --offline
run cargo test --example gallery --locked --offline
run cargo clippy --all-targets --locked --offline -- -D warnings
run env RUSTDOCFLAGS='-D warnings -W missing-docs' cargo doc --no-deps --locked --offline
run python3 -I docs/proposals/gpui-cjk-closing/verify.py --workdir <evidence>/cjk-gate
run python3 -I docs/proposals/gpui-text-i18n/verify-word-break.py --workdir <prior-evidence>/word-break
run python3 -I docs/proposals/gpui-text-i18n/verify-logical-wrap.py --workdir <prior-evidence>/logical-wrap
run python3 -I docs/proposals/gpui-trailing-space-wrap/verify.py --workdir <prior-evidence>/trailing-space-current
run python3 -m unittest discover -s scripts/tests -v
run python3 -m unittest discover -s website/tests -v
exit "$result"
