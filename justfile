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

build:
    cargo build --workspace

build-release: check test
    cargo build --release --workspace
