# Default recipe
default:
    @just --list

# Run native dev
native-dev:
    RUST_BACKTRACE=full cargo run

# Run native release
native-release:
    cargo run --release --no-default-features

# Run web dev
web-dev:
    RUST_BACKTRACE=full trunk serve

# Run web release
web-release:
    trunk serve --release --no-default-features

