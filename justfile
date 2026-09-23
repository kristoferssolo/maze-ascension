export RUST_BACKTRACE := env("RUST_BACKTRACE", "1")

set shell := ["bash", "-cu"]

alias b := build
alias c := check
alias d := docs
alias f := fmt
alias l := lint
alias r := run
alias t := test
alias w := watch

# List available recipes
default:
    @just --list

# Run the app
[group("run")]
run: native-dev

# Run native development build
[group("run")]
native-dev:
    RUST_BACKTRACE=full cargo run

# Run native release build
[group("run")]
native-release:
    cargo run --release --no-default-features

# Run web development build
[group("run")]
web-dev:
    RUST_BACKTRACE=full trunk serve

# Run web release build
[group("run")]
web-release:
    trunk serve --release --no-default-features

# Start bacon (background checker)
[group("run")]
watch:
    bacon

# Run all checks
[group("quality")]
check: fmt lint docs test

# Run tests
[group("quality")]
test:
    LD_LIBRARY_PATH="$(rustc --print target-libdir)${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" cargo test --doc --locked --workspace --all-features
    cargo nextest run --workspace --all-features

# Format code
[group("quality")]
fmt:
    cargo fmt --all
    # bevy lint --fix

# Check formatting without modifying (for CI)
[group("quality")]
fmt-check:
    cargo fmt --all --check

# Run Bevy Lint and clippy
[group("quality")]
lint:
    bevy lint --workspace --all-targets --all-features
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Verify the release configuration compiles (dev features off)
[group("quality")]
check-release:
    cargo check --workspace --all-targets --no-default-features

# Build documentation
[group("quality")]
docs:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

# Open documentation in browser
[group("quality")]
docs-open:
    cargo doc --workspace --no-deps --all-features --open

# Build release binaries (shipping config, no dev features)
[group("build")]
build:
    cargo build --workspace --release --no-default-features

# Build web release bundle
[group("build")]
build-web:
    trunk build --release --no-default-features

# Clean build artifacts
[group("build")]
clean:
    cargo clean

# Install development dependencies
[group("setup")]
setup:
    rustup target add wasm32-unknown-unknown
    cargo install cargo-nextest trunk bacon
    bevy lint install main --yes
