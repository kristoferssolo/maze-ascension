# Default recipe
default:
    @just --list

# Run native dev
native-dev:
    RUSTC_WRAPPER=sccache RUST_BACKTRACE=full cargo run

# Run native release
native-release:
   RUSTC_WRAPPER=sccache  cargo run --release --no-default-features

# Run web dev
web-dev:
    RUST_BACKTRACE=full trunk serve

# Run web release
web-release:
    RUSTC_WRAPPER=sccache trunk serve --release --no-default-features

# Run tests
test:
    RUSTC_WRAPPER=sccache RUST_BACKTRACE=full cargo test --doc --locked --workspace --no-default-features
    RUSTC_WRAPPER=sccache RUST_BACKTRACE=full cargo nextest run --no-default-features --all-targets

# Run CI localy
ci:
    #!/bin/bash
    set -e
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- --deny warnings
    cargo doc --workspace --all-features --document-private-items --no-deps
    just test
