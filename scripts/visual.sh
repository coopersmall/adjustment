#!/bin/bash

set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install package with the detected package manager
install_package() {
    local package_name=$1

    echo "Installing $package_name..."

    if command_exists brew; then
        # macOS with Homebrew
        brew install "$package_name"
    elif command_exists choco; then
        # Windows with Chocolatey
        choco install "$package_name"
    elif command_exists dnf; then
        # Fedora with dnf
        sudo dnf install -y "$package_name"
    elif command_exists pacman; then
        # Arch Linux with pacman
        sudo pacman -Sy --noconfirm "$package_name"
    elif command_exists apt; then
        # Ubuntu/Debian with apt
        sudo apt update
        sudo apt install -y "$package_name"
    else
        echo "Unsupported package manager. Please install $package_name manually."
        exit 1
    fi

    echo "$package_name installed successfully!"
}

# Install figlet
if ! command_exists figlet; then
    install_package "figlet"
else
    echo "figlet is already installed."
fi
