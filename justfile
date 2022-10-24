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

checkr:
    cargo +nightly fmt --check
    cargo clippy --all-targets --all-features --workspace --release -- -D warnings

doc:
    cargo doc --workspace --all-features --document-private-items --open

test:
    cargo test --workspace --all-features

testr:
    cargo test --workspace --all-features

test-out:
    cargo test --workspace --all-features -- --nocapture

bench:
    cargo bench --workspace --all-features

bench-native:
    RUSTFLAGS="-C target-cpu=native" cargo bench --workspace --all-features

build:
    cargo build --workspace

buildr: check test
    cargo build --release --workspace

pkg PACKAGE:
    #cargo publish --dry-run --allow-dirty --no-verify -p {{PACKAGE}}
    cargo package --list --allow-dirty --no-verify -p {{PACKAGE}}

run-t1:
    cargo run --release -- ./asm.tmp --asm
    cargo run --release -- ./asm.allot
