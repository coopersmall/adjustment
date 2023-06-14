#!/bin/bash

set -e

# Install Rustup
if ! command -v rustup &>/dev/null; then
    echo "Installing Rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    echo "Rustup installed successfully!"
else
    echo "Rustup is already installed."
fi

# Install Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy
echo "Rust components installed successfully!"

# Install cargo-edit
if ! command -v cargo-edit &>/dev/null; then
    echo "Installing cargo-edit..."
    cargo install cargo-edit
    echo "cargo-edit installed successfully!"
else
    echo "cargo-edit is already installed."
fi

# Install cargo-watch
if ! command -v cargo-watch &>/dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
    echo "cargo-watch installed successfully!"
else
    echo "cargo-watch is already installed."
fi

echo "Tooling installation complete."

