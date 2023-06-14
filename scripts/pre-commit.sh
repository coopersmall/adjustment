#!/bin/bash

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if cargo exists
if ! command_exists cargo; then
    echo "cargo is not installed. Installing cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Check if rustfmt exists
if ! command_exists rustfmt; then
    echo "rustfmt is not installed. Installing rustfmt..."
    cargo install rustfmt --force
fi

# Run cargo fmt
echo "Running cargo fmt..."
cargo fmt -- --check
RESULT=$?

if [ $RESULT -ne 0 ]; then
    echo "Code formatting issues found. Please run 'cargo fmt' before committing."
    exit 1
fi

exit 0

