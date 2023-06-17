#!/bin/bash

# Function to check if a command is available
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if cargo is installed
if ! command_exists cargo; then
    echo "cargo not found. Installing cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Check if rustc is installed
if ! command_exists rustc; then
    echo "rustc not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Check if cargo-fmt is installed
if ! command_exists cargo-fmt; then
    echo "cargo-fmt not found. Installing cargo-fmt..."
    cargo install --force cargo-fmt
fi

# Check if cargo-clippy is installed
if ! command_exists cargo-clippy; then
    echo "cargo-clippy not found. Installing cargo-clippy..."
    cargo install --force cargo-clippy
fi

# Run cargo build
cargo build

# Run cargo test
cargo test

# Run cargo fmt
cargo fmt --all -- --check

# Run cargo clippy
cargo clippy --all-targets -- -D warnings

# If any of the commands above fail, exit with a non-zero status
if [ $? -ne 0 ]; then
    echo "Pre-push checks failed. Please fix the errors before pushing."
    exit 1
fi

