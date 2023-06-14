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

#!/bin/bash

set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install figlet
if ! command_exists figlet; then
    echo "Installing figlet..."

    # Check the package manager
    if command_exists brew; then
        # macOS with Homebrew
        brew install figlet
    elif command_exists choco; then
        # Windows with Chocolatey
        choco install figlet
    elif command_exists dnf; then
        # Fedora with dnf
        sudo dnf install -y figlet
    elif command_exists pacman; then
        # Arch Linux with pacman
        sudo pacman -Sy --noconfirm figlet
    elif command_exists apt; then
        # Ubuntu/Debian with apt
        sudo apt update
        sudo apt install -y figlet
    else
        echo "Unsupported package manager. Please install figlet manually."
        exit 1
    fi

    echo "figlet installed successfully!"
else
    echo "figlet is already installed."
fi

