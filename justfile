default:
    just -l

# Format the project
fmt:
    cargo +nightly fmt

# Runs formatting and clippy checks
check:
    cargo +nightly fmt --check
    cargo clippy --all-targets --all-features --workspace -- -D warnings

test:
    cargo test --workspace --all-features

build:
    cargo build --workspace

build-release: check test
    cargo build --release --workspace


