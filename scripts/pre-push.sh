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

# Function to compare two versions
version_compare() {
    if [[ $1 == $2 ]]; then
        return 0
    fi

    IFS='.' read -ra v1 <<< "$1"
    IFS='.' read -ra v2 <<< "$2"

    for ((i=0; i<${#v1[@]}; i++)); do
        if ((10#${v1[i]} > 10#${v2[i]})); then
            return 1
        elif ((10#${v1[i]} < 10#${v2[i]})); then
            return 2
        fi
    done

    return 0
}

# Check if it's the first push to origin
first_push=false
if ! git rev-parse --verify origin/master >/dev/null 2>&1; then
    first_push=true
fi

# Update crate versions if changes were made
if [ "$first_push" = true ]; then
    echo "First push to origin. Bumping versions for all crates."
    cargo update --workspace

    # Increase the patch version for each crate with changes
    for crate in "utils" "common" "macros"; do
        if git log --oneline "origin/master..HEAD" -- . | awk '/Bump/{exit 1}'; then
            crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
            major="${crate_version%%.*}"
            minor="${crate_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
            echo "Bumped ${crate} version to ${new_version}"
            git add "${crate}/Cargo.toml"
        fi
    done

    # Increase the patch version for the workspace if it has changes
    if git log --oneline "origin/master..HEAD" -- . | awk '/Bump/{exit 1}'; then
        workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
        echo "Bumped workspace version to ${new_version}"
        git add Cargo.toml
    fi

    # Commit the changes if there are modifications
    if git diff --cached --quiet; then
        echo "No version changes detected. Nothing to commit."
    else
        git commit -m "Bump versions"
    fi
else
    # Compare crate versions with master and update if necessary
    for crate in "utils" "common" "macros"; do
        crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
        if git log --oneline --name-only "origin/master..HEAD" -- "${crate}/" | grep -q '^'"${crate}/"; then
            master_version=$(git show "origin/master:${crate}/Cargo.toml" | awk -F'"' '/version =/{print $2}')
            if version_compare "$crate_version" "$master_version" && [[ $? == 2 ]]; then
                major="${crate_version%%.*}"
                minor="${crate_version#*.}"
                patch="${minor#*.}"
                new_patch=$((patch + 1))
                new_version="${major}.${minor}.${new_patch}"
                sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
                echo "Bumped ${crate} version to ${new_version}"
                git add "${crate}/Cargo.toml"
            fi
        fi
    done

    # Compare workspace version with master and update if necessary
    workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
    if git log --oneline --name-only "origin/master..HEAD" -- "${crate}/" | grep -q '^'"${crate}/"; then
        master_workspace_version=$(git show "origin/master:Cargo.toml" | awk -F'"' '/version =/{print $2}')
        if version_compare "$workspace_version" "$master_workspace_version" && [[ $? == 2 ]]; then
            major="${workspace_version%%.*}"
            minor="${workspace_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
            echo "Bumped workspace version to ${new_version}"
            git add Cargo.toml
        fi
    fi

    # Commit the changes if there are modifications
    if git diff --cached --quiet; then
        echo "No version changes detected. Nothing to commit."
    else
        git commit -m "Bump versions"
    fi
fi

# Run cargo build
cargo build

# Run cargo test
cargo test

# Run cargo fmt
cargo fmt --all -- --check

# Run cargo clippy
# cargo clippy

