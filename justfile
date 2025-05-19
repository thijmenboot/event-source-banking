# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Fix linting errors where possible
fix: fmt && check
    cargo clippy --fix --allow-staged --workspace -- -D warnings --no-deps

# Check for linting errors
check:
    cargo clippy -- -D warnings

# Format the Rust code
[private]
fmt:
    cargo fmt --all