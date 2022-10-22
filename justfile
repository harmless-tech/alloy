default:
    just -l

# Format the project
fmt:
    cargo +nightly fmt

full-check: check test build

# Runs formatting and clippy checks
check:
    cargo +nightly fmt --check
    cargo clippy --all-targets --all-features --workspace -- -D warnings

doc:
    cargo doc --workspace --all-features --open

test:
    cargo test --workspace --all-features

test-out:
    cargo test --workspace --all-features -- --nocapture

bench:
    cargo +nightly bench --workspace --all-features

build:
    cargo build --workspace

build-release: check test
    cargo build --release --workspace

publish-dry:
    cargo publish --dry-run --allow-dirty -p allot_runtime
    cargo package --list --allow-dirty -p allot_runtime
    cargo publish --dry-run --allow-dirty -p allot
    cargo package --list --allow-dirty -p allot
