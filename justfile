default: fmt lint test

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --lib --tests --bins -- -D warnings

test:
    cargo test --workspace --lib --tests --bins --no-fail-fast

ci: fmt-check lint test

publish-dryrun:
    cargo publish --dry-run -p gpui-engram-theme
    cargo publish --dry-run -p gpui-engram-ui
    cargo publish --dry-run -p gpui-engram
